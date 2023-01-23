$env:RUST_LOG="debug"
# runs tests for the auth0 portion of this crate
cargo test auth0_management_v2_client -- --nocapture