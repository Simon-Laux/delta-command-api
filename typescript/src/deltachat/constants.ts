export enum Chattype {
    Undefined = 0,
    Single = 100,
    Group = 120,
    Mailinglist = 140,
}

export const enum MessageStatus {
    MsgInFresh = 10,
    MsgInNoticed = 13,
    MsgInSeen = 16,
    MsgOutPreparing = 18,
    MsgOutDraft = 19,
    MsgOutPending = 20,
    MsgOutFailed = 24,
    MsgOutDelivered = 26,
    /** Seen by other side */
    MsgOutMdnRcvd = 28,
}