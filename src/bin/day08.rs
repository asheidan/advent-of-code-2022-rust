use std::collections::HashSet;
use std::io::BufRead;

fn visible_trees<'a, I>(trees: I) -> Vec<usize>
where
    I: Iterator<Item = (usize, &'a i8)>,
{
    let mut visible: Vec<usize> = Vec::new();

    trees.fold(-1, |highest, (x, tree)| {
        if highest < *tree {
            visible.push(x);

            *tree
        } else {
            highest
        }
    });

    visible
}

/// Visible trees from the outside from all directions
fn solve_a(data: &[Vec<i8>]) -> i32 {
    let mut visible_coordinates: HashSet<(usize, usize)> = HashSet::new();

    data.iter().enumerate().for_each(|(y, row)| {
        let right = visible_trees(row.iter().enumerate());
        let left = visible_trees(row.iter().enumerate().rev());

        for &x in right.iter().chain(left.iter()) {
            visible_coordinates.insert((x, y));
        }
    });

    for x in 0..(data[0].len()) {
        let top = visible_trees(data.iter().map(|row| &row[x]).enumerate());
        let bottom = visible_trees(data.iter().map(|row| &row[x]).enumerate().rev());

        for &y in top.iter().chain(bottom.iter()) {
            visible_coordinates.insert((x, y));
        }
    }

    visible_coordinates.len() as i32
}

/// Highest "scenic score"
///
/// To measure the viewing distance from a given tree, look up, down, left, and right from that
/// tree; stop if you reach an edge or at the first tree that is the same height or taller than the
/// tree under consideration. (If a tree is right on the edge, at least one of its viewing
/// distances will be zero.)
///
/// A tree's scenic score is found by multiplying together its viewing distance in each of the four
/// directions.
///
/// -----------------------------------------------------------------------------------------------
///
/// Since the score is created by multiplying each direction and the viewing distance on the edges
/// are 0 towards the edge we can skip the outermost trees since their score will all be 0.
fn solve_b(data: &[Vec<i8>]) -> i32 {
    (0..data.len())
        .map(|y| (0..data[y].len()).map(move |x| (x, y)))
        .flatten()
        .fold(0, |acc, (x, y)| {
            acc
        });

    0
}

fn main() {
    let stdin = std::io::stdin();
    let data: Vec<Vec<i8>> = stdin
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| (c as i8) - ('0' as i8))
                .collect()
        })
        .collect();

    println!("A: {}", solve_a(&data));
    println!("B: {}", solve_b(&data));
}
