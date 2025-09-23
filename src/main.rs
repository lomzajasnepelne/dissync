use std::{sync::mpsc, thread};

use dissync::{
    bptp::{
        self,
        msg::{self, Msg, NodeId, RespToReqPld},
        state::{self, RequesterState, ResponderState, WaitForDelayResp},
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
                return;
            }
            RequesterState::Done(state_info) => {
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
            ResponderState::WaitForDelayResp(_) => {
                return;
            }
            ResponderState::Done(_) => {
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
