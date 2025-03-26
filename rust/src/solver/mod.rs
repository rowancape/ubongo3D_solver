use itertools::Itertools;

pub mod block;
pub mod types;

use crate::solver::block::Block;
use crate::solver::types::Grid3D;
use crate::solver::types::Coordinate;
use crate::solver::types::BlockCombination;

pub struct Solver {
    pub field: Grid3D,
    pub blocks: Vec<Block>,
    new_rotational_combinations: Vec<BlockCombination>,
}


impl Solver {
    pub fn new(field: Grid3D, blocks: Vec<Block>) -> Solver {
        // Makes the "new_rotational_combinations" vector a single vector containing the
        // starting, unrotated states of each block
        let new_rotational_combinations: Vec<BlockCombination> = 
            vec![blocks.iter().map(|b| b.block.clone()).collect()];

        Solver {
            field,
            blocks,
            new_rotational_combinations,
        }
    }

    fn does_candidate_overlap_field(&self, candidate: Coordinate, block: &Grid3D) -> bool {
        let x = candidate[0];
        let y = candidate[1];
        let z = candidate[2];

        for (li, layer) in block.iter().enumerate() {
            for (ri, row) in layer.iter().enumerate() {
                for (pi, point) in row.iter().enumerate() {
                    if self.field[z+li][y+ri][x+pi] + point > 1 {
                        return true;
                    }
                }
            }
        }

        false
    }

    // Validates the intial start coord candidates by checking if they cause the block to overlap with field.
    fn validate_candidate_coords(&self, candidates: Vec<Coordinate>, block: &Grid3D) -> Vec<Coordinate> {
        let mut valid_coords: Vec<Coordinate> = Vec::new();

        for candidate in candidates {
            if !self.does_candidate_overlap_field(candidate, block) {
                valid_coords.push(candidate);
            }
        }

        valid_coords
    }

    // Finds the initial start coord candidates for a single block
    fn find_start_coord_candidates(&self, block: &Grid3D) -> Vec<Coordinate> {
        let mut single_block_coords: Vec<Coordinate> = Vec::new();

        let field_layers = self.field.len();
        let field_rows = self.field[0].len();
        let field_points = self.field[0][0].len();

        let block_layers = block.len();
        let block_rows = block[0].len();
        let block_points = block[0][0].len();
            
        for z in 0..field_layers {
            // If block can't fit in the z axis, break.
            if field_layers - z < block_layers {
                break;
            }

            for y in 0..field_rows {
                // If block can't fit in the y axis, break.
                if field_rows - y < block_rows {
                    break;
                }

                for x in 0..field_points {
                    // If block can't fit in the x axis, break.
                    if field_points - x < block_points {
                        break;
                    }
                    // If block fits at start coordinate [x, y, z] in all axis then save that coord.
                    single_block_coords.push([x, y, z]);
                }
            }
        }

        single_block_coords
    }

    // Gets all the valid start coords for each object and returns them in a Vec<Vec<Coordinate>>>
    // Return value is a vector containing "block number" of vectors, each with some amount of start coords.
    // start_coords[i] are the start coords corresponding to the passed in combo[i] block
    fn find_valid_start_coords(&self, combo: &BlockCombination) -> Vec<Vec<Coordinate>> {
        // Possible start coordinates for each block in a combination of blocks.
        // If there are four blocks in a combination this would be vector of four vectors,
        // each containing some number of possible start coordinates for the corresponding block.
        let mut start_coords_each_block: Vec<Vec<Coordinate>> = Vec::new();

        for block in combo {
            let candidates = self.find_start_coord_candidates(&block);
            let valid_start_coords = self.validate_candidate_coords(candidates, &block);

            start_coords_each_block.push(valid_start_coords);
        }

        start_coords_each_block
    }

    fn is_solve_success(&self, field: &mut Grid3D, blocks: &Vec<Grid3D>, sps: &Vec<[usize; 3]>) -> bool {
        for (spi, sp) in sps.iter().enumerate() {
            let x = sp[0];
            let y = sp[1];
            let z = sp[2];
    
            for (li, layer) in blocks[spi].iter().enumerate() {
                for (ri, row) in layer.iter().enumerate() {
                    for (pi, point) in row.iter().enumerate() {
                        if field[li+z][ri+y][pi+x] + point > 1 {
                            return false;
                        } 
                        else {
                            field[li+z][ri+y][pi+x] += point;
                        }
                    }
                }
            }
        }
        
        true
    }

    fn which_block_to_rotate_next(&self) -> Option<usize> {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.rot_amount == [3, 3, 3] {
                continue;
            }
            else {
                return Some(i);
            }
        } 

        None
    }

    fn rotate_block(&mut self, block_index: &usize) {
        let block = &mut self.blocks[*block_index];

        for i in 0..3 {
            if block.rot_amount[i] == 3 {
                block.rotate_with_index(i);
                block.rot_amount[i] = 0;
                continue;
            }
            else {
                block.rotate_with_index(i);
                block.rot_amount[i] += 1;
                break;
            }
        }
    }

    fn is_current_rotation_unique(&self, block_index: &usize) -> bool {
        !self.blocks[*block_index].past_rotations.contains(&self.blocks[*block_index].block)
    }

    fn get_combined_blocks(&self, b: usize) -> Vec<BlockCombination> {
        let mut result: Vec<BlockCombination> = Vec::new();

        for (i, block) in self.blocks.iter().enumerate() {
            if i == b {
                result.push(vec![block.block.clone()]);
            } 
            else {
                result.push(block.past_rotations.clone());
            }
        }

        result
    }

    fn compute_new_rotation(&mut self) {
        if let Some(b) = self.which_block_to_rotate_next() {
            
            loop {
                self.rotate_block(&b);
                if self.blocks[b].rot_amount == [3, 3, 3] {
                    break;
                }
                else if self.is_current_rotation_unique(&b) {
                    let unique_block = self.blocks[b].block.clone();
                    self.blocks[b].past_rotations.push(unique_block);
                    break;
                }
            }

            let temp = self.get_combined_blocks(b);
            let new_rotational_combinations: Vec<BlockCombination> = temp.into_iter().multi_cartesian_product().collect();
            self.new_rotational_combinations = new_rotational_combinations;
        }
    }

    pub fn solve(&mut self) -> Option<(Vec<Grid3D>, Vec<Coordinate>)> {
        while self.blocks.last().unwrap().rot_amount != [3, 3, 3]  {
            for block_combo in &self.new_rotational_combinations {
    
                let start_coords_each_block = self.find_valid_start_coords(block_combo);
    
                let start_combos_iterator = start_coords_each_block.into_iter().multi_cartesian_product();
    
                for start_combo in start_combos_iterator {
    
                    let mut active_field = self.field.clone();
    
                    if self.is_solve_success(&mut active_field, block_combo, &start_combo) {
                        return Some((block_combo.clone(), start_combo));
                    }
                }
            }

            self.compute_new_rotation();
        }

        None
    }
}