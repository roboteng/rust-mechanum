#![cfg_attr(not(test), no_std)]

use arduino_hal::{
    hal::port::{Dynamic, PB4, PB5, PB6, PH6},
    port::{
        mode::{Output, PwmOutput},
        Pin,
    },
    simple_pwm::*,
};

pub struct Dir {
    x: i8,
    y: i8,
}

impl Dir {
    pub fn new(x: i8, y: i8) -> Dir {
        Dir { x, y }
    }
    pub fn dot(&self, other: &Dir) -> i8 {
        return self.x * other.x + self.y * other.y;
    }
}

trait WH {
    fn go(&mut self, dir: i8);
    fn dir() -> Dir;
}

pub struct FRWheel {
    pub speed: Pin<PwmOutput<Timer2Pwm>, PH6>,
    pub forward: Pin<Output, Dynamic>,
    pub backward: Pin<Output, Dynamic>,
}

impl WH for FRWheel {
    fn go(&mut self, dir: i8) {
        if dir == 0 {
            self.speed.set_duty(0);
            self.forward.set_low();
            self.backward.set_low();
        }
        if dir > 0 {
            self.speed.set_duty((dir) as u8);
            self.forward.set_high();
            self.backward.set_low();
        }
        if dir < 0 {
            let dir = -dir;
            self.speed.set_duty((dir) as u8);
            self.forward.set_low();
            self.backward.set_high();
        }
    }

    fn dir() -> Dir {
        Dir::new(1, 1)
    }
}
pub struct FLWheel {
    pub speed: Pin<PwmOutput<Timer2Pwm>, PB4>,
    pub forward: Pin<Output, Dynamic>,
    pub backward: Pin<Output, Dynamic>,
}

impl WH for FLWheel {
    fn go(&mut self, dir: i8) {
        if dir == 0 {
            self.speed.set_duty(0);
            self.forward.set_low();
            self.backward.set_low();
        }
        if dir > 0 {
            self.speed.set_duty((dir) as u8);
            self.forward.set_high();
            self.backward.set_low();
        }
        if dir < 0 {
            let dir = -dir;
            self.speed.set_duty((dir) as u8);
            self.forward.set_low();
            self.backward.set_high();
        }
    }

    fn dir() -> Dir {
        Dir::new(-1, 1)
    }
}
pub struct RLWheel {
    pub speed: Pin<PwmOutput<Timer1Pwm>, PB6>,
    pub forward: Pin<Output, Dynamic>,
    pub backward: Pin<Output, Dynamic>,
}

impl WH for RLWheel {
    fn go(&mut self, dir: i8) {
        if dir == 0 {
            self.speed.set_duty(0);
            self.forward.set_low();
            self.backward.set_low();
        }
        if dir > 0 {
            self.speed.set_duty((dir) as u8);
            self.forward.set_high();
            self.backward.set_low();
        }
        if dir < 0 {
            let dir = -dir;
            self.speed.set_duty((dir) as u8);
            self.forward.set_low();
            self.backward.set_high();
        }
    }

    fn dir() -> Dir {
        Dir::new(1, 1)
    }
}
pub struct RRWheel {
    pub speed: Pin<PwmOutput<Timer1Pwm>, PB5>,
    pub forward: Pin<Output, Dynamic>,
    pub backward: Pin<Output, Dynamic>,
}

impl WH for RRWheel {
    fn go(&mut self, dir: i8) {
        if dir == 0 {
            self.speed.set_duty(0);
            self.forward.set_low();
            self.backward.set_low();
        }
        if dir > 0 {
            self.speed.set_duty((dir) as u8);
            self.forward.set_high();
            self.backward.set_low();
        }
        if dir < 0 {
            let dir = -dir;
            self.speed.set_duty((dir) as u8);
            self.forward.set_low();
            self.backward.set_high();
        }
    }

    fn dir() -> Dir {
        Dir::new(-1, 1)
    }
}

pub struct Robot {
    pub front_right: FRWheel,
    pub front_left: FLWheel,
    pub back_right: RRWheel,
    pub back_left: RLWheel,
}

impl Robot {
    pub fn new(fr: FRWheel, fl: FLWheel, br: RRWheel, bl: RLWheel) -> Robot {
        Robot {
            front_right: fr,
            front_left: fl,
            back_left: bl,
            back_right: br,
        }
    }
    pub fn go(&mut self, dir: &Dir) {
        self.front_left.go(dir.dot(&FLWheel::dir()));
        self.front_right.go(dir.dot(&FRWheel::dir()));
        self.back_left.go(dir.dot(&RLWheel::dir()));
        self.back_right.go(dir.dot(&RRWheel::dir()));
    }
}

pub fn sin(x: i8) -> i8 {
    let x = x as i32;
    if x > 127 {
        return sin((256 - x) as i8);
    }
    if x < -127 {
        return sin((-x) as i8);
    }
    ((22 * x / 7) - (x * x * x / 3170) + (x * x * x * x * x / 105262034)) as i8
}

pub fn cos(x: i8) -> i8 {
    sin(x + 64)
}
