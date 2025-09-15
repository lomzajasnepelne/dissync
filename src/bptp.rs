mod msg;
pub mod state;

#[cfg(test)]
mod tests {
    use crate::timestamp::Timestamp;

    use super::*;

    #[test]
    fn responder_has_two_deltas() {
        let responder = state::WaitForSync;
        let _sync = msg::Sync;
        let sync_rcvd = Timestamp { time_ns: 1 };
        let responder =
            state::WaitForSyncFollowUp::from_previous(responder, sync_rcvd);
        let sync_fup = msg::SyncFollowUp {
            sync_sent: Timestamp { time_ns: 0 },
        };
        let delay_req_sent = Timestamp { time_ns: 3 };
        let responder = state::WaitForDelayResp::from_previous(
            responder,
            sync_fup.sync_sent,
            delay_req_sent,
        );
    }
}
