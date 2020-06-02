use gdnative::*;
use std::ops::{Index, IndexMut};
use super::world_generator;
use std::thread;

pub const chunk_size : usize = 32;
pub const block_size : f32 = 0.5;

#[derive(Copy, Clone, PartialEq)]
pub enum BlockType {
    BLOCK_NONE,
    BLOCK_GRASS,
    BLOCK_DIRT,
}
#[derive(Copy, Clone)]
pub struct BlockData {
    pub value : BlockType
}

pub struct ChunkData {
    data : [BlockData ; (chunk_size + 2)*(chunk_size + 2)*(chunk_size + 2)]
}

impl ChunkData {
    pub fn new() -> Self {
        Self {
            data : [BlockData{value : BlockType::BLOCK_NONE}; (chunk_size + 2)*(chunk_size + 2)*(chunk_size + 2)]
        }
    }
}

impl Index<(isize, isize, isize)> for ChunkData {
    type Output = BlockData;
    fn index(&self, idx : (isize, isize, isize)) -> &Self::Output {
        let udx = (idx.0 as usize + 1 , idx.1 as usize + 1, idx.2 as usize + 1);
        &self.data[(chunk_size+2)*((chunk_size+2)*udx.0 + udx.1) + udx.2]
    }
}

impl IndexMut<(isize, isize, isize)> for ChunkData {
    fn index_mut(&mut self, idx : (isize, isize, isize)) -> &mut Self::Output {
        let udx = (idx.0 as usize + 1 , idx.1 as usize + 1, idx.2 as usize + 1);
        &mut self.data[(chunk_size+2)*((chunk_size+2)*udx.0 + udx.1) + udx.2]
    }
}

pub struct Chunk {
    pub position : (i64, i64, i64),
    pub mesh : Rid,
    pub data : Option<Box<ChunkData>>,
    pub cached : bool,
    pub instance : Rid,
    pub arrays : VariantArray,
}

impl Drop for Chunk {
    fn drop(&mut self) {
        let mut visual_server = VisualServer::godot_singleton();
        visual_server.mesh_clear(self.mesh);
        //godot_print!("Dropped chunk at {:?}, in thread {:?}", self.position, thread::current());
    }
}

