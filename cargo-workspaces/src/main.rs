//A workspace is a collection of multiple related crates that share a single Cargo.lock file and a common target/ directory, improving dependency management and build efficiency.
/*
The top-level Cargo.toml contains a [workspace] section instead of a [package] section.
New crates can be created using cargo new crate_name or cargo new crate_name --lib.
Workspace members are listed in the members array of the workspace Cargo.toml.
Crates inside a workspace do not automatically depend on each other.
Local dependencies are added using the path attribute:
add_one = { path = "../add_one" }
All crates share one target/ directory, reducing recompilation and speeding up builds.
External dependencies are declared separately for each crate.
cargo build builds all crates, while cargo run -p crate_name runs a specific crate.
cargo test runs tests for all crates, and cargo test -p crate_name tests a specific crate.*/