
use bevy::{prelude::*, render::mesh::{PrimitiveTopology, Indices}};
use noise::{OpenSimplex, Seedable, NoiseFn};

const MAP_SIZE: usize = 64;

fn fractal(noise: OpenSimplex, x: f64, y: f64, layers: usize) -> f64 {
    let mut sum = 0f64;
    let mut frequency = 1f64;
    let mut amplitude = 4f64;
    for _i in 0..layers {
        sum += (noise.get([(x / 10.0) * frequency, (y / 10.0) * frequency])) * amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    sum
}

fn gen_noise(seed: u32) -> [[f64; MAP_SIZE]; MAP_SIZE] {
    let noise = OpenSimplex::new();
    let mut result = [[0f64; MAP_SIZE]; MAP_SIZE];
    noise.set_seed(seed);
    for x in 0..MAP_SIZE{
        for y in 0..MAP_SIZE{
            result[x][y] = fractal(noise, x as f64, y as f64, 10);
        }
    }
    return result;
}

fn vingette(mut map: [[f64; MAP_SIZE]; MAP_SIZE]) -> [[f64; MAP_SIZE]; MAP_SIZE] {
    let center = MAP_SIZE as i32 / 2;
    for x in 0..MAP_SIZE as i32 {
        for y in 0..MAP_SIZE as i32 {
            let dist = f64::sqrt(((x - center).abs().pow(2) + (y - center).abs().pow(2)) as f64);
            if dist >= 20.0 {
                let ux = x as usize;
                let uy = y as usize;
                map[ux][uy] = map[ux][uy] - ((dist / 5.0) - 4.0)
            } 
        }
    }
    map
}

pub fn gen_terrain_mesh(seed: u32) -> Mesh {
    let size: u32 = MAP_SIZE as u32;

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let noise = gen_noise(seed);
    let noise = vingette(noise);
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    for x in 0..MAP_SIZE{
        for y in 0..MAP_SIZE{
            vertices.push([x as f32, noise[x][y] as f32, y as f32]);
            normals.push([0.0, 1.0, 0.0]);
            uvs.push([x as f32 / (size as f32 - 1.0), y as f32 / (size as f32 - 1.0)])
        }
    }

    for i in 0..(size * size) {
        let y: u32 = i / MAP_SIZE as u32;
        let x: u32 = i % MAP_SIZE as u32;
        if x != 0 && x % (size - 1) == 0 { continue };
        if y != 0 && y % (size - 1) == 0 { continue };

        indices.push(i);
        indices.push(i + 1);
        indices.push((( y + 1 ) * size) + x);

        indices.push(i + 1);
        indices.push((( y + 1 ) * size) + ( x + 1));
        indices.push((( y + 1 ) * size) + x);

    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}