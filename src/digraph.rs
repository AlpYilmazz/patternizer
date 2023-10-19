pub const DIM: usize = 256;
pub type DigraphMap = [[usize; DIM]; DIM];
pub type DigraphMapNormalized = [[u32; DIM]; DIM];

pub trait Zeroed {
    fn zeroed() -> Self;
}

impl Zeroed for DigraphMap {
    fn zeroed() -> Self {
        [[0; DIM]; DIM]
    }
}

impl Zeroed for DigraphMapNormalized {
    fn zeroed() -> Self {
        [[0; DIM]; DIM]
    }
}

pub fn create_digraph_bytes(bytes: &[u8]) -> Vec<u8> {
    let map = normalize(&generate_digraph(bytes));
    let mut vec = Vec::with_capacity(4 * DIM * DIM);

    for i in 0..DIM {
        for j in 0..DIM {
            vec.extend_from_slice(&map[i][j].to_le_bytes());
        }
    }

    vec
    // normalize(&generate_digraph(bytes)).into_iter().flatten().map(|p| p.to_be_bytes()).flatten().collect()
}

pub fn generate_digraph(bytes: &[u8]) -> Box<DigraphMap> {
    let mut map = Box::new(DigraphMap::zeroed());

    // DIGRAPH := 2
    for window in bytes.windows(2) {
        let (x, y) = (window[0] as usize, window[1] as usize);
        map[y][x] += 1;
    }

    map
}

pub fn normalize(map: &DigraphMap) -> Box<DigraphMapNormalized> {
    let mut normalized_map = Box::new(DigraphMapNormalized::zeroed());

    let max_value = *map.iter().flatten().max().unwrap() as f32;
    let max_value = max_value.ln();
    for i in 0..DIM {
        for j in 0..DIM {
            // normal_value -> 0..255
            let value = map[i][j] as f32;
            let value = value.ln();
            let normal_value = 255.0 * (value / max_value);

            // normalized_map[i][j] = normal_value as u8;
            let val = normal_value as u32;
            normalized_map[i][j] = 0xFF000000 | val | (val << 8) | (val << 16);
        }
    }

    normalized_map
}
