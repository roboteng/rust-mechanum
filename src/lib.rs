#![no_std]

use arduino_hal::{port::{Pin, mode::Output}, hal::port::{ Dynamic}};

pub struct Wheel {
  pub speed: Pin<Output, Dynamic>,
  pub forward: Pin<Output, Dynamic>,
  pub backward: Pin<Output, Dynamic>,
}

impl Wheel {
  pub fn stop(&mut self) {
      self.speed.set_low();
      self.forward.set_low();
      self.backward.set_low();
  }
  pub fn go_forward(&mut self) {
      self.speed.set_high();
      self.forward.set_high();
      self.backward.set_low();
  }
  pub fn go_bacwards(&mut self) {
      self.speed.set_high();
      self.forward.set_low();
      self.backward.set_high();
  }
}

pub struct Robot<'a> {
  pub front_right:&'a mut Wheel,
  pub  front_left:&'a mut Wheel,
  pub back_right:&'a mut Wheel,
  pub back_left:&'a mut Wheel,
}

impl<'a> Robot<'a> {
  pub  fn forward(&mut self) {
      self.front_left.go_forward();
      self.front_right.go_forward();
      self.back_left.go_forward();
      self.back_right.go_forward();
  }
  pub fn backward(&mut self) {
      self.front_left.go_bacwards();
      self.front_right.go_bacwards();
      self.back_left.go_bacwards();
      self.back_right.go_bacwards();
  }

  pub fn turn_left(&mut self) {
      self.front_left.go_bacwards();
      self.front_right.go_forward();
      self.back_left.go_bacwards();
      self.back_right.go_forward();
  }

  pub  fn turn_right(&mut self) {
      self.front_left.go_forward();
      self.front_right.go_bacwards();
      self.back_left.go_forward();
      self.back_right.go_bacwards();
  }

  pub fn right(&mut self) {
      self.front_left.go_forward();
      self.front_right.go_bacwards();
      self.back_left.go_bacwards();
      self.back_right.go_forward();
  }

  pub fn left(&mut self) {
      self.front_left.go_bacwards();
      self.front_right.go_forward();
      self.back_left.go_forward();
      self.back_right.go_bacwards();
  }

  pub fn stop(&mut self) {
      self.back_left.stop();
      self.back_right.stop();
      self.front_left.stop();
      self.front_right.stop();
  }
}
