extern crate sdl2;

use std::path::Path;

use sdl2::SdlResult;
use sdl2::surface::Surface;
use sdl2::event::Event;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::rect::Rect;

trait RendererUtils {
    fn load_bmp<P: AsRef<Path>>(&self, name: P) -> SdlResult<Texture>;
    fn render_texture(&mut self, tex: &Texture, x: i32, y: i32);
}

impl<'a> RendererUtils for Renderer<'a> {
    fn load_bmp<P: AsRef<Path>>(&self, name: P) -> SdlResult<Texture> {
        let surface = Surface::load_bmp(name).or_die("load BMP");
        self.create_texture_from_surface(surface)
    }

    fn render_texture(&mut self, tex: &Texture, x: i32, y: i32) {
        let tq = tex.query();
        let rect = Rect::new(x, y, tq.width, tq.height).unwrap();
        self.copy(tex, None, rect);
    }

}

trait FatalAction<T, E> {
    fn or_die(self, s: &str) -> T;
}

impl<T, E: std::fmt::Debug> FatalAction<T, E> for Result<T, E> {
    fn or_die(self, s: &str) -> T {
        print!("Trying to {}...", s);
        match self {
            Ok(t) => {
                println!("OK");
                t
            },
            Err(e) => {
                panic!("ERROR: {:?}", e);
            }
        }
    }
}

fn draw_bg(renderer: &mut Renderer, background: &Texture) {
    let bgq = background.query();
    for i in &[0,bgq.width] {
        for j in &[0,bgq.height] {
            renderer.render_texture(background, *i as i32, *j as i32);
        }
    }
}

fn draw_fg(renderer: &mut Renderer, image: &Texture, screen_width: u32, screen_height: u32) {
    let fgq = image.query();
    let fg_x = (screen_width / 2 - fgq.width / 2) as i32;
    let fg_y = (screen_height / 2 - fgq.height / 2) as i32;
    renderer.render_texture(&image, fg_x, fg_y);
}

fn main() {
    let screen_width = 640;
    let screen_height = 480;

    let context = sdl2::init().or_die("create SDL context");
    let video = context.video().or_die("create SDL video");
    let mut timer = context.timer().or_die("create Timer");
    let mut events = context.event_pump().or_die("create Events");

    let builder = video.window("Cretacious Island", screen_width, screen_height).build().or_die("create Window");
    let mut renderer = builder.renderer().accelerated().present_vsync().build().or_die("create Renderer");
    let background = renderer.load_bmp("background.bmp").or_die("create texture");
    let image = renderer.load_bmp("image.bmp").or_die("create texture");
    let mut quit = false;
    while !quit {
        let start_ticks = timer.ticks();
        // Events
        for ev in events.poll_iter() {
            match ev {
                Event::Quit {timestamp: _} => {
                    quit = true;
                }
                _ => {}
            }
        }
        // Logic
        // Rendering
        renderer.clear();
        draw_bg(&mut renderer, &background);
        draw_fg(&mut renderer, &image, screen_width, screen_height);
        renderer.present();

        while timer.ticks() - start_ticks < 1000 / 60 {
            timer.delay(1);
        }
    }
}
