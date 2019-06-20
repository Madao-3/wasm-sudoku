use wasm_bindgen::prelude::*;

#[derive(Debug)]
struct Point(i32, i32);

#[wasm_bindgen]
pub fn solver(board: Vec<i32>) -> Vec<i32> {
    // set_panic_hook();
    solve(board).unwrap()
}

fn NUMBER() -> Vec<i32> {
    vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
}

fn col(board: &Vec<i32>, y: i32) -> Vec<i32> {
    let mut list: Vec<i32> = Vec::new();
    for i in 0..9 {
        let index: usize = (i * 9 + y) as usize;
        if board[index] != 0 {
            list.push(board[index])
        }
    }
    let diff: Vec<i32> = NUMBER().into_iter().difference(list.into_iter().clone());
    diff
}

fn row(board: &Vec<i32>, x: i32) -> Vec<i32> {
    let mut list: Vec<i32> = Vec::new();
    for i in 0..9 {
        let index: usize = (i + 9 * x) as usize;
        if board[index] != 0 {
            list.push(board[index])
        }
    }
    let diff: Vec<i32> = NUMBER().into_iter().difference(list.into_iter().clone());
    diff
}

fn square(board: &Vec<i32>, pos: &Point) -> Vec<i32> {
    let sx = 3 * (pos.0 / 3);
    let sy = 3 * (pos.1 / 3);
    let mut square: Vec<i32> = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            let index: usize = ((sx + i) * 9 + (sy + j)) as usize;
            square.push(board[index]);
        }
    }
    let diff: Vec<i32> = NUMBER().into_iter().difference(square.into_iter().clone());
    diff
}

fn allows(board: &Vec<i32>, pos: &Point) -> Vec<i32> {
    let diff: Vec<i32> = row(board, pos.0)
        .into_iter()
        .intersect(col(board, pos.1).into_iter());
    square(board, &pos).into_iter().intersect(diff.into_iter())
}

fn empty(board: &Vec<i32>) -> Option<Point> {
    for pos in 0..board.len() {
        if board[pos] == 0 {
            return Some(Point((pos / 9) as i32, (pos % 9) as i32));
        }
    }
    None
}

fn solve(mut board: Vec<i32>) -> Option<Vec<i32>> {
    match empty(&board) {
        Some(point) => {
            let mut allow_places = allows(&board, &point);
            while allow_places.len() > 0 {
                let index = (point.0 * 9 + point.1) as usize;
                board[index] = allow_places.remove(0);
                match solve(board.clone()) {
                    Some(result) => {
                        return Some(result.clone());
                    }
                    _ => {
                        board[index] = 0;
                    }
                }
            }
        }
        _ => return Some(board.clone()),
    }
    None
}

pub trait IterOps<T, I>: IntoIterator<Item = T>
where
    I: IntoIterator<Item = T>,
    T: PartialEq,
{
    fn intersect(self, other: I) -> Vec<T>;
    fn difference(self, other: I) -> Vec<T>;
}

impl<T, I> IterOps<T, I> for I
where
    I: IntoIterator<Item = T>,
    T: PartialEq,
{
    fn intersect(self, other: I) -> Vec<T> {
        let mut common = Vec::new();
        let mut v_other: Vec<_> = other.into_iter().collect();

        for e1 in self.into_iter() {
            if let Some(pos) = v_other.iter().position(|e2| e1 == *e2) {
                common.push(e1);
                v_other.remove(pos);
            }
        }

        common
    }

    fn difference(self, other: I) -> Vec<T> {
        let mut diff = Vec::new();
        let mut v_other: Vec<_> = other.into_iter().collect();

        for e1 in self.into_iter() {
            if let Some(pos) = v_other.iter().position(|e2| e1 == *e2) {
                v_other.remove(pos);
            } else {
                diff.push(e1);
            }
        }

        diff.append(&mut v_other);
        diff
    }
}
