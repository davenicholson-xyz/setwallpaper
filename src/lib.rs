//! This module provides functionality to manage system wallpapers.
//!
//! You can use the functions in this module to set wallpapers on different platforms.#[cfg(target_family = "unix")]
pub mod unix;

#[cfg(target_family = "windows")]
pub mod windows;

/// This function sets the wallpaper from the given file path.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the file path of the wallpaper.
///
/// # Example
///
/// ```
/// let result = set_wallpaper("/path/to/wallpaper.png");
/// assert!(result.is_ok());
/// ```
pub fn from_file(filepath: &str) -> Result<(), std::io::Error> {
    #[cfg(target_family = "unix")]
    unix::set_wallpaper(filepath)?;

    #[cfg(target_family = "windows")]
    windows::set_wallpaper(filepath)?;
    Ok(())
}
