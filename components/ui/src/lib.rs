use slint::PlatformError;

slint::include_modules!();

pub fn create_ui() -> Result<HelloWorld, PlatformError> {
    return HelloWorld::new();
}

pub fn test() -> u8 {
    44
}