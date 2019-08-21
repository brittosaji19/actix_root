# actix_root

A quick start mvc base project for Actix web-framework for Rust

## Geting Started

### Pre-Requesites
    - Rust (Can be installed with rustup)
    - Cargo (Package manager for rust)
    - Diesel ORM ()
    - systemfd and cargo-watch (Required for Auto Reloading)
  - `cargo install systemfd cargo-watch `
  
### Initializing project

```
    cargo run //Builds and runs the package without auto reload
    systemfd --no-pid -s http::3000 -- cargo watch -x run //Run with auto-reload
    
```

### Databases
- For all databases
```
    cargo install diesel_cli
```
- For Postgres featurs only
```
cargo install diesel_cli --no-default-features --features postgres
```

Modify `.env` file to include your database url

- Apply migrations

`diesel migration run`

#### users table is generated and authentication is implemented
    