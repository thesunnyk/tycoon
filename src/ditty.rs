
/*
 * A ditty in this context is a "screen" displayed in a particular way. This could be an intro
 * sequence or a cut-scene, or the main game, or the "new game, continue, etc" screen.
 */
extern crate sdl2;

use self::sdl2::render::Renderer;
use self::sdl2::render::Texture;

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
    pub fn new<'a>() -> BackgroundDitty {
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
        renderer.clear();
        let logo = self.logo.as_ref().unwrap();
        BackgroundDitty::draw_tex(renderer, &logo, width, height);
        renderer.present();
    }
}
