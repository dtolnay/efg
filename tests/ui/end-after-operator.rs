use efg::efg;

#[efg(unix ||)]
struct S;

#[efg((unix ||) && window)]
struct S;

fn main() {}
