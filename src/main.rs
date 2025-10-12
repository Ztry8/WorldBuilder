use image::{ColorType, DynamicImage, GenericImage, Rgba};
use noise::{NoiseFn, Perlin};

const SIZE: u32 = 1280;
const SIZE_K: f64 = 0.1;

const GRID_COLOR: [u8; 4] = [230, 230, 230, 255];

const WATER_COLOR: [u8; 4] = [0, 0, 255, 255];
const NO_COLOR: [u8; 4] = [0, 0, 0, 255];
const LOW_COLOR: [u8; 4] = [255, 255, 255, 255];
const MID_COLOR: [u8; 4] = [100, 100, 100, 255];
const HIGH_COLOR: [u8; 4] = [50, 50, 50, 255];

const SEED: u32 = 0;
const BORDERS: bool = false;

fn main() {
    let generator = Perlin::new(SEED);
    let mut map = DynamicImage::new(SIZE, SIZE, ColorType::Rgba8);

    let is_border = |x: u32, offset: i64| {
        let x = (x as i64 + offset) as u32;

        x == 0 || x % 64 == 0 || (x >= 63 && (x - 63) % 64 == 0)
    };

    for x in 0..SIZE {
        for y in 0..SIZE {
            map.put_pixel(
                x,
                y,
                Rgba::<u8>(if BORDERS && (is_border(x, 0) || is_border(y, 0)) {
                    GRID_COLOR
                } else if BORDERS
                    && (is_border(x, -1) || is_border(y, -1) || is_border(x, 1) || is_border(y, 1))
                {
                    LOW_COLOR
                } else {
                    let height = generator.get([x as f64 * SIZE_K, y as f64 * SIZE_K]);
                    let c = (height * 128.0 + 128.0) as u8;

                    if c < 10 {
                        WATER_COLOR
                    } else if c < 200 {
                        NO_COLOR
                    } else if c < 230 {
                        LOW_COLOR
                    } else if c < 250 {
                        MID_COLOR
                    } else {
                        HIGH_COLOR
                    }
                }),
            );
        }
    }

    map.save("map.png").unwrap();
}
