/*
 * This code is for testing basic button input on an ESP32c6.
 * Currently, it is set up to react to a single button, connected
 * to GPIO4.
 */

#![no_std]
#![no_main]

#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)] 

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    main,
    rmt::Rmt,
    time::Rate,
    gpio::{Input, InputConfig, Pull}
};
use esp_backtrace as _;
esp_bootloader_esp_idf::esp_app_desc!();
// Imports for controlling the LED strip
//TODO: esp_hal_smartled seems to be outdated/abandoned, check for alternatives crates
use esp_hal_smartled::{smart_led_buffer, SmartLedsAdapter};
use smart_leds::{brightness, gamma, SmartLedsWrite, RGB8};

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]

/* 
 * The `main` function initializes the hardware peripherals,
 * sets up a button and enters an infinite loop where it checks
 * the state of a button and lights up an LED when pressed.
 */
#[main]
fn main() -> ! {
    //test code, ignore for now
    //esp_println::println!("running");

    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

     
    /*
     * delay used later to delay the loop by 500 milliseconds
     * rmt initializes the RMT peripheral with the RMT channel and the clock rate
     * rmt_buffer creates a buffer for controlling the LED strip, stores the LED color data
     * led declares an LED adapter using the RMT peripheral, GPIO pin 8, and the rmt_buffer
     */
    let delay = Delay::new();
    let rmt = Rmt::new(peripherals.RMT,  Rate::from_mhz(80)).unwrap();
    let mut rmt_buffer = smart_led_buffer!(1);
    let mut led = SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO8, &mut rmt_buffer);

    //declares a single button input, connected to GPIO4
    let button = Input::new(
        peripherals.GPIO4,
        InputConfig::default().with_pull(Pull::Up) //up means button will read as low when pressed
    );

    //infinite loop, which checks the state of the button and updates the LED as needed
    loop {
    //check the state of the button
    if button.is_low() {
        let _ = led.write(
            brightness(gamma([RGB8::new(255, 0, 0)].into_iter()), 40,)  
        );
        esp_println::println!("button is down");
    } else {
        let _ = led.write(
            brightness(gamma([RGB8::new(0, 0, 0)].into_iter()), 40,)
        );
        esp_println::println!("button is up");
    }
        delay.delay_millis(500);
        esp_println::println!("loop complete, re-looping");
    }

    /*
    // Test loop for making sure led is working, disregards any button state
    loop {
    let _ = led.write(brightness(gamma([RGB8::new(255, 0, 0)].into_iter()), 20));
    delay.delay_millis(500);

    let _ = led.write(brightness(gamma([RGB8::new(0, 0, 0)].into_iter()), 20));
    delay.delay_millis(500);
    }
    */
     
}