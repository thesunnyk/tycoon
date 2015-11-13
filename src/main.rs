
mod gameloop;
mod utils;
mod rendererutils;
mod spath;
mod ditty;
mod svg;

use gameloop::GameLoop;
use spath::PathElem;
use utils::FatalAction;

fn main() {

    let mut paths = svg::get_paths("assets/logo.svg").iter().map(|s| spath::read_path(s)).collect();

    let ditty = ditty::PathDitty::new(paths);

    let mainloop = GameLoop::new().or_die("create Game Loop");
    mainloop.run(ditty).or_die("run Game Loop");
}
