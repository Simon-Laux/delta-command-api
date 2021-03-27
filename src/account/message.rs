use super::account::Account;
use crate::error::ErrorInstance;
use deltachat_command_derive::api_function2;
use serde::Deserialize;

use deltachat::chat::{ChatId, ChatItem};

api_function2!(
    async fn get_chat_message_ids(chat_id: u32) -> Result<Vec<u32>, ErrorInstance> {
        Ok(
            deltachat::chat::get_chat_msgs(&account.ctx, ChatId::new(chat_id), 0, None)
                .await
                .iter()
                .map(|m| match m {
                    ChatItem::Message { msg_id } => Some(msg_id),
                    _ => None,
                })
                .filter(|m| m.is_some())
                .map(|m| m.unwrap().to_u32())
                .collect(),
        )
    }
);


