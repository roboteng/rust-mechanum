#![no_std]
#![no_main]

use arduino_hal::{port::{Pin, mode::Output}, hal::port::{PB7}};
use arduino_learn::{Wheel, Robot};
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

    let mut robot = Robot{
        front_right: &mut front_right,
        front_left: &mut front_left,
        back_right: &mut back_right,
        back_left: &mut back_left,
    };

    robot.forward();
    arduino_hal::delay_ms(1000);
    robot.stop();
    arduino_hal::delay_ms(1000);

    robot.left();
    arduino_hal::delay_ms(1000);
    robot.stop();
    arduino_hal::delay_ms(1000);

    robot.backward();
    arduino_hal::delay_ms(1000);
    robot.stop();
    arduino_hal::delay_ms(1000);

    robot.right();
    arduino_hal::delay_ms(1000);
    robot.stop();
    arduino_hal::delay_ms(1000);

    Context { led }
}

fn iter( ctx:&mut Context) {
    ctx.led.toggle();
    arduino_hal::delay_ms(1000);
}
