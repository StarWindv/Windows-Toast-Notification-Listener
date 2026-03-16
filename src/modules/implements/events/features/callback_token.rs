use crate::modules::types::events::features::callback_token::CallbackToken;
use std::sync::atomic::Ordering;

impl CallbackToken {
    pub fn new() -> Self {
        let id = crate::modules::types::events::features::callback_token::TOKEN_COUNTER
            .fetch_add(1, Ordering::SeqCst);
        Self { id }
    }
}
