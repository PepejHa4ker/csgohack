#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate nom;

use winapi::_core::marker::PhantomData;

use crate::mem::{Process, Module};

use std::thread::sleep;
use std::time::Duration;
use crate::config::Config;
use std::process::exit;
use std::collections::BTreeMap;
use crate::entities::{LocalPlayer, Player, EntityPlayer};
use crate::cheats::*;
use std::fmt::Debug;
use crate::settings::Settings;

use std::sync::{Arc, Mutex};
// use crate::gui::render::UI;
use crate::gui::app::App;
use std::thread;
use std::rc::Rc;
use eframe::egui;
use crate::gui::components::frame_history::FrameHistory;
use eframe::egui::Ui;
use winapi::um::wincon::FreeConsole;
use crate::util::helpers::cheat_dir;
use crate::cheats::fov::FOVOverride;


pub mod entities;
pub mod gui;
pub mod mem;
pub mod config;
pub mod sigscan;
pub mod cheats;
pub mod settings;
pub mod util;
pub mod data;

type Map<T> = BTreeMap<String, T>;

#[derive(Clone)]
pub struct Runtime {
    pub process: Process,
    pub client: usize,
    pub engine: usize,
    pub netvars: Map<usize>,
    pub signatures: Map<usize>,
    pub settings: Arc<Mutex<Settings>>,
}

impl Runtime {
    pub fn get_address(&self, offset: usize, client: bool) -> usize {
        if client {
            self.client + offset
        } else {
            self.engine + offset
        }
    }

    pub unsafe fn get_local_player(&self) -> Option<LocalPlayer> {
        LocalPlayer::new(self)
    }
    /*
        // // pub fn get_map_name(&self) -> Bsp {
        //     let data = std::fs::read("maps/de_dust2.bsp").unwrap();
            let bsp = data::bsp::lib::Bsp::read(&data).unwrap();
            bsp
        }

        pub fn trace_ray(&self, origin: &Vector, dist: &Vector, out: &mut Trace, bsp: &Bsp) {
            if !bsp.planes.is_empty() {
                out.fraction = 1.0;
                out.fraction_left_solid = 0.0;
                self.ray_cast_node(0, 0.0, 1.0, origin, dist, out, bsp);
                if out.fraction < 1.0 {
                    out.end_pos.x = origin.x + out.fraction + dist.x - origin.x;
                    out.end_pos.y = origin.y + out.fraction + dist.y - origin.y;
                    out.end_pos.z = origin.z + out.fraction + dist.z - origin.z;
                }
            }
        }


        pub fn ray_cast_node(&self, node_index: i32, start_fraction: f32, end_fraction: f32, origin: &Vector, dest: &Vector, out: &mut Trace, bsp: &Bsp) {
            if out.fraction <= start_fraction {
                return ();
            }
            if node_index < 0 {
                let leaf = bsp.leaves.get((-node_index - 1) as usize).unwrap();
                for i in 0..leaf.leaf_brush_count {
                    let brush_index = bsp.leaf_brushes.get((leaf.first_leaf_brush + i) as usize).unwrap().brush;
                    let brush = bsp.brushes.get(brush_index as usize);
                    if brush.is_none() || !(brush.unwrap().texture & (0x1 | 0x4000 | 0x2000000 | 0x2 | 0x4000000 | 0x8) == 0) {
                        continue;
                    }
                    let brush = brush.unwrap().clone();
                    self.ray_cast_brush(brush, out, origin, dest, bsp);
                    if out.fraction == 0.0 {
                        return ();
                    }
                    out.brush = brush;
                }

                if out.start_solid || out.fraction < 1.0 {
                    return ();
                }
                for i in 0..leaf.leaf_face_count {
                    // ray_cast_surface(bsp.leaf_faces.get((leaf.first_leaf_face + i) as usize), origin, dest, out, bsp);
                }
                return ();
            }

            let node = bsp.nodes.get(node_index as usize);
            if node.is_none() {
                return ();
            }
            let mut plane = bsp.plane(node.unwrap().plane_index as usize);
            if plane.is_none() {
                return ();
            }

            let mut start_dist = 0.0f32;
            let mut end_dist = 0.0f32;

            if plane.unwrap().ty < 3 {
                match plane.unwrap().ty {
                    0 => {
                        start_dist = origin.x - plane.unwrap().dist;
                        end_dist = dest.x - plane.unwrap().dist;
                    }
                    1 => {
                        start_dist = origin.y - plane.unwrap().dist;
                        end_dist = dest.y - plane.unwrap().dist;
                    }
                    2 => {
                        start_dist = origin.z - plane.unwrap().dist;
                        end_dist = dest.z - plane.unwrap().dist;
                    }
                    _ => {}
                }
            } else {
                start_dist = (origin.x * plane.unwrap().normal.x + origin.y * plane.unwrap().normal.y + origin.z * plane.unwrap().normal.z) - plane.unwrap().dist;
                end_dist = (dest.x * plane.unwrap().normal.x + dest.y * plane.unwrap().normal.y + dest.z * plane.unwrap().normal.z) - plane.unwrap().dist;
            }

            if start_dist >= 0.0 && end_dist >= 0.0 {
                self.ray_cast_node(*node.unwrap().children.get(0).unwrap(), start_fraction, end_fraction, origin, dest, out, bsp)
            } else if start_dist < 0.0 && end_dist < 0.0 {
                self.ray_cast_node(*node.unwrap().children.get(1).unwrap(), start_fraction, end_fraction, origin, dest, out, bsp)
            } else {
                let mut side_id = 0;
                let mut fraction_first = 0.0_f32;
                let mut fraction_second = 0.0_f32;
                let mut fraction_middle = 0.0_f32;
                let mut middle = Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                if start_dist < end_dist {
                    side_id = 1;
                    let inv_distance = 1.0 / (start_dist - end_dist);
                    fraction_first = (start_dist + f32::EPSILON) * inv_distance;
                    fraction_second = (start_dist + f32::EPSILON) * inv_distance;
                } else if end_dist < start_dist {
                    side_id = 0;
                    let inv_distance = 1.0 / (start_dist - end_dist);
                    fraction_first = (start_dist + f32::EPSILON) * inv_distance;
                    fraction_second = (start_dist - f32::EPSILON) * inv_distance;
                } else {
                    side_id = 0;
                    fraction_first = 1.0;
                    fraction_second = 0.0;
                }
                if fraction_first < 0.0 {
                    fraction_first = 0.0;
                } else if fraction_first > 1.0 {
                    fraction_first = 1.0;
                }
                if fraction_second < 0.0 {
                    fraction_second = 0.0;
                } else if fraction_second > 1.0 {
                    fraction_second = 1.0;
                }
                fraction_middle = start_fraction + (end_fraction - start_fraction) * fraction_first;
                middle.x = origin.x + fraction_first * (dest.x - origin.x);
                middle.y = origin.y + fraction_first * (dest.y - origin.y);
                middle.z = origin.z + fraction_first * (dest.z - origin.z);
                self.ray_cast_node(*node.unwrap().children.get(side_id).unwrap(), start_fraction, fraction_middle, origin, &middle, out, bsp);
                fraction_middle = start_fraction + (end_fraction - start_fraction) * fraction_second;
                middle.x = origin.x + fraction_second * (dest.x - origin.x);
                middle.y = origin.y + fraction_second * (dest.y - origin.y);
                middle.z = origin.z + fraction_second * (dest.z - origin.z);
                self.ray_cast_node(*node.unwrap().children.get(!side_id).unwrap(), fraction_middle, end_fraction, &middle, dest, out, bsp)
            }
        }

        pub fn ray_cast_brush(&self, brush: Brush, trace: &mut Trace, origin: &Vector, dest: &Vector, bsp: &Bsp) {
            if brush.num_brush_sides == 0 {
                return ();
            }
            let mut fraction_to_enter = -99.0_f32;
            let mut fraction_to_leave = 1.0_f32;
            let mut starts_out = false;
            let mut ends_out = false;
            for i in 0..brush.num_brush_sides {
                let mut brush_side = bsp.brush_sides.get((brush.brush_side + i) as usize);
                if brush_side.is_none() || brush_side.unwrap().bevel == 0 {
                    continue;
                }
                let brush_side = brush_side.unwrap();
                let plane = bsp.planes.get(brush_side.plane as usize);
                if plane.is_none() {
                    continue;
                }
                let plane = plane.unwrap();
                let mut start_distance = (origin.x * plane.normal.x + origin.y * plane.normal.y + origin.z * plane.normal.z) - plane.dist;
                let mut end_distance = (dest.x * plane.normal.x + dest.y * plane.normal.y + dest.z * plane.normal.z) - plane.dist;
                if start_distance > 0.0 {
                    starts_out = true;
                    if end_distance > 0.0 {
                        return ();
                    }
                } else {
                    if end_distance <= 0.0 {
                        continue;
                    }
                    ends_out = true
                }
                if start_distance > end_distance {
                    let mut fraction = (start_distance - 0.03125).max(0.0);
                    fraction = fraction / (start_distance - end_distance);
                    if fraction > fraction_to_enter {
                        fraction_to_enter = fraction;
                    }
                } else {
                    let mut fraction = (start_distance + 0.03125) / (start_distance-end_distance);
                    if fraction < fraction_to_leave {
                        fraction_to_leave = fraction;
                    }
                }
            }
            if starts_out {
                if trace.fraction_left_solid - fraction_to_enter > 0.0 {
                    starts_out = false
                }
            }

            if !starts_out {
                trace.start_solid = true;
                trace.contents = brush.texture as i32;

                if !ends_out {
                    trace.all_solid = true;
                    trace.fraction = 0.0;
                    trace.fraction_left_solid = 0.0;
                } else {
                    if fraction_to_leave != 1.0 && fraction_to_leave > trace.fraction_left_solid {
                        trace.fraction_left_solid = fraction_to_leave;
                        if trace.fraction <= fraction_to_leave {
                            trace.fraction = 1.0;
                        }
                    }
                }
                return ();
            }

            if fraction_to_enter < fraction_to_leave {
                if fraction_to_enter > -99.0 && fraction_to_enter < trace.fraction {
                    if fraction_to_enter < 0.0 {
                        fraction_to_enter = 0.0;
                    }
                    trace.fraction = fraction_to_enter;
                    trace.contents = brush.texture as i32;
                    trace.brush = brush;
                }
            }
        }

    */


// pub fn get_map_name()

    #[inline]
    pub fn get_signature(&self, signature: &'static str) -> usize {
        *self.signatures.get(signature).unwrap()
    }

    #[inline]
    pub fn get_netvar(&self, netvar: &'static str) -> usize {
        self.get_netvar_safely(netvar).unwrap()
    }

    #[inline]
    pub fn get_netvar_safely(&self, netvar: &'static str) -> Option<usize> {
        self.netvars.get(netvar).map(|a| *a)
    }

    #[inline]
    pub unsafe fn get_entities(&self) -> impl Iterator<Item=EntityPlayer> {
        (1..16).map(move |i| EntityPlayer::get(self, i))
            .flatten()
            .filter(|entity| entity.is_alive())
    }

    #[inline]
    pub unsafe fn get_enemies(&self) -> impl Iterator<Item=EntityPlayer> {
        self.get_entities()
            .filter(move |enemy| enemy.get_team() != self.get_local_player().unwrap().get_team())
    }

    #[inline]
    pub unsafe fn write_client<T>(&self, value: &T) {
        self.process
            .write(self.client as u32, value);
    }

    #[inline]
    pub unsafe fn write_engine<T>(&self, value: &T) {
        self.process
            .write(self.engine as u32, value);
    }

    #[inline]
    pub unsafe fn read_ptr<T>(&self, offset: usize, client: bool) -> Option<RemotePtr<T>> {
        let address: usize = self.read_offset(offset, client).expect(&*format!("Failed to read pointer: 0x{:X}", offset));
        if address == 0 {
            None
        } else {
            Some(RemotePtr {
                address,
                runtime: self,
                inner: PhantomData,
            })
        }
    }

    #[inline]
    pub unsafe fn read_offset<T>(&self, offset: usize, client: bool) -> Option<T> {
        let address = self.get_address(offset, client);
        self.process.read(address)
    }


    #[inline]
    pub unsafe fn write_offset<T>(&self, offset: usize, value: &T, client: bool) {
        let address = self.get_address(offset, client);
        self.process.write(address as u32, value);
    }
}

#[derive(Clone)]
pub struct RemotePtr<'a, T> {
    address: usize,
    runtime: &'a Runtime,
    inner: PhantomData<T>,
}

impl<'a, T> RemotePtr<'a, T> {
    #[inline]
    pub unsafe fn read(&self) -> T {
        self.runtime
            .process
            .read(self.address)
            .expect(format!("Failed to read pointer: 0x{:16X}", self.address).as_str())
    }

    #[inline]
    pub unsafe fn write(&self, value: &T) -> bool {
        self.runtime
            .process
            .write(self.address as u32, value)
    }

    #[inline]
    pub fn add(&self, offset: usize) -> Self {
        Self {
            address: self.address + offset,
            ..*self
        }
    }

    #[inline]
    pub fn cast<R>(&self) -> RemotePtr<R> {
        RemotePtr {
            address: self.address,
            runtime: self.runtime,
            inner: PhantomData,
        }
    }
}

pub unsafe trait CheatModule: Send {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings);
}

fn main() {
    unsafe {
        FreeConsole();
    }
    std::env::set_current_dir(cheat_dir()).expect("chdir failed");


    let mut cheats= Vec::<Box<dyn CheatModule>>::with_capacity(10);
    cheats.push(Box::new(WallHack::new()));
    cheats.push(Box::new(BHop::new()));
    cheats.push(Box::new(AimAssist::new()));
    cheats.push(Box::new(Trigger::new()));
    cheats.push(Box::new(Aimbot::new()));
    cheats.push(Box::new(Radar::new()));
    cheats.push(Box::new(Recoil::new()));
    cheats.push(Box::new(FastTap::new()));
    cheats.push(Box::new(AntiFlash::new()));
    cheats.push(Box::new(FOVOverride::new()));
    inject_cheats(cheats);
}


fn inject_cheats(mut cheats: Vec<Box<dyn CheatModule>>) {
    let config = Config::load();
    let process = mem::from_name(&config.executable)
        .ok_or_else(|| {
            error!("Could not open process {}!", config.executable);
            exit(1);
        })
        .unwrap();


    let client = process.get_module("client.dll")
        .or_else(|| process.get_module("client_panorama.dll"))
        .unwrap();
    let engine = process.get_module("engine.dll").unwrap().base;
    let sigs = scan_signatures(&config, &process);
    let netvars = scan_netvars(&sigs, &config, &client).unwrap();
    let settings = Arc::new(Mutex::new(Settings::load().expect("Failed to load config")));
    let runtime = Runtime {
        process,
        client: client.base,
        engine,
        netvars,
        signatures: sigs,
        settings,
    };


    let settings = Arc::clone(&runtime.settings);
    let app = App::create(settings);
    let native_options = eframe::NativeOptions::default();
    thread::spawn(move || {
        eframe::run_native(Box::new(app), native_options);
    });
    let settings = Arc::clone(&runtime.settings);
    loop {
        let settings = settings.lock().unwrap();
        unsafe {
            if let Some(player) = runtime.get_local_player() {
                for cheat in &mut cheats {
                    cheat.handle(&player, &settings);
                }
            }
        }
        drop(settings);
        sleep(Duration::from_millis(1));
    }
}


pub trait Inverse {
    fn inverse(&mut self);
}

impl Inverse for bool {
    fn inverse(&mut self) {
        *self = !*self;
    }
}

/// Scan the signatures from the config and return a `Map<usize>`.
fn scan_signatures(conf: &Config, process: &Process) -> Map<usize> {
    debug!(
        "Starting signature scanning: {} items",
        conf.signatures.len()
    );
    let mut res = BTreeMap::new();

    for sig in &conf.signatures {
        match sigscan::find_signature(sig, process) {
            Ok(r) => {
                res.insert(sig.name.clone(), r);
                info!("Found signature: {} => {:#X}", sig.name, r);
            }
            Err(err) => warn!("{} sigscan failed: {:?}", sig.name, err),
        };
    }

    info!(
        "Finished signature scanning: {}/{} items successful",
        res.len(),
        conf.signatures.len()
    );
    res
}

/// Scan the netvars from the config and return a `Option<Map<i32>>`.
fn scan_netvars(sigs: &Map<usize>, conf: &Config, client_module: &Module) -> Option<Map<usize>> {
    debug!("Starting netvar scanning: {} items", conf.netvars.len());

    let first = sigs.get("dwGetAllClasses")?;
    let netvars = mem::csgo::NetvarManager::new(*first, client_module)?;

    let mut res = Map::new();
    for netvar in &conf.netvars {
        match netvars.get_offset(&netvar.table, &netvar.prop) {
            Some(o) => {
                res.insert(netvar.name.clone(), o as usize + netvar.offset);
                info!("Found netvar: {} => {:#X}", netvar.name, o);
            }
            None => warn!("{} netvar failed!", netvar.name),
        };
    }

    debug!(
        "Finished netvar scanning: {}/{} items successful",
        res.len(),
        conf.netvars.len()
    );
    Some(res)
}
