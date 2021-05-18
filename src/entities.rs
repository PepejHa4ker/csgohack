use crate::offsets::*;
use crate::{RemotePtr, Runtime};
use cgmath::{Vector3, Vector2, InnerSpace};
use std::ffi::{CStr, OsStr};
use std::thread::sleep;
use std::time::Duration;
use std::ops::Deref;


pub unsafe trait Player<'a> {
    fn get_base_address(&self) -> RemotePtr<'a, usize>;

    unsafe fn get_head_bone_position(&self) -> Option<Vector3<f32>> {
        self.get_bone_position(8)
    }

    unsafe fn get_bone_position(&self, bone: usize) -> Option<Vector3<f32>> {
        let runtime = self.get_runtime();
        if let Some(bone_matrix) = runtime.get_netvar_safely("m_dwBoneMatrix") {
            let self_bone = self.get_base_address().add(bone_matrix).cast::<usize>().read();
            if let Some(x) = runtime.process.read(self_bone + ((0x30 * bone) + 0x0C)) {
                let y: f32 = runtime.process.read(self_bone + ((0x30 * bone) + 0x1C)).unwrap();
                let z: f32 = runtime.process.read(self_bone + ((0x30 * bone) + 0x2C)).unwrap();
                return Some(Vector3::new(x, y, z));
            }
        }
        None
    }

    unsafe fn get_health(&self) -> usize {
        self.get_base_address().add(MIHEALTH).cast().read()
    }

    unsafe fn get_team(&self) -> usize {
        self.get_base_address().add(MITEAMNUM).cast().read()
    }

    unsafe fn is_alive(&self) -> bool {
        let health = self.get_health();
        health > 0 && health <= 100
    }

    unsafe fn get_position(&self) -> Vector3<f32> {
        self.get_base_address().add(self.get_runtime().get_netvar("m_vecOrigin")).cast().read()
    }

    unsafe fn get_distance_flatten(&self, other: &dyn Player) -> f32 {
        (self.get_position() - other.get_position()).truncate().magnitude()
    }

    unsafe fn get_index(&self) -> i32 {
        self.get_base_address().add(0x64).cast().read()
    }

    unsafe fn get_glow_index(&self) -> i32 {
        self.get_base_address().add(MIGLOWINDEX).cast().read()
    }

    unsafe fn get_flags(&self) -> usize {
        self.get_base_address().add(MFFLAGS).cast().read()
    }

    unsafe fn is_scoped(&self) -> bool {
        self.get_base_address().add(self.get_runtime().get_netvar("m_bIsScoped")).cast().read()
    }

    unsafe fn get_glow_object(&self) -> usize {
        self.get_runtime()
            .read_ptr(self.get_runtime().get_signature("dwGlowObjectManager"), true)
            .unwrap()
            .read()
    }

    unsafe fn is_immune(&self) -> bool {
        self.get_base_address().add(MBGUNIMMUNITY).cast().read()
    }

    unsafe fn get_fov(&self) -> i32 {
        self.get_base_address().add(self.get_runtime().get_netvar("m_iFOV")).cast().read()
    }

    unsafe fn get_crosshair_id(&self) -> Option<usize> {
        let temp = self.get_base_address().add(self.get_runtime().get_netvar("m_iCrosshairId")).cast::<usize>().read();
        if temp <= 0 || temp > 32 {
            None
        } else {
            Some(temp)
        }
    }

    unsafe fn is_spotted(&self) -> bool {
        self.get_base_address().add(self.get_runtime().get_netvar("m_bSpotted")).cast().read()
    }

    unsafe fn set_spotted(&self, value: bool) {
        self.get_base_address().add(self.get_runtime().get_netvar("m_bSpotted")).cast().write(&value);
    }

    unsafe fn get_view_angles(&self) -> Vector2<f32> {
        let runtime = self.get_runtime();
        runtime.read_ptr::<usize>(runtime.get_signature("dwClientState"), false).unwrap().add(runtime.get_signature("dwClientState_ViewAngles")).cast().read()
    }

    unsafe fn get_view_offset(&self) -> usize {
        self.get_base_address().add(self.get_runtime().get_netvar("m_vecViewOffset")).cast().read()
    }

    unsafe fn get_shots_fired(&self) -> i32 {
        self.get_base_address().add(MISHOTSFIRED).cast().read()
    }

    unsafe fn is_on_ground(&self) -> bool {
        self.get_flags() & 8 == 1
    }

    unsafe fn get_flash_duration(&self) -> f32 {
        self.get_base_address().add(self.get_runtime().get_netvar("m_flFlashDuration")).cast().read()
    }

    unsafe fn is_sniper_weapon_in_hand(&self) -> bool {
        let runtime = self.get_runtime();
        let init_wep = self.get_base_address().add(runtime.get_netvar("m_hActiveWeapon")).cast::<i32>().read();
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
    fn get_base_address(&self) -> RemotePtr<'a, usize> {
        self.inner.clone()
    }

    fn get_runtime(&self) -> &'a Runtime {
        self.runtime
    }
}

impl<'a> EntityPlayer<'a> {
    pub unsafe fn get(runtime: &'a Runtime, index: usize) -> Option<Self> {
        let inner = runtime.read_ptr::<usize>(runtime.get_signature("dwEntityList") + (index * 0x10), true)?;
        Some(EntityPlayer {
            runtime,
            inner,
        })
    }
}

unsafe impl<'a> Player<'a> for LocalPlayer<'a> {
    fn get_base_address(&self) -> RemotePtr<'a, usize> {
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

    pub unsafe fn set_fov(&self, fov: i32) {
        self.inner.add(self.runtime.get_netvar("m_iFOV")).cast::<i32>().write(&fov);
    }

    pub unsafe fn force_jump(&self) {
        self.runtime.write_offset(self.runtime.get_signature("dwForceJump"), &5, true);
        sleep(Duration::from_millis(1));
        self.runtime.write_offset(self.runtime.get_signature("dwForceJump"), &4, true);
    }

    pub unsafe fn force_attack(&self) {
        self.runtime.write_offset(self.runtime.get_signature("dwForceAttack"), &5, true);
        sleep(Duration::from_millis(1));
        self.runtime.write_offset(self.runtime.get_signature("dwForceAttack"), &4, true);
    }

    pub unsafe fn set_flash_duration(&self, duration: f32) {
        self.inner.add(self.runtime.get_netvar("m_flFlashDuration")).cast().write(&duration);
    }

    pub unsafe fn set_velocity(&self, velocity: Vector3<f32>) {
        self.inner.add(0x114).cast().write(&velocity);
    }

    pub unsafe fn set_view_angles(&self, angles: Vector2<f32>) {
        self.runtime.read_ptr::<usize>(
            self.runtime.get_signature("dwClientState"), false).unwrap().add(self.runtime.get_signature("dwClientState_ViewAngles")).cast().write(&angles);
    }

    pub unsafe fn get_punch_angles(&self) -> Vector2<f32> {
         self.inner.add(self.runtime.get_netvar("m_aimPunchAngle")).cast().read()
    }
}

