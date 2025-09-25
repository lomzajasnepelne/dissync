use std::{sync::mpsc, thread};

use dissync::{
    bptp::{
        msg::{self, ReqToRespPld, RespToReqPld},
        state::{self, RequesterState, ResponderState},
    },
    timestamp::Timestamp,
};

fn example_requester(
    tx: mpsc::Sender<msg::ReqToRespPld>,
    rx: mpsc::Receiver<msg::RespToReqPld>,
) {
    let mut state = RequesterState::new();
    loop {
        match &state {
            RequesterState::Idle => {
                let sync = msg::Sync;
                let pld = msg::ReqToRespPld::Sync(sync);
                tx.send(pld).unwrap();
                let sync_sent = Timestamp { time_ns: 0 };
                let sync_fup = msg::SyncFollowUp { sync_sent };
                let pld = msg::ReqToRespPld::SyncFollowUp(sync_fup);
                tx.send(pld).unwrap();
                let state_info = state::WaitForDelayReq { sync_sent };
                state = RequesterState::WaitForDelayReq(state_info);
            }
            RequesterState::WaitForDelayReq(state_info) => {
                let pld = rx.recv().unwrap();
                let delay_req_rcvd = Timestamp { time_ns: 3 };
                let delay_req = match pld {
                    RespToReqPld::DelayReq(d) => d,
                    _ => panic!(),
                };
                let state_info = state::WaitForDelayReqFollowUp {
                    sync_sent: state_info.sync_sent,
                    sync_rcvd: delay_req.sync_rcvd,
                    delay_req_rcvd,
                };
                state = RequesterState::WaitForDelayReqFollowUp(state_info);
            }
            RequesterState::WaitForDelayReqFollowUp(state_info) => {
                let rx_pld = rx.recv().unwrap();
                let delay_req_fup = match rx_pld {
                    RespToReqPld::DelayReqFollowUp(d) => d,
                    _ => panic!(),
                };
                let delay_resp = msg::DelayResp {
                    delay_req_rcvd: state_info.delay_req_rcvd,
                };
                let tx_pld = ReqToRespPld::DelayResp(delay_resp);
                tx.send(tx_pld).unwrap();
                let req_to_resp_delta =
                    state_info.sync_rcvd.time_ns - state_info.sync_sent.time_ns;
                let resp_to_req_delta = state_info.delay_req_rcvd.time_ns
                    - delay_req_fup.delay_req_sent.time_ns;
                let state_info = state::RequesterDone {
                    req_to_resp_delta: Timestamp {
                        time_ns: req_to_resp_delta,
                    },
                    resp_to_req_delta: Timestamp {
                        time_ns: resp_to_req_delta,
                    },
                };
                state = RequesterState::Done(state_info);
            }
            RequesterState::Done(state_info) => {
                println!(
                    "Requester calculated responder to requester delta: {} ns",
                    state_info.resp_to_req_delta.time_ns
                );
                println!(
                    "Requester calculated requester to responder delta: {} ns",
                    state_info.req_to_resp_delta.time_ns
                );
                return;
            }
        }
    }
}

fn example_responder(
    tx: mpsc::Sender<msg::RespToReqPld>,
    rx: mpsc::Receiver<msg::ReqToRespPld>,
) {
    let mut state = state::ResponderState::new();
    loop {
        match &state {
            ResponderState::WaitForSync => {
                let pld = rx.recv().unwrap();
                let sync_rcvd = Timestamp { time_ns: 1 };
                let _sync = match pld {
                    msg::ReqToRespPld::Sync(sync) => sync,
                    _ => panic!(),
                };
                let state_info = state::WaitForSyncFollowUp { sync_rcvd };
                state = ResponderState::WaitForSyncFollowUp(state_info);
            }
            ResponderState::WaitForSyncFollowUp(state_info) => {
                let rx_pld = rx.recv().unwrap();
                let sync_fup = match rx_pld {
                    msg::ReqToRespPld::SyncFollowUp(sync_fup) => sync_fup,
                    _ => panic!(),
                };
                let delay_req = msg::DelayReq {
                    sync_rcvd: state_info.sync_rcvd,
                };
                let tx_pld = msg::RespToReqPld::DelayReq(delay_req);
                tx.send(tx_pld).unwrap();
                let delay_req_sent = Timestamp { time_ns: 2 };
                let delay_req_fup = msg::DelayReqFollowUp { delay_req_sent };
                let tx_pld = msg::RespToReqPld::DelayReqFollowUp(delay_req_fup);
                tx.send(tx_pld).unwrap();
                let state_info = state::WaitForDelayResp {
                    sync_sent: sync_fup.sync_sent,
                    sync_rcvd: state_info.sync_rcvd,
                    delay_req_sent,
                };
                state = ResponderState::WaitForDelayResp(state_info);
            }
            ResponderState::WaitForDelayResp(state_info) => {
                let pld = rx.recv().unwrap();
                let delay_resp = match pld {
                    msg::ReqToRespPld::DelayResp(d) => d,
                    _ => panic!(),
                };
                let req_to_resp_delta =
                    state_info.sync_rcvd.time_ns - state_info.sync_sent.time_ns;
                let resp_to_req_delta = delay_resp.delay_req_rcvd.time_ns
                    - state_info.delay_req_sent.time_ns;
                let state_info = state::ResponderDone {
                    req_to_resp_delta: Timestamp {
                        time_ns: req_to_resp_delta,
                    },
                    resp_to_req_delta: Timestamp {
                        time_ns: resp_to_req_delta,
                    },
                };
                state = ResponderState::Done(state_info);
            }
            ResponderState::Done(state_info) => {
                println!(
                    "Responder calculated responder to requester delta: {} ns",
                    state_info.resp_to_req_delta.time_ns
                );
                println!(
                    "Responder calculated requester to responder delta: {} ns",
                    state_info.req_to_resp_delta.time_ns
                );
                return;
            }
        }
    }
}

fn example_exchange() {
    let (req_to_resp_tx, req_to_resp_rx) = mpsc::channel();
    let (resp_to_req_tx, resp_to_req_rx) = mpsc::channel();
    thread::scope(|s| {
        s.spawn(|| example_requester(req_to_resp_tx, resp_to_req_rx));
        s.spawn(|| example_responder(resp_to_req_tx, req_to_resp_rx));
    })
}

fn main() {
    example_exchange();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_exchange_does_not_panic() {
        example_exchange();
    }
}
