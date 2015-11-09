extern crate sdl2;

use self::sdl2::Sdl;
use self::sdl2::EventPump;
use self::sdl2::SdlResult;
use self::sdl2::TimerSubsystem;
use self::sdl2::VideoSubsystem;
use self::sdl2::event::Event;
use self::sdl2::render::Renderer;
use self::sdl2::render::Texture;

use utils::ResultMap;
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
    pub fn new(width: u32, height: u32) -> SdlResult<GameLoop> {
        sdl2::init().res_map(|context| context.video().map(|video| GameLoop {
                context: context,
                width: width,
                height: height,
                video: video
            }))
    }

    fn build_renderer(&self) -> SdlResult<Renderer> {
        self.video.window("Cretacious Island", self.width, self.height).build()
            .res_map(|builder| builder.renderer().accelerated().present_vsync().build())
    }

    fn do_run(&self,
              mut renderer: Renderer,
              mut timer: TimerSubsystem,
              mut events: EventPump,
              background: Texture,
              image: Texture) {
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

    pub fn run(&self) -> SdlResult<()> {
        self.build_renderer().res_map(|renderer| self.context.timer()
            .res_map(|timer| self.context.event_pump()
            .res_map(|events| renderer.load_bmp("background.bmp")
            .res_map(|background| renderer.load_bmp("image.bmp")
            .map(|image| self.do_run(renderer, timer, events, background, image))))))
    }
}


