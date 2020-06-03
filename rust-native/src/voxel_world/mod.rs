mod chunk;
mod world_generator;
mod mesh_generator;
mod collision_generator;
use world_generator::WorldGeneratorTrait;
use gdnative::*;

use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::ops::DerefMut;


#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct VoxelWorld {
    chunks : HashMap<(i64, i64, i64), Arc<Mutex<chunk::Chunk>>>,
    distance : i64,
    root : (i64, i64, i64),
    generator : Arc<world_generator::PerlinWorldGenerator>,
    mesher : Arc<mesh_generator::MeshGenerator>,
    visual_server : VisualServer,
    physics_server : PhysicsServer,
    scenario : Rid,
    space : Rid,
    instant : Instant,
    camera : Option<Spatial>,
    material : Resource,
    tx : Sender<((i64, i64, i64),Arc<Mutex<chunk::Chunk>>)>,
    rx : Receiver<((i64, i64, i64),Arc<Mutex<chunk::Chunk>>)>,
    is_processing : bool,
    processing_list : HashMap<(i64, i64, i64), JoinHandle<()>>,
    max_threads : usize,
    positions : Vec<(i64, i64, i64)>,
    max_time : u128,
}

#[methods]
impl VoxelWorld {
    pub fn _init(_owner: Spatial) -> Self {
        let (tx, rx) = channel::<((i64, i64, i64), Arc<Mutex<chunk::Chunk>>)>();
        let distance = 12;
        let mut positions = Vec::new();
        for i in -distance..distance + 1 {
            for j in -distance..distance + 1 {
                for k in -distance..distance+1 {
                    positions.push((i,j,k));
                }
            }
        }

        positions.sort_by(|a, b| {
            let dist_a = a.0*a.0 + a.1*a.1 + a.2*a.2;
            let dist_b = b.0*b.0 + b.1*b.1 + b.2*b.2;
            return dist_a.cmp(&dist_b);
        });
        Self{
            chunks : HashMap::new(),
            distance,
            root : (0, 0, 0),
            generator : Arc::new(world_generator::PerlinWorldGenerator::new()),
            mesher : Arc::new(mesh_generator::MeshGenerator::new()),
            visual_server : VisualServer::godot_singleton(),
            physics_server : PhysicsServer::godot_singleton(),
            scenario : Rid::new(),
            space : Rid::new(),
            instant : Instant::now(),
            camera : None,
            material : Resource::new(),
            tx,
            rx,
            is_processing : false,
            processing_list : HashMap::new(),
            max_threads : 8,
            positions,
            max_time : 6,
        }
    }
    
    #[export]
    unsafe fn _ready(&mut self, _owner : Spatial) {
        self.scenario = _owner.get_world().unwrap().get_scenario();
        self.space = _owner.get_world().unwrap().get_space();
        self.material = ResourceLoader::godot_singleton().load(GodotString::from_str("res://Assets/Materials/ground.material"), GodotString::from_str(""), false).unwrap();
        godot_print!("Voxel world ready");
        self.update_chunks(_owner);
    }

    #[export] 
    unsafe fn set_origin(&mut self, _owner : Spatial, value : Spatial) {
        self.camera = Some(value);
    }

    #[export]
    unsafe fn _physics_process(&mut self, _owner : Spatial, delta : f64) {
        if self.camera.is_some() {
            for camera in self.camera.iter() {
                let new_root : Vector3 = (camera.get_translation()/(chunk::chunk_size as f32*chunk::block_size)).floor();
                self.root = (new_root.x as i64, new_root.y as i64, new_root.z as i64);
            }
            self.update_chunks(_owner);
        }
    }

    #[export]
    fn update_chunks(&mut self, _owner : Spatial) {
        self.instant = Instant::now();
        if self.processing_list.len() > 0 {
            loop {
                match self.rx.try_recv() {
                    Ok((position  , chunk_arc)) => {
                        let mut chunk = chunk_arc.lock().unwrap();
                        self.visual_server.mesh_clear(chunk.mesh);
                        self.physics_server.body_clear_shapes(chunk.body);
                        for shape in chunk.shapes.iter() {
                            self.physics_server.free_rid(*shape);
                        }
                        chunk.shapes.clear();
                        if Vector3Array::from_variant(chunk.arrays.get_ref(ArrayMesh::ARRAY_VERTEX as i32)).unwrap().len() > 0 {
                            self.visual_server.mesh_add_surface_from_arrays(chunk.mesh, Mesh::PRIMITIVE_TRIANGLES, chunk.arrays.new_ref(), VariantArray::new(), 97280);
                            let shape = self.physics_server.shape_create(PhysicsServer::SHAPE_CONCAVE_POLYGON);
                            self.physics_server.shape_set_data(shape, chunk.arrays.get_val(ArrayMesh::ARRAY_VERTEX as i32));
                            self.physics_server.body_add_shape(chunk.body, shape, Transform {basis : Basis::identity(), origin : Vector3::zero()}, false);
                            chunk.shapes.push(shape);
                        }
                        chunk.arrays.clear();
                        //for i in 0..chunk.shape_parameters.len() {
                        //    let shape = self.physics_server.shape_create(PhysicsServer::SHAPE_BOX);
                        //    self.physics_server.shape_set_data(shape, chunk.shape_parameters[i].1.to_variant());
                        //    self.physics_server.body_add_shape(chunk.body, shape, chunk.shape_parameters[i].0, false); 
                        //    chunk.shapes.push(shape);
                        //}
                        //godot_print!("added {} shapes", chunk.shapes.len());
                        
                        
                        self.processing_list.remove(&position).unwrap().join().expect("Thread already joined");
                    }
                    Err(_) => {
                        break;
                    }
                }
                if self.instant.elapsed().as_millis() > self.max_time {
                    //godot_print!("took {:?} to finish", self.instant.elapsed());
                    return;
                }
            }
        }
        let distance = self.distance;
        let root = self.root;
        let list = &self.processing_list;
        self.chunks.retain(|key, value| {
            let result = Self::check_ranges(distance, root,key.clone()) || list.get(&key).is_some();
            //godot_print!("ready to drop chunk at {:?}? : {}", key, !result);
            //godot_print!("chunk in range? : {}", Self::check_ranges(distance, root,key.0, key.1, key.2));
            //godot_print!("chunk is processing? : {}",list.get(&key).is_some());
            //godot_print!("references to chunk: {}", Arc::strong_count(value));
            //godot_print!("origin at : {:?}", root);
            return result;
        });
        //godot_print!("chunks alive: {}", self.chunks.len());
        //godot_print!("chunks processing: {}", self.processing_list.len());
        for pos in self.positions.iter() {
            if self.processing_list.len() > self.max_threads {
                return;
            }
            if self.instant.elapsed().as_millis() > self.max_time {
                //godot_print!("took {:?} to finish", self.instant.elapsed());
                return;
            }
            let position = (pos.0 + self.root.0, pos.1 + self.root.1, pos.2 +self.root.2);
            if Self::check_ranges(distance, root, position) && !self.processing_list.get(&position).is_some() {
                match self.chunks.get(&position).clone() {
                    Some(chunk_arc) => {
                        let mut chunk_guard = chunk_arc.lock().unwrap();
                        if !chunk_guard.cached {
                            self.is_processing = true;
                            let generator = self.generator.clone();
                            let mesher = self.mesher.clone();
                            let tx = self.tx.clone();
                            let chunk_arc = chunk_arc.clone();
                            let handle = thread::spawn(move || {
                                let mut chunk_guard = chunk_arc.lock().unwrap();
                                let chunk = chunk_guard.deref_mut();
                                generator.generate_chunk(chunk);
                                mesher.generate_chunk(chunk);
                                collision_generator::CollisionGenerator::generate_collisions(chunk);
                                tx.send((chunk.position, chunk_arc.clone()));
                            });
                            self.processing_list.insert(position, handle);
                        }
                    }
                    None => {
                        godot_print!("Creating a brand new chunk!");
                        let instance = self.visual_server.instance_create();
                        self.visual_server.instance_set_scenario(instance, self.scenario);
                        let mesh = self.visual_server.mesh_create();
                        self.visual_server.instance_set_base(instance, mesh);
                        let transform = Transform{basis : Basis::identity(), 
                            origin : Vector3::new(
                                (position.0*chunk::chunk_size as i64) as f32*chunk::block_size, 
                                (position.1*chunk::chunk_size as i64) as f32*chunk::block_size,  
                                (position.2*chunk::chunk_size as i64) as f32*chunk::block_size)};
                        self.visual_server.instance_set_transform(instance, transform);
                        self.visual_server.instance_geometry_set_material_override(instance, self.material.get_rid());
                        let body = self.physics_server.body_create(PhysicsServer::BODY_MODE_STATIC, true);
                        self.physics_server.body_set_space(body, self.space);
                        self.physics_server.body_set_state(body, PhysicsServer::BODY_STATE_TRANSFORM, transform.to_variant());
                        let mut chunk = chunk::Chunk{
                            position,
                            cached : false,
                            data : None,
                            mesh,
                            instance,
                            arrays : VariantArray::new(),
                            body,
                            shapes : Vec::new(),
                            shape_parameters : Vec::new(),
                        };
                        let chunk_arc = Arc::new(Mutex::new(chunk));
                        self.chunks.insert(position, chunk_arc.clone());

                        self.is_processing = true;
                        let generator = self.generator.clone();
                        let mesher = self.mesher.clone();
                        let tx = self.tx.clone();
                        let chunk_arc = chunk_arc.clone();
                        let handle =thread::spawn(move || {
                            let mut chunk_guard = chunk_arc.lock().unwrap();
                            let chunk = chunk_guard.deref_mut();
                            generator.generate_chunk(chunk);
                            mesher.generate_chunk(chunk);
                            collision_generator::CollisionGenerator::generate_collisions(chunk);
                            tx.send((chunk.position, chunk_arc.clone()));
                        });
                        self.processing_list.insert(position, handle);
                    }
                }
            }
        }
        
        //if chunk_count > 0 {
        //    godot_print!("Updated {} chunks in {:?}", chunk_count, self.instant.elapsed());
        //}
    }
    fn check_ranges(distance : i64, root : (i64, i64, i64), position : (i64, i64, i64)) -> bool {
        return (position.0-root.0).pow(2) + (position.1-root.1).pow(2) + (position.2-root.2).pow(2) <= distance.pow(2);
    }
}



