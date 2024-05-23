#[cfg(not(any(cxxqt_qt_version_major = "5", cxxqt_qt_version_major = "6")))]
compile_error!("cxxqt_qt_version_major must be either \"5\" or \"6\"");

mod kcoreaddons;
mod ki18n;

pub use crate::kcoreaddons::*;
pub use crate::ki18n::*;
