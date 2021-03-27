use super::account::Account;
use crate::account::util::color_int_to_hex;
use crate::error::ErrorInstance;
use deltachat_command_derive::api_function2;
use serde::{Deserialize, Serialize};

use deltachat::chat::{get_chat_contacts, Chat, ChatId, ChatVisibility};
use deltachat::constants::{Chattype, DC_CONTACT_ID_SELF};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FullChat {
    id: u32,
    name: String,
    is_protected: bool,
    profile_image: Option<String>,

    archived: bool,
    // subtitle: String, -> removed for now because this field needs to be translated
    #[serde(rename = "type")]
    chat_type: Chattype,
    /** new chat but no initial message sent */
    is_unpromoted: bool,
    // contacts: contacts,
    contact_ids: Vec<u32>,
    color: String,
    fresh_message_counter: usize,
    is_group: bool,
    is_deaddrop: bool,
    is_self_talk: bool,
    is_device_chat: bool,
    is_self_in_group: bool,
}

api_function2!(
    async fn get_full_chat_by_id(chat_id_number: u32) -> Result<FullChat, ErrorInstance> {
        let chat_id = ChatId::new(chat_id_number);
        let chat = Chat::load_from_db(&account.ctx, chat_id).await?;

        let visibility = chat.get_visibility();
        let avatar_path = match chat.get_profile_image(&account.ctx).await {
            Some(path) => Some(path.to_str().unwrap_or("invalid/path").to_owned()),
            None => None,
        };
        let contact_ids = get_chat_contacts(&account.ctx, chat_id).await;
        let self_in_group = contact_ids.contains(&DC_CONTACT_ID_SELF);

        let color = color_int_to_hex(chat.get_color(&account.ctx).await);
        let fresh_message_counter = chat_id.get_fresh_msg_cnt(&account.ctx).await;

        Ok(FullChat {
            id: chat_id.to_u32(),
            name: chat.get_name().to_owned(),
            is_protected: chat.is_protected(),
            profile_image: avatar_path,
            archived: visibility == ChatVisibility::Archived,
            chat_type: chat.get_type(),
            /** new chat but no initial message sent */
            is_unpromoted: !chat.is_promoted(),
            // contacts: contacts,
            contact_ids,
            color,
            fresh_message_counter,
            is_group: chat.get_type() == Chattype::Group,
            is_deaddrop: chat_id.is_deaddrop(),
            is_self_talk: chat.is_self_talk(),
            is_device_chat: chat.is_device_talk(),
            is_self_in_group: self_in_group,
        })
    }
);
