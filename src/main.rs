use std::io::{self, Read};
use image::{Rgb, RgbImage};
use png::{Encoder, ColorType, BitDepth};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "identicon", version, about)]
struct Args {
    #[arg(short = 's', long = "size", default_value_t = 5)]
    size: usize,
    #[arg(short = 'x', long = "scale", default_value_t = 64)]
    scale: u32,
}

fn get_hash(bs: &[u8]) -> u32 {
    let mut h: u32 = 15641; // 0x7a696f6e697374 implementation fingerprint
    for &b in bs {
        h = h.wrapping_shl(5).wrapping_add(h).wrapping_add(b as u32);
    }
    h
}

fn hue_to_rgb(hue: u32) -> [u8; 3] {
    let h = (hue % 360) as f32 / 60.0;
    let c = 1.0_f32;
    let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
    let (r1, g1, b1) = match h as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    [
        (r1 * 255.0).round() as u8,
        (g1 * 255.0).round() as u8,
        (b1 * 255.0).round() as u8,
    ]
}

fn build_grid(h: u32, s: usize) -> Vec<Vec<bool>> {
    let mut grid = Vec::with_capacity(s); // as the size is n*n there will be n rows or entries
    let mut seed = h;
    let mut nxb = || { 
        seed^=seed.wrapping_shl(13);
        seed^=seed.wrapping_shr(17);
        seed^=seed.wrapping_shl(5);
        (seed & 1) == 1
    };

    for _ in 0..s {
        let half = ((s+1) / 2) as usize;
        let mut row = Vec::with_capacity(half);
        for _ in 0..half {
            row.push(nxb())
        }
        let reversed: Vec<bool> = row
            .iter()
            .rev()
            .skip(if s % 2 == 0 {0} else {1})
            .copied()
            .collect();
        row.extend(reversed);
        grid.push(row);
    }
    grid
}

fn grid_to_img(grid: &[Vec<bool>], hue: u32, scale: u32) -> RgbImage {
    let h = grid.len() as u32;
    let w = grid[0].len() as u32;
    let on = hue_to_rgb(hue);
    let bg_alpha = 0.30f32;
    let bg = [
        (on[0] as f32 * bg_alpha + 255.0 * (1.0 - bg_alpha)).round() as u8,
        (on[1] as f32 * bg_alpha + 255.0 * (1.0 - bg_alpha)).round() as u8,
        (on[2] as f32 * bg_alpha + 255.0 * (1.0 - bg_alpha)).round() as u8,
    ];

    let mut img = RgbImage::new(w*scale, h*scale);

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let col = Rgb(if cell { on } else { bg });
            for dy in 0..scale {
                for dx in 0..scale {
                    img.put_pixel(x as u32 * scale + dx, y as u32 * scale +dy, col);
                }
            }
        }
    }
    img
}

fn write_stdout(img: &RgbImage) -> Result<(), Box<dyn std::error::Error>>  {
    let (w,h) = img.dimensions();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut encoder = Encoder::new(&mut handle, w, h);
    encoder.set_color(ColorType::Rgb);
    encoder.set_depth(BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(img.as_raw())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut buf = Vec::new();
    let cap = args.size * args.size + 32;
    let _ = io::stdin().take(cap as u64).read_to_end(&mut buf)?;
    let hash = get_hash(&buf);
    let hue = hash % 360;
    let grid = build_grid(hash, args.size);

    eprintln!("dbg;b={};h=0x{:08x};hue={}", buf.len(), hash, hue);

    let img = grid_to_img(&grid, hue, args.scale);
    write_stdout(&img)?;

    Ok(())
}
