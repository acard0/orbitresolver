use getset::{Getters, Setters};
use il2cppinterop_core::mono::definitions::object::{self, SystemObject};
use il2cppinterop_macros::Mono;


#[derive(Debug, Mono, Getters, Setters)]
#[getset(set = "pub", get = "pub with_prefix")] 
pub struct ShipSelectRequest {
    #[base]
    #[getset(skip)]
    object: SystemObject,
    target_id: i32,
    target_x: i32,
    target_y: i32,
    pos_x: i32,
    pos_y: i32,
    click_x: i32,
    click_y: i32,
    radius: i32,
}

impl ShipSelectRequest {
    pub fn new() -> &'static mut Self {
        object::new_from_namespace("net.bigpoint.darkorbit.com.module.ShipSelectRequest")
            .expect("Failed to create net.bigpoint.darkorbit.com.module.ShipSelectRequest")
    }
}