use gdnative::*;
mod voxel_world;
pub use voxel_world::VoxelWorld;
#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct GameWorld {
}

#[methods]
impl GameWorld {
    fn _init(_owner: Spatial) -> Self {
        Self{}
    }

    #[export]
    unsafe fn _ready(&mut self, _owner: Spatial) {
        godot_print!("Test");
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<GameWorld>();
    handle.add_class::<VoxelWorld>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();