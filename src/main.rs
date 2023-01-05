extern crate rustbitmap;

use rustbitmap::BitMap;
use rustbitmap::Rgba;

use bitmaps::Bitmap;

extern "C" {
    fn get_font() -> *const u8;
}

fn font_draw(mut bitmap: &mut BitMap<>, font: &[u8], pos: (u16, u16)) -> Result<(), String> {
    let colum = font[0];
    let pixelcolor = 1;

    println!("Coluna {}", colum);

    for pages in 0..2 {
        let fist_page = pages * 8;

        for linerow in 1..colum + 1 {
            let mut data = font[((pages * colum) + linerow) as usize];
            //println!("Valor data = {}", data);
            for bit_row in 0..8 {
                let color_to_write = 0 ^ !(data & 1);

                let pixel_color = if color_to_write == 255 { Rgba::white() } else { Rgba::black() };
                &bitmap.set_pixel((pos.0 + (linerow) as u16 - 1) as u32, (pos.1 + bit_row + fist_page as u16) as u32, pixel_color).map_err(|x| { x.to_string() })?;

                data >>= 1;
            }
        }
    }

    Ok(())
}

fn main() {
    let font = unsafe { std::slice::from_raw_parts(get_font(), 2679) };
    println!("Font: {}", font[0]);

    let mut bitmap = BitMap::new(1000, 1000);
    let mut pos = (0, 0);
    let mut limite_min = 0;
    let mut limite_max: u32 = (font[0] * 2) as u32;
    let mut font_copy: &[u8];

    while limite_max < 2679 {
        font_copy = &font[limite_min..((limite_max + 1) as usize)];
        font_draw(&mut bitmap, &font_copy, pos).expect("Erro na leitura da font");
        limite_min = (limite_max + 1) as usize;
        println!("{}", limite_min);
        limite_max = ((font[(limite_max + 1) as usize] * 2) as u32 + (limite_max + 1)) as u32;
        pos.0 += 16;
        if pos.0 > 500 {
            pos.0 = 0;
            pos.1 += 16;
        }
    }

    bitmap.save_as("teste.bmp");
    bitmap.simplify_and_save_as("teste.bmp");
}
