use crate::Account;
use crate::ErrorInstance;
use deltachat_command_derive::api_function2;
use serde::Deserialize;

use deltachat::chat::ChatId;

api_function2!(
    fn get_chat_message_ids(chat_id: u32) -> Result<Vec<u32>, ErrorInstance> {
        Ok(
            deltachat::chat::get_chat_msgs(&account.ctx, ChatId::new(chat_id), 0, None)
                .iter()
                .map(|m| m.to_u32())
                .collect(),
        )
    }
);
