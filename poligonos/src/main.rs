// Poligono 1

use image::{ImageBuffer, RgbImage, Rgb};

type Point = (i32, i32);

fn draw_polygon(polygon: &[Point], image: &mut RgbImage, fill_color: Rgb<u8>, border_color: Rgb<u8>) {
    let height = image.height() as i32;

    // bordes
    for i in 0..polygon.len() {
        let (x0, y0) = polygon[i];
        let (x1, y1) = polygon[(i + 1) % polygon.len()];
        draw_line(x0, y0, x1, y1, image, border_color);
    }

    // relleno
    let min_y = polygon.iter().map(|(_, y)| *y).min().unwrap_or(0);
    let max_y = polygon.iter().map(|(_, y)| *y).max().unwrap_or(height);

    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        for i in 0..polygon.len() {
            let (x0, y0) = polygon[i];
            let (x1, y1) = polygon[(i + 1) % polygon.len()];

            if (y0 <= y && y1 > y) || (y1 <= y && y0 > y) {
                let dy = y1 - y0;
                let dx = x1 - x0;
                if dy != 0 {
                    let x = x0 + dx * (y - y0) / dy;
                    intersections.push(x);
                }
            }
        }

        intersections.sort_unstable();

        for pair in intersections.chunks(2) {
            if let [x_start, x_end] = pair {
                for x in *x_start..=*x_end {
                    if x >= 0 && x < image.width() as i32 && y >= 0 && y < height {
                        image.put_pixel(x as u32, y as u32, fill_color);
                    }
                }
            }
        }
    }
}

fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut RgbImage, color: Rgb<u8>) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let (mut x, mut y) = (x0, y0);

    loop {
        if x >= 0 && x < image.width() as i32 && y >= 0 && y < image.height() as i32 {
            image.put_pixel(x as u32, y as u32, color);
        }
        if x == x1 && y == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

fn main() {
    let mut image: RgbImage = ImageBuffer::new(800, 600);

    let poly1 = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)
    ];

    draw_polygon(&poly1, &mut image, Rgb([255, 255, 0]), Rgb([255, 255, 255]));
    
    image.save("out.bmp").unwrap();
    image.save("out.png").unwrap(); // Lo coloque para visualizar las imagenes desde el git más facil
}
