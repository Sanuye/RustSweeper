use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::{Context, GameResult};

//In meshes sind alle gegebenen Mesh-Konstruktionen, die wir unten einfügen

struct State{
    meshes: Vec<graphics::Mesh>,
}

//Für jeden gegebenen Mesh soll ein MeshBuilder erstellt werden
//Wissen noch nicht wie sinnvoll dies letzendlich ist

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        let meshes = vec![build_mesh(ctx)];
    }
}

//Hier wird der MeshBuilder für unsere Meshes implementiert
//Mesh wird somit am Nede erstellt

fn build_mesh(ctx: &mut Context) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    mb.build(ctx)
}

//Sollte unsere MeshBuilder-Aktion erfolgreich ablaufen, so wird ein OK im Status angegeben

impl event::EventHandler for State {
    fn update (&mut self, ctx: &mut Context) -> GameResult {
        OK(())
    }
	
	//unsere draw_Funktion soll ein schwarzes Rechteck mit gegebenen Maßen erstellen
	
    fn draw(&mut self, ctx: &mut Context) -> GameResult {														
        let rect = graphics::Rect::new(100.0, 100.0, 100.0, 50.0);
        let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(1.0), rect, graphics::BLACK);
        graphics::draw(ctx, &r1, DrawParam::default());
    }
}

//Main-Funktion

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("drawing", "ggez");
    let (ctx, events_loop) = &mut cb.build();
    let state = &mut State::new(ctx).unwrap();
    event::run(ctx, events_loop, state)
}

