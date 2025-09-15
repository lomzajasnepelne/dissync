use dissync::{bptp, timestamp::Timestamp};

fn example_exchange() {
    let wait_for_sync = bptp::state::WaitForSync;
    let sync_rcvd = Timestamp { time_ns: 0 };
    let wait_for_sync_fup = bptp::state::WaitForSyncFollowUp::from_previous(
        wait_for_sync,
        sync_rcvd,
    );
    let sync_sent = Timestamp { time_ns: 1 };
    let delay_req_sent = Timestamp { time_ns: 2 };
    let _wait_for_delay_resp = bptp::state::WaitForDelayResp::from_previous(
        wait_for_sync_fup,
        sync_sent,
        delay_req_sent,
    );
}

fn main() {
    example_exchange();
}
