use super::chunk::{BlockData, Chunk, chunk_size, block_size, ChunkData, BlockType};
use gdnative::*;

pub struct CollisionGenerator;

impl CollisionGenerator {
    pub fn generate_collisions(chunk : &mut Chunk) {
        //Self::recursive_generator(chunk, (0,0,0), (chunk_size as isize, chunk_size as isize, chunk_size as isize));
    }
    fn recursive_generator(chunk : &mut Chunk, start : (isize, isize, isize), end : (isize, isize, isize)) {
        let size = (end.0 - start.0).max(end.1 - start.1).max(end.2 - start.2);
        if size == 0 {
            return;
        }
        let mut valid : bool = true;
        'outer : for data in chunk.data.iter() {
            for i in start.0..end.0 {
                for j in start.1..end.1 {
                    for k in start.2..end.2 {
                        if data[(i, j, k)].value == BlockType::BLOCK_NONE {
                            valid = false;
                            break 'outer;
                        }
                    }
                }
            }
        }
        let middle = ((start.0 + end.0)/2,(start.1 + end.1)/2,(start.2 + end.2)/2);
        if valid {
            let vector = Vector3::new(
                block_size*((end.0 - start.0) as f32), 
                block_size*((end.1 - start.1) as f32),
                block_size*((end.2 - start.2) as f32))/2.0;
            let origin = Vector3::new(start.0 as f32, start.1 as f32, start.2 as f32)*block_size + vector;
            let transform : Transform = Transform {
                basis: Basis::identity(),
                origin,
            };
            chunk.shape_parameters.push((transform, vector));
        }
        else if size == 1 {
            return;
        }
        else if size > 1{
            Self::recursive_generator(chunk, (start.0, start.1, start.2), (middle.0, middle.1, middle.2));
            Self::recursive_generator(chunk, (start.0, start.1, middle.2), (middle.0, middle.1, end.2));
            Self::recursive_generator(chunk, (start.0, middle.1, start.2), (middle.0, end.1, middle.2));
            Self::recursive_generator(chunk, (start.0, middle.1, middle.2), (middle.0, end.1, end.2));
            Self::recursive_generator(chunk, (middle.0, start.1, start.2), (end.0, middle.1, middle.2));
            Self::recursive_generator(chunk, (middle.0, start.1, middle.2), (end.0, middle.1, end.2));
            Self::recursive_generator(chunk, (middle.0, middle.1, start.2), (end.0, end.1, middle.2));
            Self::recursive_generator(chunk, (middle.0, middle.1, middle.2), (end.0, end.1, end.2));
        }
    }
}