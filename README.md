# telepass

Telegram Mini Apps [init data](https://core.telegram.org/bots/webapps#webappinitdata) validation and parsing for Rust.

## Installation

```bash
cargo add telepass
```

## Usage

### Validating Init Data

```rust
use telepass::init_data::InitData;
use std::time::Duration;

fn handle_request(raw_init_data: &str, bot_token: &str) {
    match InitData::validate(raw_init_data, bot_token, Duration::from_secs(86400)) {
        Ok(data) => println!("Valid init data: {}", data),
        Err(e) => eprintln!("Validation failed: {}", e),
    }
}
```

### Parsing Init Data

```rust
use telepass::init_data::InitData;

let init_data = InitData::parse(raw_init_data).expect("failed to parse init data");

println!("User: {:?}", init_data.user);
println!("Chat: {:?}", init_data.chat);
println!("Auth date: {:?}", init_data.get_auth_date());
```

### Validate Then Parse

```rust
use telepass::init_data::InitData;
use std::time::Duration;

let raw = InitData::validate(raw_init_data, bot_token, Duration::from_secs(86400))
.expect("invalid init data");

let parsed = InitData::parse( & raw).expect("failed to parse");

if let Some(user) = & parsed.user {
println!("Hello, {}!", user.first_name);
}
```

## Errors

| Error              | Description                          |
|--------------------|--------------------------------------|
| `AuthDateMissing`  | `auth_date` parameter is missing     |
| `AuthDateInvalid`  | `auth_date` is not a valid timestamp |
| `SignMissing`      | `hash` parameter is missing          |
| `SignInvalid`      | HMAC signature does not match        |
| `UnexpectedFormat` | Init data could not be parsed        |
| `Expired`          | Init data has expired                |