use sdl2::{event::Event, keyboard::Keycode, EventPump};
use road_intersection::vehicles::*;
use std::collections::HashMap;
use sdl2::pixels::Color;
use road_intersection::road::Road;


#[derive(Debug,Clone, Copy,Eq, Hash, PartialEq)]
pub enum Cardinals {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

#[derive(Debug,Clone, Copy,Eq, Hash, PartialEq)]
pub enum Sens {
    FROM,
    TO,
}


pub enum Action {
    UP,
    DOWN,
    RIGHT,
    LEFT,
    RKEY,
    QUIT,
    None,
}

#[derive(Debug,Clone, Copy)]
pub enum Direction {
    Forward,
    Left,
    Right,
}

pub fn keybinds(event_pump: &mut EventPump) -> Action{
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return Action::QUIT,
            // Event::KeyDown { keycode: Some(Keycode::UP), .. } => {
            //     return Some(Key::Any("UP"));
            // }
            // Event::KeyDown { keycode: Some(Keycode::DOWN), .. } => {
            //     return Some(Key::Any("DOWN"));
            // }
            // Event::KeyDown { keycode: Some(Keycode::LEFT), .. } => {
            //     return Some(Key::Any("LEFT"));
            // }
            // Event::KeyDown { keycode: Some(Keycode::RIGHT), .. } => {
            //     return Some(Key::Any("RIGHT"));
            // }
            // Event::KeyDown { keycode: Some(Keycode::R), .. } => {
            //     return Some(Key::Any("R"));
            // }
            _ => {}
        }
    }
    Action::None
}

pub fn spawn_vehicle(roads: HashMap<(Sens,Cardinals),Road>,direction: Direction,sens: Sens,card: Cardinals) -> Option<vehicles::Vehicle> {
    let road = roads.get(&(sens,card))?;
    let color = match direction {
        Direction::Left => Color::YELLOW,
        Direction::Right => Color::MAGENTA,
        Direction::Forward => Color::BLUE,
      };
    let car = vehicles::Vehicle::new(road.line.start, color);
    todo!()
}
