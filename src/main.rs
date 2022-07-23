#![no_std]
#![no_main]

use arduino_hal::{port::{Pin, mode::Output}, hal::port::{PB7, Dynamic}, delay_ms};
use panic_halt as _;

struct Context{
    led: Pin<Output, PB7>
}

#[arduino_hal::entry]
fn main() -> ! {
    let mut ctx = init();
    loop {
        iter(&mut ctx);
    }
}

struct Wheel {
    speed: Pin<Output, Dynamic>,
    forward: Pin<Output, Dynamic>,
    backward: Pin<Output, Dynamic>,
}

impl Wheel {
    fn stop(&mut self) {
        self.speed.set_low();
        self.forward.set_low();
        self.backward.set_low();
    }
    fn go_forward(&mut self) {
        self.speed.set_high();
        self.forward.set_high();
        self.backward.set_low();
    }
    fn go_bacwards(&mut self) {
        self.speed.set_high();
        self.forward.set_low();
        self.backward.set_high();
    }
}

fn init()-> Context {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let led = pins.d13.into_output();
    let mut front_left = Wheel {
        speed: pins.d10.into_output().downgrade(),
        forward:  pins.d28.into_output().downgrade(),
        backward: pins.d26.into_output().downgrade(),
    };
    let mut back_left = Wheel {
        speed: pins.d12.into_output().downgrade(),
        forward:  pins.d8.into_output().downgrade(),
        backward: pins.d7.into_output().downgrade(),
    };
    let mut front_right = Wheel {
        speed: pins.d9.into_output().downgrade(),
        forward:  pins.d24.into_output().downgrade(),
        backward: pins.d22.into_output().downgrade(),
    };
    let mut back_right = Wheel {
        speed: pins.d11.into_output().downgrade(),
        forward:  pins.d6.into_output().downgrade(),
        backward: pins.d5.into_output().downgrade(),
    };
    let mut wheels = [&mut front_right, &mut front_left, &mut back_right, &mut back_left];

    wheels.iter_mut().for_each(|w| w.go_forward());
    delay_ms(1000);

    wheels.iter_mut().for_each(|w| w.stop());
    delay_ms(1000);

    wheels.iter_mut().for_each(|w| w.go_bacwards());
    delay_ms(1000);

    wheels.iter_mut().for_each(|w| w.stop());
    delay_ms(1000);

    Context { led }
}

fn iter( ctx:&mut Context) {
    ctx.led.toggle();
    arduino_hal::delay_ms(1000);
}
