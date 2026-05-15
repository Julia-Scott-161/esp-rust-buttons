/*
 * This code is for testing basic button input on an ESP32c6.
 * Currently, it is reacting to 5 buttons, but can now be easily
 * modified to support more or fewer buttons.
 */

#![no_std]
#![no_main]

#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)] 

use esp_hal::system::SleepSource;
use esp_hal::{
    rtc_cntl::Rtc,
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

//defines buttons as bits (important later when implementing reports)
bitflags::bitflags! {
    pub struct Buttons: u8 {
        const RED = 1 << 0;
        const BLUE = 1 << 1;
        const YELLOW = 1 << 2;
        const GREEN = 1 << 3;
        const POWER = 1 << 4;
    }
}

    /* Scans the state of the buttons and returns a Buttons struct 
     * with the corresponding bits set for each button that is 
     * pressed.
     */
    fn scan_inputs (
        red_button: &esp_hal::gpio::Input,
        blue_button: &esp_hal::gpio::Input,
        yellow_button: &esp_hal::gpio::Input,
        green_button: &esp_hal::gpio::Input,
        power_button: &esp_hal::gpio::Input,
    ) -> Buttons {

        let mut state = Buttons::empty();
        if red_button.is_low() {
            state |= Buttons::RED;
        }

        if blue_button.is_low() {
            state |= Buttons::BLUE;
        }

        if yellow_button.is_low() {
            state |= Buttons::YELLOW;
        }

        if green_button.is_low() {
            state |= Buttons::GREEN;
        }

        if power_button.is_low() {
            state |= Buttons::POWER;
        }

        state
    }

    /* Maps the button states to an RGB color for the LED on the ESP32 
     * using the bitflags defined earlier.
     */
    fn map_inputs(inputs : Buttons) -> RGB8 {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        if inputs.contains(Buttons::RED) {
            red = 255;
        }

        if inputs.contains(Buttons::BLUE) {
            blue = 255;
        }

        if inputs.contains(Buttons::YELLOW) {
            red = 255;
            green = 255;
        }

        if inputs.contains(Buttons::GREEN) {
            green = 255;
        }

        if inputs.contains(Buttons::POWER) {
            red = 0;
            green = 0;
            blue = 0;
        }

        RGB8::new(red, green, blue)
    }

/* 
 * The `main` function initializes the hardware peripherals,
 * sets up buttons and enters an infinite loop where it checks
 * the state of the buttons and lights up an LED accordingly.
 */
#[main]
fn main() -> ! {
    esp_println::println!("Running button input test on the ESP32c6.");
    /*
     * peripherals initializes the hardware peripherals, specifically the RMT peripheral and GPIO pins
     * delay used later to delay the loop by 500 milliseconds
     * rmt initializes the RMT peripheral with the RMT channel and the clock rate
     * rmt_buffer creates a buffer for controlling the LED strip, stores the LED color data
     * led declares an LED adapter using the RMT peripheral, GPIO pin 8, and the rmt_buffer
     */
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));
    let delay = Delay::new();
    let rmt = Rmt::new(peripherals.RMT,  Rate::from_mhz(80)).unwrap();
    let mut rmt_buffer = smart_led_buffer!(1);
    let mut led = SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO8, &mut rmt_buffer);
    
    //declares a red button input, connected to GPIO4
    let red_button = Input::new(
        peripherals.GPIO4,
        InputConfig::default().with_pull(Pull::Up)
    );

    //declares a blue button input, connected to GPIO5
    let blue_button = Input::new(
        peripherals.GPIO5,
        InputConfig::default().with_pull(Pull::Up)
    );

    //declares a yellow button input, connected to GPIO4
    let yellow_button = Input::new(
        peripherals.GPIO6,
        InputConfig::default().with_pull(Pull::Up)
    );

    //declares a green button input, connected to GPIO5
    let green_button = Input::new(
        peripherals.GPIO7,
        InputConfig::default().with_pull(Pull::Up)
    );

    //declares a power button input, connected to GPIO0
    let power_button = Input::new(
        peripherals.GPIO0,
        InputConfig::default().with_pull(Pull::Up)
    );

    loop {
    let inputs = scan_inputs(&red_button, &blue_button, &yellow_button, &green_button, &power_button);
    let color = map_inputs(inputs);
    let _ = led.write(brightness(gamma([color].into_iter()), 40,));
        delay.delay_millis(500);     
    }
}