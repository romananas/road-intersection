use sdl2::{pixels::Color, rect::*, render::*, video::*};


#[derive(Debug,Clone, Copy)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

#[derive(Debug,Clone, Copy)]
pub struct Road {
    pub line: Line,
    width: u32,
    color: Color,
}

impl Road {
    pub fn new(start: Point,end: Point,width: u32) -> Self {
        Self {
            line: Line {start,end},
            width,
            color: Color::GRAY,
        }
    }

    pub fn display(&self,canvas:&mut Canvas<Window>) -> Result<(), Box<dyn std::error::Error>>{
        let border_a = self.line.new_offset(self.width as i32 /2);
        let border_b = self.line.new_offset(-(self.width as i32 /2));
        canvas.set_draw_color(self.color);
        canvas.draw_line(border_a.start, border_a.end)?;
        canvas.draw_line(border_b.start, border_b.end)?;
        Ok(())
    }

    pub fn display_points(&self,canvas:&mut Canvas<Window>) -> Result<(), Box<dyn std::error::Error>>{
        canvas.set_draw_color(Color::BLUE);
        canvas.fill_rect(Rect::from_center(self.line.start, 3, 3))?;

        canvas.set_draw_color(Color::MAGENTA);
        canvas.fill_rect(Rect::from_center(self.line.end, 3, 3))?;
        Ok(())
    }
}

impl Line {
    // Génère une ligne parallèle avec un offset donné
    fn new_offset(&self, offset: i32) -> Self {
        // Calcul du vecteur directeur de la ligne
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;

        // Norme du vecteur directeur
        let length = ((dx.pow(2) + dy.pow(2)) as f64).sqrt() as i32;

        // Vecteur perpendiculaire unitaire
        let offset_dx = -dy / length;
        let offset_dy = dx / length;

        // Appliquer l'offset pour créer la nouvelle ligne
        Self {
            start: Point::new(self.start.x + offset_dx * offset as i32, self.start.y + offset_dy * offset as i32),
            end: Point::new(self.end.x + offset_dx * offset as i32, self.end.y + offset_dy * offset as i32),
        }
    }
}