// This struct represents a signature.
#[derive(Debug, Clone)]
pub struct Signature {
    // Signature name.
    pub name: String,

    // Signature pattern.
    pub pattern: String,

    // Module name.
    pub module: String,

    // Signature offsets for dereferencing.
    pub offsets: Vec<isize>,

    // Extra to be added to the result.
    pub extra: isize,

    // If true, subtract module base from result.
    pub relative: bool,

    // If true, read a u32 at the position and add it to the result.
    pub rip_relative: bool,

    // Offset to the rip relative.
    pub rip_offset: isize,
}

// This struct represents a netvar.
#[derive(Debug, Clone)]
pub struct Netvar {
    // Netvar name.
    pub name: String,

    // Table name.
    pub table: String,

    // Prop name.
    pub prop: String,

    // Offset to be added to the result.
    pub offset: usize,
}

impl Default for Signature {
    fn default() -> Self {
        Signature {
            name: "".to_string(),
            pattern: "".to_string(),
            module: "".to_string(),
            offsets: vec![],
            extra: 0,
            relative: false,
            rip_relative: false,
            rip_offset: 0,
        }
    }
}

// This struct represents the config.
#[derive(Debug, Clone)]
pub struct Config {
    // Executable target name.
    pub executable: String,

    // Output file names
    pub filename: String,

    // `Vec` containing the `Signature`s.
    pub signatures: Vec<Signature>,

    // `Vec` containing the `Netvar`s.
    pub netvars: Vec<Netvar>,
}

impl Config {
    pub fn load() -> Config {
        Config {
            executable: "csgo.exe".to_string(),
            filename: "csgo".to_string(),
            signatures: vec![
                Signature {
                    name: "dwClientState".to_string(),
                    pattern: "A1 ? ? ? ? 33 D2 6A 00 6A 00 33 C9 89 B0".to_string(),
                    module: "engine.dll".to_string(),
                    offsets: vec![1],
                    extra: 0,
                    relative: true,
                    rip_relative: false,
                    rip_offset: 0,
                },
                Signature {
                    name: "dwClientState_Map".to_string(),
                    pattern: "05 ? ? ? ? C3 CC CC CC CC CC CC CC A1".to_string(),
                    module: "engine.dll".to_string(),
                    offsets: vec![1],
                    extra: 0,
                    relative: false,
                    rip_relative: false,
                    rip_offset: 0
                },
                Signature {
                    name: "dwClientState_MapDirectory".to_string(),
                    pattern: "B8 ? ? ? ? C3 05 ? ? ? ? C3".to_string(),
                    module: "engine.dll".to_string(),
                    offsets: vec![7],
                    extra: 0,
                    relative: false,
                    rip_relative: false,
                    rip_offset: 0
                },
                Signature {
                    name: "dwClientState_ViewAngles".to_string(),
                    pattern: "F3 0F 11 80 ? ? ? ? D9 46 04 D9 05".to_string(),
                    module: "engine.dll".to_string(),
                    offsets: vec![4],
                    extra: 0,
                    relative: false,
                    rip_relative: false,
                    rip_offset: 0,
                },
                Signature {
                    name: "dwEntityList".to_string(),
                    pattern: "BB ? ? ? ? 83 FF 01 0F 8C ? ? ? ? 3B F8".to_string(),
                    module: "client.dll".to_string(),
                    offsets: vec![1],
                    extra: 0,
                    relative: true,
                    rip_relative: false,
                    rip_offset: 0,
                },
                Signature {
                    name: "dwForceAttack".to_string(),
                    pattern: "89 0D ? ? ? ? 8B 0D ? ? ? ? 8B F2 8B C1 83 CE 04".to_string(),
                    module: "client.dll".to_string(),
                    offsets: vec![2],
                    extra: 0,
                    relative: true,
                    rip_relative: false,
                    rip_offset: 0,
                },
                Signature {
                    name: "dwForceJump".to_string(),
                    pattern: "8B 0D ? ? ? ? 8B D6 8B C1 83 CA 02".to_string(),
                    module: "client.dll".to_string(),
                    offsets: vec![2],
                    extra: 0,
                    relative: true,
                    rip_relative: false,
                    rip_offset: 0,
                },
                Signature {
                    name: "dwGetAllClasses".to_string(),
                    pattern: "A1 ? ? ? ? C3 CC CC CC CC CC CC CC CC CC CC A1 ? ? ? ? B9".to_string(),
                    module: "client.dll".to_string(),
                    offsets: vec![1, 0],
                    extra: 0,
                    relative: true,
                    rip_relative: false,
                    rip_offset: 0,
                },
                Signature {
                    name: "dwGlowObjectManager".to_string(),
                    pattern: "A1 ? ? ? ? A8 01 75 4B".to_string(),
                    module: "client.dll".to_string(),
                    offsets: vec![1],
                    extra: 4,
                    relative: true,
                    rip_relative: false,
                    rip_offset: 0,
                },
                Signature {
                    name: "dwLocalPlayer".to_string(),
                    pattern: "8D 34 85 ? ? ? ? 89 15 ? ? ? ? 8B 41 08 8B 48 04 83 F9 FF".to_string(),
                    module: "client.dll".to_string(),
                    offsets: vec![3],
                    extra: 4,
                    relative: true,
                    rip_relative: false,
                    rip_offset: 0,
                },
                Signature {
                    name: "dwViewMatrix".to_string(),
                    pattern: "0F 10 05 ? ? ? ? 8D 85 ? ? ? ? B9".to_string(),
                    module: "client.dll".to_string(),
                    offsets: vec![3],
                    extra: 176,
                    relative: true,
                    rip_relative: false,
                    rip_offset: 0,
                },
                Signature {
                    name: "dwLocalPlayer".to_string(),
                    pattern: "8D 34 85 ? ? ? ? 89 15 ? ? ? ? 8B 41 08 8B 48 04 83 F9 FF".to_string(),
                    module: "client.dll".to_string(),
                    offsets: vec![3],
                    extra: 4,
                    relative: true,
                    rip_relative: false,
                    rip_offset: 0,
                },
            ],
            netvars: vec![
                Netvar {
                    name: "m_aimPunchAngle".to_string(),
                    table: "DT_BasePlayer".to_string(),
                    prop: "m_aimPunchAngle".to_string(),
                    offset: 0,
                },
                Netvar {
                    name: "m_bSpotted".to_string(),
                    table: "DT_BaseEntity".to_string(),
                    prop: "m_bSpotted".to_string(),
                    offset: 0,
                },
                Netvar {
                    name: "m_dwBoneMatrix".to_string(),
                    table: "DT_BaseAnimating".to_string(),
                    prop: "m_nForceBone".to_string(),
                    offset: 28,
                },
                Netvar {
                    name: "m_flFlashDuration".to_string(),
                    table: "DT_CSPlayer".to_string(),
                    prop: "m_flFlashDuration".to_string(),
                    offset: 0,
                },
                Netvar {
                    name: "m_iCrosshairId".to_string(),
                    table: "DT_CSPlayer".to_string(),
                    prop: "m_bHasDefuser".to_string(),
                    offset: 92,
                },
                Netvar {
                    name: "m_iShotsFired".to_string(),
                    table: "DT_CSPlayer".to_string(),
                    prop: "m_iShotsFired".to_string(),
                    offset: 0,
                },
                Netvar {
                    name: "m_vecOrigin".to_string(),
                    table: "DT_BasePlayer".to_string(),
                    prop: "m_vecOrigin".to_string(),
                    offset: 0,
                },
                Netvar {
                    name: "m_viewPunchAngle".to_string(),
                    table: "DT_BasePlayer".to_string(),
                    prop: "m_viewPunchAngle".to_string(),
                    offset: 0,
                },
                Netvar {
                    name: "m_vecViewOffset".to_string(),
                    table: "DT_BasePlayer".to_string(),
                    prop: "m_viewPunchAngle".to_string(),
                    offset: 0,
                },
                Netvar {
                    name: "m_hActiveWeapon".to_string(),
                    table: "DT_BasePlayer".to_string(),
                    prop: "m_hActiveWeapon".to_string(),
                    offset: 0
                },
                Netvar {
                    name: "m_iFOV".to_string(),
                    table: "DT_CSPlayer".to_string(),
                    prop: "m_iFOV".to_string(),
                    offset: 0
                },
                Netvar {
                    name: "m_bIsScoped".to_string(),
                    table: "DT_CSPlayer".to_string(),
                    prop: "m_bIsScoped".to_string(),
                    offset: 0
                },
                Netvar {
                    name: "m_iItemDefinitionIndex".to_string(),
                    table: "DT_BaseCombatWeapon".to_string(),
                    prop: "m_iItemDefinitionIndex".to_string(),
                    offset: 0
                },
                Netvar {
                    name: "m_vecVelocity".to_string(),
                    table: "m_vecVelocity[0]".to_string(),
                    prop: "DT_CSPlayer".to_string(),
                    offset: 0
                }
            ],
        }
    }
}