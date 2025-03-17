use std::collections::HashSet;
use blake3::Hasher;
use itertools::Itertools;
use std::time::Instant;

type Grid3D = Vec<Vec<Vec<u8>>>;
type PreviousRotationBlocks = Vec<Grid3D>;
type PreviousRotationHashes = HashSet<[u8; 32]>;
type Hash = [u8; 32];
type BlockCombination = Vec<Grid3D>;
type Coordinate = [usize; 3];

#[derive(PartialEq)]
enum Color {
    Red,
    Yellow,
    Green,
    Blue,
}
struct Block {
    block: Grid3D,
    color: Color,
    past_rotations: PreviousRotationBlocks,
    rotation_hashes: PreviousRotationHashes,
    rot_amount: [u8; 3]
}

impl Block {
    // Constructs a new block
    fn new(block: Grid3D, color: Color) -> Block {
        
        let mut block = Block {
            block,
            color,
            past_rotations: Vec::new(),
            rotation_hashes: HashSet::new(),
            rot_amount: [0, 0, 0],
        };

        block.past_rotations.push(block.block.clone());
        block.rotation_hashes.insert(block.hash_current_rotation());

        block
    }

    // Prints the block to std_out with a specified amount of empty lines at the top and bottom
    fn print(&self, spacing: usize) {
        for _ in 0..spacing {
            println!()
        }
        
        for layer in &self.block {
            println!("———————————");
            for row in layer {
                println!("{row:?}");
            }
        }
    
        for _ in 0..spacing {
            println!()
        }
    }

    // Rotates the block on the x axis amount * 90 degrees
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
    
    // Rotates the block on the y axis amount * 90 degrees
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
    
    // Rotates the block on the z axis amount * 90 degrees
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

    fn rotate_with_index(&mut self, i: usize) {
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

    fn hash_current_rotation(&self) -> Hash {
        let mut hasher = Hasher::new();

        for layer in &self.block {
            for row in layer {
                hasher.update(&row);
            }
        }

        hasher.finalize().into()
    }
}

struct Solver {
    field: Grid3D,
    blocks: Vec<Block>,
    rotating_block_num: u8,
    new_rotational_combinations: Vec<BlockCombination>,
}

impl Solver {
    fn new(field: Grid3D, blocks: Vec<Block>) -> Solver {
        // Makes the "new_rotational_combinations" vector a single vector containing the
        // starting, unrotated states of each block
        let new_rotational_combinations: Vec<BlockCombination> = 
            vec![blocks.iter().map(|b| b.block.clone()).collect()];

        Solver {
            field,
            blocks,
            rotating_block_num: 1,
            new_rotational_combinations,
        }
    }
    
    // If a single start point makes the block overlap with the field returns true, otherwise false.
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

    fn solve(&mut self) -> Option<(Vec<Grid3D>, Vec<Coordinate>)> {
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

fn main() {

    let start_time = Instant::now();

    let field: Grid3D = vec![vec![
                vec![1, 0, 0, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 1]],
            vec![
                vec![1, 0, 0, 1], 
                vec![0, 0, 0, 0], 
                vec![0, 0, 0, 1]]];


    let block1 = Block::new(
        vec![vec![
            vec![1, 1, 1], 
            vec![0, 0, 1]]],
        Color::Green,
    );

    let block2 = Block::new(
        vec![vec![
            vec![1, 1, 1], 
            vec![0, 1, 0]],
            vec![
            vec![1, 0, 0], 
            vec![0, 0, 0]]],
        Color::Yellow,
    );

    let block3 = Block::new(
        vec![vec![
            vec![1, 1, 0], 
            vec![0, 1, 1]],
            vec![
            vec![1, 0, 0], 
            vec![0, 0, 0]]],
        Color::Green,
    );

    let block4 = Block::new(
        vec![vec![
            vec![1, 0], 
            vec![1, 1]], 
        vec![
            vec![1, 0], 
            vec![0, 0]]],
        Color::Yellow,
    );

    let mut solver = Solver::new(
        field,
        vec![block1, block2, block3, block4],
    );

    fn print_solution(field: Vec<Vec<Vec<&str>>>) {
        
        for layer in field {

            println!();
            for row in layer {
                println!();
                for point in row {
                    print!("{point}");
                }
            }
        }
    }

    fn produce_solution_field
    (
        field: &mut Vec<Vec<Vec<&str>>>, 
        block: &Grid3D, 
        coord: &Coordinate, 
        color: &Color,
        prev_colors: &mut Vec<&str>
    ) {
        let mut c = "";
        
        match color {
            Color::Blue => { 
                if prev_colors.contains(&"🟦") { c = "🔵" }
                else { c = "🟦" } 
            },
            Color::Red => {
                if prev_colors.contains(&"🟥") { c = "🔴" }
                else { c = "🟥" } 
            },
            Color::Yellow => {
                if prev_colors.contains(&"🟨") { c = "🟡" }
                else { c = "🟨" } 
            },
            Color::Green => {
                if prev_colors.contains(&"🟩") { c = "🟢" }
                else { c = "🟩" }
            }
        }

        let x = coord[0];
        let y = coord[1];
        let z = coord[2];
        
        prev_colors.push(c);

        for (li, layer) in block.iter().enumerate() {
            for (ri, row) in layer.iter().enumerate() {
                for (pi, point) in row.iter().enumerate() {
                    if *point == 1 {
                        field[z+li][y+ri][x+pi] = c;
                    }
                }
            }
        }
    }

    match solver.solve() {
        Some(solution) => {
            let blocks: BlockCombination = solution.0;
            let coords: Vec<Coordinate> = solution.1;

            let mut field: Vec<Vec<Vec<&str>>> = solver.field
                .iter()
                .map(|v| 
                    v.iter()
                     .map(|inner| inner.iter().map(|_| "⬛").collect())
                     .collect()
                )
                .collect();

            let mut prev_colors: Vec<&str> = Vec::new();

            for i in 0..blocks.len() {
                let block_color = &solver.blocks[i].color;
                produce_solution_field(&mut field, &blocks[i], &coords[i], block_color, &mut prev_colors);
            }

            print_solution(field);

            println!();
            println!();
            println!("Took {:?} to compute.", start_time.elapsed())
        }
        None => {
            println!("Yeah this ain't really no surprise but this shit ain't workin'.")
        }
    }
}
