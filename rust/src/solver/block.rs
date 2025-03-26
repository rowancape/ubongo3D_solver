use super::types::Grid3D;
use super::types::Color;
use super::types::BlockVec;

pub struct Block {
    pub block: Grid3D,
    pub color: Color,
    pub past_rotations: BlockVec,
    pub rot_amount: [u8; 3]
}


impl Block {
    pub fn new(block: Grid3D, color: Color) -> Block {
        
        let mut block = Block {
            block,
            color,
            past_rotations: Vec::new(),
            rot_amount: [0, 0, 0],
        };

        block.past_rotations.push(block.block.clone());

        block
    }

    fn rotate_x(&mut self, amount: usize) {
        let mut new_block: Grid3D = Vec::new();
    
        let layers = self.block.len();
        let rows = self.block[0].len();

        if amount == 1 {
            for i in (0..rows).rev() {
                let mut new_layer: Vec<Vec<u8>> = Vec::new();
                for j in 0..layers {
                    new_layer.push(self.block[j][i].clone());
                }
                new_block.push(new_layer);
            }
        }

        if amount == 2 {
            for i in (0..layers).rev() {
                let mut new_layer: Vec<Vec<u8>> = Vec::new();
                for j in (0..rows).rev() {
                    new_layer.push(self.block[i][j].clone());
                }
                new_block.push(new_layer);
            }
        }

        if amount == 3 {
            for i in 0..rows {
                let mut new_layer: Vec<Vec<u8>> = Vec::new();
                for j in (0..layers).rev() {
                    new_layer.push(self.block[j][i].clone());
                }
                new_block.push(new_layer);
            }
        }

        self.block = new_block;
    }

    fn rotate_y(&mut self, amount: usize) {
        let mut new_block: Grid3D = Vec::new();
    
        let layers = self.block.len();
        let rows = self.block[0].len();
        let points = self.block[0][0].len();

        if amount == 1 {
            for i in (0..points).rev() {
                let mut new_layer: Vec<Vec<u8>> = Vec::new();
                for j in 0..rows {
                    let mut new_row: Vec<u8> = Vec::new();
                    for k in 0..layers {
                        new_row.push(self.block[k][j][i].clone());
                    }
                    new_layer.push(new_row);
                }
                new_block.push(new_layer);
            }
        }

        if amount == 2 {
            for i in (0..layers).rev() {
                let mut new_layer: Vec<Vec<u8>> = Vec::new();
                for j in 0..rows {
                    new_layer.push(self.block[i][j].clone().into_iter().rev().collect());
                }
                new_block.push(new_layer);
            }
        }

        if amount == 3 {
            for i in 0..points {
                let mut new_layer: Vec<Vec<u8>> = Vec::new();
                for j in 0..rows {
                    let mut new_row: Vec<u8> = Vec::new();
                    for k in (0..layers).rev() {
                        new_row.push(self.block[k][j][i].clone());
                    }
                    new_layer.push(new_row);
                }
                new_block.push(new_layer);
            }
        }

        self.block = new_block;
    }

    fn rotate_z(&mut self, amount: usize) {
        let mut new_block: Grid3D = Vec::new();
        
        let layers = self.block.len();
        let rows = self.block[0].len();
        let points = self.block[0][0].len();

        if amount == 1 {
            for i in 0..layers {
                let mut new_layer: Vec<Vec<u8>> = Vec::new();
                for j in 0..points {
                    let mut new_row: Vec<u8> = Vec::new();
                    for k in (0..rows).rev() {
                        new_row.push(self.block[i][k][j].clone());
                    }
                    new_layer.push(new_row);
                }
                new_block.push(new_layer);
            }
        }

        if amount == 2 {
            for i in 0..layers {
                let mut new_layer: Vec<Vec<u8>> = Vec::new();
                for j in (0..rows).rev() {
                    new_layer.push(self.block[i][j].clone().into_iter().rev().collect());
                }
                new_block.push(new_layer);
            }
        }

        if amount == 3 {
            for i in 0..layers {
                let mut new_layer: Vec<Vec<u8>> = Vec::new();
                for j in (0..points).rev() {
                    let mut new_row: Vec<u8> = Vec::new();
                    for k in 0..rows {
                        new_row.push(self.block[i][k][j].clone());
                    }
                    new_layer.push(new_row);
                }
                new_block.push(new_layer);
            }
        }

        self.block = new_block;
    }

    pub fn rotate_with_index(&mut self, i: usize) {
        if i == 0 {
            self.rotate_x(1);
        }
        else if i == 1 {
            self.rotate_y(1);
        }
        else if i == 2 {
            self.rotate_z(1);
        }
        else {
            panic!("Incorrect index entered into rotate_with_index!")
        }
    }
}