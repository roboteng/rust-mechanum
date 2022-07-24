#![no_std]

use arduino_hal::{port::{Pin, mode::{Output, PwmOutput}}, hal::port::{ Dynamic,  PH6}, simple_pwm::*, Peripherals};

trait WH {
    fn go(&mut self, dir: u8);
}

pub struct FRWheel {
  pub speed: Pin<PwmOutput<Timer2Pwm>, PH6>,
  pub forward: Pin<Output, Dynamic>,
  pub backward: Pin<Output, Dynamic>,
}

impl FRWheel {
    pub fn new(dp: Peripherals) -> FRWheel{
      let pins = arduino_hal::pins!(dp);
      let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

      FRWheel {
        speed: pins.d9.into_output().into_pwm(&mut timer2),
        forward:  pins.d24.into_output().downgrade(),
        backward: pins.d22.into_output().downgrade(),
      }
    }
}

impl WH for FRWheel {
    fn go(&mut self, dir: u8) {
        if dir == 0{
          self.speed.set_duty(0);
          self.forward.set_low();
          self.backward.set_low();
        }
        if dir > 0 {
          self.speed.set_duty(dir);
          self.forward.set_high();
          self.backward.set_low();
        }
    }
}

pub struct Robot {
  pub front_right:FRWheel,
}

impl Robot {
  pub fn new(fr: FRWheel) -> Robot {
    Robot { front_right:fr }
  }
  pub fn go(&mut self,p: u8) {
    self.front_right.go(p);
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
