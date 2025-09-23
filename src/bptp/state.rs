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

impl RequesterDone {
    pub fn from_previous(
        prev: WaitForDelayReqFollowUp,
        delay_req_sent: Timestamp,
    ) -> Self {
        let req_to_resp = prev.sync_rcvd.time_ns - prev.sync_sent.time_ns;
        let resp_to_req = prev.delay_req_rcvd.time_ns - delay_req_sent.time_ns;
        Self {
            req_to_resp_delta: Timestamp {
                time_ns: req_to_resp,
            },
            resp_to_req_delta: Timestamp {
                time_ns: resp_to_req,
            },
        }
    }
}

pub struct ResponderDone {
    pub req_to_resp_delta: Timestamp,
    pub resp_to_req_delta: Timestamp,
}

impl ResponderDone {
    pub fn from_previous(
        prev: WaitForDelayResp,
        delay_req_rcvd: Timestamp,
    ) -> Self {
        let req_to_resp = prev.sync_rcvd.time_ns - prev.sync_sent.time_ns;
        let resp_to_req = delay_req_rcvd.time_ns - prev.delay_req_sent.time_ns;
        Self {
            req_to_resp_delta: Timestamp {
                time_ns: req_to_resp,
            },
            resp_to_req_delta: Timestamp {
                time_ns: resp_to_req,
            },
        }
    }
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
