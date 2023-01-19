$env:RUST_LOG="debug"
# runs tests for the auth0 portion of this crate
cargo test --features serde,figment,reqwest auth0_client -- --nocapture