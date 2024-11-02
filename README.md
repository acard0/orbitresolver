tooling base for il2cpp unity game.

# sample code that randomly selects player ships.
- finds component system class instance via pattern matching
- filters Player game objects and picks random player
- creates an instance of ShipSelectRequest DTO object using mono activator fn exported by il2cpp
- sends the ShipSelectRequest to the server using the network session object found via pattern matching

```rust
#![allow(non_snake_case)]
#![allow(unused)]

use std::{ptr::eq, thread, time::{Duration, Instant}};

use il2cppinterop_core::{il2cpp_farproc, mono::{self, definitions::object::{self, SystemObject}, reflection::{class, domain}, runtime::{self, Il2cppObject}}, unity::{camera::{self, CameraEye}, engine::Vector3, object::find_objects_of_type_by_name}, utils::debug, MainParams};
use retour::static_detour;

use crate::definitions::{commands::{movement::MoveRequest, targeting::ShipSelectRequest}, components::{session::Session, unit::{Unit, UnitComponent, UnitKind, UnitObject}}, exports::DARKORBIT_SESSION_SYSTEM};

pub unsafe fn main(_params: MainParams) -> u32 {  
    preinit();

    thread::spawn(|| unsafe { logical() });

    1
}

unsafe fn preinit() {
    hook_setup_SessionSystemSend();
}

unsafe fn logical() {
    let mut session: Option<&mut Session> = None;
    let mut unit_component: Option<&mut UnitComponent> = None;

    loop {
        let pth = runtime::thread::attach(domain::get());

        if wait_session(&mut session) && wait_unit_component(&mut unit_component) {
            if let (Some(session), Some(unit_component)) = (session.as_mut(), unit_component.as_mut()) {
                let now = Instant::now();
                step(session, unit_component);
                println!("took {:?} to step", now.elapsed());
            }
        }

        runtime::thread::detach(pth);
        thread::sleep(Duration::from_millis(150));
    }
}

fn step(session: &mut Session, components: &mut UnitComponent) {
    let ships = components.get_units().expect("DarkOrbit.UnitComponent.Units has null ptr")
        .into_iter().filter(|entry| 
            entry.get_value().is_some_and(|unit| 
                *unit.get_ship_pattern().get_unit_kind() == UnitKind::Player
            )
        ).collect::<Vec<_>>();

    println!("player count: {}", ships.len());

    let random_index = (rand::random::<f64>() * (ships.len() as f64)) as usize;
    let random = ships.get(random_index).unwrap();
    println!("random unit from collection: {}, id: {}", random.get_key().unwrap().to_string(), random.get_value().unwrap().get_id());

    select_ship(session, random.get_value().unwrap());
}

fn select_ship(session: &mut Session, unit: &Unit) -> bool { unsafe {
    let request = ShipSelectRequest::new();
    //  {"targetId":"Some(150200793)", "targetX":"Some(2794)", "targetY":"Some(-6836)", "posX":"Some(2794)", "posY":"Some(-6840)", "clickX":"Some(1088)", "clickY":"Some(483)", "radius":"Some(45)", }

    // you would really want to calculate screen-pos and set target pos accordingly but its skipped here.
    // server only checks validity of the params, does not compare with server-side state, yet.
    request.set_target_id(*unit.get_id() as i32).set_target_x(2794).set_target_y(-6836)
        .set_pos_x(2794).set_pos_y(-6840)
        .set_click_x(1088).set_click_y(483)
        .set_radius(45);

    send_to_session(session, request)
}}

fn move_hero(session: &mut Session, world_x: i32, world_y: i32) -> bool { unsafe {
// il2cppinterop_core::mono::definitions::object::new_from_namespace("Game.MoveRequest")
            .expect("Failed to create Game.MoveRequest")
    let request = MoveRequest::new()
        .set_target_x(world_x).set_target_y(world_y)
        /*.set_position_x(val).set_position_y(val)*/; // these should be set for proper request, user interaction points
    
    send_to_session(session, request)
}}

fn send_to_session(session: &mut Session, module: &SystemObject) -> bool { unsafe {
    debug::intermediate_serialize(module);
    class::get_method_pointer_by_name(&DARKORBIT_SESSION_SYSTEM, "Send", 2)
        .inspect(|mptr| il2cpp_farproc!(fn(&Session, &SystemObject), *mptr) (session, module))
        .is_some()
}}

fn wait_session(current: &mut Option<&'static mut Session>) -> bool {
    match current {
        Some(instance) if !instance.is_connected() => {
            log::info!("Session disconnected");
            *current = None;
            false
        }
        None => {
            log::info!("Waiting for session");
            *current = get_session();
            if let Some(_) = current {
                log::info!("Session detected");
                return true;
            }
            false
        }
        _ => true
    }
}

fn wait_unit_component(current: &mut Option<&'static mut UnitComponent>) -> bool {
    match current {
        Some(instance) if instance.is_invalid() => {
            log::info!("Unit Component is invalidated");
            *current = None;
            false
        }
        None => {
            log::info!("Waiting for unit component");
            *current = get_unit_component();
            if let Some(_) = current {
                log::info!("Unit Component found");
                return true;
            }
            false
        }
        _ => true
    }
}

/// Attempts to find DarkOrbit.Session mono object via pattern matching
fn get_session() -> Option<&'static mut Session> {
    let cz_session = class::find("DarkOrbit.Session")
        .expect("failed to find Session class");

    let session: Option<*mut Session> = mono::definitions::object::find(cz_session, |address| { unsafe {
        *(address.add(0x8) as *const isize) == 0x0 // _children
        && *(address.add(0xC) as *const isize) == 0x0 // _childrenDB
        && *(address.add(0x10) as *const isize) == 0x1a // _status
    }}).expect("Encountered error while searching for DarkOrbit.Session instance");

    session.map(|ptr| unsafe { &mut *ptr })
}

/// Attempts to find DarkOrbit.UnitComponent mono object via pattern matching
fn get_unit_component() -> Option<&'static mut UnitComponent> { unsafe {
    let cz_unitCompnent = class::find("DarkOrbit.UnitComponent")
    .expect("failed to find UnitComponent class");

    let component: Option<*mut UnitComponent> = object::find(cz_unitCompnent, |address| {
        *(address.add(0x8) as *const isize) != 0x0 // _children
        && *(address.add(0xC) as *const isize) == 0x0 // _childrenDB
        && *(address.add(0x10) as *const isize) == 0x1e // _status
    }).expect("Encountered error while searching for DarkOrbit.UnitComponent instance");

    component.and_then(|a| a.as_mut())
}}

/// Hooks into DarkOrbit.SessionSystem::Send
unsafe fn hook_setup_SessionSystemSend() {
    // see https://github.com/darfink/detour-rs
    static_detour! { static Hook_SessionSystem_Send: unsafe extern "C" fn(*mut Session, *mut SystemObject); }

    fn mHook_SessionSystem_Send(_session: *mut Session, _module: *mut SystemObject) { unsafe {
        let module = &*_module;
        debug::intermediate_serialize(module);
       
        Hook_SessionSystem_Send.call(_session, _module);
    }}

    let mp_sessionSystem_Send = class::get_method_pointer_by_name("DarkOrbit.SessionSystem", "Send", 2)
        .expect("failed to find DarkOrbit.SessionSystem::Send");

    match Hook_SessionSystem_Send
        .initialize(std::mem::transmute(mp_sessionSystem_Send), mHook_SessionSystem_Send)
        .and_then(|_| {
            Hook_SessionSystem_Send.enable()
        }) {
            Ok(_) => log::info!("DarkOrbit.SessionSystem::Send detour set. Instruction adr: {mp_sessionSystem_Send:p}"),
            Err(err) => log::error!("Failed to set DarkOrbit.SessionSystem::Send detour. Instruction adr: {mp_sessionSystem_Send:p}. {:?}", err),
        }
}

/// Iterates through id to unit dictionary and prints out the key-value pair
fn iterate_units(components: &mut UnitComponent) {
    let units = components.get_units().expect("DarkOrbit.UnitComponent.Units has null ptr");
    log::info!("units Units: {:p}, count: {}", units as *const _, units.get_count());

    units.into_iter().for_each(|entry| {
        let key = entry.get_key().unwrap();
        if let Some(value) = entry.get_value() {
            println!("UnitInfos iter, entry: {:p}, seems valid? K: {}, V: {}", entry, key.to_string(), value);
        }
    });
}

/// Prints out UnitObjects found via unity icall (UnityObject::FindObjectsOfType(System.Type,System.Boolean))
fn print_unit_objects() {
    let camera = camera::get_main();

    match find_objects_of_type_by_name("DarkOrbit.UnitObject",false) {
        Some(objects) => {
            objects.into_iter().for_each(|unitObject: &UnitObject| {
                let gameObject = unitObject.get_component().get_game_object();
                let transform = gameObject.get_transform();
                let locWorld = transform.get_position();
                let sname = unitObject.get_component().get_name();
    
                let mut locScreen = Vector3::default();
                camera.world_to_screen(&locWorld, CameraEye::Center, &mut locScreen);
    
                if unitObject.get_info_bind().is_some() {
                    println!("{}@{:p}, unit id: {}, @world(x:{}, y: {}, z: {}), @screen(x:{}, y: {}, z: {})",
                        sname, unitObject, unitObject.get_info_bind().unwrap().get_unit_id(), locWorld.x, locWorld.y, locWorld.z, locScreen.x, locScreen.y, locScreen.z
                    );
                }
            });
        },
        None => {}
    };
}
```
