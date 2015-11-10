extern crate sdl2;

use self::sdl2::Sdl;
use self::sdl2::EventPump;
use self::sdl2::SdlResult;
use self::sdl2::TimerSubsystem;
use self::sdl2::VideoSubsystem;
use self::sdl2::event::Event;
use self::sdl2::render::Renderer;
use self::sdl2::render::Texture;

use rendererutils::RendererUtils;

fn draw_tex(renderer: &mut Renderer, image: &Texture, screen_width: u32, screen_height: u32) {
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
    pub fn new() -> SdlResult<GameLoop> {
        sdl2::init().and_then(|context| context.video()
            .and_then(|video| video.current_display_mode(0)
            .map(|curmode| {
                println!("Using mode: {}x{}", curmode.w, curmode.h);
                GameLoop {
                    context: context,
                    width: curmode.w as u32,
                    height: curmode.h as u32,
                    video: video
                }
            })))
    }

    fn build_renderer(&self) -> SdlResult<Renderer> {
        self.video.window("Cretacious Island", self.width, self.height).fullscreen().build()
            .and_then(|builder| builder.renderer().accelerated().present_vsync().build())
    }

    fn do_run(&self,
              mut renderer: Renderer,
              mut timer: TimerSubsystem,
              mut events: EventPump,
              logo: Texture) {
        loop {
            let start_ticks = timer.ticks();
            // Events
            for ev in events.poll_iter() {
                match ev {
                    Event::Quit {timestamp: _} => {
                        println!("Quitting");
                        break;
                    }
                    _ => {}
                }
            }
            // Logic
            // Rendering
            renderer.clear();
            draw_tex(&mut renderer, &logo, self.width, self.height);
            renderer.present();

            while timer.ticks() - start_ticks < 1000 / 60 {
                timer.delay(1);
            }
        }
    }

    pub fn run(&self) -> SdlResult<()> {
        self.build_renderer().and_then(|renderer| self.context.timer()
            .and_then(|timer| self.context.event_pump()
            .and_then(|events| renderer.load_bmp("logo.bmp")
            .map(|logo| self.do_run(renderer, timer, events, logo)))))
    }
}


