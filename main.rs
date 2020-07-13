//RustSweeper
//by Alexander Chmielus and Jasmine Cavael
//Rust ABV SoSe2020
//Freie Universitaet Berlin

use ggez::*;
use rand::Rng;

//GameState

struct State{
	fields: Vec<Field>,
	game_over: bool,
	game_won: bool,
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

	fn get_x(id: i32, size: i32) -> f32 {						//gebe x Position aus
		let x0 = 50;
		let x1 = x0 + size * (id % 5);
		return x1 as f32
	}
	
	fn get_y(id: i32, size: i32) -> f32 {						//gebe y Position aus : komplizierter, da wir in der Reihe eine bestimmte Anzahl an Feldern haben wollen (Orientierung daran)
		let y0 = 50;
		if id % 5 == 0 {
			return (y0 + id * size) as f32;
		} else {
			return (y0 + size * (id - (id % 5))) as f32;
		}
	}

	fn new_field (id: i32, x: f32, y: f32) -> Field {			//wir erstellen ein neues Feld
		Field {									
			mine: false,										//erstmal keine Miene gegeben, wird später random implementiert
			x: x,												//x und y sind die Koordianten
			y: y,		
			counts: 0,											//counter erstmal null, wird durch die Mienen und Counter-Funktion weiter unten später bestimmt
			id: id,												//Feld hat ID, um spätere Funktionen wie Count leichter implementieren zu können
			clicked: false,										//clicked wird später true, falls das Feld angeklickt wird und freigegeben werden muss
		}	
	}
	
	fn place_mine(&mut self) {									//hier werden random 1-3 Miene auf unsere Felder platziert
		let mut rng = rand::thread_rng();
		let m = rng.gen_range(0, 5);
		if m == 0 {
			self.mine = true;
		}
	}
}

fn get_count(state: &State, i: i32) -> i32 {					//hier wird überprüft, ob sich eine Miene in der Nähe des Feldes befindet
																//Wir müssen immer zuerst gucken, ob das Feld, das wir überprüfen auch das Feld ist, das wir meinen.
	let mut c: i32 = 0;											
	if (i+1) % 5 != 0 && i+1 < 40{								//rechtes Nachbarfeld
		let x = i as usize;
		if state.fields[x+1].mine == true {
				c = c + 1;
		}
	}
	if (i-1) % 5 != 4 && i-1 >= 0 {								//linkes Nachbarfeld
		let x = i-1;
		let dex = x as usize;
		if state.fields[dex].mine == true {
				c = c + 1;
		}
	}
	if (i+5) < 40 {												//Nachbarfeld direkt darunter
		let x = i as usize;
		if state.fields[x+5].mine == true {
				c = c + 1;
		}
	}
	if i-5 >= 0 {												//Nachbarfeld direkt darüber
		let x = i-5;
		let dex = x as usize;
		if state.fields[dex].mine == true {
				c = c + 1;
		}
	}
	if (i+4) % 5 != 4 && i+4 < 40 {								//Nachbarfeld links unten
		let x = i as usize;
		if state.fields[x+4].mine == true {
				c = c + 1;
		}
	}
	if (i-4) % 5 != 0 && i-4 >= 0 {								//Nachbarfeld rechts oben
		let x = i-4;
		let dex = x as usize;
		if state.fields[dex].mine == true {
				c = c + 1;
		}
	}
	if (i+6) % 5 != 0 && i+6 < 40 {								//Nachbarfeld rechts unten
		let x = i as usize;
		if state.fields[x+6].mine == true {
				c = c + 1;
		}
	}
	if (i-6) % 5 != 4 && i-6 >= 0 {								//Nachbarfeld links oben
		let x = i-6;
		let dex = x as usize;
		if state.fields[dex].mine == true {
				c = c + 1;
		}
	}
	return c;													//zum Schluss wird ein Counter c zurückgegeben, welcher die Anzahl der in der Nähe befindlichen Mienen zurück gibt
}

fn set_counts(state: &mut State) {								//Umwandlung in usize (muss wohl gemacht werden in Rust?! XD)
	let mut i = 0;
	while i < 40 {
		let c = get_count(&state, i);
		let x = i as usize;
		let f = &mut state.fields[x];
		f.counts = c;											//count Wert aus get_count wird ins Feld eingefügt
		i = i + 1;
	}
}

impl State {
	fn check_field (&mut self) -> bool{
		let mut win = false;
		for field in &mut self.fields {
			if field.mine && !field.clicked {
				win = true;
			}
			else if !field.mine && field.clicked {
				win = true;
			}
			else {
				win = false;
			}
		}
		return win;
	}
}
				

impl ggez::event::EventHandler for State {												//der EventHandler updated stetig unser Spielfeld und zeichnet die passenden Rechtecke
	fn update (&mut self, ctx: &mut Context) -> GameResult<()> {						//Spielupdate
		if !self.game_over && !self.game_won{
			if self.check_field() {
				self.game_won = true;
			}
		}
		if self.game_won {
			self.game_over = true;
		}
		Ok(())
	}
	
	fn mouse_button_down_event(&mut self, ctx: &mut Context, button: ggez::input::mouse::MouseButton, x: f32, y: f32) {
		for field in &mut self.fields {
			if (field.x + 50.0) > x && x >= field.x && (field.y + 50.0) > y && y >= field.y {
				field.clicked = true;
				if field.mine == true {
					self.game_over = true;
				}
			}
		}
		if self.game_over && !self.game_won {
			println!("Miene Getroffen, Game Over.");
		}
		if self.game_won{
			println!("Gewonnen!");
		}
	}
	
	
	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {								//Rechteck zeichnen
		for field in &self.fields {
			if !self.game_over {															//Wenn noch nicht Gameover, update Feld.
				if field.mine == false && field.counts == 1 {								//helleres blau bei counts=1
					let rectangle = graphics::Mesh::new_rectangle(
						ctx,
						graphics::DrawMode::fill(),
						graphics::Rect::new(field.x, field.y, 50.0, 50.0),
						graphics::Color::new(0.0, 0.0, 1.0, 1.0),
					)?;
					graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
				}
				else if field.mine == false && field.counts == 2 {							//dunkleres grün bei counts=2
					let rectangle = graphics::Mesh::new_rectangle(
						ctx,
						graphics::DrawMode::fill(),
						graphics::Rect::new(field.x, field.y, 50.0, 50.0),
						graphics::Color::new(0.0, 0.5, 0.0, 1.0),
					)?;
					graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
				}
				else if field.mine == false && field.counts == 3 {							//helleres rot bei counts=3
					let rectangle = graphics::Mesh::new_rectangle(
						ctx,
						graphics::DrawMode::fill(),
						graphics::Rect::new(field.x, field.y, 50.0, 50.0),
						graphics::Color::new(1.0, 0.0, 0.0, 1.0),
					)?;
					graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
				}
				else if field.mine == false && field.counts == 4 {							//dunkleres blau bei counts=4
					let rectangle = graphics::Mesh::new_rectangle(
						ctx,
						graphics::DrawMode::fill(),
						graphics::Rect::new(field.x, field.y, 50.0, 50.0),
						graphics::Color::new(0.0, 0.0, 0.52, 1.0),
					)?;
					graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
				}
				else if field.mine == false && field.counts == 5 {							//dunkleres rot bei counts=5
					let rectangle = graphics::Mesh::new_rectangle(
						ctx,
						graphics::DrawMode::fill(),
						graphics::Rect::new(field.x, field.y, 50.0, 50.0),
						graphics::Color::new(0.52, 0.0, 0.0, 1.0),
					)?;
					graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
				}
				else if field.mine == false && field.counts == 6 {							//türkis bei counts=6
					let rectangle = graphics::Mesh::new_rectangle(
						ctx,
						graphics::DrawMode::fill(),
						graphics::Rect::new(field.x, field.y, 50.0, 50.0),
						graphics::Color::new(0.0, 0.5, 0.52, 1.0),
					)?;
					graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
				}
				else if field.mine == false && field.counts == 7 {							//lila bei counts=7
					let rectangle = graphics::Mesh::new_rectangle(
						ctx,
						graphics::DrawMode::fill(),
						graphics::Rect::new(field.x, field.y, 50.0, 50.0),
						graphics::Color::new(0.52, 0.0, 0.52, 1.0),
					)?;
					graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
				}
				else if field.mine == false && field.counts == 8 {							//grau bei counts=8
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
						graphics::WHITE,													//weiß, wenn keine Miene drauf ist
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
			else {																			//Wenn Gameover, zeichne alle Mienen.
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
				}
			}
		}
		graphics::present(ctx);
		Ok(())
	}
}

pub fn main() {
	let mut fields = Vec::new();
	let mut draw_counter = 0;
		while draw_counter < 40 {																					//besitzen 39 Felder
			let x = Field::get_x(draw_counter, 50);
			let y = Field::get_y(draw_counter, 10);
			let mut f = Field::new_field(draw_counter, x, y);
			f.place_mine();
			fields.push(f);
			draw_counter = draw_counter + 1;
		}
	let gameover = false;
	let gamewon = false;
	let state = &mut State{fields: fields, game_over: gameover, game_won: gamewon};
	set_counts(state);
	let (ref mut ctx, ref mut event_loop) = ggez::ContextBuilder::new("hello_ggez", "awesome person")
		.build()
		.unwrap();
	event::run(ctx, event_loop, state).unwrap();
}
