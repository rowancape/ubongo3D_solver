pub mod solver;

use std::time::Instant;

use crate::solver::types::Grid3D;
use crate::solver::types::Color;
use crate::solver::types::Coordinate;
use crate::solver::types::BlockCombination;

use crate::solver::block::Block;

use crate::solver::Solver;

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
            vec![1, 1, 0], 
            vec![0, 1, 1]],
            vec![
                vec![1, 0, 0],
                vec![0, 0, 0],
            ]],
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
            vec![0, 1, 1], 
            vec![1, 1, 0]],
            vec![
            vec![0, 0, 0], 
            vec![1, 0, 0]]],
        Color::Blue,
    );

    let block4 = Block::new(
        vec![vec![
            vec![1, 1], 
            vec![0, 1]]],
        Color::Blue,
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
