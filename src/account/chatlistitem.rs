use super::account::Account;
use crate::account::util::color_int_to_hex;
use crate::error::{ErrorInstance, ErrorType};
use crate::genericError;
use deltachat::context::Context;
use deltachat::message::MsgId;
use deltachat_command_derive::api_function2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;

use deltachat::chat::{get_chat_contacts, Chat, ChatId, ChatItem, ChatVisibility};
use deltachat::chatlist::Chatlist;
use deltachat::constants::{Chattype, DC_CONTACT_ID_SELF};

use num_traits::cast::ToPrimitive;

api_function2!(
    async fn get_chat_list_ids<'t>(
        listflags: usize,
        query: Option<&'t str>,
        query_contact_id: Option<u32>,
    ) -> Result<Vec<u32>, ErrorInstance> {
        let list = Chatlist::try_load(&account.ctx, listflags, query, query_contact_id).await?;
        let mut l: Vec<u32> = Vec::new();
        for i in 0..list.len() {
            l.push(list.get_chat_id(i).to_u32());
        }
        Ok(l)
    }
);

#[derive(Serialize)]
#[serde(tag = "type")]
pub(crate) enum ChatListItemFetchResult {
    #[serde(rename_all = "camelCase")]
    ChatListItem {
        id: u32,
        name: String,
        avatar_path: Option<String>,
        color: String,
        last_updated: Option<i64>,
        summary_text1: String,
        summary_text2: String,
        summary_status: i64,
        is_protected: bool,
        is_group: bool,
        fresh_message_counter: usize,
        is_self_talk: bool,
        is_device_talk: bool,
        is_sending_location: bool,
        is_self_in_group: bool,
        is_archived: bool,
        is_pinned: bool,
        is_muted: bool,
    },
    #[serde(rename_all = "camelCase")]
    DeadDrop {
        last_updated: i64,
        sender_name: String,
        sender_address: String,
        sender_contact_id: u32,
        message_id: u32,
        summary_text1: String,
        summary_text2: String,
    },
    ArchiveLink,
    #[serde(rename_all = "camelCase")]
    Error {
        id: u32,
        error: String,
    },
}

async fn _get_chat_list_items_by_id(
    ctx: &Context,
    chat_id: ChatId,
) -> Result<ChatListItemFetchResult, ErrorInstance> {
    if chat_id.is_archived_link() {
        return Ok(ChatListItemFetchResult::ArchiveLink);
    }

    let last_message_id_option = match deltachat::chat::get_chat_msgs(&ctx, chat_id, 0, None)
        .await
        .last()
    {
        Some(ChatItem::Message { msg_id }) => Some(*msg_id),
        _ => None,
    };

    let last_message_id = match last_message_id_option {
        Some(id) => id,
        None => MsgId::new(0),
    };

    let chat = Chat::load_from_db(&ctx, chat_id).await?;
    let summary = Chatlist::get_summary2(&ctx, chat_id, last_message_id, Some(&chat)).await;

    let summary_text1 = summary.get_text1().unwrap_or("").to_owned();
    let summary_text2 = summary.get_text2().unwrap_or("").to_owned();

    if chat_id.is_deaddrop() {
        let last_message_id = last_message_id_option
            .ok_or(genericError!("couldn't fetch last chat message on deadrop"))?;
        let last_message = deltachat::message::Message::load_from_db(&ctx, last_message_id).await?;

        let contact =
            deltachat::contact::Contact::load_from_db(&ctx, last_message.get_from_id()).await?;

        return Ok(ChatListItemFetchResult::DeadDrop {
            last_updated: last_message.get_timestamp() * 1000,
            sender_name: contact.get_display_name().to_owned(),
            sender_address: contact.get_addr().to_owned(),
            sender_contact_id: contact.get_id(),
            message_id: last_message_id.to_u32(),
            summary_text1,
            summary_text2,
        });
    }

    let visibility = chat.get_visibility();

    let avatar_path = match chat.get_profile_image(ctx).await {
        Some(path) => Some(path.to_str().unwrap_or("invalid/path").to_owned()),
        None => None,
    };

    let last_updated = match last_message_id_option {
        Some(id) => {
            let last_message = deltachat::message::Message::load_from_db(&ctx, id).await?;
            Some(last_message.get_timestamp() * 1000)
        }
        None => None,
    };

    let self_in_group = get_chat_contacts(&ctx, chat_id)
        .await
        .contains(&DC_CONTACT_ID_SELF);

    let fresh_message_counter = chat_id.get_fresh_msg_cnt(&ctx).await;
    let color = color_int_to_hex(chat.get_color(&ctx).await);

    Ok(ChatListItemFetchResult::ChatListItem {
        id: chat_id.to_u32(),
        name: chat.get_name().to_owned(),
        avatar_path,
        color,
        last_updated,
        summary_text1,
        summary_text2,
        summary_status: summary.get_state().to_i64().expect("impossible"), // idea and a function to transform the constant to strings? or return string enum
        is_protected: chat.is_protected(),
        is_group: chat.get_type() == Chattype::Group,
        fresh_message_counter,
        is_self_talk: chat.is_self_talk(),
        is_device_talk: chat.is_device_talk(),
        is_self_in_group: self_in_group,
        is_sending_location: chat.is_sending_locations(),
        is_archived: visibility == ChatVisibility::Archived,
        is_pinned: visibility == ChatVisibility::Pinned,
        is_muted: chat.is_muted(),
    })
}

api_function2!(
    async fn get_chat_list_items_by_ids(
        chat_ids: Vec<u32>,
    ) -> Result<HashMap<u32, ChatListItemFetchResult>, ErrorInstance> {
        let mut result: HashMap<u32, ChatListItemFetchResult> = HashMap::new();
        for (i, item) in chat_ids.iter().enumerate() {
            let chat_id = ChatId::new(*item);
            result.insert(
                i.try_into().unwrap(),
                match _get_chat_list_items_by_id(&account.ctx, chat_id).await {
                    Ok(res) => res,
                    Err(err) => ChatListItemFetchResult::Error {
                        id: chat_id.to_u32(),
                        error: format!("{:?}", err),
                    },
                },
            );
        }
        Ok(result)
    }
);
