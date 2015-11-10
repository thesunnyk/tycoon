
mod gameloop;
mod utils;
mod rendererutils;
// Just to make spath compile
mod spath;

use gameloop::GameLoop;
use utils::FatalAction;

fn main() {
    let mainloop = GameLoop::new().or_die("create Game Loop");

    mainloop.run().or_die("run Game Loop");
}
