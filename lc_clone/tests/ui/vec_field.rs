use lc_clone::LcClone;

#[derive(LcClone)]
struct BadStruct {
    items: Vec<i32>, // Vec does not implement LcClone
}

fn main() {}
