use crate::ErrorInstance;
use crate::{Account, ErrorType};
use deltachat::context::Context;
use deltachat_command_derive::api_function2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;

use deltachat::chat::{get_chat_contacts, Chat, ChatId, ChatVisibility};
use deltachat::chatlist::Chatlist;
use deltachat::constants::{Chattype, DC_CONTACT_ID_SELF};

api_function2!(
    fn get_chat_list_ids<'t>(
        listflags: usize,
        query: Option<&'t str>,
        query_contact_id: Option<u32>,
    ) -> Result<Vec<u32>, ErrorInstance> {
        match Chatlist::try_load(&account.ctx, listflags, query, query_contact_id) {
            Ok(list) => {
                let mut l: Vec<u32> = Vec::new();
                for i in 0..list.len() {
                    l.push(list.get_chat_id(i).to_u32());
                }
                Ok(l)
            }
            Err(err) => Err(ErrorInstance {
                kind: ErrorType::DeltaChatError,
                message: format!("{:?}", err),
            }),
        }
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
        summary_status: String,
        is_verified: bool,
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

fn _get_chat_list_items_by_id(
    ctx: &Context,
    chat_id: ChatId,
) -> Result<ChatListItemFetchResult, deltachat::error::Error> {
    if chat_id.is_archived_link() {
        return Ok(ChatListItemFetchResult::ArchiveLink);
    }

    let last_message_id_option = deltachat::chat::get_chat_msgs(&ctx, chat_id, 0, None)
        .last()
        .copied();

    if chat_id.is_deaddrop() {
        let last_message_id = last_message_id_option.ok_or(deltachat::error::Error::Message(
            "couldn't fetch last chat message on deadrop".to_owned(),
        ))?;
        let last_message = deltachat::message::Message::load_from_db(&ctx, last_message_id)?;

        let contact = deltachat::contact::Contact::load_from_db(&ctx, last_message.get_from_id())?;

        return Ok(ChatListItemFetchResult::DeadDrop {
            last_updated: last_message.get_timestamp() * 1000,
            sender_name: contact.get_display_name().to_owned(),
            sender_address: contact.get_addr().to_owned(),
            sender_contact_id: contact.get_id(),
            message_id: last_message_id.to_u32(),
            summary_text1: "Name".to_owned(), // needs jikstras pr
            summary_text2: "Not Implemented".to_owned(), // needs jikstras pr
        });
    }
    let chat = Chat::load_from_db(&ctx, chat_id)?;

    let visibility = chat.get_visibility();

    let avatar_path = match chat.get_profile_image(ctx) {
        Some(path) => Some(path.to_str().unwrap_or("invalid/path").to_owned()),
        None => None,
    };

    let last_updated = match last_message_id_option {
        Some(id) => {
            let last_message = deltachat::message::Message::load_from_db(&ctx, id)?;
            Some(last_message.get_timestamp() * 1000)
        }
        None => None,
    };

    let self_in_group = get_chat_contacts(&ctx, chat_id).contains(&DC_CONTACT_ID_SELF);

    Ok(ChatListItemFetchResult::ChatListItem {
        id: chat_id.to_u32(),
        name: chat.get_name().to_owned(),
        avatar_path: avatar_path,
        color: format!("#{:x}", chat.get_color(&ctx)),
        last_updated: last_updated,
        summary_text1: "Name".to_owned(), // needs jikstras pr
        summary_text2: "Not Implemented".to_owned(), // needs jikstras pr
        summary_status: "unknown".to_owned(), // needs jikstras pr - and a function to transform the constant to strings? or return string enum
        // deaddrop: Option<Message object>,
        is_verified: chat.is_verified(),
        is_group: chat.get_type() == Chattype::Group || chat.get_type() == Chattype::VerifiedGroup,
        fresh_message_counter: chat_id.get_fresh_msg_cnt(&ctx),
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
    fn get_chat_list_items_by_ids(
        chat_ids: Vec<u32>,
    ) -> Result<HashMap<u32, ChatListItemFetchResult>, ErrorInstance> {
        let mut result: HashMap<u32, ChatListItemFetchResult> = HashMap::new();
        for i in 0..chat_ids.len() {
            let chat_id = ChatId::new(chat_ids[i]);
            result.insert(
                i.try_into().unwrap(),
                match _get_chat_list_items_by_id(&account.ctx, chat_id) {
                    Ok(res) => res,
                    Err(err) => ChatListItemFetchResult::Error {
                        id: chat_id.to_u32(),
                        error: format!("{}", err),
                    },
                },
            );
        }
        Ok(result)
    }
);
