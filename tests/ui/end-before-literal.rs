use efg::efg;

#[efg(feature =)]
struct S;

#[efg((feature =) || unix)]
struct S;

fn main() {}
