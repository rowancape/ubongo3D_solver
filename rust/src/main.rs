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

fn compute_unique_rotations(block: &Grid3D) -> Vec<Grid3D> {
    let mut unique_rotated_blocks: Vec<Grid3D> = Vec::new();

    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                let rotated_block: Grid3D = rotate_x(&rotate_y(&rotate_z(&block, i), j), k);
                if !unique_rotated_blocks.contains(&rotated_block) {
                    unique_rotated_blocks.push(rotated_block);
                }
            }
        }
    }

    unique_rotated_blocks
}

fn main() {
    let field: Grid3D = vec![
        vec![vec![1, 0, 0, 1], vec![0, 0, 0, 0], vec![0, 0, 0, 1]],
        vec![vec![1, 0, 0, 1], vec![0, 0, 0, 0], vec![0, 0, 0, 1]],
    ];

    let block1 = Block {
        block: vec![vec![vec![1, 1, 1], vec![0, 0, 1]]],
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
        let unique_rotations: Vec<Grid3D> = compute_unique_rotations(block);
        unique_rotations_all_blocks.push(unique_rotations);
    }

    for unique_rotations in &unique_rotations_all_blocks {
        for block in unique_rotations {
            println!("New rotation!:");
            print_object(&block);
        }
        println!(
            "Amount of unique rotations for block: {}",
            unique_rotations.len()
        )
    }
}
