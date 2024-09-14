use slint::{PlatformError, Rgb8Pixel};

slint::include_modules!();

pub fn create_ui() -> Result<HelloWorld, PlatformError> {
    return HelloWorld::new();
}

pub fn test() -> u8 {
    44
}


pub trait UiInferface {
    fn get_temp_sensor_value() -> f32;
    fn get_light_sensor_value() -> i32;
    fn set_leds(leds: Vec<Rgb8Pixel>) -> Result<(), i32 /* TODO */>;

}