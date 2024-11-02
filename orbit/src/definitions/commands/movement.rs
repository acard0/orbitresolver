use getset::{Getters, Setters};
use il2cppinterop_core::mono::definitions::object::{self, SystemObject};
use il2cppinterop_macros::Mono;


#[derive(Debug, Mono, Getters, Setters)]
#[getset(set = "pub", get = "pub with_prefix")]
pub struct MoveRequest {
    #[base]
    #[getset(skip)]
    object: SystemObject,
    position_x: i32,
    target_y: i32,
    target_x: i32,
    position_y: i32,
}

impl MoveRequest {
    pub fn new() -> &'static mut Self {
        object::new_from_namespace("net.bigpoint.darkorbit.com.module.MoveRequest")
            .expect("Failed to create net.bigpoint.darkorbit.com.module.MoveRequest")
    }
}