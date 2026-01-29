use lc_clone::LcClone;

#[derive(LcClone)]
struct BadStruct {
    name: String, // String does not implement LcClone
}

fn main() {}
