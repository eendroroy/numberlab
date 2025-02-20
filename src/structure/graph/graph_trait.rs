use num::traits::real::Real;
use std::fmt::Display;

pub trait GraphWeightTrait: Real + Copy + Clone + Display {}

impl<W: Real + Copy + Clone + Display> GraphWeightTrait for W {}
