
mod gameloop;
mod utils;
mod rendererutils;

use gameloop::GameLoop;

fn main() {
    let mainloop = GameLoop::new(640, 480);

    mainloop.run();
}
