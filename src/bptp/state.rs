use crate::timestamp::Timestamp;

pub struct WaitForSyncFollowUp {
    pub sync_rcvd: Timestamp,
}

pub struct WaitForDelayReq {
    pub sync_sent: Timestamp,
}

pub struct WaitForDelayReqFollowUp {
    pub sync_sent: Timestamp,
    pub sync_rcvd: Timestamp,
    pub delay_req_rcvd: Timestamp,
}

pub struct WaitForDelayResp {
    pub sync_sent: Timestamp,
    pub sync_rcvd: Timestamp,
    pub delay_req_sent: Timestamp,
}

pub struct RequesterDone {
    pub req_to_resp_delta: Timestamp,
    pub resp_to_req_delta: Timestamp,
}

pub struct ResponderDone {
    pub req_to_resp_delta: Timestamp,
    pub resp_to_req_delta: Timestamp,
}

pub enum RequesterState {
    Idle,
    WaitForDelayReq(WaitForDelayReq),
    WaitForDelayReqFollowUp(WaitForDelayReqFollowUp),
    Done(RequesterDone),
}

impl RequesterState {
    pub fn new() -> Self {
        Self::Idle
    }
}

impl Default for RequesterState {
    fn default() -> Self {
        Self::new()
    }
}

pub enum ResponderState {
    WaitForSync,
    WaitForSyncFollowUp(WaitForSyncFollowUp),
    WaitForDelayResp(WaitForDelayResp),
    Done(ResponderDone),
}

impl ResponderState {
    pub fn new() -> Self {
        Self::WaitForSync
    }
}

impl Default for ResponderState {
    fn default() -> Self {
        Self::new()
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

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
