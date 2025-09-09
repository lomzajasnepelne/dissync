#[derive(Clone, Copy)]
pub struct Timestamp {
    pub time_ns: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_doesnt_overflow_for_100_years() {
        let ts = Timestamp {
            time_ns: core::i64::MAX,
        };
        let secs = ts.time_ns / 1_000_000_000;
        let hours = secs / 3600;
        let days = hours / 24;
        let years = days / 365;
        assert!(years > 100);
    }
}
