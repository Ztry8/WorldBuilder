use image::{ColorType, DynamicImage, GenericImage, Rgba};
use noise::{NoiseFn, Perlin};
use std::time::{SystemTime, UNIX_EPOCH};

mod config;

fn main() {
    let cfg = config::get_config();

    let generator = Perlin::new(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos(),
    );

    let mut map = DynamicImage::new(cfg.size, cfg.size, ColorType::Rgba8);

    let is_border = |x: u32, offset: i64, tile_size| {
        let x = (x as i64 + offset) as u32;

        x == 0
            || x % tile_size == 0
            || (x >= (tile_size - 1) && (x - (tile_size - 1)) % tile_size == 0)
    };

    for x in 0..cfg.size {
        for y in 0..cfg.size {
            map.put_pixel(
                x,
                y,
                Rgba::<u8>(
                    if let Some(tile_size) = cfg.borders
                        && (is_border(x, 0, tile_size) || is_border(y, 0, tile_size))
                    {
                        cfg.mid_color
                    } else if let Some(tile_size) = cfg.borders
                        && (is_border(x, -1, tile_size)
                            || is_border(y, -1, tile_size)
                            || is_border(x, 1, tile_size)
                            || is_border(y, 1, tile_size))
                    {
                        cfg.low_color
                    } else {
                        let height = generator.get([x as f64 * cfg.size_k, y as f64 * cfg.size_k]);
                        let c = (height * 128.0 + 128.0) as u8;

                        if c < 10 {
                            cfg.water_color
                        } else if c < 200 {
                            cfg.no_color
                        } else if c < 230 {
                            cfg.low_color
                        } else if c < 250 {
                            cfg.mid_color
                        } else {
                            cfg.high_color
                        }
                    },
                ),
            );
        }
    }

    map.save("map.png").unwrap();
}
