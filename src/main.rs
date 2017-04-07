#[macro_use]
extern crate glium;
use glium::{DisplayBuild, Surface, Program};
use glium::{glutin, index, vertex};

extern crate tetris;
use tetris::{Tetris, Color};

use std::f32;
use std::thread;
use std::time;
use std::sync::{Arc, Mutex};


#[derive(Copy, Clone)]
struct Vertex {
  pos: [f32; 2],
  color: [f32; 4],
}
implement_vertex!(Vertex, pos, color);


const VERTEX_SHADER: &'static str = r#"
#version 400

in vec2 pos;
in vec4 color;
out vec4 v_color;

void main() {
  v_color = color;
  gl_Position = vec4(pos, 0, 1);
}
"#;

const FRAGMENT_SHADER: &'static str = r#"
#version 400

in vec4 v_color;
out vec4 f_color;

void main() {
  f_color = v_color;
}

"#;


pub fn color_to_rgba(c: Color) -> [f32; 4] {
  match c {
    Color::Black   => [0.0, 0.0, 0.0, 0.0],
    Color::Red     => [0.5, 0.0, 0.0, 0.5],
    Color::Green   => [0.0, 0.5, 0.0, 0.5],
    Color::Blue    => [0.0, 0.0, 0.5, 0.5],
    Color::Yellow  => [0.5, 0.5, 0.0, 0.5],
    Color::Cyan    => [0.0, 0.5, 0.5, 0.5],
    Color::Magenta => [0.5, 0.0, 0.5, 0.5],
    Color::White   => [0.5, 0.5, 0.5, 0.5]
  }
}

fn tetris_to_vertexs(tetris: &Tetris) -> Vec<Vertex> {
  let mut vs = vec!();
  let mut y: f32 = 0.80;
  let mut iy: usize = 0;

  while y >= -0.885 {
    let mut x: f32 = -0.375;
    let mut ix: usize = 0;

    while x <= 0.46  {
      if tetris.block.blocks.iter().any(|&(yy,xx)| iy as i32 == yy && ix as i32 == xx) {
        vs.push(Vertex { 
          pos: [x, y], 
          color: color_to_rgba(tetris.block.color)
        });
      }
      else {
        vs.push(Vertex { 
          pos: [x, y], 
          color: color_to_rgba(tetris.field[iy][ix])
        });
      }

      x += 0.085;
      ix += 1;
    }

    y -= 0.085;
    iy += 1;
  }
  return vs;
}

fn main() {
  let display = glutin::WindowBuilder::new()
    .with_dimensions(600, 600).build_glium().unwrap();
  println!("{:?}", display. get_framebuffer_dimensions());

  let index = index::NoIndices(index::PrimitiveType::Points);
  let uniform = uniform!();
  let param = glium::DrawParameters { 
    point_size: Some(26.0),
    .. Default::default()
  };

  let program = match Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None) {
    Ok(p) => p,
    Err(e) => {
      println!("{}", e);
      return;
    }
  };

  let mutex_main = Arc::new(Mutex::new(Tetris::new()));
  let mutex_timer = mutex_main.clone();

  thread::spawn(move || {
    loop {
      thread::sleep(time::Duration::from_millis(1000));
      let mut t = mutex_timer.lock().unwrap();
      t.fall();
    }
  });

  loop {
    let mut t = mutex_main.lock().unwrap();

    'event: for e in display.poll_events() {
      match e {
        glutin::Event::KeyboardInput(
          glutin::ElementState::Pressed, _, Some(keycode)
        ) => {
          //println!("{:?}", e);
          match keycode {
            glutin::VirtualKeyCode::Up => {
              t.control(tetris::Control::Rotate);
            },

            glutin::VirtualKeyCode::Down => {
              t.fall();
            },

            glutin::VirtualKeyCode::Left => {
              t.control(tetris::Control::Left);
            },

            glutin::VirtualKeyCode::Right => {
              t.control(tetris::Control::Right);
            },

            glutin::VirtualKeyCode::Q => {
              return
            },

            _ => {}
          }

          break 'event;
        },

        _ => break 'event
      }
    }

    let mut frame = display.draw();

    let vertex_buffer = vertex::VertexBuffer::new(
       &display, &tetris_to_vertexs(&t) 
     ).unwrap();
   
    frame.clear_color(0.0, 0.0, 0.0 ,0.0);
    frame.draw(&vertex_buffer, &index, &program, &uniform, &param).unwrap();
    frame.finish().unwrap();
  } 
}
