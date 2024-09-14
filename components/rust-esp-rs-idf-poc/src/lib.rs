use anyhow::Ok;
use display_interface_spi::SPIInterface;
use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::gpio::{Gpio5, PinDriver};
use esp_idf_svc::hal::spi::config::{Config, MODE_3};
use esp_idf_svc::hal::spi::{self, SpiDeviceDriver};
use esp_idf_svc::hal::{
    gpio::{Input, Level, Output, Pull},
    prelude::Peripherals,
    spi::{Spi, SpiDriver, SpiDriverConfig},
};
use esp_idf_svc::hal::units::FromValueType;
use mipidsi::models::GC9A01;
use mipidsi::Builder;
use embedded_graphics::prelude::*;

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
    let dc = PinDriver::output(peripherals.pins.gpio19).unwrap();
    let spi = peripherals.spi2;

    let mut delay = Ets;

    let config = Config::new()
        .baudrate(26.MHz().into())
        .data_mode(MODE_3);

    let device = SpiDeviceDriver::new_single(
        spi,
        sclk,
        serial_out,
        Some(serial_in),
        Some(cs),
        &SpiDriverConfig::new(),
        &config,
    )?;

    let di = SPIInterface::new(device, dc);

    // create driver
    let mut display = Builder::new(GC9A01, di)
        .display_size(240, 240)
        .init(&mut delay)
        .unwrap();

    display.clear(RgbColor::MAGENTA).unwrap();

    Ok(())
}