use sdl2::{rect::*,render::*,video::*,pixels::Color};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
// use std::time::{SystemTime, UNIX_EPOCH};

pub enum Task {
    GoTo(Point),
    WaitFor(Rc<dyn Fn() -> bool>)
}

pub struct Vehicle{
    pub rect: Rect,
    // pub direction: Direction,

    speed: u32,
    color: Color,

    task: Vec<Task>,
}


impl Deref for Vehicle {
    type Target = Rect;
    fn deref(&self) -> &Self::Target {
        &self.rect
    }
}

impl DerefMut for Vehicle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rect
    }
}


impl Vehicle {
    pub fn new(center: Point,color: Color) -> Self {
        let rect = Rect::from_center(center, 10, 10);

        Self {
            rect,
            speed: 10,
            color,
            // direction,
            task: Vec::new(),
        }
    }

    pub fn add_task(&mut self,task: Task) {
        self.task.push(task);
    }

    pub fn display(&self,canvas: &mut Canvas<Window>) -> Result<(),String> {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.rect)?;
        Ok(())
    }

    pub fn refresh(&mut self) -> bool{
        if self.task.len() == 0 {
            return true;
        }
        let task = match &self.task[0] {
            Task::GoTo(i) => self.go_to(*i),
            Task::WaitFor(f) => {(*f)()},
        };
        if task {
            self.task.remove(0);
        }
        false
    }

    fn go_to(&mut self,target: Point) -> bool {
        let pts_dist = (((target.x - self.x).pow(2) + (target.y - self.y).pow(2)) as f64).sqrt();
        let distance = self.speed as f64;

        if pts_dist <= distance {
            self.x = target.x;
            self.y = target.y;
            return true;
        }
         // Calcul du vecteur direction (flottants pour la précision)
         let dx = (target.x - self.x) as f64;
         let dy = (target.y - self.y) as f64;
 
         // Calcul de la norme du vecteur
         let magnitude = (dx.powi(2) + dy.powi(2)).sqrt();
 
         // Normalisation du vecteur direction
         let unit_x = dx / magnitude;
         let unit_y = dy / magnitude;

        self.x = (self.x as f64 + unit_x * distance).round() as i32;
        self.y = (self.y as f64 + unit_y * distance).round() as i32;

        self.x == target.x && self.y == target.y
    }
}