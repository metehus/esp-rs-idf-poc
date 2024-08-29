use anyhow::Ok;
use esp_idf_svc::hal::spi;
use esp_idf_svc::hal::{
    gpio::{Input, Level, Output, Pull},
    prelude::Peripherals,
    spi::{Spi, SpiDriver, SpiDriverConfig},
};
use gc9a01::GC9A01;

extern "C" {
    fn read_temp_sensor() -> f32;
}

#[no_mangle]
extern "C" fn rust_main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let temperature = unsafe { read_temp_sensor() };

    log::info!("temp: {}", temperature);

    main();
}


fn main() -> anyhow::Result<()> {
    let peripherals = Peripherals::take()?;

    let sclk = peripherals.pins.gpio1;
    let serial_in = peripherals.pins.gpio16; // SDI
    let serial_out = peripherals.pins.gpio17; // SDO
    let cs = peripherals.pins.gpio3;
    let dc = peripherals.pins.gpio19;

    let spi = SpiDriver::new(
        peripherals.spi2,
        sclk,
        serial_out,
        Some(serial_in),
        &SpiDriverConfig::new(),
    )?;

    let mut display = GC9A01 { spi, cs, dc };
    display.setup();

    Ok(())
}