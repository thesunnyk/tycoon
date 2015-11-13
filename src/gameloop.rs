extern crate sdl2;

use self::sdl2::Sdl;
use self::sdl2::EventPump;
use self::sdl2::SdlResult;
use self::sdl2::TimerSubsystem;
use self::sdl2::VideoSubsystem;
use self::sdl2::event::Event;
use self::sdl2::render::Renderer;

use ditty::Ditty;

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
            .map(move |curmode| {
                println!("Using mode: {}x{}", curmode.w, curmode.h);
                GameLoop {
                    context: context,
                    video: video,
                    width: curmode.w as u32,
                    height: curmode.h as u32,
                }
            })))
    }

    fn build_renderer(&self) -> SdlResult<Renderer> {
        self.video.window("Cretacious Island", self.width, self.height).fullscreen().build()
            .and_then(|builder| builder.renderer().accelerated().present_vsync().build())
    }

    fn do_run<T: Ditty>(&self,
              mut renderer: Renderer,
              mut timer: TimerSubsystem,
              mut events: EventPump,
              mut ditty: T) {
        ditty.init(&mut renderer);
        loop {
            let start_ticks = timer.ticks();
            // Events
            for ev in events.poll_iter() {
                match ev {
                    Event::Quit {timestamp: _} => {
                        println!("Quitting");
                        return;
                    }
                    _ => {}
                }
            }
            // Logic
            // Rendering
            renderer.clear();
            ditty.render(&mut renderer, self.width, self.height);
            renderer.present();

            while timer.ticks() - start_ticks < 1000 / 60 {
                timer.delay(1);
            }
        }
    }

    pub fn run<T: Ditty>(&self, ditty: T) -> SdlResult<()> {
        self.build_renderer().and_then(|renderer| self.context.timer()
            .and_then(|timer| self.context.event_pump()
            .map(|events| self.do_run(renderer, timer, events, ditty))))
    }
}


