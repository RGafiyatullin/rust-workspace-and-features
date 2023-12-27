
# Workspaces and Crate Features

This workspace demonstrates how a conditional compilation may be used consistently, 
so that `cargo` can be used to build and test, in both debug and release modes.

In this demo, the "library" crates provide:
- two functionality related features — "light" and "full";
- a feature enabling the mock, that can be used from outside of the crate;

The "libraries" form a dependency chain as follows: c -> b -> a.

This depth is necessary to demonstrate the transient dependency of `c` on `a` 
(`c` does not select `a`'s features directly).

# Demo

## bin: light

`light` depends on `c/light`.

Its tests require no additional features.

```toml
[dependencies]
c = {path = "../../lib/c", features = ["light"]}
```

Running `light`:

```
➜  features-demo git:(main) ✗ cargo run --release -p light                                                                     
....>8....>8....>8....
[
    "a::do_it",
    "a: cfg(feature = light)",
    "b::do_it",
    "b: cfg(feature = light)",
    "c::do_it",
    "c: cfg(feature = light)",
]
```

Running `light`'s tests:

```
➜  features-demo git:(main) ✗ cargo test --release -p light -- --nocapture
....>8....>8....>8....
[
    "a::do_it",
    "a: cfg(feature = light)",
    "b::do_it",
    "b: cfg(feature = light)",
    "c::do_it",
    "c: cfg(feature = light)",
]
```

## bin: full

`full` depends on `c/full`.

Its tests (additionally) require `c/enable-mock-c`.


```toml
[dependencies]
c = {path = "../../lib/c", features = ["full"]}

[dev-dependencies]
c = {path = "../../lib/c", features = ["enable-mock-c"]}
```

Running `full`:

```
➜  features-demo git:(main) ✗ cargo run --release -p full 
....>8....>8....>8....
[
    "a::do_it",
    "a: cfg(feature = full)",
    "b::do_it",
    "b: cfg(feature = full)",
    "c::do_it",
    "c: cfg(feature = full)",
]
```

Running `full`'s tests:

```
➜  features-demo git:(main) ✗ cargo test --release -p full -- --nocapture 
....>8....>8....>8....
[
    "a::do_it", 
    "a: cfg(feature = full)", 
    "b::do_it", 
    "b: cfg(feature = full)", 
    "c::do_it", 
    "c: cfg(feature = full)", 
    "c: cfg(feature = enable-mock-c)"
]
```

## lib: a (test --release)


```toml
[features]
light = []
full = []
enable-mock-a = []
```

Running `a`'s tests:

```
➜  features-demo git:(main) ✗ cargo test --release -p a -- --nocapture
....>8....>8....>8....
[
    "a::do_it", 
    "a: cfg(test)",
]
```

## lib: b (test --release)

```toml
[features]
light = ["a/light"]
full = ["a/full"]
enable-mock-b = []

[dependencies]
a = {path = "../a", features = []}

[dev-dependencies]
a = {path = "../a", features = ["enable-mock-a"]}
```

Running `b`'s tests:

```
➜  features-demo git:(main) ✗ cargo test --release -p b -- --nocapture
....>8....>8....>8....
[
    "a::do_it", 
    "a: cfg(feature = enable-mock-a)", 
    "b::do_it", 
    "b: cfg(test)",
]
```

## lib: c (test --release)

```toml
[features]
light = ["b/light"]
full = ["b/full"]
enable-mock-c = []

[dependencies]
b = {path = "../b", features = []}

[dev-dependencies]
b = {path = "../b", features = ["enable-mock-b"]}
```


Running `c`'s tests:

```
➜  features-demo git:(main) ✗ cargo test --release -p c -- --nocapture
....>8....>8....>8....
[
    "a::do_it", 
    "b::do_it", 
    "b: cfg(feature = enable-mock-b)", 
    "c::do_it", 
    "c: cfg(test)",
]
```
