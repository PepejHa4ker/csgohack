use std::ffi::CString;
use std::slice::Iter;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum WeaponType {
    Knife,
    Pistol,
    SubMachineGun,
    Rifle,
    ShotGun,
    SniperRifle,
    MachineGun,
    C4,
    Placeholder,
    Grenade,
    Unknown,
    StackableItem,
    Fists,
    BreachCharge,
    BumpMine,
    Tablet,
    Melee,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum WeaponId {
    Deagle = 1,
    Elite,
    FiveSeven,
    Glock,
    Ak47 = 7,
    Aug,
    Awp,
    Famas,
    G3sg1,
    GalilAr = 13,
    M249,
    M4a1 = 16,
    Mac10,
    P90 = 19,
    ZoneRepulsor,
    Mp5sd = 23,
    Ump45,
    Xm1014,
    Bizon,
    Mag7,
    Negev,
    SawedOff,
    Tec9,
    Taser,
    Hkp2000,
    Mp7,
    Mp9,
    Nova,
    P250,
    Shield,
    Scar20,
    Sg553,
    Ssg08,
    GoldenKnife,
    Knife,
    FlashBang = 43,
    HeGrenade,
    SmokeGrenade,
    Molotov,
    Decoy,
    IncGrenade,
    C4,
    HealthShot = 57,
    KnifeT = 59,
    M4a1S,
    UspS,
    Cz75a = 63,
    Revolver,
    TaGrenade = 68,
    Axe,
    Hammer,
    Spanner = 78,
    GhostKnife = 80,
    FireBomb,
    Diversion,
    FragGrenade,
    SnowBall,
    BumpMine,
    Bayonet = 500,
    ClassicKnife = 503,
    Flip = 505,
    Gut,
    Karambit,
    M9Bayonet,
    Huntsman,
    Falchion = 512,
    Bowie = 514,
    Butterfly,
    Daggers,
    Paracord,
    SurvivalKnife,
    Ursus = 519,
    Navaja,
    NomadKnife,
    Stiletto = 522,
    Talon,
    SkeletonKnife = 525,
    GloveStuddedBrokenFang = 4725,
    GloveStuddedBloodHound = 5027,
    GloveT,
    GloveCt,
    GloveSporty,
    GloveSlick,
    GloveLeatherwrap,
    GloveMotorcycle,
    GloveSpecialist,
    GloveHydra,
}

#[derive(Debug)]
pub struct WeaponInfo {
    pad1: [u8; 32],
    max_clip: i32,
    pad2: [u8; 204],
    name: CString,
    pad3: [u8; 72],
    ty: WeaponType,
    pad4: [u8; 4],
    price: i32,
    pad5: [u8; 12],
    cycle_time: f32,
    pad6: [u8; 12],
    full_auto: bool,
    pad7: [u8; 3],
    damage: i32,
    armor_ratio: f32,
    bullets: i32,
    penetration: f32,
    pad8: [u8; 8],
    range: f32,
    range_modifier: f32,
    pad9: [u8; 16],
    silencer: bool,
    pad10: [u8; 23],
    max_speed: f32,
    max_speed_alt: f32,
    pad11: [u8; 100],
    recoil_magnitude: f32,
    recoil_magnitude_alt: f32,
    pad12: [u8; 16],
    recovery_timestand: f32,
}

impl std::fmt::Display for WeaponId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl WeaponId {
    pub fn get_from_index(id: usize) -> Self {
        match id {
            1 => Self::Deagle,
            2 => Self::Elite,
            3 => Self::FiveSeven,
            4 => Self::Glock,
            7 => Self::Ak47,
            8 => Self::Aug,
            9 => Self::Awp,
            10 => Self::Famas,
            11 => Self::G3sg1,
            13 => Self::GalilAr,
            14 => Self::M249,
            16 => Self::M4a1,
            17 => Self::Mac10,
            19 => Self::P90,
            20 => Self::ZoneRepulsor,
            23 => Self::Mp5sd,
            24 => Self::Ump45,
            25 => Self::Xm1014,
            25 => Self::Bizon,
            26 => Self::Mag7,
            27 => Self::Negev,
            28 => Self::SawedOff,
            29 => Self::Tec9,
            30 => Self::Taser,
            31 => Self::Hkp2000,
            32 => Self::Mp7,
            35 => Self::Mp9,
            34 => Self::Nova,
            35 => Self::P250,
            36 => Self::Shield,
            37 => Self::Scar20,
            38 => Self::Sg553,
            39 => Self::Ssg08,
            40 => Self::GoldenKnife,
            41 => Self::Knife,
            43 => Self::FlashBang,
            45 => Self::HeGrenade,
            46 => Self::SmokeGrenade,
            47 => Self::Molotov,
            48 => Self::Decoy,
            49 => Self::IncGrenade,
            50 => Self::C4,
            57 => Self::HealthShot,
            59 => Self::KnifeT,
            60 => Self::M4a1S,
            61 => Self::UspS,
            63 => Self::Cz75a,
            68 => Self::TaGrenade,
            75 => Self::Axe,
            76 => Self::Hammer,
            78 => Self::Spanner,
            80 => Self::GhostKnife,
            81 => Self::FireBomb,
            82 => Self::Diversion,
            83 => Self::FragGrenade,
            84 => Self::SnowBall,
            85 => Self::BumpMine,
            500 => Self::Bayonet,
            503 => Self::ClassicKnife,
            505 => Self::Flip,
            506 => Self::Gut,
            507 => Self::Karambit,
            508 => Self::M9Bayonet,
            509 => Self::Huntsman,
            512 => Self::Falchion,
            514 => Self::Bowie,
            515 => Self::Butterfly,
            516 => Self::Daggers,
            517 => Self::Paracord,
            518 => Self::SurvivalKnife,
            519 => Self::Ursus,
            520 => Self::Navaja,
            521 => Self::NomadKnife,
            522 => Self::Stiletto,
            523 => Self::Talon,
            525 => Self::SkeletonKnife,
            4725 => Self::GloveStuddedBrokenFang,
            5027 => Self::GloveStuddedBloodHound,
            5028 => Self::GloveT,
            5029 => Self::GloveCt,
            5030 => Self::GloveSporty,
            5031 => Self::GloveSlick,
            5032 => Self::GloveLeatherwrap,
            5033 => Self::GloveMotorcycle,
            5034 => Self::GloveSpecialist,
            5035 => Self::GloveHydra,
            _ => panic!("Invalid Weapon index")
        }
    }

    pub fn is_sniper(&self) -> bool {
        return match self {
            WeaponId::Scar20 |
            WeaponId::Awp |
            WeaponId::Sg553 |
            WeaponId::G3sg1 => true,
            _ => false
        };
    }


    pub fn get_weapon_class(&self) -> i32 {
        return match self {
            WeaponId::Glock |
            WeaponId::Hkp2000 |
            WeaponId::UspS |
            WeaponId::Elite |
            WeaponId::P250 |
            WeaponId::Tec9 |
            WeaponId::FiveSeven |
            WeaponId::Cz75a |
            WeaponId::Deagle |
            WeaponId::Revolver => 35,

            WeaponId::Nova |
            WeaponId::Xm1014 |
            WeaponId::SawedOff |
            WeaponId::Mag7 |
            WeaponId::M249 |
            WeaponId::Negev => 36,

            WeaponId::Mac10 |
            WeaponId::Mp9 |
            WeaponId::Mp7 |
            WeaponId::Mp5sd |
            WeaponId::Ump45 |
            WeaponId::P90 |
            WeaponId::Bizon => 37,

            WeaponId::GalilAr |
            WeaponId::Famas |
            WeaponId::Ak47 |
            WeaponId::M4a1 |
            WeaponId::M4a1S |
            WeaponId::Ssg08 |
            WeaponId::Sg553 |
            WeaponId::Aug |
            WeaponId::Awp |
            WeaponId::G3sg1 |
            WeaponId::Scar20 |
            WeaponId::Taser => 38,
            _ => 0
        };
    }
}

