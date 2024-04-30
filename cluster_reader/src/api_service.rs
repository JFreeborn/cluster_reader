pub mod api_service {
    
    use std::io::Error;

    pub fn check_config(environment_variable_key: &str, config_location: &str) -> Result<(), Error> {

        use std::fs;
        use std::io::ErrorKind;

        let directory_metadata = fs::metadata(&config_location)?;

        if directory_metadata.is_dir() {
            return Err(Error::new(ErrorKind::Unsupported, "Supplied config location is a directory"));        
        } else {
            println!("Setting environtment varible of {} to {}", environment_variable_key, config_location);
            std::env::set_var(environment_variable_key, config_location);
            return Ok(());
        }
    }
}