use efg::efg;

#[efg(unix || (windows, wasm))]
struct S;

fn main() {}
