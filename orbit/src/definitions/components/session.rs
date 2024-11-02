use std::ffi::c_void;

use il2cppinterop_macros::Mono;

use super::entity::Entity;

#[derive(Debug, Mono)]
#[repr(C)]
pub struct Session {
    #[base]
    entity: Entity,
    pub request_callbacks: *mut c_void,
    pub a_service: *mut c_void,
    pub last_recv_time: i64,
    pub last_send_time: i64,
    pub error: i32,
    pub remote_address: *mut c_void,
}

impl Session {
    pub fn is_connected(&self) -> bool {
        self.error == 0
    }
}