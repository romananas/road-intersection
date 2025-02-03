#[allow(dead_code,unused_imports)]

use std::{cell::{Cell, RefCell}, rc::Rc};
use road_intersection::traffic_lights::*;

pub fn is_green(t: &Cell<TrafficLight>) -> bool {
    t.get().state == State::OPEN
}