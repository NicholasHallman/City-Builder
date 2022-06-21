use noise::{OpenSimplex, Seedable, NoiseFn};


pub fn gen_terrain(seed: u32) -> [[f64; 32]; 32] {
    const MAP_SIZE: usize = 32;
    let noise = OpenSimplex::new();
    let mut result = [[0f64; MAP_SIZE]; MAP_SIZE];
    noise.set_seed(seed);
    for x in 0..MAP_SIZE{
        for y in 0..MAP_SIZE{
            result[x][y] = (noise.get([x as f64, y as f64]) + 1.0) / 2.0;
        }
    }
    return result;
}