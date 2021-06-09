pub mod account;
pub mod commands;
pub mod error;

use deltachat::{Event, EventType};

use serde_json::json;
use serde_json::value::Value;

pub fn event_to_json(event: Event) -> String {
    let (field1, field2): (Value, Value) = match &event.typ {
        // events with a single string in field1
        EventType::Info(txt)
        | EventType::SmtpConnected(txt)
        | EventType::ImapConnected(txt)
        | EventType::SmtpMessageSent(txt)
        | EventType::ImapMessageDeleted(txt)
        | EventType::ImapMessageMoved(txt)
        | EventType::NewBlobFile(txt)
        | EventType::DeletedBlobFile(txt)
        | EventType::Warning(txt)
        | EventType::Error(txt)
        | EventType::ErrorNetwork(txt)
        | EventType::ErrorSelfNotInGroup(txt) => (json!(txt), Value::Null),
        EventType::ImexFileWritten(path) => (json!(path.to_str()), Value::Null),
        // single number
        EventType::MsgsNoticed(chat_id) | EventType::ChatModified(chat_id) => {
            (json!(chat_id), Value::Null)
        }
        EventType::ImexProgress(progress) => (json!(progress), Value::Null),
        // both fields contain numbers
        EventType::MsgsChanged { chat_id, msg_id }
        | EventType::IncomingMsg { chat_id, msg_id }
        | EventType::MsgDelivered { chat_id, msg_id }
        | EventType::MsgFailed { chat_id, msg_id }
        | EventType::MsgRead { chat_id, msg_id } => (json!(chat_id), json!(msg_id)),
        EventType::ChatEphemeralTimerModified { chat_id, timer } => (json!(chat_id), json!(timer)),
        EventType::SecurejoinInviterProgress {
            contact_id,
            progress,
        }
        | EventType::SecurejoinJoinerProgress {
            contact_id,
            progress,
        } => (json!(contact_id), json!(progress)),
        // field 1 number or null
        EventType::ContactsChanged(maybe_number) | EventType::LocationChanged(maybe_number) => (
            match maybe_number {
                Some(number) => json!(number),
                None => Value::Null,
            },
            Value::Null,
        ),
        // number and maybe string
        EventType::ConfigureProgress { progress, comment } => (
            json!(progress),
            match comment {
                Some(content) => json!(content),
                None => Value::Null,
            },
        ),
    };

    json!({
        "event": true,
        "id": [event.typ.as_id()],
        "field1": field1,
        "field2": field2
    })
    .to_string()
}
