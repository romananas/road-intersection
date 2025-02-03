use sdl2::{pixels::Color, rect::{Point, Rect}, render::*, video::*};

#[derive(Debug,PartialEq,Clone,Copy)]
pub enum State {
    OPEN,
    CLOSED,
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct TrafficLight {
    pub state: State,
    rect: Rect,
    open_state_color: Color,
    closed_state_color: Color,
}

impl TrafficLight {
    /// Create a traffic light of the wanted size and position
    /// The default state is CLOSED
    /// The default colors for OPEN is green
    /// The default colors for closed is red
    pub fn new(center: Point,size: u32) -> Self{
        let rect = Rect::from_center(center, size, size);
        Self {
            rect,
            state:  State::CLOSED,
            open_state_color: Color::GREEN,
            closed_state_color: Color::RED,
        }
    }

    pub fn set_colors(&mut self,open: Color,closed: Color) {
        self.closed_state_color = closed;
        self.open_state_color = open;
    }

    pub fn display(&self,canvas: &mut Canvas<Window>) {
        match self.state {
            State::OPEN => canvas.set_draw_color(self.open_state_color),
            State::CLOSED => canvas.set_draw_color(self.closed_state_color),
        }
        canvas.draw_rect(self.rect).unwrap();
    }

    pub fn is_open(&self) -> bool {
        match self.state {
            State::OPEN => true,
            State::CLOSED => false,
        }
    }
}