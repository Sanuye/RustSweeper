use ggez::*;
use rand::Rng;

//GameState
struct State{
}

// Alle Felder
struct Field {
	mine: bool,		// Hat Miene oder nicht
	size: i32,	// Größe des Feldes
	x: f32,		//Position
	y: f32,		//Position
	counts: i32,	// Zähler, wie viele Mienen in der Nähe sind. if mine: counts = 0 
	id: i32,			// Später zufällige IDs mit Mienen belegen. 0-29 bei 30 Feldern.
}

//To-Do Funktionen:
//Mienen
//Pos
//Counts

impl Field {	

	fn get_x(id: i32, size: i32) -> f32 {
		let x0 = 50;
		let x1 = x0 + size * (id % 5);
		return x1 as f32
	}
	
	fn get_y(id: i32, size: i32) -> f32 {
		let y0 = 50;
		if id % 5 == 0 {
			return (y0 + id * size) as f32;
		} else {
			return (y0 + size * (id - (id % 5))) as f32;
		}
	}

	fn new_field (id: i32, x: f32, y: f32) -> Field {
		Field {
			mine: false,
			size: 20,
			x: x,
			y: y,
			counts: 0,
			id: id,
		}
	}
	
	fn place_mine(&mut self) {
		let mut rng = rand::thread_rng();
		let m = rng.gen_range(0, 10);
		if m == 0 {
			self.mine = true;
		}
	}
	
	fn draw(ctx: &mut Context) -> GameResult<()> {
		let mut draw_counter = 0;
		while draw_counter < 25 {
			let x = Field::get_x(draw_counter, 50);
			let y = Field::get_y(draw_counter, 10);
			println!("{}, {}, {}", x, y, draw_counter);
			let f = Field::new_field(draw_counter, x, y);
			let rectangle = graphics::Mesh::new_rectangle(
				ctx,
				graphics::DrawMode::stroke(3.0),
				graphics::Rect::new(f.x, f.y, 50.0, 50.0),
				graphics::WHITE,
			)?;
			graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
			graphics::present(ctx)?;
			draw_counter = draw_counter + 1;
		}
		Ok(())
	}
}


impl ggez::event::EventHandler for State {
	fn update (&mut self, ctx: &mut Context) -> GameResult<()> {
		Ok(())
	}
	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		Field::draw(ctx)?;
		Ok(())
	}
}

pub fn main() {
	let state = &mut State{};
	let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "awesome person")
		.build()
		.unwrap();
	event::run(ctx, event_loop, state).unwrap();
}