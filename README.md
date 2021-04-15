# Rocket simple application
[![Build Status](https://travis-ci.org/giorgikhachidze/rocket_simple_app.svg?branch=master)](https://travis-ci.org/giorgikhachidze/rocket_simple_app)
[![Crates.io](https://img.shields.io/crates/l/rocket/0.4.2)](https://www.apache.org/foundation/license-faq.html)
[![GitHub repo size](https://img.shields.io/github/repo-size/giorgikhachidze/rocket_simple_app?label=code%20size)](https://github.com/giorgikhachidze/rocket_simple_app)
[![Crates.io](https://img.shields.io/crates/v/rocket?&label=rocket)](https://rocket.rs/)
[![Crates.io](https://img.shields.io/crates/v/diesel?&label=diesel)](http://diesel.rs/)

Rocket is a web framework for Rust (nightly) with a focus on ease-of-use,
expressibility, and speed. Here's an example of user register and authorization.

## Getting Started

#1 We need to install a safe, extensible ORM and Query Builder for Rust "[https://diesel.rs/guides/getting-started](https://diesel.rs/guides/getting-started)."
```rust
cargo install diesel_cli
```

#2 Build our application.

```rust
cargo build
```

#3 Database connection .env.
```rust
DATABASE_URL=mysql://username:password@hostname/dbname
```

#4 Run migration
```rust
giorgi@giorgi:~/rocket_simple_app$ diesel migration run
Running migration 2019-11-16-000655_create_users_sessions
Running migration 2019-11-19-080726_create_users
```

#5 Runing our application.

```rust
giorgi@giorgi:~/rocket_simple_app$ cargo run
   Compiling rocket_simple_app v0.1.0 (/home/giorgi/rocket_simple_app)
    Finished dev [unoptimized + debuginfo] target(s) in 3.55s
     Running `target/debug/rocket_simple_app`
🔧 Configured for development.
    => address: localhost
    => port: 8000
    => log: normal
    => workers: 8
    => secret key: generated
    => limits: forms = 32KiB
    => keep-alive: 5s
    => tls: disabled
🛰  Mounting /:
    => GET / (index)
    => GET /login (login)
    => POST /login (authorization)
    => GET /register (register)
    => POST /register (registration)
📡 Fairings:
    => 1 request: Templates
🚀 Rocket has launched from http://localhost:8000
```

Visiting http://localhost:8000/.
