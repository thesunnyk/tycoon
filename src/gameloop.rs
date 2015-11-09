extern crate sdl2;

use self::sdl2::Sdl;
use self::sdl2::EventPump;
use self::sdl2::SdlResult;
use self::sdl2::TimerSubsystem;
use self::sdl2::VideoSubsystem;
use self::sdl2::event::Event;
use self::sdl2::render::Renderer;
use self::sdl2::render::Texture;

use utils::FatalAction;
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

struct RunLoopData<'a> {
    renderer: Renderer<'a>,
    timer: TimerSubsystem,
    events: EventPump
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

    fn build_runloop_data(&self) -> SdlResult<RunLoopData> {
        self.build_renderer().res_map(|renderer| self.context.timer()
            .res_map(|timer| self.context.event_pump().map(|events| RunLoopData {
                renderer: renderer,
                timer: timer,
                events: events
            })))
    }

    pub fn do_run(&self, mut rd: RunLoopData) {
        let background = rd.renderer.load_bmp("background.bmp").or_die("create texture");
        let image = rd.renderer.load_bmp("image.bmp").or_die("create texture");

        let mut quit = false;
        while !quit {
            let start_ticks = rd.timer.ticks();
            // Events
            for ev in rd.events.poll_iter() {
                match ev {
                    Event::Quit {timestamp: _} => {
                        quit = true;
                    }
                    _ => {}
                }
            }
            // Logic
            // Rendering
            rd.renderer.clear();
            draw_bg(&mut rd.renderer, &background);
            draw_fg(&mut rd.renderer, &image, self.width, self.height);
            rd.renderer.present();

            while rd.timer.ticks() - start_ticks < 1000 / 60 {
                rd.timer.delay(1);
            }
        }
    }

    pub fn run(&self) -> SdlResult<()> {
        self.build_runloop_data().map(|rd| self.do_run(rd))
    }
}


