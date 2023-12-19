#[cfg(test)]
mod tests {
    use crate::{get_threshold, Threshold, ReclaimLoop, ReclaimState};
    use std::collections::HashMap;
    use std::time::Instant;
    use failure::_core::time::Duration;
    use std::path::PathBuf;

    #[test]
    fn test_threshold() {
        let string = "50%";
        let threshold = get_threshold(string).unwrap();
        assert_eq!(threshold, Threshold::Percent(50f64));

        let string = "100";
        let threshold = get_threshold(string).unwrap();
        assert_eq!(threshold, Threshold::Bytes(100));

        let string = "100KB";
        let threshold = get_threshold(string).unwrap();
        assert_eq!(threshold, Threshold::Bytes(100_000));

        let string = "100KiB";
        let threshold = get_threshold(string).unwrap();
        assert_eq!(threshold, Threshold::Bytes(102_400));

        let string = "100MB";
        let threshold = get_threshold(string).unwrap();
        assert_eq!(threshold, Threshold::Bytes(100_000_000));

        let string = "100MiB";
        let threshold = get_threshold(string).unwrap();
        assert_eq!(threshold, Threshold::Bytes(104_857_600));

        let string = "100GB";
        let threshold = get_threshold(string).unwrap();
        assert_eq!(threshold, Threshold::Bytes(100_000_000_000));

        let string = "100GiB";
        let threshold = get_threshold(string).unwrap();
        assert_eq!(threshold, Threshold::Bytes(107_374_182_400));
    }

    #[test]
    fn test_reclaim_loop_cleanup() {
        let reclaim_loop = ReclaimLoop {
            parent: PathBuf::new(),
            interval: 0,
            cooldown: 0,
            threshold: Threshold::Bytes(0),
        };

        let second = Duration::from_secs(1);
        let now = Instant::now();
        let before = now - second;
        let after = now + second;
        let mut states = HashMap::new();

        states.insert(PathBuf::from("never"), ReclaimState{
            last_seen: None, last_reclaimed: None, last_error: None});
        states.insert(PathBuf::from("before"), ReclaimState{
            last_seen: Some(before), last_reclaimed: None, last_error: None});
        states.insert(PathBuf::from("after"), ReclaimState{
            last_seen: Some(after), last_reclaimed: None, last_error: None});

        reclaim_loop.cleanup(&now, &mut states);

        assert!(! states.contains_key(&PathBuf::from("never")));
        assert!(! states.contains_key(&PathBuf::from("before")));
        assert!(states.contains_key(&PathBuf::from("after")));
    }
}
