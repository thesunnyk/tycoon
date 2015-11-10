extern crate sdl2;

use std::path::Path;

use self::sdl2::surface::Surface;
use self::sdl2::SdlResult;
use self::sdl2::rect::Rect;
use self::sdl2::render::Renderer;
use self::sdl2::render::Texture;

pub trait RendererUtils {
    fn load_bmp<P: AsRef<Path>>(&self, name: P) -> SdlResult<Texture>;
    fn render_texture(&mut self, tex: &Texture, x: i32, y: i32);
}

impl<'a> RendererUtils for Renderer<'a> {
    fn load_bmp<P: AsRef<Path>>(&self, name: P) -> SdlResult<Texture> {
        Surface::load_bmp(name).and_then(|surface| self.create_texture_from_surface(surface))
    }

    fn render_texture(&mut self, tex: &Texture, x: i32, y: i32) {
        let tq = tex.query();
        let rect = Rect::new(x, y, tq.width, tq.height).unwrap();
        self.copy(tex, None, rect);
    }

}

