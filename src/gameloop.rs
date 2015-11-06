extern crate sdl2;

use self::sdl2::Sdl;
use self::sdl2::VideoSubsystem;
use self::sdl2::event::Event;
use self::sdl2::render::Renderer;
use self::sdl2::render::Texture;

use utils::FatalAction;
use rendererutils::RendererUtils;

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

pub struct GameLoop {
    context: Sdl,
    video: VideoSubsystem,
    width: u32,
    height: u32,
}

impl GameLoop {
    pub fn new(width: u32, height: u32) -> GameLoop {
        let context = sdl2::init().or_die("create SDL context");
        let video = context.video().or_die("create SDL video");
        GameLoop {
            context: context,
            width: width,
            height: height,
            video: video
        }
    }

    fn build_renderer(&self) -> Renderer {
        let builder = self.video.window("Cretacious Island", self.width, self.height).build()
            .or_die("create Window");
        builder.renderer().accelerated().present_vsync().build()
                .or_die("create Renderer")
    }

    pub fn run(&self) {
        let mut renderer = self.build_renderer();
        let mut timer = self.context.timer().or_die("create Timer");
        let mut events = self.context.event_pump().or_die("create Events");
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
            draw_fg(&mut renderer, &image, self.width, self.height);
            renderer.present();

            while timer.ticks() - start_ticks < 1000 / 60 {
                timer.delay(1);
            }
        }
    }
}


