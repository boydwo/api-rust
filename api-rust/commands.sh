rustup default nightly
cargo update

diesel migration generate users
diesel migration run
diesel print-schema > src/schema.rs
