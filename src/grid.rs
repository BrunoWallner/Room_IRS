#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

pub struct Grid<const D: usize> {
    blocks: Vec<Option<Block>>,
}

impl<const D: usize> Grid<D> {
    pub fn new() -> Self {
        let size = D * D * D;
        Grid {
            blocks: vec![None; size],
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<&Option<Block>> {
        if x < D && y < D && z < D {
            Some(&self.blocks[x + y * D + z * D * D])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, block: Option<Block>) {
        if x < D && y < D && z < D {
            self.blocks[x + y * D + z * D * D] = block;
        }
    }
}
