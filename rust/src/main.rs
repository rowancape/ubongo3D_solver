type Grid3D = Vec<Vec<Vec<u8>>>;
type AllRotations = Vec<Grid3D>;
type RotationCombination = Vec<Grid3D>;
type StartPoint = [usize; 3];
type StartPoints = Vec<StartPoint>;
enum Color {
    Red,
    Yellow,
    Green,
    Blue,
}
struct Block {
    block: Grid3D,
    color: Color,
    unique_rotations: AllRotations,
}

impl Block {
    fn new(block: Grid3D, color: Color) -> Block {
        Block {
            block,
            color,
            unique_rotations: Vec::new()
        }
    }

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
                        new_row.push(self.block[i][j][k].clone());
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
    
    fn compute_unique_rotations(&mut self, field: &Grid3D) {
        let mut unique_rotated_blocks: AllRotations = Vec::new();
    
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    self.rotate_x(k);
                    self.rotate_y(j);
                    self.rotate_z(i);
                    if unique_rotated_blocks.contains(&self.block) || self.block.len() > field.len() {
                        continue;
                    }
                    unique_rotated_blocks.push(self.block.clone());
                }
            }
        }
    
        if unique_rotated_blocks.len() == 0 {
            panic!("At least one of the blocks could not be placed in the field in any rotational configuration!")
        }
    }
}

struct Solver {
    field: Grid3D,
    blocks: Vec<Block>,
    rotational_combinations: Vec<RotationCombination>
}

impl Solver {
    fn compute_rotational_combinations(&mut self, lists: Vec<AllRotations>, prefix: RotationCombination) {
        // Check if lists is empty
        if lists.is_empty() {
            // If it is then push the last state of prefix to result and return 
            self.rotational_combinations.push(prefix);
            return;
        }

        // If not then pop the first list out and loop over the blocks in it
        let first = &lists[0];
        let rest = &lists[1..];
    
        for block in first {
            // In the loop create a new prefix variable and push the current block to it
            let mut new_prefix = prefix.clone();
            new_prefix.push(block.clone());
    
            // Then recursively call itself with the new prefix and all but the first element in lists.
            self.compute_rotational_combinations(rest.to_vec(), new_prefix);
        }
    }

    fn compute_valid_start_coords(&self, block: &Grid3D) -> StartPoints {
        let mut initial_start_points: StartPoints = Vec::new();
        let field = &self.field;
    
        for layer in 0..field.len()-block.len()+1 {
            for row in 0..field[0].len()-block[0].len()+1 {
                for point in 0..field[0][0].len()-block[0][0].len()+1 {
                    initial_start_points.push([layer, row, point]);
                }
            }
        }
    
        let mut start_points: StartPoints = Vec::new();
        
        for sp in initial_start_points {
            let mut is_valid = true;
    
            for (li, layer) in block.iter().enumerate() {
                for (ri, row) in layer.iter().enumerate() {
                    for (pi, point) in row.iter().enumerate() {
                        if field[sp[0]+li][sp[1]+ri][sp[2]+pi] + point == 2 {
                            is_valid = false;
                        }
                    }
                }
            }
    
            if is_valid {
                start_points.push(sp);
            }
        }
    
        start_points
    }
    
    fn compute_start_point_combinations(&self, lists: Vec<StartPoints>, prefix: StartPoints, result: &mut Vec<Vec<[usize; 3]>>) {
        // Check if lists is empty
        if lists.is_empty() {
            // If it is then push the last state of prefix to result and return 
            result.push(prefix);
            return;
        }
    
        // If not then pop the first list out and loop over the blocks in it
        let first = &lists[0];
        let rest = &lists[1..];
    
        for block in first {
            // In the loop create a new prefix variable and push the current block to it
            let mut new_prefix = prefix.clone();
            new_prefix.push(block.clone());
    
            // Then recursively call itself with the new prefix and all but the first element in lists.
            self.compute_start_point_combinations(rest.to_vec(), new_prefix, result);
        }
    }
}




fn main() {
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
        vec![
            vec![vec![1, 1, 1], vec![0, 0, 1]],
            vec![vec![0, 0, 0], vec![0, 0, 1]],
        ],
        Color::Blue,
    );

    let block3 = Block::new(
        vec![vec![vec![1, 1], vec![1, 1]], vec![vec![1, 0], vec![0, 0]]],
        Color::Red,
    );

    let block4 = Block::new(
        vec![vec![vec![1, 1], vec![1, 0]], vec![vec![0, 0], vec![1, 0]]],
        Color::Red,
    );

    let solver = Solver {
        field,
        blocks: vec![block1, block2, block3, block4],
        rotational_combinations: Vec::new(),
    };


    for block in solver.blocks {
        block.compute_unique_rotations(&solver.field);
    }

    let unique_rotations_all_blocks: Vec<AllRotations> = 
        solver.blocks.iter().map(|block| block.unique_rotations).collect();

    solver.compute_rotational_combinations(unique_rotations_all_blocks, Vec::new());

    for combination in solver.rotational_combinations {

        let mut is_valid = true;

        let mut all_start_points: Vec<StartPoints> = Vec::new();

        for block in &combination {
            let start_points = solver.compute_valid_start_coords(&block);
            all_start_points.push(start_points);
        }

        let mut start_point_combinations: Vec<Vec<[usize; 3]>> = Vec::new();
        solver.compute_start_point_combinations(all_start_points, Vec::new(), &mut start_point_combinations);

        for spc in start_point_combinations {
            let mut step_field = vec![field.clone()];
            let mut active_field = field.clone();
            for (bi, block) in combination.iter().enumerate() {
                for (li, layer) in block.iter().enumerate() {
                    for (ri, row) in layer.iter().enumerate() {
                        for (pi, point) in row.iter().enumerate() {
                            if active_field[spc[bi][0]+li][spc[bi][1]+ri][spc[bi][2]+pi] + point == 2 {
                                is_valid = false;
                                break;
                            }
                            active_field[spc[bi][0]+li][spc[bi][1]+ri][spc[bi][2]+pi] += point;
                        }
                        if !is_valid { break; }
                    }
                    if !is_valid { break; }
                }
                if !is_valid { break; }
                let previous_field = step_field.last().unwrap();
                let mut modified_field = active_field.clone();
                for i in 0..modified_field.len() {
                    for j in 0..modified_field[0].len() {
                        for k in 0..modified_field[0][0].len() {
                            // If a cell is newly activated (was 0 before and now is 1)
                            if previous_field[i][j][k] == 0 && modified_field[i][j][k] == 1 {
                                modified_field[i][j][k] = 1 + bi as u8;
                            }
                        }
                    }
                }
                step_field.push(modified_field);
            }
            if is_valid {
                println!("FOUND SOLUTION!");
                print_object(&step_field[0]);
                println!();
                println!("PLACED OBJECT1 AT {:?}", spc[0]);
                print_object(&combination[0]);
                println!();
                print_object(&step_field[1]);
                println!();
                println!("PLACED OBJECT2 AT {:?}", spc[1]);
                print_object(&combination[1]);
                println!();
                print_object(&step_field[2]);
                println!();
                println!("PLACED OBJECT3 AT {:?}", spc[2]);
                print_object(&combination[2]);
                println!();
                print_object(&step_field[3]);
                println!();
                println!("PLACED OBJECT4 AT {:?}", spc[3]);
                print_object(&combination[3]);
                break;
            }
        }
        if is_valid {
            break;
        }
    }
}
