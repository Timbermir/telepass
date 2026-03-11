use crate::error::TelepassError;
use crate::init_data::InitData;
use crate::sign::sign;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const TEST_TOKEN: &str = "5768337691:AAH5YkoiEuPk8-FZa32hStHTqXiLPtAEhx8";

fn make_test_init_data() -> String {
    let auth_date = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let pairs = vec![
        format!("auth_date={}", auth_date),
        "query_id=AAHdF6IQAAAAAN0XohDhrOrc".to_string(),
        "user={\"id\":279058397,\"first_name\":\"Vladislav\",\"last_name\":\"Kibenko\",\"username\":\"vdkfrost\",\"language_code\":\"ru\",\"is_premium\":true}".to_string(),
    ];
    let mut sorted = pairs.clone();
    sorted.sort();
    let data_check_string = sorted.join("\n");
    let hash = sign(&data_check_string, TEST_TOKEN);
    format!(
        "query_id=AAHdF6IQAAAAAN0XohDhrOrc&user=%7B%22id%22%3A279058397%2C%22first_name%22%3A%22Vladislav%22%2C%22last_name%22%3A%22Kibenko%22%2C%22username%22%3A%22vdkfrost%22%2C%22language_code%22%3A%22ru%22%2C%22is_premium%22%3Atrue%7D&auth_date={}&hash={}",
        auth_date, hash
    )
}

#[test]
fn validate_expiration() {
    let test_expired_init_data = "query_id=AAHdF6IQAAAAAN0XohDhrOrc&user=%7B%22id%22%3A279058397%2C%22first_name%22%3A%22Vladislav%22%2C%22last_name%22%3A%22Kibenko%22%2C%22username%22%3A%22vdkfrost%22%2C%22language_code%22%3A%22ru%22%2C%22is_premium%22%3Atrue%7D&auth_date=1662771648&hash=c501b71e775f74ce10e377dea85a7ea24ecd640b223ea86dfe453e0eaed2e2b2";
    let result = InitData::validate(test_expired_init_data, TEST_TOKEN, Duration::from_mins(30));
    assert_eq!(result.unwrap_err(), TelepassError::Expired)
}

#[test]
fn validate_sign_missing() {
    let result = InitData::validate("something=wrong", TEST_TOKEN, Duration::from_mins(30));
    assert_eq!(result.unwrap_err(), TelepassError::SignMissing)
}

#[test]
fn validate_auth_date_missing() {
    let result = InitData::validate("hash=abc", TEST_TOKEN, Duration::from_mins(30));
    assert_eq!(result.unwrap_err(), TelepassError::AuthDateMissing)
}

#[test]
fn validate_sign_invalid() {
    let result = InitData::validate(
        &(make_test_init_data() + "rust"),
        TEST_TOKEN,
        Duration::from_mins(30),
    );
    assert_eq!(result.unwrap_err(), TelepassError::SignInvalid)
}

#[test]
fn validate_auth_date_invalid() {
    let result = InitData::validate(
        "hash=abc&auth_date=rust",
        TEST_TOKEN,
        Duration::from_mins(30),
    );
    assert_eq!(result.unwrap_err(), TelepassError::AuthDateInvalid)
}

#[test]
fn parse_unexpected_format() {
    let result = InitData::parse("rust");
    assert_eq!(result.unwrap_err(), TelepassError::UnexpectedFormat)
}
