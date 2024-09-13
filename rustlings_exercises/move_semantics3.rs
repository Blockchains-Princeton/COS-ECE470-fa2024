// move_semantics3.rs
//
// Make me compile without adding new lines-- just changing existing lines! (no
// lines with multiple semicolons necessary!)
//
// Scroll down for hints!

// I AM NOT DONE

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0.clone());

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}





















// The difference between this one and the previous ones is that the first line
// of `fn fill_vec` that had `let mut vec = vec;` is no longer there. You can,
// instead of adding that line back, add `mut` in one place that will change
// an existing binding to be a mutable binding instead of an immutable one :)