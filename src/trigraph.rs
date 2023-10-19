use colors_transform::{Hsl, Color};

pub const DIM: usize = 256;
pub type TrigraphMap = [[[usize; DIM]; DIM]; DIM];
pub type TrigraphMapNormalized = [[u32; DIM]; DIM];

pub trait Zeroed {
    fn zeroed() -> Self;
}

impl Zeroed for TrigraphMap {
    fn zeroed() -> Self {
        [[[0; DIM]; DIM]; DIM]
    }
}

impl Zeroed for TrigraphMapNormalized {
    fn zeroed() -> Self {
        [[0; DIM]; DIM]
    }
}

pub fn create_trigraph_bytes(bytes: &[u8]) -> Vec<u8> {
    let map = normalize(&generate_trigraph(bytes));
    let mut vec = Vec::with_capacity(4 * DIM * DIM);

    for i in 0..DIM {
        for j in 0..DIM {
            vec.extend_from_slice(&map[i][j].to_le_bytes());
        }
    }

    vec
}

pub fn generate_trigraph(bytes: &[u8]) -> Box<TrigraphMap> {
    let mut map = Box::new(TrigraphMap::zeroed());

    // TRIGRAPH := 3
    for window in bytes.windows(3) {
        let (x, y, z) = (window[0] as usize, window[1] as usize, window[2] as usize);
        map[z][y][x] += 1;
    }

    map
}

// dont think it is doable by coloring
pub fn normalize(map: &TrigraphMap) -> Box<TrigraphMapNormalized> {
    let mut normalized_map = Box::new(TrigraphMapNormalized::zeroed());

    let max_value = *map.iter().flatten().flatten().max().unwrap() as f32;
    let max_value = max_value.ln();
    for z in 0..DIM {
        for x in 0..DIM {
            for y in 0..DIM {
                // normal_value -> 0..255
                let value = map[z][y][x] as f32;
                let value = value.ln();
                let normal_value = 100.0 * (value / max_value);

                let h = 10.0;
                let s = 80.0;
                let l = normal_value;
                let _rgb = Hsl::from(h, s, l).to_rgb();

                let val = normal_value as u32;
                normalized_map[y][x] = 0xFF000000 | val | (val << 8) | (val << 16);
            }
        }
    }

    normalized_map
}
