# HS DEVS AUTH — Rust Client SDK

Official Rust client library for HS DEVS AUTH licensing and authentication service.

## Integration

Add to your `Cargo.toml`:
```toml
[dependencies]
hsauth = { path = "../sdk/rust" }
```

## Quick Start

```rust
use hsauth::HSAuth;

fn main() {
    let mut auth = HSAuth::new("AppName", "OwnerID", "Secret", "1.0");
    if auth.init() {
        if auth.license("XXXX-XXXX-XXXX-XXXX") {
            println!("Welcome! Expires: {}", auth.user_data.expires_at);
        }
    }
}
```
