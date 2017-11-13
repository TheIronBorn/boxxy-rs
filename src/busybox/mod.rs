//! Builtin commands.

/// Disk related commands
pub mod disk;
#[cfg(unix)]
/// Privilege related commands
pub mod privs;
/// Process related commands
pub mod procs;

pub use self::disk::*;
#[cfg(unix)]
pub use self::privs::*;
pub use self::procs::*;
