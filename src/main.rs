use std::f32::consts::PI;
use ggez::{conf, event, Context, ContextBuilder, GameError, GameResult, graphics, mint};
use ggez::conf::{FullscreenType, WindowMode};
use ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect};
use ggez::input::keyboard;
use ggez::mint::Point2;
use ggez::winit::event::VirtualKeyCode;

const WIDTH: f32 = 500.0;
const UWIDTH: u32 = WIDTH as u32;
const HEIGHT: f32 = 500.0;
const UHEIGHT: u32 = HEIGHT as u32;
const UP: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const RIGHT: usize = 3;

#[derive(Debug)]
struct State {
    dt: std::time::Duration,
    px:f32,py:f32,
    pa:f32,
    map: [[bool;8];8]
}

impl State {
    fn new(px:f32,py:f32) -> Self {
        let gameMap = [
            [1,1,1,1,1,1,1,1],
            [1,0,1,0,0,0,0,1],
            [1,0,1,0,0,0,0,1],
            [1,0,1,0,0,0,0,1],
            [1,0,0,0,0,0,0,1],
            [1,0,0,0,0,1,0,1],
            [1,0,0,0,0,0,0,1],
            [1,1,1,1,1,1,1,1],
        ].map(|x| x.map(|y| y != 0));
        State{dt: std::time::Duration::new(0,0),px:px,py:py, pa: 0.0, map: gameMap }
    }
}

impl ggez::event::EventHandler<GameError> for State {

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let diff = (self.dt.as_micros() as f32 / 16000.0);
        println!("{:?}",self.pa);
        // self.dt = ggez::ContextBuilder::
        if ctx.keyboard.is_key_pressed(VirtualKeyCode::Up) {
            self.px+=self.pa.cos()*diff;
            self.py+=self.pa.sin()*diff;

        }
        if ctx.keyboard.is_key_pressed(VirtualKeyCode::Down) {
            self.px-=self.pa.cos()*diff;
            self.py-=self.pa.sin()*diff;
        }
        if ctx.keyboard.is_key_pressed(VirtualKeyCode::Left) {
            self.pa-=0.15;
            self.pa=wrapAround(self.pa)
        }
        if ctx.keyboard.is_key_pressed(VirtualKeyCode::Right) {
            self.pa+=0.15;
            self.pa=wrapAround(self.pa)

        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        // let square =
        let mesh: MeshData = MeshBuilder::new()
            .rectangle(
                DrawMode::Fill(Default::default()),
                graphics::Rect::new(0.0,0.0,10.0,10.0),
                Color::WHITE
            )?
            .build(ctx);
        let spacing:f32= 5.0;
        // for y in 0..self.map.len() {
        //     for x in 0..self.map.len() {
                canvas.draw(&mesh, graphics::DrawParam::new().offset(Point2{x:x as f32 *WIDTH,y:y as f32*WIDTH}));
        //     }
        //
        // }

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2{x: self.px, y: self.py},
            5.0,
            0.1,
            graphics::Color::new(100.0,41.2,70.6, 255.0),
        )?;

        let plr_line = graphics::Mesh::new_line(
            ctx,
            &[mint::Point2{x:self.px,y:self.py},mint::Point2{x:self.px+self.pa.cos()*20.0,y:self.py+self.pa.sin()*20.0}],
            2.0,
            graphics::Color::new(255.0,255.0,255.0,255.0)
        )?;

        canvas.draw(&circle, graphics::DrawParam::default());
        canvas.draw(&plr_line, graphics::DrawParam::default());
        canvas.finish(ctx)?;
        Ok(())    }
}

fn main() {
    let state = State::new(
        WIDTH/2.0,WIDTH/2.0
    );

    let c = ggez::conf::Conf::new().window_mode(WindowMode{
        width: WIDTH,
        height: HEIGHT,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 20.0,
        min_height: 20.0,
        max_width: 0.0,
        max_height: 0.0,
        resizable: true,
        visible: true,
        resize_on_scale_factor_change: false
    }).backend(ggez::conf::Backend::OpenGL { major: 3, minor: 3 });

    let (ctx, event_loop) = ggez::ContextBuilder::new("hello_ggez", "Luna")
        .default_conf(c)
        .build()
        .unwrap();
    ggez::graphics::set_window_title(&ctx, "Rays by Luna");
    ggez::graphics::window(&ctx).set_title("Hello?~");
    println!("Hello, world!");
    event::run(ctx, event_loop, state);

}
const PI2: f32 = PI*2.0;
fn wrapAround(angle: f32) -> f32 {
    print!("{},{}",angle>=PI2,angle-PI2);
if angle>=PI2 {
    return angle-PI2;
} else if angle<=0.0 {
    return angle+PI2;
}
    return angle
//     todo!();
}
