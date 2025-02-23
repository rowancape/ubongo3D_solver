type Grid3D = Vec<Vec<Vec<u8>>>;
enum Color {
    Red,
    Yellow,
    Green,
    Blue,
}
struct Block {
    block: Grid3D,
    color: Color,
}

fn print_object(object: &Grid3D) {
    for (i, layer) in object.iter().enumerate() {
        if i != 0 {
            println!("—————————");
        }
        for row in layer {
            println!("{:?}", row);
        }
    }
}

fn rotate_x(object: &Grid3D, rotations: usize) -> Grid3D {
    let mut result = object.clone();
    let r = rotations % 4;
    for _ in 0..r {
        let layers = result.len();
        let rows = result[0].len();
        let points = result[0][0].len();
        let new_obj: Grid3D = (0..rows)
            .map(|i| {
                (0..layers)
                    .map(|j| {
                        (0..points)
                            .map(|k| result[j][rows - 1 - i][k])
                            .collect::<Vec<u8>>()
                    })
                    .collect::<Vec<Vec<u8>>>()
            })
            .collect();
        result = new_obj;
    }
    result
}

fn rotate_y(object: &Grid3D, rotations: usize) -> Grid3D {
    let mut result = object.clone();
    let r = rotations % 4;
    for _ in 0..r {
        let layers = result.len();
        let rows = result[0].len();
        let points = result[0][0].len();
        let new_obj: Grid3D = (0..points)
            .map(|i| {
                (0..rows)
                    .map(|j| {
                        (0..layers)
                            .map(|k| result[k][j][points - 1 - i])
                            .collect::<Vec<u8>>()
                    })
                    .collect::<Vec<Vec<u8>>>()
            })
            .collect();
        result = new_obj;
    }
    result
}

fn rotate_z(object: &Grid3D, rotations: usize) -> Grid3D {
    let mut result = object.clone();
    let r = rotations % 4;
    for _ in 0..r {
        let layers = result.len();
        let rows = result[0].len();
        let points = result[0][0].len();
        let new_obj: Grid3D = (0..layers)
            .map(|i| {
                (0..points)
                    .map(|j| {
                        (0..rows)
                            .map(|k| result[i][rows - 1 - k][j])
                            .collect::<Vec<u8>>()
                    })
                    .collect::<Vec<Vec<u8>>>()
            })
            .collect();
        result = new_obj;
    }
    result
}

fn compute_unique_rotations(block: &Grid3D, field: &Grid3D) -> Vec<Grid3D> {
    let mut unique_rotated_blocks: Vec<Grid3D> = Vec::new();

    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                let rotated_block: Grid3D = rotate_x(&rotate_y(&rotate_z(&block, i), j), k);
                if unique_rotated_blocks.contains(&rotated_block) || rotated_block.len() > field.len() {
                    continue;
                }
                unique_rotated_blocks.push(rotated_block);
            }
        }
    }

    if unique_rotated_blocks.len() == 0 {
        panic!("At least one of the blocks could not be placed in the field in any rotational configuration!")
    }

    unique_rotated_blocks
}

fn compute_rotational_combinations(lists: Vec<Vec<Grid3D>>, prefix: Vec<Grid3D>, result: &mut Vec<Vec<Grid3D>>) {
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
        compute_rotational_combinations(rest.to_vec(), new_prefix, result);
    }
}

fn compute_valid_start_coords(block: &Grid3D, field: &Grid3D) -> Vec<[usize; 3]> {
    let mut initial_start_points: Vec<[usize; 3]> = Vec::new();

    for layer in 0..field.len()-block.len()+1 {
        for row in 0..field[0].len()-block[0].len()+1 {
            for point in 0..field[0][0].len()-block[0][0].len()+1 {
                initial_start_points.push([layer, row, point]);
            }
        }
    }

    let mut start_points: Vec<[usize; 3]> = Vec::new();
    
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

fn compute_start_point_combinations(lists: Vec<Vec<[usize; 3]>>, prefix: Vec<[usize; 3]>, result: &mut Vec<Vec<[usize; 3]>>) {
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
        compute_start_point_combinations(rest.to_vec(), new_prefix, result);
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
            vec![0, 0, 0, 1]],
    ];

    let block1 = Block {
        block: vec![vec![
            vec![1, 1, 1], 
            vec![0, 0, 1]]],
        color: Color::Green,
    };

    let block2 = Block {
        block: vec![
            vec![vec![1, 1, 1], vec![0, 0, 1]],
            vec![vec![0, 0, 0], vec![0, 0, 1]],
        ],
        color: Color::Blue,
    };

    let block3 = Block {
        block: vec![vec![vec![1, 1], vec![1, 1]], vec![vec![1, 0], vec![0, 0]]],
        color: Color::Red,
    };

    let block4 = Block {
        block: vec![vec![vec![1, 1], vec![1, 0]], vec![vec![0, 0], vec![1, 0]]],
        color: Color::Red,
    };

    let blocks = [&block1.block, &block2.block, &block3.block, &block4.block];

    let mut unique_rotations_all_blocks: Vec<Vec<Grid3D>> = Vec::new();

    for block in blocks {
        let unique_rotations: Vec<Grid3D> = compute_unique_rotations(block, &field);
        unique_rotations_all_blocks.push(unique_rotations);
    }

    let mut rotational_combinations: Vec<Vec<Grid3D>> = Vec::new();
    compute_rotational_combinations(unique_rotations_all_blocks, Vec::new(), &mut rotational_combinations);

    for combination in rotational_combinations {

        let mut all_start_points: Vec<Vec<[usize; 3]>> = Vec::new();

        for block in &combination {
            let start_points = compute_valid_start_coords(&block, &field);
            all_start_points.push(start_points);
        }

        let mut start_point_combinations: Vec<Vec<[usize; 3]>> = Vec::new();
        compute_start_point_combinations(all_start_points, Vec::new(), &mut start_point_combinations);

        for spc in start_point_combinations {
            let mut step_field = vec![field.clone()];
            let mut active_field = field.clone();
            let mut is_valid = true;
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
            }
        }
    }
}
