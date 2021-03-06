
/*
 * A ditty in this context is a "screen" displayed in a particular way. This could be an intro
 * sequence or a cut-scene, or the main game, or the "new game, continue, etc" screen.
 */
extern crate sdl2;

use std::borrow::Borrow;

use self::sdl2::pixels::Color;
use self::sdl2::render::Renderer;
use self::sdl2::render::Texture;
use self::sdl2::rect::Point;
use self::sdl2::rect::Rect;

use spath::PathElem;
use rendererutils::RendererUtils;
use utils::FatalAction;

pub trait Ditty {
    fn init(&mut self, renderer: &mut Renderer);
    fn render(&mut self, renderer: &mut Renderer, width: u32, height: u32);
}

pub struct BackgroundDitty {
    logo: Option<Texture>
}

impl BackgroundDitty {
    pub fn new() -> BackgroundDitty {
        BackgroundDitty {
            logo: None
        }
    }

    fn draw_tex(renderer: &mut Renderer, image: &Texture, screen_width: u32, screen_height: u32) {
        let fgq = image.query();
        let fg_x = (screen_width / 2 - fgq.width / 2) as i32;
        let fg_y = (screen_height / 2 - fgq.height / 2) as i32;
        renderer.render_texture(&image, fg_x, fg_y);
    }

}

impl Ditty for BackgroundDitty {
    fn init(&mut self, renderer: &mut Renderer) {
        renderer.load_bmp("logo.bmp").map(|logo| {
            self.logo = Some(logo)
        }).or_die("load bmp");
    }

    fn render(&mut self, renderer: &mut Renderer, width: u32, height: u32) {
        let logo = self.logo.as_ref().unwrap();
        BackgroundDitty::draw_tex(renderer, &logo, width, height);
    }
}

pub struct PathDitty {
    paths: Vec<Vec<PathElem>>
}

impl PathDitty {
    pub fn new(paths: Vec<Vec<PathElem>>) -> PathDitty {
        PathDitty { paths: paths }
    }
}

impl Ditty for PathDitty {
    fn init(&mut self, renderer: &mut Renderer) {
    }

    fn render(&mut self, renderer: &mut Renderer, width: u32, height: u32) {
        for path in &self.paths {
            let mut cp = Point::new(0, 0);
            for elem in path {
                match elem {
                    &PathElem::MoveTo { x, y } => {
                        cp = Point::new(x as i32, y as i32);
                    },
                    &PathElem::LineTo { x, y } => {
                        let np = Point::new(x as i32, y as i32);
                        renderer.set_draw_color(Color::RGB(255, 0, 0));
                        renderer.draw_line(cp, np);
                        cp = np;
                    },
                    &PathElem::CurveTo { x1, y1, x2, y2, x, y } => {
                        let np = Point::new(x as i32, y as i32);
                        let cp1 = Point::new(x1 as i32, y1 as i32);
                        let cp2 = Point::new(x2 as i32, y2 as i32);
                        renderer.set_draw_color(Color::RGB(64, 64, 64));
                        renderer.draw_line(cp, cp1);
                        renderer.draw_line(np, cp2);

                        let cpr1 = Rect::new_unwrap((x1 as i32) - 1, (y1 as i32) - 1, 2, 2);
                        let cpr2 = Rect::new_unwrap((x2 as i32) - 1, (y2 as i32) - 1, 2, 2);
                        renderer.set_draw_color(Color::RGB(255, 128, 0));
                        renderer.draw_rect(cpr1);
                        renderer.draw_rect(cpr2);

                        renderer.draw_line(cp, np);
                        cp = np;
                    },
                    &PathElem::QuadraticTo { x, y, .. } => {
                        let np = Point::new(x as i32, y as i32);
                        renderer.set_draw_color(Color::RGB(0, 255, 0));
                        renderer.draw_line(cp, np);
                        cp = np;
                    },
                    &PathElem::ArcTo { x, y, .. } => {
                        let np = Point::new(x as i32, y as i32);
                        renderer.set_draw_color(Color::RGB(255, 255, 0));
                        renderer.draw_line(cp, np);
                        cp = np;
                    }
                }
            }
        }
        renderer.set_draw_color(Color::RGB(0, 0, 0));
    }
}
