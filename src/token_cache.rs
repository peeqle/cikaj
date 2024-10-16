use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

static CLIENT_TOKEN_BUFFER: Lazy<Arc<Mutex<Box<HashMap<String, TokenBody>>>>> = Lazy::new(|| Arc::new(Mutex::new(Box::new(HashMap::new()))));

pub async fn add(key: &str) {
    let mut map = CLIENT_TOKEN_BUFFER.lock().await;
    map.insert(String::from(key),
               TokenBody { valid: true, checked_at: SystemTime::now().duration_since(UNIX_EPOCH).expect("Cannot fetch time").as_millis() });
}

pub async fn drop(key: &str) {
    let mut map = CLIENT_TOKEN_BUFFER.lock().await;
    map.remove(key);
}

pub async fn is_present(key: &str) -> bool {
    let mut map = CLIENT_TOKEN_BUFFER.lock().await;

    match map.get(key) {
        None => { false }
        Some(val) => {
            let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Cannot fetch time").as_millis();

            match time - val.checked_at > 10000 {
                true => {
                    true
                }
                false => {
                    drop(key).await;
                    false
                }
            }
        }
    }
}
#[derive(Serialize, Deserialize, Eq, PartialEq)]
struct TokenBody {
    valid: bool,
    checked_at: u128,
}