use crate::timestamp::Timestamp;

#[derive(Clone, Copy, PartialEq)]
pub struct ExchangeId(pub u32);

#[derive(Clone, Copy, PartialEq)]
pub struct NodeId(pub u32);

pub struct Sync;

pub struct SyncFollowUp {
    pub sync_sent: Timestamp,
}

pub struct DelayReq {
    pub sync_rcvd: Timestamp,
}

pub struct DelayReqFollowUp {
    pub delay_req_sent: Timestamp,
}

pub struct DelayResp {
    pub delay_req_rcvd: Timestamp,
}

pub enum ReqToRespPld {
    Sync(Sync),
    SyncFollowUp(SyncFollowUp),
    DelayResp(DelayResp),
}

pub enum RespToReqPld {
    DelayReq(DelayReq),
    DelayReqFollowUp(DelayReqFollowUp),
}

pub enum Pld {
    ReqToResp(ReqToRespPld),
    RespToReq(RespToReqPld),
}

pub struct Msg {
    sender_id: NodeId,
    exchange_id: ExchangeId,
    pld: Pld,
}
