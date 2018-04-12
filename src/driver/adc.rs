use driver::debug_led::*;
use embedded_stm32f7::adc::Adc as EmbeddedAdc;
use stm32f7::embedded::interfaces::gpio::Pin;

pub struct Adc<T> {
    adc: &'static mut EmbeddedAdc,
    adc_pins: T,
}

pub enum AdcChannel {
    Channel0 = 0,
    Channel1 = 1,
    Channel2 = 2,
    Channel3 = 3,
    Channel4 = 4,
    Channel5 = 5,
    Channel6 = 6,
    Channel7 = 7,
    Channel8 = 8,
    Channel9 = 9,
    Channel10 = 10,
    Channel11 = 11,
    Channel12 = 12,
    Channel13 = 13,
    Channel14 = 14,
    Channel15 = 15,
}

pub struct AdcPin {
    port_address: u32,
    pin: Pin,
    channel: AdcChannel,
}

impl AdcPin {
    pub fn new(port_address: u32, pin: Pin, channel: AdcChannel) -> AdcPin {
        AdcPin {
            port_address: port_address,
            pin: pin,
            channel: channel,
        }
    }

    pub fn init(&self) {
        let addr: *mut u32 = self.port_address as *mut u32;
        let x = unsafe { ::core::ptr::read_volatile(addr) };
        unsafe { ::core::ptr::write_volatile(addr, x | ((0b11 as u32) << (2 * self.pin as u32))) };
    }
}

fn init_adc(adc: &mut EmbeddedAdc) {
    adc.cr1.update(|cr1| {
        //Set Overrun interrupt enable
        cr1.set_ovrie(false);
        //Set resolution (12 bit)
        cr1.set_res(0);
        //Set Analog Watchdog enable for regular channels
        cr1.set_awden(false);
        //Set Analog Watchdog enable for injected channels
        cr1.set_jawden(false);
        //Set Discontinuous mode channel count (1 Channel)
        cr1.set_discnum(0x00);
        //Set continuous mode enabled on injected channels
        cr1.set_jdiscen(false);
        //Set continuous mode enabled on regular channels
        cr1.set_discen(false);
        //Set Automatic injected group conversion
        cr1.set_jauto(false);
        //Set watchdog on a single channel in scan mode
        cr1.set_awdsgl(false);
        //Set Scan mode
        cr1.set_scan(false);
        //Set interrupt enabled for injected channels
        cr1.set_jeocie(false);
        //Set analog watchdog  interrupt enabled
        cr1.set_awdie(false);
        // Set End Of Conversion interrupt enabled
        cr1.set_eocie(false);
        //Set analog watchdog channel select bits
        cr1.set_awdch(0x00);
    });

    adc.cr2.update(|cr2| {
        //Set External trigger detection (Trigger detection disabled)
        cr2.set_exten(0x00);
        // Set External event select for regular group (Timer 1 CH1)
        cr2.set_extsel(0x00);
        // Set  External trigger for injected channels (Trigger detection disabled)
        cr2.set_jexten(0x00);
        //Set external event select for injected group (Timer 1 TRGO)
        cr2.set_jextsel(0x00);
        // Set data alignment (Right alignment)
        cr2.set_align(false);
        //Set  End of conversion selection
        cr2.set_eocs(false);
        //Set DMA disable selection
        cr2.set_dds(false);
        //Set DMA enable
        cr2.set_dma(false);
        //Set continuous mode enable
        cr2.set_cont(false);
        //Set ADC enable
        cr2.set_adon(true);
    });

    adc.sqr1.update(|sqr1| sqr1.set_l(0x00));

    adc.smpr1.update(|smpr1| {
        //Sample with 84 cycles
        smpr1.set_smp10(0b0100);
        //Sample with 84 cycles
        smpr1.set_smp11(0b0100);
        //Sample with 84 cycles
        smpr1.set_smp12(0b0100);
        //Sample with 84 cycles
        smpr1.set_smp13(0b0100);
        //Sample with 84 cycles
        smpr1.set_smp14(0b0100);
        //Sample with 84 cycles;
        smpr1.set_smp15(0b0100);
        //Sample with 84 cycles
        smpr1.set_smp16(0b0100);
        //Sample with 84 cycles
        smpr1.set_smp17(0b0100);
    });

    adc.smpr2.update(|smpr2| {
        //Sample with 84 cycles
        smpr2.set_smp0(0b0100);
        //Sample with 84 cycles
        smpr2.set_smp1(0b0100);
        //Sample with 84 cycles
        smpr2.set_smp2(0b0100);
        //Sample with 84 cycles
        smpr2.set_smp3(0b0100);
        //Sample with 84 cycles
        smpr2.set_smp4(0b0100);
        //Sample with 84 cycles;
        smpr2.set_smp5(0b0100);
        //Sample with 84 cycles
        smpr2.set_smp6(0b0100);
        //Sample with 84 cycles
        smpr2.set_smp7(0b0100);
    });
}

impl<T: AsRef<[AdcPin]>> Adc<T> {
    pub fn new(adc: &'static mut EmbeddedAdc, pins: T) -> Self {
        init_adc(adc);
        for pin in pins.as_ref() {
            pin.init();
        }

        Adc {
            adc: adc,
            adc_pins: pins,
        }
    }

    pub fn sample(&mut self, channel: u8) -> u16 {
        let adc = &mut self.adc;
        let ret_val: u16;

        // Select channel
        adc.sqr3.update(|sqr3| sqr3.set_sq1(channel));

        // Reset states
        adc.sr.update(|sr| {
            sr.set_eoc(false);
            sr.set_ovr(false);
            sr.set_strt(false);
        });

        // Start conversion
        adc.cr2.update(|cr2| cr2.set_swstart(true));

        while adc.cr2.read().swstart() {}

        if adc.sr.read().strt() {
            DebugLed::info_on();
        }

        while !adc.sr.read().eoc() {}

        DebugLed::info_off();

        ret_val = adc.dr.read().data();

        adc.cr2.update(|cr2| cr2.set_swstart(false));
        return ret_val;
    }
}
