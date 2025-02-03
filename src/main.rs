#[allow(dead_code,unused_imports)]

mod utils;
mod setups;
mod logic;


use logic::is_green;
use road_intersection::*;
use traffic_lights::{lights, State};
use std::{cell::Cell, rc::Rc, time::Duration};

use sdl2::{ rect::Point, video::Window, EventPump};


/// Init the event pump and the window of sdl2
fn init(size: u32) -> Result<(Window,EventPump), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;     
    
    let window = video_subsystem
        .window("Crossroad", size, size)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let event_pump = sdl_context.event_pump()?;
    Ok((window,event_pump))
}

fn main() -> Result<(), String> {
    let size = 800;
    let (win,mut event_pump) = init(size).unwrap();
    let mut canvas = win.clone().into_canvas().build().map_err(|e| e.to_string())?;

    let roads = setups::setup_roads(size as i32, size as i32, 50);

    let lights = setups::setup_lights(size, size, 30, 50);
    
    // Loop
    'running: loop {
        // Gestion des événements
        use utils::Action;
        match utils::keybinds(&mut event_pump) {
            Action::QUIT => break 'running,
            _ => {},
        }

        for (_,road) in roads.iter() {
            road.display(&mut canvas).unwrap();
            road.display_points(&mut canvas).unwrap();
        }
        for light in lights.iter() {
            light.get().display(&mut canvas);
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
