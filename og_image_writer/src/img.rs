use std::collections::HashSet;

use crate::Error;

use super::style::BorderRadius;
use image::{
    load_from_memory_with_format, open, DynamicImage, ImageBuffer, ImageError, ImageFormat, Rgba,
};
use imageproc::drawing::draw_line_segment_mut;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum ImageInputFormat {
    Png,
    Jpeg,
    // WebP,
    // Avif,
}

impl ImageInputFormat {
    pub(super) fn as_image_format(&self) -> ImageFormat {
        match self {
            ImageInputFormat::Png => ImageFormat::Png,
            ImageInputFormat::Jpeg => ImageFormat::Jpeg,
            // ImageInputFormat::WebP => ImageFormat::WebP,
            // ImageInputFormat::Avif => ImageFormat::Avif,
        }
    }
}

pub(super) struct Size {
    pub(super) height: u32,
    pub(super) width: u32,
}

pub(super) struct ImageInfo(pub(super) ImageBuffer<Rgba<u8>, Vec<u8>>, pub(super) Size);

pub(super) fn open_and_resize(src: &str, w: u32, h: u32) -> Result<ImageInfo, Error> {
    let rgba = open(src)?.into_rgba8();
    let buffer = DynamicImage::ImageRgba8(rgba).thumbnail(w, h).into_rgba8();
    let height = buffer.height();
    let width = buffer.width();
    Ok(ImageInfo(buffer, Size { height, width }))
}

pub(super) fn open_and_resize_with_data(
    data: &[u8],
    w: u32,
    h: u32,
    format: ImageInputFormat,
) -> Result<ImageInfo, ImageError> {
    let rgba = load_from_memory_with_format(data, format.as_image_format())?.into_rgba8();
    let buffer = DynamicImage::ImageRgba8(rgba).thumbnail(w, h).into_rgba8();
    let height = buffer.height();
    let width = buffer.width();
    Ok(ImageInfo(buffer, Size { height, width }))
}

// see: https://stackoverflow.com/questions/48478497/javascript-gecko-border-radius-adaptation-on-html-canvas-css-border-radius
// fn calculate_border_radius(r: &mut BorderRadius, w:f32, h: f32) {
//     let BorderRadius(tl, tr, bl, br) = r;
//     let max_radius_width = cmp::max(*tl + *tr, *bl + *br) as f32;
//     let max_radius_height = cmp::max(*tl + *bl, *tr + *br) as f32;
//     let width_ratio = w / max_radius_width;
//     let height_ratio = h / max_radius_height;
//     let scale_ratio = f32::min(f32::min(width_ratio, height_ratio), 1.);

//     *tl = (*tl as f32 * scale_ratio) as u32;
//     *tr = (*tr as f32 * scale_ratio) as u32;
//     *bl = (*tr as f32 * scale_ratio) as u32;
//     *br = (*tr as f32 * scale_ratio) as u32;
// }

fn while_radius<F>(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, r: i32, size: (i32, i32), draw: F)
where
    F: Fn(&mut ImageBuffer<Rgba<u8>, Vec<u8>>, (i32, i32), (i32, i32), Rgba<u8>),
{
    if r <= 0 {
        return;
    }

    let r = (r as f32 / 1.25) as i32;
    let mut x = 0i32;
    let mut y = r;

    let color = Rgba([0, 0, 0, 0]);
    let mut p = 1 - r;

    let (x0, y0) = size;

    while x <= y {
        draw(img, (x0, y0), (x, y), color);

        x += 1;
        if p < 0 {
            p += 2 * x + 1;
        } else {
            y -= 1;
            p += 2 * (x - y) + 1;
        }
    }
}

// fn border_top_left_radius(buf: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, r: i32) {
//     while_radius(buf, r - 1, (0, 0), |img, (x0, y0), (x, y), color| {
//         draw_line_segment_mut(
//             img,
//             ((x0 + x) as f32, (y0) as f32),
//             ((x0) as f32, (y0 + y) as f32),
//             color,
//         );

//         draw_line_segment_mut(
//             img,
//             ((x0) as f32, (y0 + x) as f32),
//             ((x0 + y) as f32, (y0) as f32),
//             color,
//         );
//     });
// }

// fn border_top_right_radius(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, r: i32) {
//     let width = img.width() as i32;

//     while_radius(
//         img,
//         r - 1,
//         (width - 1, 0),
//         |img, (x0, y0), (x, y), color| {
//             draw_line_segment_mut(
//                 img,
//                 ((x0 - x) as f32, (y0) as f32),
//                 ((x0) as f32, (y0 + y) as f32),
//                 color,
//             );

//             draw_line_segment_mut(
//                 img,
//                 ((x0) as f32, (y0 + x) as f32),
//                 ((x0 - y) as f32, (y0) as f32),
//                 color,
//             );
//         },
//     );
// }

// fn border_bottom_left_radius(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, r: i32) {
//     let height = img.height() as i32;

//     while_radius(
//         img,
//         r - 1,
//         (0, height - 1),
//         |img, (x0, y0), (x, y), color| {
//             draw_line_segment_mut(
//                 img,
//                 ((x0 + x) as f32, (y0) as f32),
//                 ((x0) as f32, (y0 - y) as f32),
//                 color,
//             );

//             draw_line_segment_mut(
//                 img,
//                 ((x0) as f32, (y0 - x) as f32),
//                 ((x0 + y) as f32, (y0) as f32),
//                 color,
//             );
//         },
//     );
// }

// fn border_bottom_right_radius(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, r: i32) {
//     let width = img.width() as i32;
//     let height = img.height() as i32;

//     while_radius(
//         img,
//         r - 1,
//         (width - 1, height - 1),
//         |img, (x0, y0), (x, y), color| {
//             draw_line_segment_mut(
//                 img,
//                 ((x0 - x) as f32, (y0) as f32),
//                 ((x0) as f32, (y0 - y) as f32),
//                 color,
//             );

//             draw_line_segment_mut(
//                 img,
//                 ((x0) as f32, (y0 - x) as f32),
//                 ((x0 - y) as f32, (y0) as f32),
//                 color,
//             );
//         },
//     );
// }

// TODO: Support border
pub(super) fn round(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, radius: &mut BorderRadius) {
    let (width, height) = img.dimensions();
    assert!(radius.0 + radius.1 <= width);
    assert!(radius.3 + radius.2 <= width);
    assert!(radius.0 + radius.3 <= height);
    assert!(radius.1 + radius.2 <= height);
    // border_top_left_radius(img, radius.0 as i32);
    // top right
    border_top_right_radius(img, radius.1, |x, y| (width - x, y - 1));
    // border_bottom_right_radius(img, radius.2 as i32);
    // border_bottom_left_radius(img, radius.3 as i32);

    // top left
    border_top_right_radius(img, radius.0, |x, y| (x - 1, y - 1));
    // top right
    border_top_right_radius(img, radius.1, |x, y| (width - x, y - 1));
    // bottom right
    border_top_right_radius(img, radius.2, |x, y| (width - x, height - y));
    // bottom left
    border_top_right_radius(img, radius.3, |x, y| (x - 1, height - y));
}

fn border_top_right_radius(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    r: u32,
    coordinates: impl Fn(u32, u32) -> (u32, u32),
) {
    let mut positions = HashSet::new();

    let (width, height) = img.dimensions();
    // img[(0, 0)] = Rgba([255, 255, 255, 255]);
    let x0 = width - r;
    let y0 = height - r;

    let r0 = r;

    // 16x antialiasing: 16x16 grid creates 256 possible shades, great for u8!
    let r = 16 * r;

    let mut x = 0;
    let mut y = r - 1;
    let mut p: i32 = 2 - r as i32;

    // ...

    let mut alpha: u16 = 0;
    let mut skip_draw = true;

    let mut draw = |img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, alpha, x, y| {
        println!("{x} {y} {r}");
        let pos = coordinates(r0 - x, r0 - y);
        println!("DRRR");
        debug_assert!((1..=256).contains(&alpha));
        assert!(positions.insert((x0 + x, y0 + y)));
        let pixel_alpha = &mut img[pos].0[3];
        *pixel_alpha = (((alpha - 1) * *pixel_alpha as u16 + 127) / 255) as u8;
        //img[(x0 + x, y0 + y)] = Rgba([0, 0, (alpha - 1) as u8, 255]);

        img[pos] = Rgba([0, 0, (alpha - 1) as u8, 255])
    };

    'l: loop {
        println!("--------");
        // remove contents below current position
        {
            let i = x / 16;
            for j in y / 16 + 1..r0 {
                let pos = coordinates(r0 - i, r0 - j);
                img[pos].0[3] = 0;
                img[pos] = Rgba([255, 0, 0, 255]);
            }
        }
        // remove contents right of current position mirrored
        {
            let j = x / 16;
            for i in y / 16 + 1..r0 {
                let pos = coordinates(r0 - i, r0 - j);
                img[pos].0[3] = 0;
                img[pos] = Rgba([255, 0, 0, 255]);
            }
        }

        // debug // img[(x0 + x / 16, y0 + y / 16)] = Rgba([0, 255, 0, 255]);

        // draw when moving to next pixel in x-direction
        if !skip_draw {
            draw(img, alpha, x / 16 - 1, y / 16);
            draw(img, alpha, y / 16, x / 16 - 1);
            alpha = 0;
        }

        for _ in 0..16 {
            skip_draw = false;

            dbg!((x / 16, x % 16), (y / 16, y % 16), p);

            println!("{alpha}, {}", y as u8 % 16 + 1);
            alpha += y as u16 % 16 + 1;
            if p < 0 {
                x += 1;
                p += (2 * x + 2) as i32;
                if x >= y {
                    dbg!((x / 16, x % 16), (y / 16, y % 16), p);
                    break 'l;
                }
            } else {
                // draw when moving to next pixel in y-direction
                if y % 16 == 0 {
                    draw(img, alpha, x / 16, y / 16);
                    draw(img, alpha, y / 16, x / 16);
                    skip_draw = true;
                    alpha = (x + 1) as u16 % 16 * 16;
                }

                x += 1;
                p -= (2 * (y - x) + 2) as i32;
                y -= 1;
                if x >= y {
                    dbg!((x / 16, x % 16), (y / 16, y % 16), p);
                    break 'l;
                }
            }
        }
    }

    // one corner pixel left
    if x / 16 == y / 16 {
        let s = y as u16 % 16 + 1;
        let alpha = 2 * alpha - s * s;
        draw(img, alpha, x / 16, y / 16);
    }

    // remove remaining square of content in the corner
    for i in y / 16 + 1..r0 {
        // yes! That's a `y` intentionally!
        for j in y / 16 + 1..r0 {
            let pos = coordinates(r0 - i, r0 - j);
            img[pos].0[3] = 0;
            img[pos] = Rgba([100, 0, 0, 255]);
        }
    }

    // let width = img.width() as i32;

    // while_radius(
    //     img,
    //     r - 1,
    //     (width - 1, 0),
    //     |img, (x0, y0), (x, y), color| {
    //         draw_line_segment_mut(
    //             img,
    //             ((x0 - x) as f32, (y0) as f32),
    //             ((x0) as f32, (y0 + y) as f32),
    //             color,
    //         );

    //         draw_line_segment_mut(
    //             img,
    //             ((x0) as f32, (y0 + x) as f32),
    //             ((x0 - y) as f32, (y0) as f32),
    //             color,
    //         );
    //     },
    // );
}
