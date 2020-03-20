// A move consists of taking a point (x, y) and transforming it to either (x, x+y) or (x+y, y).
// Given a starting point (sx, sy) and a target point (tx, ty), return True if and only if a sequence of moves exists to transform the point (sx, sy) to (tx, ty). Otherwise, return False.

/// Input: sx = 1, sy = 1, tx = 3, ty = 5
/// Output: True
/// Explanation:
/// One series of moves that transforms the starting point to the target is:
/// (1, 1) -> (1, 2)
/// (1, 2) -> (3, 2)
/// (3, 2) -> (3, 5)
/// Input: sx = 1, sy = 1, tx = 2, ty = 2
/// Output: False
/// Input: sx = 1, sy = 1, tx = 1, ty = 1
/// Output: True
///
pub fn reaching_points2(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
    // println!("Trying ({}, {})", sx, sy);
    // Finishing cases
    if sx == tx && sy == ty {
        true
    // No transform will solve it, return false
    } else if sy > ty || sx > tx {
        false
    } else {
        reaching_points2(sx + sy, sy, tx, ty) || reaching_points2(sx, sx + sy, tx, ty)
    }
}

#[test]
fn simple_test() {
    let sx = 1;
    let sy = 1;
    let tx = 3;
    let ty = 5;
    assert_eq!(reaching_points2(sx, sy, tx, ty), true);
}

#[test]
fn simple_test_false() {
    let sx = 1;
    let sy = 1;
    let tx = 2;
    let ty = 2;
    assert_eq!(reaching_points2(sx, sy, tx, ty), false);
}

#[test]
fn simple_test_failing() {
    let sx = 3;
    let sy = 3;
    let tx = 12;
    let ty = 9;
    assert_eq!(reaching_points2(sx, sy, tx, ty), true);
}

#[test]
fn simple_test_failing_2() {
    let sx = 9;
    let sy = 10;
    let tx = 9;
    let ty = 19;
    assert_eq!(reaching_points2(sx, sy, tx, ty), true);
}

#[test]
fn it_works_with_large_numbers() {
    let sx = 35;
    let sy = 13;
    let tx = 455955547;
    let ty = 420098884;
    assert_eq!(reaching_points2(sx, sy, tx, ty), true);
}