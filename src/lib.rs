#[cfg(target_family = "unix")]
pub mod unix;

#[cfg(target_family = "windows")]
pub mod windows;

pub fn from_file(filepath: &str) -> Result<(), std::io::Error> {
    #[cfg(target_family = "unix")]
    unix::set_wallpaper(filepath)?;

    #[cfg(target_family = "windows")]
    windows::set_wallpaper(filepath)?;
    Ok(())
}
