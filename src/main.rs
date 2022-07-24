#![no_std]
#![no_main]

use arduino_hal::simple_pwm::IntoPwmPin;
use arduino_hal::{
    delay_ms,
    hal::port::PB7,
    port::{mode::PwmOutput, Pin},
    simple_pwm::{Prescaler, Timer0Pwm, Timer1Pwm, Timer2Pwm},
};
use arduino_learn::{FLWheel, FRWheel, RLWheel, RRWheel, Robot};
use panic_halt as _;

struct Context {
    led: Pin<PwmOutput<Timer0Pwm>, PB7>,
}

#[arduino_hal::entry]
fn main() -> ! {
    let mut ctx = init();
    let mut t = 0;
    loop {
        iter(&mut ctx, t);
        delay_ms(10);
        t += 1;
    }
}

fn init() -> Context {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let led = pins.d13.into_output().into_pwm(&mut timer0);
    let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

    let mut robot = Robot::new(
        FRWheel {
            speed: pins.d9.into_output().into_pwm(&mut timer2),
            forward: pins.d24.into_output().downgrade(),
            backward: pins.d22.into_output().downgrade(),
        },
        FLWheel {
            speed: pins.d10.into_output().into_pwm(&mut timer2),
            forward: pins.d28.into_output().downgrade(),
            backward: pins.d26.into_output().downgrade(),
        },
        RRWheel {
            speed: pins.d11.into_output().into_pwm(&mut timer1),
            forward: pins.d6.into_output().downgrade(),
            backward: pins.d5.into_output().downgrade(),
        },
        RLWheel {
            speed: pins.d12.into_output().into_pwm(&mut timer1),
            forward: pins.d8.into_output().downgrade(),
            backward: pins.d7.into_output().downgrade(),
        },
    );
    robot.front_right.speed.enable();
    robot.front_left.speed.enable();
    robot.back_left.speed.enable();
    robot.back_right.speed.enable();

    (0..255).for_each(|i| {
        robot.go(255 - i, 255 - i);
        delay_ms(10);
    });

    // let mut front_left = Wheel {
    //     speed: pins.d10.into_output().downgrade(),
    //     forward:  pins.d28.into_output().downgrade(),
    //     backward: pins.d26.into_output().downgrade(),
    // };
    // let mut back_left = Wheel {
    //     speed: pins.d12.into_output().downgrade(),
    //     forward:  pins.d8.into_output().downgrade(),
    //     backward: pins.d7.into_output().downgrade(),
    // };
    // let mut front_right = Wheel {
    // speed: pins.d9.into_output().downgrade(),
    //     forward:  pins.d24.into_output().downgrade(),
    //     backward: pins.d22.into_output().downgrade(),
    // };
    // let mut back_right = Wheel {
    // speed: pins.d11.into_output().downgrade(),
    //     forward:  pins.d6.into_output().downgrade(),
    //     backward: pins.d5.into_output().downgrade(),
    // };

    // let mut robot = Robot{
    //     front_right: &mut front_right,
    //     front_left: &mut front_left,
    //     back_right: &mut back_right,
    //     back_left: &mut back_left,
    // };

    // robot.forward();
    // arduino_hal::delay_ms(1000);
    // robot.stop();
    // arduino_hal::delay_ms(1000);

    // robot.left();
    // arduino_hal::delay_ms(1000);
    // robot.stop();
    // arduino_hal::delay_ms(1000);

    // robot.backward();
    // arduino_hal::delay_ms(1000);
    // robot.stop();
    // arduino_hal::delay_ms(1000);

    // robot.right();
    // arduino_hal::delay_ms(1000);
    // robot.stop();
    // arduino_hal::delay_ms(1000);

    Context { led }
}

fn iter(ctx: &mut Context, _: i32) {
    ctx.led.enable();
    ctx.led.set_duty(255);
    arduino_hal::delay_ms(1000);
    ctx.led.set_duty(64);
    arduino_hal::delay_ms(1000);
}
