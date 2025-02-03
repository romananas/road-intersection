use crate::utils::*;
use road_intersection::traffic_lights::TrafficLight;
use std::collections::HashMap;
use road_intersection::road::Road;
use std::cell::Cell;


use sdl2::rect::Point;

pub fn setup_roads(win_height: i32,win_width: i32,road_width: u32) -> HashMap<(Sens,Cardinals),Road>{
    let mut result: HashMap<(Sens,Cardinals),Road> = HashMap::new();

    let h_center = win_height/2;
    let w_center = win_width/2;
    let demi_w = road_width as i32 / 2;

    // Sometime you have bad days.
    let vec_line = vec![
        (Point::new(w_center - road_width as i32, h_center - demi_w),Point::new(0, h_center - demi_w),Sens::FROM,Cardinals::EAST),
        (Point::new(0, h_center + demi_w),Point::new(w_center - road_width as i32, h_center + demi_w),Sens::TO,Cardinals::EAST),

        (Point::new(win_height, h_center - demi_w),Point::new(w_center + road_width as i32, h_center - demi_w),Sens::TO,Cardinals::WEST),
        (Point::new(w_center + road_width as i32, h_center + demi_w),Point::new(win_height, h_center + demi_w),Sens::FROM,Cardinals::WEST),

        (Point::new(w_center - demi_w, 0),Point::new(w_center - demi_w,w_center - road_width as i32),Sens::FROM,Cardinals::NORTH),
        (Point::new(w_center + demi_w,w_center - road_width as i32),Point::new(w_center + demi_w, 0),Sens::TO,Cardinals::NORTH),

        (Point::new(w_center - demi_w,w_center + road_width as i32),Point::new(w_center - demi_w, win_width),Sens::TO,Cardinals::SOUTH),
        (Point::new(w_center + demi_w, win_width),Point::new(w_center + demi_w,w_center + road_width as i32),Sens::FROM,Cardinals::SOUTH),
    ];

    for (start,end,s,c) in vec_line {
        result.insert((s,c), Road::new(start, end, road_width));
    }
    result
}

pub fn setup_lights(win_height: u32,win_width: u32,size: u32,road_width: u32) -> Vec<Cell<TrafficLight>> {
    let h_center = win_height/2;
    let w_center = win_width/2;

    let up_left = Cell::new(TrafficLight::new(Point::new(((w_center - road_width) - size/2) as i32, ((h_center - road_width) - size/2) as i32), size));
    let up_right = Cell::new(TrafficLight::new(Point::new(((w_center + road_width) + size/2 + 1) as i32, ((h_center - road_width) - size/2) as i32), size));

    let down_left =  Cell::new(TrafficLight::new(Point::new(((w_center - road_width) - size/2) as i32, ((h_center + road_width) + size/2 + 1) as i32), size));
    let down_right = Cell::new(TrafficLight::new(Point::new(((w_center + road_width) + size/2 + 1) as i32, ((h_center + road_width) + size/2 + 1) as i32), size));
    vec![up_left,up_right,down_left,down_right]
}