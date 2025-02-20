use num::Num;
use std::fmt::Display;

pub trait GraphWeightTrait: Num + Copy + Clone + Display + PartialOrd {}

impl<W: Num + Copy + Clone + Display + PartialOrd> GraphWeightTrait for W {}
