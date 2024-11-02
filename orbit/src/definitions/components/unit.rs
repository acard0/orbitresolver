use derive_more::derive::Display;
use getset::Getters;
use il2cppinterop_core::{mono::definitions::{dictionary::Il2cppDictionary, list::Il2cppList, object::SystemObject, string::SystemString}, unity::{component::UnityComponent, engine::Vector3}};
use il2cppinterop_macros::Mono;

use super::entity::Entity;

#[derive(Debug, Mono)]
#[repr(C)]
pub struct UnitComponent {
    #[base]
    entity: Entity,
    collect_infos: *mut Il2cppDictionary<SystemString, UnitInfo>,
    unit_infos: *mut Il2cppDictionary<SystemString, UnitInfo>,
    units: *mut Il2cppDictionary<SystemString, Unit>,
}

impl UnitComponent {
    pub fn get_collectables(&self) -> Option<&'static mut Il2cppDictionary<SystemString, UnitInfo>> {
        unsafe { self.collect_infos.as_mut() }
    }

    pub fn get_unit_infos(&self) -> Option<&'static mut Il2cppDictionary<SystemString, UnitInfo>> { 
        unsafe { self.unit_infos.as_mut() }
    }

    pub fn get_units(&self) -> Option<&'static mut Il2cppDictionary<SystemString, Unit>> {
        unsafe { self.units.as_mut() }
    }
}

#[derive(Display, Debug, Mono, Getters)]
#[getset(get = "pub with_prefix")]
#[display("Unit({})", self.type_id.to_string())]
#[repr(C)]
pub struct Unit {
    #[base]
    #[getset(skip)]
    entity: Entity, 
    #[getset(skip)]
    _pad0: [u8; 0x34],
    type_id: &'static mut SystemString,
    hash: &'static mut SystemString,
    ship_pattern: &'static mut ShipPattern
}

#[derive(Debug, Mono)]
#[repr(C)]
pub struct UnitObject {
    #[base]
    component: UnityComponent,
    unit_info_bind: *mut UnitInfoBind,
}

impl UnitObject {
    pub fn get_component(&self) -> &UnityComponent {
        return &self.component;
    }

    pub fn get_info_bind(&self) -> Option<&UnitInfoBind> {
        match self.unit_info_bind.is_null() {
            true => return None,
            false => unsafe { Some(&*self.unit_info_bind) },
        }
    }
}

#[derive(Debug, Mono)]
#[repr(C)]
pub struct UnitInfoBind {
    #[base]
    component: UnityComponent,
    pub unit_id: *mut SystemString,
    pub own_id: *mut SystemString,
    pub init_scale: *mut Il2cppList<Vector3>,
    pub click_area: i32,
    pub glow_amount: f32,
    pub glossiness: f32,
    pub unit: *mut Unit
}

impl UnitInfoBind {
    pub fn get_component(&self) -> &UnityComponent {
        &self.component
    }

    pub fn get_unit_id(&self) -> String {
        (unsafe { &*self.unit_id }).to_string()
    }

    pub fn get_own_id(&self) -> String {
        (unsafe { &*self.own_id }).to_string()
    }
}

#[derive(Debug, Mono)]
#[repr(C)]
pub struct UnitInfo {
    #[base]
    object: SystemObject
}

#[derive(Debug, Mono, Getters)]
#[getset(get = "pub with_prefix")]
#[repr(C)]
pub struct ShipPattern {
    #[base]
    #[getset(skip)]
    object: SystemObject,
    #[getset(skip)]
    _pad0: [u8;0x74],
    unit_kind: UnitKind
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnitKind {
    None,
    Player,
    Collect,
    Monster,
    Npc,
    Other,
    Pet,
    Mine,
    Drone,
}