use super::chunk::{BlockData, Chunk, chunk_size, block_size, ChunkData, BlockType};
use gdnative::*;


pub struct MeshGenerator {
    face_up : Vector3Array,     //y+
    face_down : Vector3Array,   //y-
    face_left : Vector3Array,   //x-
    face_right : Vector3Array,  //x+
    face_front : Vector3Array,  //z-
    face_back : Vector3Array,   //z+
    normals_up : Vector3Array,     //y+
    normals_down : Vector3Array,   //y-
    normals_left : Vector3Array,   //x-
    normals_right : Vector3Array,  //x+
    normals_front : Vector3Array,  //z-
    normals_back : Vector3Array,
    uv : Vector2Array,
    padding : f32,
}

impl MeshGenerator {
    pub fn new() -> Self {
        let padding = 1.0/32.0;
        let mut face_up = Vector3Array::new();
        face_up.push(&(Vector3::new(0.0, 1.0, 0.0)*block_size));
        face_up.push(&(Vector3::new(1.0, 1.0, 0.0)*block_size));
        face_up.push(&(Vector3::new(1.0, 1.0, 1.0)*block_size));
        face_up.push(&(Vector3::new(0.0, 1.0, 0.0)*block_size));
        face_up.push(&(Vector3::new(1.0, 1.0, 1.0)*block_size));
        face_up.push(&(Vector3::new(0.0, 1.0, 1.0)*block_size));
        let mut face_down = Vector3Array::new();
        face_down.push(&(Vector3::new(0.0, 0.0, 1.0)*block_size));
        face_down.push(&(Vector3::new(1.0, 0.0, 1.0)*block_size));
        face_down.push(&(Vector3::new(1.0, 0.0, 0.0)*block_size));
        face_down.push(&(Vector3::new(0.0, 0.0, 1.0)*block_size));
        face_down.push(&(Vector3::new(1.0, 0.0, 0.0)*block_size));
        face_down.push(&(Vector3::new(0.0, 0.0, 0.0)*block_size));
        let mut face_left = Vector3Array::new();
        face_left.push(&(Vector3::new(0.0, 1.0, 0.0)*block_size));
        face_left.push(&(Vector3::new(0.0, 1.0, 1.0)*block_size));
        face_left.push(&(Vector3::new(0.0, 0.0, 1.0)*block_size));
        face_left.push(&(Vector3::new(0.0, 1.0, 0.0)*block_size));
        face_left.push(&(Vector3::new(0.0, 0.0, 1.0)*block_size));
        face_left.push(&(Vector3::new(0.0, 0.0, 0.0)*block_size));
        let mut face_right = Vector3Array::new();
        face_right.push(&(Vector3::new(1.0, 1.0, 1.0)*block_size));
        face_right.push(&(Vector3::new(1.0, 1.0, 0.0)*block_size));
        face_right.push(&(Vector3::new(1.0, 0.0, 0.0)*block_size));
        face_right.push(&(Vector3::new(1.0, 1.0, 1.0)*block_size));
        face_right.push(&(Vector3::new(1.0, 0.0, 0.0)*block_size));
        face_right.push(&(Vector3::new(1.0, 0.0, 1.0)*block_size));
        let mut face_front = Vector3Array::new();
        face_front.push(&(Vector3::new(1.0, 1.0, 0.0)*block_size));
        face_front.push(&(Vector3::new(0.0, 1.0, 0.0)*block_size));
        face_front.push(&(Vector3::new(0.0, 0.0, 0.0)*block_size));
        face_front.push(&(Vector3::new(1.0, 1.0, 0.0)*block_size));
        face_front.push(&(Vector3::new(0.0, 0.0, 0.0)*block_size));
        face_front.push(&(Vector3::new(1.0, 0.0, 0.0)*block_size));
        let mut face_back = Vector3Array::new();
        face_back.push(&(Vector3::new(0.0, 1.0, 1.0)*block_size));
        face_back.push(&(Vector3::new(1.0, 1.0, 1.0)*block_size));
        face_back.push(&(Vector3::new(1.0, 0.0, 1.0)*block_size));
        face_back.push(&(Vector3::new(0.0, 1.0, 1.0)*block_size));
        face_back.push(&(Vector3::new(1.0, 0.0, 1.0)*block_size));
        face_back.push(&(Vector3::new(0.0, 0.0, 1.0)*block_size));

        let mut normals_up = Vector3Array::new();
        normals_up.push(&Vector3::new(0.0,1.0,0.0));
        normals_up.push(&Vector3::new(0.0,1.0,0.0));
        normals_up.push(&Vector3::new(0.0,1.0,0.0));
        normals_up.push(&Vector3::new(0.0,1.0,0.0));
        normals_up.push(&Vector3::new(0.0,1.0,0.0));
        normals_up.push(&Vector3::new(0.0,1.0,0.0));
        let mut normals_down = Vector3Array::new();
        normals_down.push(&Vector3::new(0.0,-1.0,0.0));
        normals_down.push(&Vector3::new(0.0,-1.0,0.0));
        normals_down.push(&Vector3::new(0.0,-1.0,0.0));
        normals_down.push(&Vector3::new(0.0,-1.0,0.0));
        normals_down.push(&Vector3::new(0.0,-1.0,0.0));
        normals_down.push(&Vector3::new(0.0,-1.0,0.0));
        let mut normals_left = Vector3Array::new();
        normals_left.push(&Vector3::new(-1.0,0.0,0.0));
        normals_left.push(&Vector3::new(-1.0,0.0,0.0));
        normals_left.push(&Vector3::new(-1.0,0.0,0.0));
        normals_left.push(&Vector3::new(-1.0,0.0,0.0));
        normals_left.push(&Vector3::new(-1.0,0.0,0.0));
        normals_left.push(&Vector3::new(-1.0,0.0,0.0));
        let mut normals_right = Vector3Array::new();
        normals_right.push(&Vector3::new(1.0,0.0,0.0));
        normals_right.push(&Vector3::new(1.0,0.0,0.0));
        normals_right.push(&Vector3::new(1.0,0.0,0.0));
        normals_right.push(&Vector3::new(1.0,0.0,0.0));
        normals_right.push(&Vector3::new(1.0,0.0,0.0));
        normals_right.push(&Vector3::new(1.0,0.0,0.0));
        let mut normals_front = Vector3Array::new();
        normals_front.push(&Vector3::new(0.0,0.0,-1.0));
        normals_front.push(&Vector3::new(0.0,0.0,-1.0));
        normals_front.push(&Vector3::new(0.0,0.0,-1.0));
        normals_front.push(&Vector3::new(0.0,0.0,-1.0));
        normals_front.push(&Vector3::new(0.0,0.0,-1.0));
        normals_front.push(&Vector3::new(0.0,0.0,-1.0));
        let mut normals_back = Vector3Array::new();
        normals_back.push(&Vector3::new(0.0,0.0,1.0));
        normals_back.push(&Vector3::new(0.0,0.0,1.0));
        normals_back.push(&Vector3::new(0.0,0.0,1.0));
        normals_back.push(&Vector3::new(0.0,0.0,1.0));
        normals_back.push(&Vector3::new(0.0,0.0,1.0));
        normals_back.push(&Vector3::new(0.0,0.0,1.0));
        let mut uv = Vector2Array::new();
        uv.push(&Vector2::new(0.0, 0.0));
        uv.push(&Vector2::new(1.0, 0.0));
        uv.push(&Vector2::new(1.0, 1.0));
        uv.push(&Vector2::new(0.0, 0.0));
        uv.push(&Vector2::new(1.0, 1.0));
        uv.push(&Vector2::new(0.0, 1.0));
        Self {
            face_up,
            face_down,
            face_left,
            face_right,
            face_front,
            face_back,
            normals_up,
            normals_down,
            normals_left,
            normals_right,
            normals_front,
            normals_back,
            uv,
            padding,
        }
    }

    pub fn generate_chunk(&self, chunk : &mut Chunk) {
        let mut visual_server = VisualServer::godot_singleton();
        let mut vertices = Vector3Array::new();
        let mut uvs = Vector2Array::new();
        let mut normals = Vector3Array::new();
        let mut count : i64 = 0;
        for data in chunk.data.iter() {
            for i in 0..chunk_size as isize {
                for j in 0..chunk_size as isize {
                    for k in 0..chunk_size as isize {
                        if data[(i,j,k)].value != BlockType::BLOCK_NONE {
                            let offset = Vector3::new(i as f32*block_size, j as f32*block_size, k as f32*block_size);
                            count = 0;
                            if data[(i-1,j,k)].value == BlockType::BLOCK_NONE {
                                vertices.push_array(&get_offset_array(&self.face_left, offset));
                                normals.push_array(&self.normals_left);
                                count+=1;
                            }
                            if data[(i+1,j,k)].value == BlockType::BLOCK_NONE {
                                vertices.push_array(&get_offset_array(&self.face_right, offset));
                                normals.push_array(&self.normals_right);
                                count+=1;
                            }
                            if data[(i,j-1,k)].value == BlockType::BLOCK_NONE {
                                vertices.push_array(&get_offset_array(&self.face_down, offset));
                                normals.push_array(&self.normals_down);
                                count+=1;
                            }
                            if data[(i,j+1,k)].value == BlockType::BLOCK_NONE {
                                vertices.push_array(&get_offset_array(&self.face_up, offset));
                                normals.push_array(&self.normals_up);
                                count+=1;
                            }
                            if data[(i,j,k-1)].value == BlockType::BLOCK_NONE {
                                vertices.push_array(&get_offset_array(&self.face_front, offset));
                                normals.push_array(&self.normals_front);
                                count+=1;
                            }
                            if data[(i,j,k+1)].value == BlockType::BLOCK_NONE {
                                vertices.push_array(&get_offset_array(&self.face_back, offset));
                                normals.push_array(&self.normals_back);
                                count+=1;
                            }
                            
                            if data[(i,j,k)].value == BlockType::BLOCK_DIRT {
                                for _ in 0..count {
                                    uvs.push_array(&get_offset_uv(&self.uv, Vector2::new(self.padding,self.padding)));
                                }
                            }
                            else if data[(i,j,k)].value == BlockType::BLOCK_GRASS {
                                for _ in 0..count {
                                    uvs.push_array(&get_offset_uv(&self.uv, Vector2::new(self.padding + 1.0*(1.0 + self.padding),self.padding)));
                                }
                            }
                        }
                    }
                }
            }
        }
        chunk.arrays.clear();
        chunk.arrays.resize(ArrayMesh::ARRAY_MAX as i32);
        chunk.arrays.set(ArrayMesh::ARRAY_VERTEX as i32, &vertices.to_variant());
        chunk.arrays.set(ArrayMesh::ARRAY_NORMAL as i32, &normals.to_variant());
        chunk.arrays.set(ArrayMesh::ARRAY_TEX_UV as i32, &uvs.to_variant());
        //visual_server.mesh_clear(chunk.mesh);
        //if vertices.len() > 0 {
        //    visual_server.mesh_add_surface_from_arrays(chunk.mesh, Mesh::PRIMITIVE_TRIANGLES, arrays, VariantArray::new(), 97280)
        //}
    }
}

fn get_offset_array(array: &Vector3Array, offset : Vector3) -> Vector3Array{
    let mut new_array = Vector3Array::new();
    new_array.resize(array.len());
    for i in 0..new_array.len() {
        new_array.set(i, &(array.get(i) + offset));
    }
    return new_array;
}

fn get_offset_uv(array: &Vector2Array, offset : Vector2) -> Vector2Array{
    let mut new_array = Vector2Array::new();
    new_array.resize(array.len());
    for i in 0..new_array.len() {
        new_array.set(i, &(array.get(i) + offset));
    }
    return new_array;
}