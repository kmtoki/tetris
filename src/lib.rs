extern crate rand;
use rand::distributions;
use rand::distributions::IndependentSample;

use std::f32;

pub struct Tetris {
  pub block: Block,
  pub field: [[Color; 10]; 20],
  pub score: i32,
}

pub struct Block {
  pub color: Color,
  pub blocks: Vec<(i32,i32)>,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Color {
  Black, Red, Green, Blue, Yellow, Cyan, Magenta, White
}

#[derive(Debug, PartialEq)]
pub enum Control {
  Down, Left, Right, Rotate
}

const COLORS: &'static [Color] = &[
  Color::Red,
  Color::Green,
  Color::Blue, 
  Color::Yellow, 
  Color::Cyan, 
  Color::Magenta,
];

const BLOCKS: &'static [&'static [(i32,i32)]] = &[ 
  &[(0,0),(0,1),(1,0),(1,1)],
  &[(0,0),(0,1),(0,2),(1,1),(2,1)],
  &[(0,0),(0,1),(0,2),(0,3)],
  &[(0,0),(0,1),(0,2),(0,3),(1,3)],
  &[(0,0),(0,1),(0,2),(0,3),(1,0)],
  &[(0,0),(0,1),(1,1),(1,2)],
  &[(1,0),(1,1),(0,1),(0,2)],
];

impl Block {
  pub fn new(c: Color, b: Vec<(i32,i32)>) -> Block {
    return Block {
      color: c,
      blocks: b
    };
  }

  pub fn rand() -> Block {
    let mut rng = rand::thread_rng();
    let blocks_range = distributions::Range::new(0, BLOCKS.len());
    let colors_range = distributions::Range::new(0, COLORS.len());
    return Block {
      color: COLORS[colors_range.ind_sample(&mut rng)],
      blocks: BLOCKS[blocks_range.ind_sample(&mut rng)].to_vec()
    };
  }

  fn down(&mut self) {
    for c in self.blocks.iter_mut() {
      c.0 += 1;
    }
  }

  fn left(&mut self) {
    for c in self.blocks.iter_mut() {
      c.1 -= 1;
    }
  }

  fn right(&mut self) {
    for c in self.blocks.iter_mut() {
      c.1 += 1;
    }
  }

  fn rotate(&mut self) {
    let r: f32 = f32::consts::PI / 2.0;
    let cy: f32 = 
      (self.blocks.iter().map(|i| i.0).sum::<i32>() as f32) / (self.blocks.len() as f32);
    let cx: f32 = 
      (self.blocks.iter().map(|i| i.1).sum::<i32>() as f32) / (self.blocks.len() as f32);

    for c in self.blocks.iter_mut() {
      let (y, x) = *c;
      let y = f32::from(y as i16);
      let x = f32::from(x as i16);
      *c = (
        (cy + (x - cx) * r.sin() + (y - cy) * r.cos()).round() as i32,
        (cx + (x - cx) * r.cos() - (y - cy) * r.sin()).round() as i32
      );
    }
  }
}

impl Tetris {
  pub fn new() -> Tetris {
    return Tetris {
      score: 0,
      block: Block::rand(),
      field: [[Color::Black; 10]; 20],
    };
  }

  pub fn control(&mut self, op: Control) {
    let pre = self.block.blocks.clone();
    match op {
      Control::Down => self.block.down(),
      Control::Left => self.block.left(),
      Control::Right => self.block.right(),
      Control::Rotate => self.block.rotate()
    }
    
    let ly = self.field.len() as i32;
    let lx = self.field[0].len() as i32;
    let exists = self.block.blocks.iter().all(|&(y,x)| {
      return 0 <= y && y < ly && 0 <= x && x < lx 
        && (self.field[y as usize][x as usize] == Color::Black);
    });

    if !exists {
      self.block.blocks = pre;
    }
  }

  pub fn delete(&mut self) {
    for y in 0 .. self.field.len() {
      if self.field[y].iter().all(|c| *c != Color::Black) {
        for x in 0 .. self.field[y].len() {
          let mut yy = y;
          for yyy in (0 .. y - 1).rev() {
            self.field[yy][x] = self.field[yyy][x];
            yy -= 1;
          }
        }
      }
    }
  }

  pub fn fall(&mut self) {
    let blocks = self.block.blocks.clone();
    self.control(Control::Down);

    let mut not_moved = true;
    let len = self.block.blocks.len();
    for i in 0 .. len {
      if self.block.blocks[i] != blocks[i] {
        not_moved = false; 
        break;
      }
    }

    if not_moved {
      {
        let ref bs = self.block.blocks;
        for &(y,x) in bs {
          self.field[y as usize][x as usize] = self.block.color;
        }
      }
      self.block = Block::rand();
      self.delete();
    }
  }
}


