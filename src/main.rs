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

struct Robot<'a> {
    front_right:&'a mut Wheel,
    front_left:&'a mut Wheel,
    back_right:&'a mut Wheel,
    back_left:&'a mut Wheel,
}

impl<'a> Robot<'a> {
    fn forward(&mut self) {
        self.front_left.go_forward();
        self.front_right.go_forward();
        self.back_left.go_forward();
        self.back_right.go_forward();
    }
    fn backward(&mut self) {
        self.front_left.go_bacwards();
        self.front_right.go_bacwards();
        self.back_left.go_bacwards();
        self.back_right.go_bacwards();
    }

    fn turn_left(&mut self) {
        self.front_left.go_bacwards();
        self.front_right.go_forward();
        self.back_left.go_bacwards();
        self.back_right.go_forward();
    }

    fn turn_right(&mut self) {
        self.front_left.go_forward();
        self.front_right.go_bacwards();
        self.back_left.go_forward();
        self.back_right.go_bacwards();
    }

    fn right(&mut self) {
        self.front_left.go_forward();
        self.front_right.go_bacwards();
        self.back_left.go_bacwards();
        self.back_right.go_forward();
    }

    fn left(&mut self) {
        self.front_left.go_bacwards();
        self.front_right.go_forward();
        self.back_left.go_forward();
        self.back_right.go_bacwards();
    }

    fn stop(&mut self) {
        self.back_left.stop();
        self.back_right.stop();
        self.front_left.stop();
        self.front_right.stop();
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
    delay_ms(1000);
    robot.stop();
    delay_ms(1000);

    robot.left();
    delay_ms(1000);
    robot.stop();
    delay_ms(1000);

    robot.backward();
    delay_ms(1000);
    robot.stop();
    delay_ms(1000);

    robot.right();
    delay_ms(1000);
    robot.stop();
    delay_ms(1000);

    Context { led }
}

fn iter( ctx:&mut Context) {
    ctx.led.toggle();
    arduino_hal::delay_ms(1000);
}
