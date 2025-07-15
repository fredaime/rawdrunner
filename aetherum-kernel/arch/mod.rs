#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "aarch64")]
pub mod arm64;

pub use self::platform::init;

#[cfg(target_arch = "x86_64")]
mod platform {
    pub use crate::arch::x86_64::init;
}
#[cfg(target_arch = "aarch64")]
mod platform {
    pub use crate::arch::arm64::init;
}
