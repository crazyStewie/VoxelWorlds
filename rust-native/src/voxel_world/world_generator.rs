use noise::{Perlin, NoiseFn, Seedable};

use super::chunk::{BlockData, Chunk, block_size, chunk_size, ChunkData, BlockType};
use gdnative::*;

pub trait WorldGeneratorTrait {
    fn generate_block(&self, x : f64, y : f64, z : f64) -> BlockData;
    fn generate_chunk(&self, chunk : &mut Chunk);
}

pub struct PerlinWorldGenerator {
    noise_generator : noise::Perlin,
    height : f64,
    scale : f64,
}

impl PerlinWorldGenerator {
    pub fn new() -> Self {
        let noise_generator = Perlin::new();
        noise_generator.set_seed(1);
        Self {
            noise_generator,
            height : 32.0,
            scale : 64.0,
        }
        
    }
}

impl WorldGeneratorTrait for PerlinWorldGenerator {
    fn generate_block(&self, x : f64, y : f64, z : f64) -> BlockData {
        let height : f64 = self.height*self.noise_generator.get([x/self.scale, y/self.scale, z/self.scale]);
        //godot_print!("Height : {}", height);
        let mut value : BlockType = BlockType::BLOCK_NONE;
        if height - y > 0.0 {
            value = BlockType::BLOCK_DIRT;
        }
        BlockData{
            value,
        }
    }

    fn generate_chunk(&self, chunk : &mut Chunk) {
        let mut new_data : Box<ChunkData>;
        if chunk.data.is_none() {
            new_data = Box::new(ChunkData::new());
        }
        else {
            new_data = chunk.data.take().unwrap();
        }
        let size = chunk_size as f64*block_size as f64;
        for i in -1..chunk_size as isize +1 {
            for j in -1..chunk_size as isize +1 {
                for k in -1..chunk_size as isize +1 {
                    let mut block_position = (chunk.position.0 as f64*size, chunk.position.1 as f64*size, chunk.position.2 as f64*size);
                    block_position = (block_position.0 + i as f64*block_size as f64,block_position.1 + j as f64*block_size as f64, block_position.2 + k as f64*block_size as f64);
                    new_data[(i, j, k)] = self.generate_block(block_position.0, block_position.1, block_position.2);
                }
            }
        }
        for i in 0..chunk_size as isize {
            for j in 0..chunk_size as isize {
                for k in 0..chunk_size as isize {
                    if new_data[(i, j, k)].value == BlockType::BLOCK_DIRT && new_data[(i, j+1, k)].value == BlockType::BLOCK_NONE {
                        new_data[(i,j,k)].value = BlockType::BLOCK_GRASS;
                    }
                }
            }
        }
        chunk.data = Some(new_data);
        chunk.cached = true;
    }
}