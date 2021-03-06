#[cfg(feature = "default")]
extern crate sensehat_screen;

#[cfg(feature = "default")]
use sensehat_screen::Rotate;
#[cfg(feature = "default")]
use sensehat_screen::{font_to_pixel_frame, FontCollection, PixelColor, PixelFrame, Screen};

#[cfg(not(feature = "default"))]
fn main() {
    unimplemented!("This examples needs the 'default' features.");
}
#[cfg(feature = "default")]
fn main() {
    let mut screen = Screen::open("/dev/fb1").unwrap();
    let fonts = FontCollection::new();

    for &(sym, color) in &[('Ñ', PixelColor::YELLOW), ('ó', PixelColor::MAGENTA)] {
        let font = fonts.get(sym).unwrap();
        let symbol = font_to_pixel_frame(&font.byte_array(), color);
        let symbol_90 = symbol.rotate(Rotate::Ccw90);
        let symbol_180 = symbol.rotate(Rotate::Ccw180);
        let symbol_270 = symbol.rotate(Rotate::Ccw270);
        for _ in 0..=4 {
            screen.write_frame(&symbol.frame_line());
            ::std::thread::sleep(::std::time::Duration::from_millis(500));
            screen.write_frame(&symbol_90.frame_line());
            ::std::thread::sleep(::std::time::Duration::from_millis(500));
            screen.write_frame(&symbol_180.frame_line());
            ::std::thread::sleep(::std::time::Duration::from_millis(500));
            screen.write_frame(&symbol_270.frame_line());
            ::std::thread::sleep(::std::time::Duration::from_millis(500));
        }
        screen.write_frame(&PixelFrame::new(&[PixelColor::BLACK; 64]).frame_line());
    }
}
