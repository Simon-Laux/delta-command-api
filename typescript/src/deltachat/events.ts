export enum Event_TypeID {
  INFO = 100,
  SMTP_CONNECTED = 101,
  IMAP_CONNECTED = 102,
  SMTP_MESSAGE_SENT = 103,
  IMAP_MESSAGE_DELETED = 104,
  IMAP_MESSAGE_MOVED = 105,
  NEW_BLOB_FILE = 150,
  DELETED_BLOB_FILE = 151,
  WARNING = 300,
  ERROR = 400,
  ERROR_NETWORK = 401,
  ERROR_SELF_NOT_IN_GROUP = 410,
  MSGS_CHANGED = 2000,
  INCOMING_MSG = 2005,
  MSGS_NOTICED = 2008,
  MSG_DELIVERED = 2010,
  MSG_FAILED = 2012,
  MSG_READ = 2015,
  CHAT_MODIFIED = 2020,
  CHAT_EPHEMERAL_TIMER_MODIFIED = 2021,
  CONTACTS_CHANGED = 2030,
  LOCATION_CHANGED = 2035,
  CONFIGURE_PROGRESS = 2041,
  IMEX_PROGRESS = 2051,
  IMEX_FILE_WRITTEN = 2052,
  SECUREJOIN_INVITER_PROGRESS = 2060,
  SECUREJOIN_JOINER_PROGRESS = 2061
}

type field1_string =
  | Event_TypeID.INFO
  | Event_TypeID.SMTP_CONNECTED
  | Event_TypeID.IMAP_CONNECTED
  | Event_TypeID.SMTP_MESSAGE_SENT
  | Event_TypeID.IMAP_MESSAGE_DELETED
  | Event_TypeID.IMAP_MESSAGE_MOVED
  | Event_TypeID.NEW_BLOB_FILE
  | Event_TypeID.DELETED_BLOB_FILE
  | Event_TypeID.WARNING
  | Event_TypeID.ERROR
  | Event_TypeID.ERROR_NETWORK
  | Event_TypeID.ERROR_SELF_NOT_IN_GROUP
  | Event_TypeID.IMEX_FILE_WRITTEN;

type field1_number =
  | Event_TypeID.MSGS_NOTICED
  | Event_TypeID.CHAT_MODIFIED
  | Event_TypeID.IMEX_PROGRESS;

type field_both_numbers =
  | Event_TypeID.MSGS_CHANGED
  | Event_TypeID.INCOMING_MSG
  | Event_TypeID.MSG_DELIVERED
  | Event_TypeID.MSG_FAILED
  | Event_TypeID.MSG_READ
  | Event_TypeID.CHAT_EPHEMERAL_TIMER_MODIFIED
  | Event_TypeID.SECUREJOIN_INVITER_PROGRESS
  | Event_TypeID.SECUREJOIN_JOINER_PROGRESS;

type field1_number_or_null =
  | Event_TypeID.CONTACTS_CHANGED
  | Event_TypeID.LOCATION_CHANGED;

type field1_number_field2_string_or_null = Event_TypeID.CONFIGURE_PROGRESS;

/** just for orientation right now */
export type backend_json_event =
  | {
      id: field1_string;
      field1: undefined;
      field2: string;
    }
  | {
      id: field_both_numbers;
      field1: number;
      field2: number;
    }
  | {
      id: field1_number;
      field1: number;
      field2: undefined;
    }
  | { id: field1_number_or_null; field1: number | null; field2: undefined }
  | {
      id: field1_number_field2_string_or_null;
      field1: number;
      field2: string | null;
    };

/**
 * the fields are called fields to avaid confusion with data1/data2,
 * because they don't nessesarly follow the specs of data1/data2
 */
export class DeltaEventEmitter {
  listeners: { [eventid: number]: CallableFunction[] };

  constructor (){
    this.listeners = {}
  }

  emit(data: backend_json_event) {
    let listeners = this.listeners[data.id];
    if (!listeners) {
      // no listeners for this event
      return;
    } else {
      for (let listener of listeners) {
        listener(data.field1, data.field2);
      }
    }
  }

  // specific types - to name the function attributes (to help with documentation)
  on(
    eventTypeId:
      | Event_TypeID.INFO
      | Event_TypeID.WARNING
      | Event_TypeID.ERROR
      | Event_TypeID.SMTP_CONNECTED
      | Event_TypeID.IMAP_CONNECTED
      | Event_TypeID.SMTP_MESSAGE_SENT
      | Event_TypeID.IMAP_MESSAGE_DELETED
      | Event_TypeID.IMAP_MESSAGE_MOVED
      | Event_TypeID.ERROR_NETWORK
      | Event_TypeID.ERROR_SELF_NOT_IN_GROUP,
    cb: (message: string) => void | Promise<void>
  ): void;
  on(
    eventTypeId: Event_TypeID.IMEX_FILE_WRITTEN,
    cb: (path: string) => void | Promise<void>
  ): void;
  on(
    eventTypeId:
      | Event_TypeID.MSGS_CHANGED
      | Event_TypeID.INCOMING_MSG
      | Event_TypeID.MSG_DELIVERED
      | Event_TypeID.MSG_FAILED
      | Event_TypeID.MSG_READ,
    cb: (chatId: number, messageId: number) => void | Promise<void>
  ): void;
  on(
    eventTypeId: Event_TypeID.IMEX_PROGRESS,
    cb: (progress: number) => void | Promise<void>
  ): void;
  on(
    eventTypeId: Event_TypeID.CONFIGURE_PROGRESS,
    cb: (progress: number, comment: string | null) => void | Promise<void>
  ): void;
  // general types -> fallback to show atleast some typing
  on(
    eventTypeId: field1_string,
    cb: (field1: string) => void | Promise<void>
  ): void;
  on(
    eventTypeId: field1_number,
    cb: (field1: number) => void | Promise<void>
  ): void;
  on(
    eventTypeId: field_both_numbers,
    cb: (field1: number, field2: number) => void | Promise<void>
  ): void;
  on(
    eventTypeId: field1_number_field2_string_or_null,
    cb: (field1: number, field2: string | null) => void | Promise<void>
  ): void;
  on(
    eventTypeId: field1_number_or_null,
    cb: (field1: number | null) => void | Promise<void>
  ): void;
  // catch undefined cases
  on(
    eventTypeId: number,
    cb: (field1: any, field2: any) => void | Promise<void>
  ) {
    if (!this.listeners[eventTypeId]) {
      this.listeners[eventTypeId] = [cb];
    } else {
      this.listeners[eventTypeId].push(cb);
    }
  }

  // todo once()

  // todo removeListener (should work both for on and once)
}
