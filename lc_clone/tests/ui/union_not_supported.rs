use lc_clone::LcClone;

#[derive(LcClone)]
union MyUnion {
    a: i32,
    b: f32,
}

fn main() {}
