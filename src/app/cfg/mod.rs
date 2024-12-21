use std::fs;
use std::path;
use crate::internals::filesystem::directories;

/// # SlinkyHomeDirectory
/// 
/// Base directory of Slinky files in user files.
pub struct SlinkyHomeDirectory(path::Path);

pub struct SlinkyDirectoryStructure {
    home: Path,
    
    // C:\Users\Account\.slinky
    user_slinky: Path,
}

pub struct SlinkySetup;

impl SlinkySetup {
    pub fn init() {
        if let Some(user_dir) = directories::UserDirs::new() {
            // Home Directory
            let home = user_dir.home_dir();

            let mut slinky_base = home.to_path_buf();

            // .slinky/
            slinky_base.push(".slinky");
            fs::create_dir(slinky_base);

            // .slinky/
                // keystore
                    // User
                    // Trusted
                    // Slinky
                    // X59
                // config
                    // manifest
                // packages
                // 

            let mut slinky_config = slinky_base;

            slinky_config.push("config");

            // .slinky/config/
            fs::create_dir(slinky_config);
        }
    }
    fn init_home() {
        // homehomehomehomehome
        if let Some

    }
    pub fn uninstall() {

    }
}