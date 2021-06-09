import { TransportMethod } from "../transportMethod";
import { Chattype } from "./constants";

export type DeadDrop = {
  type: "DeadDrop";
  lastUpdated: number;
  messageId: number;
  senderAddress: string;
  senderContactId: number;
  senderName: string;
  summaryText1: string;
  summaryText2: string;
};

export type ChatListItem = {
  type: "ChatListItem";
  id: number;
  name: string;
  avatarPath: null | string;
  color: string;
  lastUpdated: number;
  freshMessageCounter: number;
  summaryStatus: number;
  summaryText1: string;
  summaryText2: string;
  isArchived: boolean;
  isDeviceTalk: boolean;
  isGroup: boolean;
  isMuted: boolean;
  isPinned: boolean;
  isSelfInGroup: boolean;
  isSelfTalk: boolean;
  isSendingLocation: boolean;
  isProtected: boolean;
};

export type ArchiveLink = { type: "ArchiveLink" };

export type ChatListItemFetchError = {
  type: "Error";
  id: number;
  error: string;
};

export type ChatListItemFetchResult =
  | DeadDrop
  | ChatListItem
  | ArchiveLink
  | ChatListItemFetchError;

export type FullChat = {
  id: number;
  name: string;
  isProtected: boolean;
  profileImage: string | null;
  archived: boolean;
  type: Chattype;
  /** new chat but no initial message sent */
  isUnpromoted: boolean;
  contactIds: number[];
  color: string;
  freshMessageCounter: number;
  isGroup: boolean;
  isDeaddrop: boolean;
  isSelfTalk: boolean;
  isDeviceChat: boolean;
  isSelfInGroup: boolean;
};

export class ChatList {
  constructor(public transport: TransportMethod) {}

  async getChatListIds(
    listFlags: number,
    options: {
      /** search word for searching */
      query?: string;
      query_contact_id?: number;
    } = {}
  ): Promise<number[]> {
    // TODO this now has two values [chatid, last message id]
    return this.transport.send(40, {
      listflags: listFlags,
      ...options,
    });
  }

  async getChatListItemsByIds(
    chat_ids: number[]
  ): Promise<ChatListItemFetchResult[]> {
    return this.transport.send(41, {
      chat_ids: chat_ids,
    });
  }

  async getFullChatById(chatId: number): Promise<FullChat> {
    return this.transport.send(46, {
      chat_id_number: chatId,
    });
  }
}
