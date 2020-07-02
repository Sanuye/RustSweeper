use ggez::*;
use rand::Rng;

//GameState

struct State{
	fields: Vec<Field>,
}

// Alle Felder
pub struct Field {
	mine: bool,		//Hat Miene oder nicht
	x: f32,			//Position
	y: f32,			//Position
	counts: i32,	//Zähler, wie viele Mienen in der Nähe sind. if mine: counts = 0 
	id: i32,		//Später zufällige IDs mit Mienen belegen. 0-29 bei 30 Feldern.
	clicked: bool,	//Ob feld schon geklickt wurde, oder nicht
}

/*Letzter Schritt:
INIT: Alle Felder weiß 	- erledigt.
Wenn Linksklick:
Wenn Über Feld, dass noch nicht angeklickt wurde:
Decke Feld auf
Wenn Miene - Decke Alle Mienen auf, rest bleibt wie er ist -> gameover
Wenn Counts >0 -> Decke Feld auf
Wenn Counts = 0 -> Decke Alle Felder herum auf und überprüfe, welche Counts = 0 haben, wiederhole es bei diesen
Wenn alle Felder, außer den Mienen aufgedeckt wurden -> GameWon
*/



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
			x: x,
			y: y,
			counts: 0,
			id: id,
			clicked: false,
		}
	}
	
	fn place_mine(&mut self) {
		let mut rng = rand::thread_rng();
		let m = rng.gen_range(0, 3);
		if m == 0 {
			self.mine = true;
		}
	}
}

fn get_count(state: &State, i: i32) -> i32 {
//Wir müssen immer zuerst gucken, ob das Feld, das wir überprüfen auch das Feld ist, das wir meinen.
	let mut c: i32 = 0;
	if (i+1) % 5 != 0 && i+1 < 40{
		let x = i as usize;
		if state.fields[x+1].mine == true {
				c = c + 1;
		}
	}
	if (i-1) % 5 != 4 && i-1 >= 0 {
		let x = i-1;
		let dex = x as usize;
		if state.fields[dex].mine == true {
				c = c + 1;
		}
	}
	if (i+5) < 40 {
		let x = i as usize;
		if state.fields[x+5].mine == true {
				c = c + 1;
		}
	}
	if i-5 >= 0 {
		let x = i-5;
		let dex = x as usize;
		if state.fields[dex].mine == true {
				c = c + 1;
		}
	}
	if (i+4) % 5 != 4 && i+4 < 40 {
		let x = i as usize;
		if state.fields[x+4].mine == true {
				c = c + 1;
		}
	}
	if (i-4) % 5 != 0 && i-4 >= 0 {
		let x = i-4;
		let dex = x as usize;
		if state.fields[dex].mine == true {
				c = c + 1;
		}
	}
	if (i+6) % 5 != 0 && i+6 < 40 {
		let x = i as usize;
		if state.fields[x+6].mine == true {
				c = c + 1;
		}
	}
	if (i-6) % 5 != 4 && i-6 >= 0 {
		let x = i-6;
		let dex = x as usize;
		if state.fields[dex].mine == true {
				c = c + 1;
		}
	}
	return c;
}

fn set_counts(state: &mut State) {
	let mut i = 0;
	while i < 40 {
		let c = get_count(&state, i);
		let x = i as usize;
		let f = &mut state.fields[x];
		f.counts = c;
		i = i + 1;
	}
}


impl ggez::event::EventHandler for State {
	fn update (&mut self, ctx: &mut Context) -> GameResult<()> {
		Ok(())
	}
	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		for field in &self.fields {
			if field.mine == true {
				let rectangle = graphics::Mesh::new_rectangle(
					ctx,
					graphics::DrawMode::fill(),
					graphics::Rect::new(field.x, field.y, 50.0, 50.0),
					graphics::BLACK,
				)?;
				graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
			}
			else if field.mine == false && field.counts == 1 {							//helleres blau
				let rectangle = graphics::Mesh::new_rectangle(
					ctx,
					graphics::DrawMode::fill(),
					graphics::Rect::new(field.x, field.y, 50.0, 50.0),
					graphics::Color::new(0.0, 0.0, 1.0, 1.0),
				)?;
				graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
			}
			else if field.mine == false && field.counts == 2 {							//dunkleres grün
				let rectangle = graphics::Mesh::new_rectangle(
					ctx,
					graphics::DrawMode::fill(),
					graphics::Rect::new(field.x, field.y, 50.0, 50.0),
					graphics::Color::new(0.0, 0.5, 0.0, 1.0),
				)?;
				graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
			}
			else if field.mine == false && field.counts == 3 {							//helleres rot
				let rectangle = graphics::Mesh::new_rectangle(
					ctx,
					graphics::DrawMode::fill(),
					graphics::Rect::new(field.x, field.y, 50.0, 50.0),
					graphics::Color::new(1.0, 0.0, 0.0, 1.0),
				)?;
				graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
			}
			else if field.mine == false && field.counts == 4 {							//dunkleres blau
				let rectangle = graphics::Mesh::new_rectangle(
					ctx,
					graphics::DrawMode::fill(),
					graphics::Rect::new(field.x, field.y, 50.0, 50.0),
					graphics::Color::new(0.0, 0.0, 0.52, 1.0),
				)?;
				graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
			}
			else if field.mine == false && field.counts == 5 {							//dunkleres rot
				let rectangle = graphics::Mesh::new_rectangle(
					ctx,
					graphics::DrawMode::fill(),
					graphics::Rect::new(field.x, field.y, 50.0, 50.0),
					graphics::Color::new(0.52, 0.0, 0.0, 1.0),
				)?;
				graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
			}
			else if field.mine == false && field.counts == 6 {							//türkis
				let rectangle = graphics::Mesh::new_rectangle(
					ctx,
					graphics::DrawMode::fill(),
					graphics::Rect::new(field.x, field.y, 50.0, 50.0),
					graphics::Color::new(0.0, 0.5, 0.52, 1.0),
				)?;
				graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
			}
			else if field.mine == false && field.counts == 7 {							//lila
				let rectangle = graphics::Mesh::new_rectangle(
					ctx,
					graphics::DrawMode::fill(),
					graphics::Rect::new(field.x, field.y, 50.0, 50.0),
					graphics::Color::new(0.52, 0.0, 0.52, 1.0),
				)?;
				graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
			}
			else if field.mine == false && field.counts == 8 {							//grau
				let rectangle = graphics::Mesh::new_rectangle(
					ctx,
					graphics::DrawMode::fill(),
					graphics::Rect::new(field.x, field.y, 50.0, 50.0),
					graphics::Color::new(0.78, 0.78, 0.8, 1.0),
				)?;
				graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
			}
			else {
				let rectangle = graphics::Mesh::new_rectangle(
					ctx,
					graphics::DrawMode::fill(),
					graphics::Rect::new(field.x, field.y, 50.0, 50.0),
					graphics::WHITE,
				)?;
				graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
			}
			if field.clicked == false {													//Wenn Feld noch nicht angeklickt wurde, bleibt es weiß
				let rectangle = graphics::Mesh::new_rectangle(
					ctx,
					graphics::DrawMode::fill(),
					graphics::Rect::new(field.x, field.y, 50.0, 50.0),
					graphics::WHITE,
				)?;
				graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
			}
		}
		graphics::present(ctx);
		Ok(())
	}
}

pub fn main() {
	let mut fields = Vec::new();
	let mut draw_counter = 0;
		while draw_counter < 40 {
			let x = Field::get_x(draw_counter, 50);
			let y = Field::get_y(draw_counter, 10);
			let mut f = Field::new_field(draw_counter, x, y);
			f.place_mine();
			fields.push(f);
			draw_counter = draw_counter + 1;
		}	
	let state = &mut State{fields: fields};
	set_counts(state);
	let (ref mut ctx, ref mut event_loop) = ggez::ContextBuilder::new("hello_ggez", "awesome person")
		.build()
		.unwrap();
	event::run(ctx, event_loop, state).unwrap();
}