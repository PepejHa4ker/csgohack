use crate::{RemotePtr, Runtime};
use cgmath::{Vector3, Vector2, InnerSpace, MetricSpace};

use std::thread::sleep;
use std::time::Duration;
use itertools::Itertools;
use crate::settings::Settings;
use crate::entities::EnemySelectingStrategy::*;
use crate::util::math::{fov, calculate_angle, truncate_y_vector};
use std::slice::Iter;


pub unsafe trait Player<'a> {
    fn get_base_ptr(&self) -> RemotePtr<'a, usize>;

    #[inline]
    unsafe fn get_head_bone_position(&self) -> Option<Vector3<f32>> {
        self.get_bone_position(8)
    }

    #[inline]
    unsafe fn get_bone_position(&self, bone: usize) -> Option<Vector3<f32>> {
        let runtime = self.get_runtime();
        if let Some(bone_matrix) = runtime.get_netvar_safely("m_dwBoneMatrix") {
            let self_bone = self.get_base_ptr().add(bone_matrix).cast::<usize>().read();
            if let Some(x) = runtime.process.read(self_bone + ((0x30 * bone) + 0x0C)) {
                let y: f32 = runtime.process.read(self_bone + ((0x30 * bone) + 0x1C)).unwrap();
                let z: f32 = runtime.process.read(self_bone + ((0x30 * bone) + 0x2C)).unwrap();
                return Some(Vector3::new(x, y, z));
            }
        }
        None
    }

    #[inline]
    unsafe fn get_health(&self) -> usize {
        self.get_base_ptr().read_netvar("m_iHealth")
    }

    #[inline]
    unsafe fn get_team(&self) -> usize {
        self.get_base_ptr().read_netvar("m_iTeamNum")
    }

    #[inline]
    unsafe fn is_alive(&self) -> bool {
        let health = self.get_health();
        health > 0 && health <= 100
    }

    #[inline]
    unsafe fn get_position(&self) -> Vector3<f32> {
        self.get_base_ptr().read_netvar("m_vecOrigin")
    }

    #[inline]
    unsafe fn get_distance_flatten(&self, other: &dyn Player) -> f32 {
       truncate_y_vector(self.get_position() - other.get_position()).magnitude()
    }

    #[inline]
    unsafe fn get_index(&self) -> i32 {
        self.get_base_ptr().add(0x64).cast().read()
    }

    #[inline]
    unsafe fn get_glow_index(&self) -> i32 {
        self.get_base_ptr().read_netvar("m_iGlowIndex")
    }

    #[inline]
    unsafe fn get_flags(&self) -> usize {
        self.get_base_ptr().read_netvar("m_fFlags")
    }

    #[inline]
    unsafe fn is_scoped(&self) -> bool {
        self.get_base_ptr().read_netvar("m_bIsScoped")
    }

    #[inline]
    unsafe fn get_glow_object(&self) -> RemotePtr<'a, usize> {
        self.get_runtime()
            .read_ptr::<usize>(self.get_runtime().get_signature("dwGlowObjectManager"), true)
            .unwrap()
    }

    #[inline]
    unsafe fn is_immune(&self) -> bool {
        self.get_base_ptr().read_netvar("m_bGunGameImmunity")
    }

    #[inline]
    unsafe fn get_fov(&self) -> i32 {
        self.get_base_ptr().read_netvar("m_iFOV")
    }

    #[inline]
    unsafe fn get_crosshair_id(&self) -> Option<usize> {
        let temp: usize = self.get_base_ptr().read_netvar("m_iCrosshairId");
        if temp <= 0 || temp > 32 {
            None
        } else {
            Some(temp)
        }
    }

    #[inline]
    unsafe fn is_spotted(&self) -> bool {
        self.get_base_ptr().read_netvar("m_bSpotted")
    }

    #[inline]
    unsafe fn set_spotted(&self, value: bool) {
        self.get_base_ptr().write_netvar("m_bSpotted", &value)
    }

    #[inline]
    unsafe fn get_view_angles(&self) -> Vector2<f32> {
        let runtime = self.get_runtime();
        runtime.read_ptr::<usize>(runtime.get_signature("dwClientState"), false).unwrap().add(runtime.get_signature("dwClientState_ViewAngles")).cast().read()
    }

    #[inline]
    unsafe fn get_view_offset(&self) -> Vector3<f32> {
        self.get_base_ptr().read_netvar("m_vecViewOffset")
    }

    #[inline]
    unsafe fn get_shots_fired(&self) -> i32 {
        self.get_base_ptr().read_netvar("m_iShotsFired")
    }

    #[inline]
    unsafe fn is_on_ground(&self) -> bool {
        self.get_flags() == 257
    }

    #[inline]
    unsafe fn get_flash_duration(&self) -> f32 {
        self.get_base_ptr().read_netvar("m_flFlashDuration")
    }

    #[inline]
    unsafe fn is_sniper_weapon_in_hand(&self) -> bool {
        let runtime = self.get_runtime();
        let init_wep: i32 = self.get_base_ptr().read_netvar("m_hActiveWeapon");
        if let Some(weapon_entity) = runtime.read_offset::<i32>(runtime.get_signature("dwEntityList") + (((init_wep & 0xFFF) - 1) * 0x10) as usize, true) {
            if let Some(idx) = runtime.get_netvar_safely("m_iItemDefinitionIndex") {
                if let Some(my_weapon) = runtime.process.read::<i32>((weapon_entity as usize) + idx) {
                    return my_weapon == 40 || my_weapon == 9 || my_weapon == 38 || my_weapon == 11;
                }
            }
        }

        false
    }

    fn get_runtime(&self) -> &'a Runtime;
}

pub struct LocalPlayer<'a> {
    runtime: &'a Runtime,
    inner: RemotePtr<'a, usize>,
}

pub struct EntityPlayer<'a> {
    runtime: &'a Runtime,
    inner: RemotePtr<'a, usize>,
}

unsafe impl<'a> Player<'a> for EntityPlayer<'a> {

    fn get_base_ptr(&self) -> RemotePtr<'a, usize> {
        self.inner.clone()
    }

    fn get_runtime(&self) -> &'a Runtime {
        self.runtime
    }
}

impl<'a> EntityPlayer<'a> {
    pub unsafe fn get(runtime: &'a Runtime, index: usize) -> Option<EntityPlayer<'a>> {
        let inner = runtime.read_ptr::<usize>(runtime.get_signature("dwEntityList") + (index * 0x10), true)?;
        Some(EntityPlayer {
            runtime,
            inner,
        })
    }
}

unsafe impl<'a> Player<'a> for LocalPlayer<'a> {
    fn get_base_ptr(&self) -> RemotePtr<'a, usize> {
        self.inner.clone()
    }

    fn get_runtime(&self) -> &'a Runtime {
        self.runtime
    }
}

impl<'a> LocalPlayer<'a> {
    pub unsafe fn new(runtime: &'a Runtime) -> Option<Self> {
        let inner = runtime.read_ptr::<usize>(runtime.get_signature("dwLocalPlayer"), true)?;
        Some(LocalPlayer {
            runtime,
            inner,
        })
    }

    #[inline]
    pub unsafe fn set_fov(&self, fov: i32) {
        self.get_base_ptr().write_netvar("m_iFOV", &fov);
    }

    #[inline]
    pub unsafe fn force_jump(&self) {
        self.runtime.write_offset(self.runtime.get_signature("dwForceJump"), &5, true);
        sleep(Duration::from_millis(1));
        self.runtime.write_offset(self.runtime.get_signature("dwForceJump"), &4, true);
    }

    #[inline]
    pub unsafe fn force_attack(&self) {
        self.runtime.write_offset(self.runtime.get_signature("dwForceAttack"), &5, true);
        sleep(Duration::from_millis(1));
        self.runtime.write_offset(self.runtime.get_signature("dwForceAttack"), &4, true);
    }

    #[inline]
    pub unsafe fn set_flash_duration(&self, duration: f32) {
        self.get_base_ptr().write_netvar("m_flFlashDuration", &duration);
    }

    #[inline]
    pub unsafe fn set_velocity(&self, velocity: Vector3<f32>) {
        self.get_base_ptr().write_netvar("m_vecVelocity", &velocity)
    }

    #[inline]
    pub unsafe fn set_view_angles(&self, angles: Vector2<f32>) {
        self.runtime.read_ptr::<usize>(
            self.runtime.get_signature("dwClientState"), false).unwrap().add(self.runtime.get_signature("dwClientState_ViewAngles")).cast().write(&angles);
    }

    #[inline]
    pub unsafe fn get_punch_angles(&self) -> Vector2<f32> {
        self.get_base_ptr().read_netvar("m_aimPunchAngle")
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum EnemySelectingStrategy {
    Health,
    Distance,
    DistanceFlatten,
    Angle,
    Index,
}


impl EnemySelectingStrategy {
    pub fn iter() -> Iter<'static, EnemySelectingStrategy> {
        static STRATEGIES: [EnemySelectingStrategy; 5] = [Health, Distance, DistanceFlatten, Angle, Index];
        STRATEGIES.iter()
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Health => {
                "Heath"
            }
            Distance => {
                "Distance"
            }
            DistanceFlatten => {
                "Distance Flatten"
            }
            Angle => {
                "Angle"
            }
            Index => {
                "Index"
            }
        }
    }
}



pub unsafe fn get_enemies_by_strategy<'a>(runtime: &'a Runtime, settings: &Settings) -> impl Iterator<Item=EntityPlayer<'a>> {
    let player = runtime.get_local_player().expect("Failed to get LocalPlayer");
    runtime.get_enemies().
        sorted_by_key(move |enemy| {
            match settings.enemy_selecting_strategy {
                Health => {
                    enemy.get_health()
                }
                Distance => {
                    enemy.get_position().distance(player.get_position()) as _
                }
                DistanceFlatten => {
                    enemy.get_distance_flatten(&player) as _
                }
                Angle => {
                    fov(player.get_view_angles(),
                        calculate_angle(&player, enemy.get_bone_position(settings.aim_target as usize).unwrap(), &settings),
                        player.get_position().distance(enemy.get_position()) as _) as _
                }
                Index => {
                    enemy.get_index() as _
                }
            }
        })
}





