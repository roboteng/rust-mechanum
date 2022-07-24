#![no_std]

use arduino_hal::{
    hal::port::{Dynamic, PB4, PB5, PB6, PH6},
    port::{
        mode::{Output, PwmOutput},
        Pin,
    },
    simple_pwm::*,
};

trait WH {
    fn go(&mut self, dir: i16);
}

pub struct FRWheel {
    pub speed: Pin<PwmOutput<Timer2Pwm>, PH6>,
    pub forward: Pin<Output, Dynamic>,
    pub backward: Pin<Output, Dynamic>,
}

impl WH for FRWheel {
    fn go(&mut self, dir: i16) {
        if dir == 0 {
            self.speed.set_duty(0);
            self.forward.set_low();
            self.backward.set_low();
        }
        if dir > 0 {
            self.speed.set_duty(dir as u8);
            self.forward.set_high();
            self.backward.set_low();
        }
        if dir < 0 {
            let dir = -dir;
            self.speed.set_duty(dir as u8);
            self.forward.set_low();
            self.backward.set_high();
        }
    }
}
pub struct FLWheel {
    pub speed: Pin<PwmOutput<Timer2Pwm>, PB4>,
    pub forward: Pin<Output, Dynamic>,
    pub backward: Pin<Output, Dynamic>,
}

impl WH for FLWheel {
    fn go(&mut self, dir: i16) {
        if dir == 0 {
            self.speed.set_duty(0);
            self.forward.set_low();
            self.backward.set_low();
        }
        if dir > 0 {
            self.speed.set_duty(dir as u8);
            self.forward.set_high();
            self.backward.set_low();
        }
        if dir < 0 {
            let dir = -dir;
            self.speed.set_duty(dir as u8);
            self.forward.set_low();
            self.backward.set_high();
        }
    }
}
pub struct RLWheel {
    pub speed: Pin<PwmOutput<Timer1Pwm>, PB6>,
    pub forward: Pin<Output, Dynamic>,
    pub backward: Pin<Output, Dynamic>,
}

impl WH for RLWheel {
    fn go(&mut self, dir: i16) {
        if dir == 0 {
            self.speed.set_duty(0);
            self.forward.set_low();
            self.backward.set_low();
        }
        if dir > 0 {
            self.speed.set_duty(dir as u8);
            self.forward.set_high();
            self.backward.set_low();
        }
        if dir < 0 {
            let dir = -dir;
            self.speed.set_duty(dir as u8);
            self.forward.set_low();
            self.backward.set_high();
        }
    }
}
pub struct RRWheel {
    pub speed: Pin<PwmOutput<Timer1Pwm>, PB5>,
    pub forward: Pin<Output, Dynamic>,
    pub backward: Pin<Output, Dynamic>,
}

impl WH for RRWheel {
    fn go(&mut self, dir: i16) {
        if dir == 0 {
            self.speed.set_duty(0);
            self.forward.set_low();
            self.backward.set_low();
        }
        if dir > 0 {
            self.speed.set_duty(dir as u8);
            self.forward.set_high();
            self.backward.set_low();
        }
        if dir < 0 {
            let dir = -dir;
            self.speed.set_duty(dir as u8);
            self.forward.set_low();
            self.backward.set_high();
        }
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
    pub fn go(&mut self, p: u8, _: u8) {
        self.front_left.go(p);
        self.front_right.go(p);
        self.back_left.go(p);
        self.back_right.go(p);
    }
}
//   pub  fn forward(&mut self) {
//       self.front_left.go_forward();
//       self.front_right.go_forward();
//       self.back_left.go_forward();
//       self.back_right.go_forward();
//   }
//   pub fn backward(&mut self) {
//       self.front_left.go_bacwards();
//       self.front_right.go_bacwards();
//       self.back_left.go_bacwards();
//       self.back_right.go_bacwards();
//   }

//   pub fn turn_left(&mut self) {
//       self.front_left.go_bacwards();
//       self.front_right.go_forward();
//       self.back_left.go_bacwards();
//       self.back_right.go_forward();
//   }

//   pub  fn turn_right(&mut self) {
//       self.front_left.go_forward();
//       self.front_right.go_bacwards();
//       self.back_left.go_forward();
//       self.back_right.go_bacwards();
//   }

//   pub fn right(&mut self) {
//       self.front_left.go_forward();
//       self.front_right.go_bacwards();
//       self.back_left.go_bacwards();
//       self.back_right.go_forward();
//   }

//   pub fn left(&mut self) {
//       self.front_left.go_bacwards();
//       self.front_right.go_forward();
//       self.back_left.go_forward();
//       self.back_right.go_bacwards();
//   }

//   pub fn stop(&mut self) {
//       self.back_left.stop();
//       self.back_right.stop();
//       self.front_left.stop();
//       self.front_right.stop();
//   }
// }
