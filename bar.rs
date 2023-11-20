#!/usr/bin/env -S cargo +nightly -Zscript --quiet
```cargo
[dependencies]
foo = { path = "./foo" }
```

fn main() {
  println!("{}", foo::add(10, 20));
}
