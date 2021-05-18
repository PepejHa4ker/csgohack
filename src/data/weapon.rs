use std::ffi::CString;

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





impl WeaponId {

    pub fn get_from_id(id: usize) -> Self {
        WeaponId::Ak47
    }

    pub fn is_sniper(&self) -> bool {
        return match self {
            WeaponId::Scar20 |
            WeaponId::Awp |
            WeaponId::Sg553 |
            WeaponId::G3sg1  => true,
            _ =>  false
        }
    }

    pub fn get_weapon_index(&self) -> i32 {
        return match self {
            WeaponId::Glock => 1,
            WeaponId::Hkp2000 => 2,
            WeaponId::UspS => 3,
            WeaponId::Elite => 4,
            WeaponId::P250 => 5,
            WeaponId::Tec9 => 6,
            WeaponId::FiveSeven => 7,
            WeaponId::Cz75a => 8,
            WeaponId::Deagle => 9,
            WeaponId::Revolver => 10,
            WeaponId::Nova => 11,
            WeaponId::Xm1014 => 12,
            WeaponId::SawedOff => 13,
            WeaponId::Mag7 => 14,
            WeaponId::M249 => 15,
            WeaponId::Negev => 16,
            WeaponId::Mac10 => 17,
            WeaponId::Mp9 => 18,
            WeaponId::Mp7 => 19,
            WeaponId::Mp5sd => 20,
            WeaponId::Ump45 => 21,
            WeaponId::P90 => 22,
            WeaponId::Bizon => 23,
            WeaponId::GalilAr => 24,
            WeaponId::Famas => 25,
            WeaponId::Ak47 => 26,
            WeaponId::M4a1 => 27,
            WeaponId::M4a1S => 28,
            WeaponId::Ssg08 => 29,
            WeaponId::Sg553 => 30,
            WeaponId::Aug => 31,
            WeaponId::Awp => 32,
            WeaponId::G3sg1 => 33,
            WeaponId::Scar20 => 34,
            WeaponId::Taser => 39,
            _ => 0
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
            WeaponId::Mp5sd|
            WeaponId::Ump45 |
            WeaponId::P90 |
            WeaponId::Bizon => 37,

            WeaponId::GalilAr |
            WeaponId::Famas|
            WeaponId::Ak47 |
            WeaponId::M4a1|
            WeaponId::M4a1S|
            WeaponId::Ssg08 |
            WeaponId::Sg553|
            WeaponId::Aug|
            WeaponId::Awp|
            WeaponId::G3sg1 |
            WeaponId::Scar20|
            WeaponId::Taser => 38,
            _ => 0
        };
    }
}

