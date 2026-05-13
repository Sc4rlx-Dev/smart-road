use std::time::Duration;

use image::{Rgba, RgbaImage};
use rusttype::{point, Font, Scale};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;

pub fn text_to_image(text: &str) -> RgbaImage {
    let font_data = include_bytes!("../assets/DejaVuSans1.ttf") as &[u8];
    let font = Font::try_from_bytes(font_data).expect("font load failed");
    let scale = Scale::uniform(20.0);
    let v_metrics = font.v_metrics(scale);

    let mut img = RgbaImage::new(380, 40);

    for (i, c) in text.chars().enumerate() {
        let glyph = font
            .glyph(c)
            .scaled(scale)
            .positioned(point(i as f32 * 11.0, v_metrics.ascent));
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, v| {
                let px = gx as i32 + bb.min.x;
                let py = gy as i32 + bb.min.y;
                if px >= 0 && py >= 0 && (px as u32) < img.width() && (py as u32) < img.height() {
                    img.put_pixel(px as u32, py as u32, Rgba([255, 255, 255, (v * 255.0) as u8]));
                }
            });
        }
    }

    img
}

pub fn draw_confirm_exit(
    canvas: &mut Canvas<Window>,
    nbr_cars: i32,
    max_velocity: f32,
    min_velocity: f32,
    max_timer: &Duration,
    min_timer: &Duration,
    close_calls: i32,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.fill_rect(Rect::new(200, 220, 400, 360))?;

    let lines = [
        format!("Total vehicles: {}", nbr_cars),
        format!("Max velocity:   {:.2}", max_velocity),
        format!("Min velocity:   {:.2}", min_velocity),
        format!("Max time:       {:.2?}", max_timer),
        format!("Min time:       {:.2?}", min_timer),
        format!("Close calls:    {}", close_calls),
        String::from("Press Esc to quit"),
    ];

    let texture_creator = canvas.texture_creator();
    for (i, line) in lines.iter().enumerate() {
        let img = text_to_image(line);
        let (w, h) = (img.width(), img.height());
        let mut surface = Surface::new(w, h, PixelFormatEnum::RGBA32).map_err(|e| e.to_string())?;
        surface.with_lock_mut(|buf: &mut [u8]| buf.copy_from_slice(&img));
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let target = Rect::new(220, 250 + (i as i32) * 40, w, h);
        canvas.copy(&texture, None, Some(target))?;
    }

    canvas.present();
    Ok(())
}
