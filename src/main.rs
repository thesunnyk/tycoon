extern crate sdl2;

use std::path::Path;
use sdl2::SdlResult;
use sdl2::surface::Surface;
use sdl2::render::Renderer;
use sdl2::render::Texture;

trait LoadsBmp {
    fn load_bmp<P: AsRef<Path>>(&self, name: P) -> SdlResult<Texture>;
}

impl<'a> LoadsBmp for Renderer<'a> {
    fn load_bmp<P: AsRef<Path>>(&self, name: P) -> SdlResult<Texture> {
        let surface = Surface::load_bmp(name).unwrap();
        self.create_texture_from_surface(surface)
    }
}


fn main() {
    println!("Hello, world!");
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let mut timer = context.timer().unwrap();

    let builder = video.window("Cretacious Island", 640, 480).build().unwrap();
    let mut renderer = builder.renderer().accelerated().present_vsync().build().unwrap();
    let texture = renderer.load_bmp("hello.bmp").unwrap();
    renderer.clear();
    renderer.copy(&texture, None, None);
    renderer.present();
    timer.delay(5000);

}
