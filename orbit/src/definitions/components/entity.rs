use std::ffi::c_void;

use getset::Getters;
use il2cppinterop_core::mono::definitions::{dictionary::Il2cppDictionary, object::SystemObject, stype::SystemType};
use il2cppinterop_macros::Mono;

#[derive(Debug, Mono, Getters)]
#[getset(get = "pub with_prefix")]
#[repr(C)]
pub struct Entity {
    #[base]
    #[getset(skip)]
    object: SystemObject,
    children: Option<&'static mut Il2cppDictionary<isize, &'static mut Entity>>, 
    children_db: Option<&'static mut Il2cppDictionary<isize, &'static mut Entity>>, 
    status: u8,                                                                    
    #[getset(skip)]
    components: Option<&'static mut Il2cppDictionary<SystemType, &'static mut Entity>>,                         
    #[getset(skip)]
    components_db: *mut c_void,   
    domain: &'static mut Entity,                                                         
    parent: &'static mut Entity,       
    instance_id: i64,                                 
    id: i64,
}

impl Entity {
    pub fn is_valid(&self) -> bool {
        /*
            [Token(Token = "0x400131F")] None = 0,
            [Token(Token = "0x4001320")] IsFromPool = 1,
            [Token(Token = "0x4001321")] IsRegister = 2,
            [Token(Token = "0x4001322")] IsComponent = 4,
            [Token(Token = "0x4001323")] IsCreated = 8,
            [Token(Token = "0x4001324")] IsNew = 16, 
         */
        self.status <= 31
    }

    pub fn is_invalid(&self) -> bool {
        self.is_valid() == false
    }
}