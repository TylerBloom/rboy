#![crate_name = "rboy"]
#![crate_type = "lib"]

pub use crate::gpu::{SCREEN_H, SCREEN_W};
pub use crate::keypad::KeypadKey;
pub use crate::serial::SerialCallback;
pub use crate::sound::AudioPlayer;

pub mod device;

pub mod cpu;
pub mod gbmode;
pub mod gpu;
pub mod keypad;
pub mod mbc;
pub mod mmu;
pub mod printer;
pub mod register;
pub mod serial;
pub mod sound;
pub mod timer;

pub type StrResult<T> = Result<T, &'static str>;
