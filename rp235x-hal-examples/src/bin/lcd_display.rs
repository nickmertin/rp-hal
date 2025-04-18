//! # LCD Display Example
//!
//! In this example, the rp235x is configured to drive a small two-line
//! alphanumeric LCD using the
//! [HD44780](https://crates.io/crates/hd44780-driver) driver.
//!
//! It drives the LCD by pushing data out of six GPIO pins. It may need to be
//! adapted to your particular board layout and/or pin assignment.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Alias for our HAL crate
use rp235x_hal as hal;

// Our LCD driver
use hd44780_driver as hd44780;

/// Tell the Boot ROM about our application
#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

/// External high-speed crystal on the Raspberry Pi Pico 2 board is 12 MHz.
/// Adjust if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

/// Entry point to our bare-metal application.
///
/// The `#[hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
///
/// The function configures the rp235x peripherals, writes to the LCD, then goes
/// to sleep.
#[hal::entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = hal::pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Create the LCD driver from some GPIO pins
    let mut lcd = hd44780::HD44780::new_4bit(
        pins.gpio16.into_push_pull_output(), // Register Select
        pins.gpio17.into_push_pull_output(), // Enable
        pins.gpio18.into_push_pull_output(), // d4
        pins.gpio19.into_push_pull_output(), // d5
        pins.gpio20.into_push_pull_output(), // d6
        pins.gpio21.into_push_pull_output(), // d7
        &mut delay,
    )
    .unwrap();

    // Clear the screen
    lcd.reset(&mut delay).unwrap();
    lcd.clear(&mut delay).unwrap();

    // Write to the top line
    lcd.write_str("rp-hal on", &mut delay).unwrap();

    // Move the cursor
    lcd.set_cursor_pos(40, &mut delay).unwrap();

    // Write more more text
    lcd.write_str("HD44780!", &mut delay).unwrap();

    // Do nothing - we're finished
    loop {
        hal::arch::wfi();
    }
}

/// Program metadata for `picotool info`
#[link_section = ".bi_entries"]
#[used]
pub static PICOTOOL_ENTRIES: [hal::binary_info::EntryAddr; 5] = [
    hal::binary_info::rp_cargo_bin_name!(),
    hal::binary_info::rp_cargo_version!(),
    hal::binary_info::rp_program_description!(c"LCD Display Example"),
    hal::binary_info::rp_cargo_homepage_url!(),
    hal::binary_info::rp_program_build_attribute!(),
];

// End of file
