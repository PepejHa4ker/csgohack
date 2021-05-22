mod aimbot;
mod bhop;
mod flash;
mod radar;
mod recoil;
mod trigger;
mod wh;
mod aimassist;
mod fast_tap;

pub use crate::cheats::aimbot::*;
pub use crate::cheats::bhop::*;
pub use crate::cheats::flash::*;
pub use crate::cheats::radar::*;
pub use crate::cheats::recoil::*;
pub use crate::cheats::trigger::*;
pub use crate::cheats::wh::*;
pub use crate::cheats::aimassist::*;
pub use crate::cheats::fast_tap::*;


#[macro_export]
macro_rules! cheat {
    ($name:ident { $($field:ident : $ty:ty = $value:expr),* }) => {
        #[derive(Debug)]
        pub struct $name {
            $($field: $ty),*
        }
        impl $name {
            pub fn new() -> Self {
                Self {
                    $($field: $value),*

                }

            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                 write!(f, "{:?}", self)
            }
        }


    };

    ($name:ident) => {

       #[derive(Debug)]
        pub struct $name {}
        impl $name {
            pub fn new() -> Self {
                Self {}

            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                 write!(f, "{:?}", self)
            }
        }
    };
}
