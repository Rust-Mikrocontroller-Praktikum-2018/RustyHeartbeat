#![no_std]
#![no_main]

extern crate embedded_stm32f7;
extern crate r0;
extern crate stm32f7_discovery as stm32f7;

use driver::adc::*;
use filter::Filter;
use heartbeat_writer::HeartBeatWriter;
use stm32f7::{board, embedded, lcd, sdram, system_clock};

mod circular_buffer;
mod driver;
mod filter;
mod heartbeat_writer;

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static __DATA_LOAD: u32;
        static __DATA_END: u32;
        static mut __DATA_START: u32;

        static mut __BSS_START: u32;
        static mut __BSS_END: u32;
    }

    let data_load = &__DATA_LOAD;
    let data_start = &mut __DATA_START;
    let data_end = &__DATA_END;

    let bss_start = &mut __BSS_START;
    let bss_end = &__BSS_END;

    // initializes the .data section
    // (copy the data segment initializers from flash to RAM)
    r0::init_data(data_start, data_end, data_load);
    // zeroes the .bss section
    r0::zero_bss(bss_start, bss_end);

    // Initialize the floating point unit
    let scb = stm32f7::cortex_m::peripheral::scb_mut();
    scb.cpacr.modify(|v| v | 0b1111 << 20);

    main(board::hw());
}

fn main(hw: board::Hardware) -> ! {
    use embedded::interfaces::gpio::*;

    let board::Hardware {
        rcc,
        pwr,
        flash,
        fmc,
        ltdc,
        gpio_a,
        gpio_b,
        gpio_c,
        gpio_d,
        gpio_e,
        gpio_f,
        gpio_g,
        gpio_h,
        gpio_i,
        gpio_j,
        gpio_k,
        adc_3,
        ..
    } = hw;

    system_clock::init(rcc, pwr, flash);

    // Enable ADC clocks
    rcc.apb2enr.update(|r| {
        r.set_adc1en(true);
        r.set_adc2en(true);
        r.set_adc3en(true);
    });

    // enable all gpio ports
    rcc.ahb1enr.update(|r| {
        r.set_gpioaen(true);
        r.set_gpioben(true);
        r.set_gpiocen(true);
        r.set_gpioden(true);
        r.set_gpioeen(true);
        r.set_gpiofen(true);
        r.set_gpiogen(true);
        r.set_gpiohen(true);
        r.set_gpioien(true);
        r.set_gpiojen(true);
        r.set_gpioken(true);
    });

    let mut gpio = Gpio::new(
        gpio_a, gpio_b, gpio_c, gpio_d, gpio_e, gpio_f, gpio_g, gpio_h, gpio_i, gpio_j, gpio_k,
    );

    // init sdram (needed for display buffer)
    sdram::init(rcc, fmc, &mut gpio);

    // lcd controller
    let mut lcd = lcd::init(ltdc, rcc, &mut gpio);
    let mut layer_1 = lcd.window_layer_1().unwrap();
    let mut layer_2 = lcd.layer_2().unwrap();

    layer_1.clear_all();
    layer_2.clear();
    lcd::init_stdout(layer_2);

    let sample_pin: AdcPin;
    sample_pin = AdcPin::new(
        0x40020000,
        stm32f7::embedded::components::gpio::stm32f7::Pin::Pin0,
        AdcChannel::Channel0,
    );

    let adc_pins = [sample_pin];

    let mut adc = Adc::new(adc_3, &adc_pins);
    let mut heart_beat_writer = HeartBeatWriter::new(layer_1, lcd::WIDTH);
    let b = [
        4.31490887804179e-06,
        9.11319407747024e-06,
        1.66455243794543e-05,
        2.52291109497020e-05,
        3.16205641630759e-05,
        2.97017189475716e-05,
        9.71274412438235e-06,
        -4.22734652436985e-05,
        -0.000144360826492747,
        -0.000318124745221734,
        -0.000586990377904722,
        -0.000973565237688662,
        -0.00149595770646612,
        -0.00216322578494236,
        -0.00297030221213582,
        -0.00389290924703200,
        -0.00488311766547121,
        -0.00586627182277468,
        -0.00674003494098334,
        -0.00737617805170233,
        -0.00762554600514187,
        -0.00732633831037168,
        -0.00631547762321797,
        -0.00444242212728042,
        -0.00158441079569371,
        0.00233819624836742,
        0.00734802433889993,
        0.0133998275841555,
        0.0203741241901004,
        0.0280764709946305,
        0.0362430504217557,
        0.0445526520326726,
        0.0526444822193738,
        0.0601406014537053,
        0.0666712499683998,
        0.0719009345048983,
        0.0755529709472847,
        0.0774302429376902,
        0.0774302429376902,
        0.0755529709472847,
        0.0719009345048983,
        0.0666712499683998,
        0.0601406014537053,
        0.0526444822193738,
        0.0445526520326726,
        0.0362430504217557,
        0.0280764709946305,
        0.0203741241901004,
        0.0133998275841555,
        0.00734802433889993,
        0.00233819624836742,
        -0.00158441079569371,
        -0.00444242212728042,
        -0.00631547762321797,
        -0.00732633831037168,
        -0.00762554600514187,
        -0.00737617805170233,
        -0.00674003494098334,
        -0.00586627182277468,
        -0.00488311766547121,
        -0.00389290924703200,
        -0.00297030221213582,
        -0.00216322578494236,
        -0.00149595770646612,
        -0.000973565237688662,
        -0.000586990377904722,
        -0.000318124745221734,
        -0.000144360826492747,
        -4.22734652436985e-05,
        9.71274412438235e-06,
        2.97017189475716e-05,
        3.16205641630759e-05,
        2.52291109497020e-05,
        1.66455243794543e-05,
        9.11319407747024e-06,
        4.31490887804179e-06,
    ];
    let mut filter = Filter::new(b);
    //let mut avg = 4095.0 / 2.0;
    loop {
        let x = adc.sample(0) as f32;

        let y = filter.filter(x);

        //avg = 0.9 * avg + 0.1 * y;

        let y = (y - filter.get_average()) * 2.0 + filter.get_average();

        let y = if y > 4095.0 {
            4095.0
        } else if y < 0.0 {
            0.0
        } else {
            y
        };

        let current_sample = ((271.0 / 4095.0) * y as f32) as usize;
        heart_beat_writer.add_new_data(&mut lcd, current_sample);
        system_clock::wait(2);
    }
}
