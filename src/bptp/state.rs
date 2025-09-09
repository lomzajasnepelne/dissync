use crate::timestamp::Timestamp;

pub struct WaitForSync;

pub struct WaitForSyncFollowUp {
    sync_rcvd: Timestamp,
}

impl WaitForSyncFollowUp {
    pub fn from_previous(_prev: WaitForSync, sync_rcvd: Timestamp) -> Self {
        Self { sync_rcvd }
    }
}

pub struct WaitForDelayReq {
    sync_sent: Timestamp,
}

pub struct WaitForDelayReqFollowUp {
    sync_sent: Timestamp,
    sync_rcvd: Timestamp,
    delay_req_rcvd: Timestamp,
}

pub struct WaitForDelayResp {
    sync_sent: Timestamp,
    sync_rcvd: Timestamp,
    delay_req_sent: Timestamp,
}

impl WaitForDelayResp {
    pub fn from_previous(
        prev: WaitForSyncFollowUp,
        sync_sent: Timestamp,
        delay_req_sent: Timestamp,
    ) -> Self {
        Self {
            sync_sent,
            sync_rcvd: prev.sync_rcvd,
            delay_req_sent,
        }
    }
}

pub struct ResponderDone {
    pub req_to_resp_delta: Timestamp,
    pub resp_to_req_delta: Timestamp,
}

pub enum RequesterState {
    WaitForDelayReq(WaitForDelayReq),
    WaitForDelayReqFollowUp(WaitForDelayReqFollowUp),
}

impl RequesterState {
    pub fn new(sync_sent: Timestamp) -> Self {
        Self::WaitForDelayReq(WaitForDelayReq { sync_sent })
    }
}

pub enum ResponderState {
    WaitForSync(WaitForSync),
    WaitForSyncFollowUp(WaitForSyncFollowUp),
    WaitForDelayResp(WaitForDelayResp),
}

impl ResponderState {
    pub fn new() -> Self {
        Self::WaitForSync(WaitForSync {})
    }
}

pub enum State {
    Requester(RequesterState),
    Responder(ResponderState),
}

impl State {
    pub fn new() -> Self {
        Self::Responder(ResponderState::new())
    }
}
