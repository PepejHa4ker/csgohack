use crate::{RemotePtr, Runtime};
use cgmath::{Vector3, Vector2, InnerSpace, MetricSpace};

use std::thread::sleep;
use std::time::Duration;
use itertools::Itertools;
use crate::settings::Settings;
use crate::entities::EnemySelectingStrategy::*;
use crate::util::math::{fov, calculate_angle, truncate_y_vector, Matrix3x4};
use std::slice::Iter;


pub unsafe trait Player<'a> {
    /// Returns the base player pointer
    fn get_base_ptr(&self) -> RemotePtr<'a, usize>;


    /// Adds the netvar offset to base player pointer
    #[inline]
    unsafe fn add_netvar(&self, netvar: &'static str) -> RemotePtr<'a, usize> {
        self.get_base_ptr().add(self.get_runtime().get_netvar(netvar))
    }

    /// Adds the signature offset to base player pointer
    #[inline]
    unsafe fn add_signature(&self, signature: &'static str) -> RemotePtr<'a, usize> {
        self.get_base_ptr().add(self.get_runtime().get_signature(signature))
    }

    /// Reads the value from game process by netvar offset
    #[inline]
    unsafe fn read_netvar<N>(&self, netvar: &'static str) -> N {
        self.add_netvar(netvar).cast().read()
    }

    /// Reads the value from game process by signature offset
    #[inline]
    unsafe fn read_singature<S>(&self, signature: &'static str) -> S {
        self.add_signature(signature).cast().read()
    }

    /// Writes the value to game process by netvar offset
    #[inline]
    unsafe fn write_netvar<N>(&self, netvar: &'static str, value: &N) {
        self.add_netvar(netvar).cast().write(value);
    }

    /// Writes the value to game process by signature offset
    #[inline]
    unsafe fn write_singature<S>(&self, signature: &'static str, value: &S) {
        self.add_signature(signature).cast().write(value);
    }

    /// Returns player head bone position vector
    #[inline]
    unsafe fn get_head_bone_position(&self) -> Option<Vector3<f32>> {
        self.get_bone_position(8)
    }

    /// Returns the player bone matrix pointer
    #[inline]
    unsafe fn get_bone_matrix_ptr(&self) -> Option<usize> {
        let global_bone_matrix_ptr = self.get_runtime().get_netvar_safely("m_dwBoneMatrix")?;
        Some(self.get_base_ptr().add(global_bone_matrix_ptr).read())
    }

    /// Returns player head bone position vector
    ///
    /// # Arguments
    ///
    /// * `bone` - The bone index to get (there are 128 bones total)
    /// Returns player head bone position vector
    ///
    /// # Arguments
    ///
    /// * `bone` - The bone index to get (there are 128 bones total)
    #[inline]
    unsafe fn get_bone_position(&self, bone: usize) -> Option<Vector3<f32>> {
        let matrix: Matrix3x4 = self.get_runtime().process.read(self.get_bone_matrix_ptr()? + (0x30 * bone))?;
        Some(Vector3::new(matrix.x.w, matrix.y.w, matrix.z.w))
    }

    /// Returns player health
    #[inline]
    unsafe fn get_health(&self) -> usize {
        self.read_netvar("m_iHealth")
    }

    /// Returns player team value
    #[inline]
    unsafe fn get_team(&self) -> usize {
        self.read_netvar("m_iTeamNum")
    }

    /// Returns player alive state
    #[inline]
    unsafe fn is_alive(&self) -> bool {
        let health = self.get_health();
        health > 0 && health <= 100
    }

    /// Returns player posiiton vector
    #[inline]
    unsafe fn get_position(&self) -> Vector3<f32> {
        self.read_netvar("m_vecOrigin")
    }

    /// Calculates distance between player and local player by X and Z coordinate (excluding Y)
    ///
    /// # Arguments
    ///
    /// * `lp` - The local player reference
    #[inline]
    unsafe fn get_distance_flatten(&self, lp: &LocalPlayer) -> f32 {
        truncate_y_vector(self.get_position() - lp.get_position()).magnitude()
    }

    /// Returns player index in map
    #[inline]
    unsafe fn get_index(&self) -> i32 {
        self.get_base_ptr().add(0x64).cast().read()
    }

    /// Returns player glow index
    #[inline]
    unsafe fn get_glow_index(&self) -> i32 {
        self.read_netvar("m_iGlowIndex")
    }

    /// Returns player flags
    #[inline]
    unsafe fn get_flags(&self) -> usize {
        self.read_netvar("m_fFlags")
    }

    /// Returns true if player is scoped
    #[inline]
    unsafe fn is_scoped(&self) -> bool {
        self.read_netvar("m_bIsScoped")
    }

    /// Returns CS:GO glow object manager pointer
    #[inline]
    unsafe fn get_glow_object(&self) -> RemotePtr<'a, usize> {
        self.get_runtime()
            .read_ptr::<usize>(self.get_runtime().get_signature("dwGlowObjectManager"), true)
            .unwrap()
    }

    /// Returns true if player is immune
    #[inline]
    unsafe fn is_immune(&self) -> bool {
        self.read_netvar("m_bGunGameImmunity")
    }

    /// Returns player Field Of View
    #[inline]
    unsafe fn get_fov(&self) -> i32 {
        self.read_netvar("m_iFOV")
    }

    /// Returns player crosshair id
    #[inline]
    unsafe fn get_crosshair_id(&self) -> Option<usize> {
        let temp: usize = self.read_netvar("m_iCrosshairId");
        if temp <= 0 || temp > 32 {
            return Some(temp);
        }
        None
    }

    /// Returns true if player is spotted
    #[inline]
    unsafe fn is_spotted(&self) -> bool {
        self.read_netvar("m_bSpotted")
    }

    /// Changes player spotted value
    ///
    /// # Arguments
    ///
    /// * `value` - The new spotted value
    #[inline]
    unsafe fn set_spotted(&self, value: bool) {
        self.write_netvar("m_bSpotted", &value)
    }

    /// Returns player view angles vector
    #[inline]
    unsafe fn get_view_angles(&self) -> Vector2<f32> {
        let runtime = self.get_runtime();
        runtime.read_ptr::<usize>(runtime.get_signature("dwClientState"), false).unwrap().add(runtime.get_signature("dwClientState_ViewAngles")).cast().read()
    }

    /// Returns player view offset
    #[inline]
    unsafe fn get_view_offset(&self) -> Vector3<f32> {
        self.read_netvar("m_vecViewOffset")
    }

    /// Returns player shots fired count
    #[inline]
    unsafe fn get_shots_fired(&self) -> i32 {
        self.read_netvar("m_iShotsFired")
    }

    /// Returns true if player is on ground
    #[inline]
    unsafe fn is_on_ground(&self) -> bool {
        self.get_flags() == 257
    }

    /// Returns flash duration (0 if player not flashed)
    #[inline]
    unsafe fn get_flash_duration(&self) -> f32 {
        self.read_netvar("m_flFlashDuration")
    }

    /// Returns player active weapon handle
    #[inline]
    unsafe fn get_active_weapon_handle(&self) -> i32 {
        self.read_netvar("m_hActiveWeapon")
    }

    /// Returns player active weapon entity
    ///
    /// # Arguments
    ///
    /// * `weapon_handle` - The weapon handle to get entity
    #[inline]
    unsafe fn get_active_weapon_entity(&self, weapon_handle: i32) -> Option<i32> {
        self.get_runtime().read_offset::<i32>(self.get_runtime().get_signature("dwEntityList") + (((weapon_handle & 0xFFF) - 1) * 0x10) as usize, true)
    }

    /// Returns player active weapon index
    #[inline]
    unsafe fn get_active_weapon_index(&self) -> Option<usize> {
        let definition_index = self.get_runtime().get_netvar_safely("m_iItemDefinitionIndex")?;
        let active_weapon_entity = self.get_active_weapon_entity(self.get_active_weapon_handle())? as usize;
        self.get_runtime().process.read(active_weapon_entity + definition_index)
    }

    /// Returns the runtime reference (Needed to implements default trait functions)
    fn get_runtime(&self) -> &'a Runtime;
}

/// The local player instance
pub struct LocalPlayer<'a> {
    /// the runtime reference
    runtime: &'a Runtime,
    /// the base local player pointer
    inner: RemotePtr<'a, usize>,
}

/// Entity instance
pub struct EntityPlayer<'a> {
    /// The runtime reference
    runtime: &'a Runtime,
    /// the base entity pointer
    inner: RemotePtr<'a, usize>,
}

unsafe impl<'a> Player<'a> for EntityPlayer<'a> {
    /// Returns the base entity player pointer (client.dll module offset + dwEntityList signature offset * (entity index * 0x10))
    fn get_base_ptr(&self) -> RemotePtr<'a, usize> {
        self.inner.clone()
    }

    /// Returns the runtime reference
    fn get_runtime(&self) -> &'a Runtime {
        self.runtime
    }
}


impl<'a> EntityPlayer<'a> {
    /// Returns entity player instance by index
    ///
    /// # Arguments
    ///
    /// * `runtime` - The runtime reference to read the signature
    /// * `index` The entity index to read
    pub unsafe fn get(runtime: &'a Runtime, index: usize) -> Option<EntityPlayer<'a>> {
        let inner = runtime.read_ptr::<usize>(runtime.get_signature("dwEntityList") + (index * 0x10), true)?;
        Some(EntityPlayer {
            runtime,
            inner,
        })
    }
}

unsafe impl<'a> Player<'a> for LocalPlayer<'a> {
    /// Returns the base local player pointer (client.dll module offset + dwLocalPlayer signature offset)
    fn get_base_ptr(&self) -> RemotePtr<'a, usize> {
        self.inner.clone()
    }

    /// Returns the runtime reference
    fn get_runtime(&self) -> &'a Runtime {
        self.runtime
    }
}

impl<'a> LocalPlayer<'a> {
    /// Returns a new local player instance
    ///
    /// # Arguments
    ///
    /// * `runtime` - The runtime to read signature
    pub unsafe fn new(runtime: &'a Runtime) -> Option<Self> {
        let inner = runtime.read_ptr::<usize>(runtime.get_signature("dwLocalPlayer"), true)?;
        Some(LocalPlayer {
            runtime,
            inner,
        })
    }

    /// Changes the player fov
    ///
    /// # Arguments
    ///
    /// * `fov` - The new fov value to set
    #[inline]
    pub unsafe fn set_fov(&self, fov: i32) {
        self.write_netvar("m_iFOV", &fov);
    }

    /// Makes the player jump
    #[inline]
    pub unsafe fn force_jump(&self) {
        self.runtime.write_offset(self.runtime.get_signature("dwForceJump"), &5, true);
        sleep(Duration::from_millis(1));
        self.runtime.write_offset(self.runtime.get_signature("dwForceJump"), &4, true);
    }

    /// Makes the player attack
    #[inline]
    pub unsafe fn force_attack(&self) {
        self.runtime.write_offset(self.runtime.get_signature("dwForceAttack"), &5, true);
        sleep(Duration::from_millis(1));
        self.runtime.write_offset(self.runtime.get_signature("dwForceAttack"), &4, true);
    }

    /// Changes the player flash duration
    ///
    /// # Arguments
    ///
    /// * `duration` - The new duration value to set
    #[inline]
    pub unsafe fn set_flash_duration(&self, duration: f32) {
        self.write_netvar("m_flFlashDuration", &duration);
    }

    /// Changes the player velocity vector
    ///
    /// # Arguments
    ///
    /// * `velocity` - The new velocity value to set
    #[inline]
    pub unsafe fn set_velocity(&self, velocity: Vector3<f32>) {
        self.write_netvar("m_vecVelocity", &velocity)
    }

    // Changes the player view angles
    ///
    /// # Arguments
    ///
    /// * `angles` - The new angles value to set
    #[inline]
    pub unsafe fn set_view_angles(&self, angles: Vector2<f32>) {
        self.runtime.read_ptr::<usize>(
            self.runtime.get_signature("dwClientState"), false).unwrap().add(self.runtime.get_signature("dwClientState_ViewAngles")).cast().write(&angles);
    }

    /// Returns the player aim punch angle vector value
    #[inline]
    pub unsafe fn get_punch_angles(&self) -> Vector2<f32> {
        self.read_netvar("m_aimPunchAngle")
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

impl std::fmt::Display for EnemySelectingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl EnemySelectingStrategy {
    pub fn iter() -> Iter<'static, EnemySelectingStrategy> {
        static STRATEGIES: [EnemySelectingStrategy; 5] = [Health, Distance, DistanceFlatten, Angle, Index];
        STRATEGIES.iter()
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





