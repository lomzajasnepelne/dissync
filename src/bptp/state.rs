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

impl WaitForDelayReqFollowUp {
    pub fn from_previous(
        prev: WaitForDelayReq,
        sync_rcvd: Timestamp,
        delay_req_rcvd: Timestamp,
    ) -> Self {
        Self { sync_sent: prev.sync_sent, sync_rcvd, delay_req_rcvd }
    }
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
    WaitForDelayReq(WaitForDelayReq),
    WaitForDelayReqFollowUp(WaitForDelayReqFollowUp),
    Done(RequesterDone),
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
    Done(ResponderDone),
}

impl ResponderState {
    pub fn new() -> Self {
        Self::WaitForSync(WaitForSync {})
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
