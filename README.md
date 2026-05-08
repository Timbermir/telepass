# telepass

Telegram Mini Apps [init data](https://core.telegram.org/bots/webapps#webappinitdata) validation and parsing for Rust.

[![crates.io](https://img.shields.io/crates/v/telepass.svg)](https://crates.io/crates/telepass)
[![docs.rs](https://docs.rs/telepass/badge.svg)](https://docs.rs/telepass)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## Features

- Validates the HMAC-SHA256 signature of Telegram Mini App init data
- Checks init data expiration
- Parses URL-encoded init data into typed Rust structs
- Optional [utoipa](https://github.com/juhaku/utoipa) OpenAPI schema support

---

## Installation

```toml
[dependencies]
telepass = "1.1.0"
```

Or via cargo:

```bash
cargo add telepass
```

### Optional Features

| Feature  | Description                                      |
|----------|--------------------------------------------------|
| `utoipa` | Derives `utoipa::ToSchema` on all public types   |

Enable with:

```toml
[dependencies]
telepass = { version = "1.1.0", features = ["utoipa"] }
```

---

## Usage

### Validate Init Data

Verifies the HMAC signature and checks that the data has not expired.

```rust
use telepass::init_data::InitData;
use std::time::Duration;

fn handle_request(raw_init_data: &str, bot_token: &str) {
    match InitData::validate(raw_init_data, bot_token, Duration::from_secs(86400)) {
        Ok(validated) => println!("Valid: {}", validated),
        Err(e) => eprintln!("Validation failed: {}", e),
    }
}
```

`validate` returns the original init data string on success, so you can pass it straight into `parse`.

---

### Parse Init Data

Deserializes URL-encoded init data into a typed [`InitData`](#initdata) struct.

```rust
use telepass::init_data::InitData;

let init_data = InitData::parse(raw_init_data).expect("failed to parse init data");

if let Some(user) = &init_data.user {
    println!("User ID:    {}", user.id);
    println!("First name: {}", user.first_name);
    println!("Premium:    {:?}", user.is_premium);
}

if let Some(chat) = &init_data.chat {
    println!("Chat: {} ({:?})", chat.title, chat.chat_type);
}

println!("Auth date: {:?}", init_data.get_auth_date());
```

---

### Validate Then Parse

The typical production flow — validate first, then parse the returned string.

```rust
use telepass::init_data::InitData;
use std::time::Duration;

let raw = InitData::validate(raw_init_data, bot_token, Duration::from_secs(86400))
    .expect("invalid init data");

let parsed = InitData::parse(&raw).expect("failed to parse");

if let Some(user) = &parsed.user {
    println!("Hello, {}!", user.first_name);
}
```

---

## API Reference

### `InitData`

The main struct representing parsed Telegram Mini App init data.

| Field                    | Type              | Telegram key      | Description                                        |
|--------------------------|-------------------|-------------------|----------------------------------------------------|
| `authentication_date_raw`| `u64`             | `auth_date`       | Unix timestamp of when the data was generated      |
| `hash`                   | `String`          | `hash`            | HMAC-SHA256 signature                              |
| `user`                   | `Option<User>`    | `user`            | The user who opened the Mini App                   |
| `receiver`               | `Option<User>`    | `receiver`        | The other user in a direct chat context            |
| `chat`                   | `Option<Chat>`    | `chat`            | Chat the Mini App was launched from                |
| `chat_type`              | `Option<ChatType>`| `chat_type`       | Type of the chat                                   |
| `chat_instance`          | `Option<i64>`     | `chat_instance`   | Global identifier for the chat                     |
| `query_id`               | `Option<String>`  | `query_id`        | Unique identifier for the Web App session          |
| `start_param`            | `Option<String>`  | `start_param`     | Value of the `startattach` parameter               |
| `seconds_to_send_after_raw` | `Option<u64>` | `can_send_after`  | Seconds after which `answerWebAppQuery` can be called |

#### Methods

```rust
// Validate signature and expiration. Returns the raw init data string on success.
InitData::validate(init_data: &str, telegram_bot_token: &str, expires_in: Duration) -> Result<String, TelepassError>

// Parse URL-encoded init data into an InitData struct.
InitData::parse(init_data: &str) -> Result<InitData, TelepassError>

// Convert the raw auth_date timestamp to a SystemTime.
init_data.get_auth_date() -> SystemTime

// Convert can_send_after to an absolute SystemTime (relative to auth_date), if present.
init_data.get_can_send_after() -> Option<SystemTime>
```

---

### `User`

Represents a Telegram user.

| Field                      | Type              | Telegram key              |
|----------------------------|-------------------|---------------------------|
| `id`                       | `i64`             | `id`                      |
| `first_name`               | `String`          | `first_name`              |
| `last_name`                | `Option<String>`  | `last_name`               |
| `username`                 | `Option<String>`  | `username`                |
| `avatar_url`               | `Option<String>`  | `photo_url`               |
| `language_code`            | `Option<String>`  | `language_code`           |
| `is_premium`               | `Option<bool>`    | `is_premium`              |
| `is_bot`                   | `Option<bool>`    | `is_bot`                  |
| `added_to_attachment_menu` | `Option<bool>`    | `added_to_attachment_menu`|
| `allows_write_to_pm`       | `Option<bool>`    | `allows_write_to_pm`      |

---

### `Chat`

Represents the chat the Mini App was launched from.

| Field        | Type        | Telegram key  |
|--------------|-------------|---------------|
| `id`         | `i64`       | `id`          |
| `title`      | `String`    | `title`       |
| `chat_type`  | `ChatType`  | `type`        |
| `username`   | `Option<String>` | `username` |
| `avatar_url` | `Option<String>` | `photo_url` |

---

### `ChatType`

```rust
pub enum ChatType {
    Sender,
    Private,
    Group,
    Supergroup,
    Channel,
}
```

---

## Errors

| Variant             | Description                                    |
|---------------------|------------------------------------------------|
| `AuthDateMissing`   | `auth_date` parameter is absent                |
| `AuthDateInvalid`   | `auth_date` is not a valid Unix timestamp      |
| `SignMissing`       | `hash` parameter is absent                     |
| `SignInvalid`       | HMAC-SHA256 signature does not match           |
| `UnexpectedFormat`  | Init data could not be URL-decoded or parsed   |
| `Expired`           | Init data is older than the allowed `expires_in` |

---

## License

MIT — see [LICENSE](LICENSE).
