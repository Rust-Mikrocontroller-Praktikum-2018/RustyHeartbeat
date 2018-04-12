use stm32f7::embedded::interfaces::gpio;
use stm32f7::embedded::interfaces::gpio::*;

static mut INFO: Option<DebugLed> = None;
static mut WARN: Option<DebugLed> = None;
static mut ERROR: Option<DebugLed> = None;

trait LedFunctions {
    fn on(&mut self);
    fn off(&mut self);
}

pub struct DebugLed {
    led_pin: OutputPin,
}

impl LedFunctions for DebugLed {
    fn on(&mut self) {
        self.led_pin.set(true);
    }

    fn off(&mut self) {
        self.led_pin.set(false);
    }
}

impl DebugLed {
    pub fn init(gpio: &mut Gpio) {
        let led_red_pin_port;
        let led_blue_pin_port;
        let led_green_pin_port;

        let led_red;
        let led_blue;
        let led_green;

        led_red_pin_port = (gpio::Port::PortB, gpio::Pin::Pin14);
        led_blue_pin_port = (gpio::Port::PortB, gpio::Pin::Pin7);
        led_green_pin_port = (gpio::Port::PortB, gpio::Pin::Pin0);

        led_red = gpio.to_output(
            led_red_pin_port,
            gpio::OutputType::PushPull,
            gpio::OutputSpeed::Low,
            gpio::Resistor::NoPull,
        ).expect("led pin already in use");

        led_blue = gpio.to_output(
            led_blue_pin_port,
            gpio::OutputType::PushPull,
            gpio::OutputSpeed::Low,
            gpio::Resistor::NoPull,
        ).expect("led pin already in use");

        led_green = gpio.to_output(
            led_green_pin_port,
            gpio::OutputType::PushPull,
            gpio::OutputSpeed::Low,
            gpio::Resistor::NoPull,
        ).expect("led pin already in use");
        unsafe {
            INFO = Some(DebugLed { led_pin: led_green });
            WARN = Some(DebugLed { led_pin: led_blue });
            ERROR = Some(DebugLed { led_pin: led_red });
        }
    }

    pub fn info_on() {
        unsafe {
            DebugLed::on(&mut INFO);
        }
    }

    pub fn info_off() {
        unsafe {
            DebugLed::off(&mut INFO);
        }
    }

    pub fn warn_on() {
        unsafe {
            DebugLed::on(&mut WARN);
        }
    }

    pub fn warn_off() {
        unsafe {
            DebugLed::off(&mut WARN);
        }
    }

    pub fn error_on() {
        unsafe {
            DebugLed::on(&mut ERROR);
        }
    }

    pub fn error_off() {
        unsafe {
            DebugLed::off(&mut ERROR);
        }
    }

    pub fn on(led_option: &mut Option<DebugLed>) {
        if let Some(ref mut led) = led_option {
            led.on();
        }
    }

    pub fn off(led_option: &mut Option<DebugLed>) {
        if let Some(ref mut led) = led_option {
            led.off();
        }
    }
}
