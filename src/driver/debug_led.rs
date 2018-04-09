use stm32f7::embedded::interfaces::gpio;
use stm32f7::embedded::interfaces::gpio::*;

trait LedFunctions {
    fn on(&mut self);
    fn off(&mut self);
}

struct Led{
    led_pin: OutputPin,
}

impl LedFunctions for Led{
    fn on(&mut self){
        self.led_pin.set(true);
    }

    fn off(&mut self){
        self.led_pin.set(false);
    }
}

pub trait DebugLedFunctions {
   fn info_on(&mut self);
   fn info_off(&mut self);

   fn warn_on(&mut self);
   fn warn_off(&mut self);

   fn error_on(&mut self);
   fn error_off(&mut self);
}

pub struct DebugLeds {
    info : Led,
    warn : Led,
    error : Led,
}

impl DebugLedFunctions for DebugLeds {
   fn info_on(&mut self){
        self.info.on();
    }

   fn info_off(&mut self){
        self.info.off();
    }

   fn warn_on(&mut self){
        self.warn.on();
    }

   fn warn_off(&mut self){
        self.warn.off();
    }

   fn error_on(&mut self){
        self.error.on();
    }

   fn error_off(&mut self){
        self.error.off();
    }
}

pub fn init(gpio :&mut Gpio) -> DebugLeds{
    let led_red_pin_port;
    let led_blue_pin_port;
    let led_green_pin_port;

    let led_red;
    let led_blue;
    let led_green;

    led_red_pin_port = (gpio::Port::PortB, gpio::Pin::Pin14);
    led_blue_pin_port = (gpio::Port::PortB, gpio::Pin::Pin7);
    led_green_pin_port = (gpio::Port::PortB, gpio::Pin::Pin0);


    led_red = gpio.to_output(led_red_pin_port,
                             gpio::OutputType::PushPull,
                             gpio::OutputSpeed::Low,
                             gpio::Resistor::NoPull).expect("led pin already in use");
                   
    led_blue = gpio.to_output(led_blue_pin_port,
                              gpio::OutputType::PushPull,
                              gpio::OutputSpeed::Low,
                              gpio::Resistor::NoPull).expect("led pin already in use");

    led_green = gpio.to_output(led_green_pin_port,
                               gpio::OutputType::PushPull,
                               gpio::OutputSpeed::Low,
                               gpio::Resistor::NoPull).expect("led pin already in use");

    let leds :DebugLeds;

    leds = DebugLeds{info: Led{led_pin:led_green},
                     warn: Led{led_pin:led_blue}, 
                     error:Led{led_pin:led_red}}; 

    return leds
}