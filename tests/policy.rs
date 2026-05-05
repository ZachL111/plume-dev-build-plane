use plume_dev_build_plane::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 65, capacity: 103, latency: 18, risk: 18, weight: 8 };
    assert_eq!(score(signal), 121);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 60, capacity: 104, latency: 14, risk: 24, weight: 11 };
    assert_eq!(score(signal), 116);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 84, capacity: 71, latency: 18, risk: 7, weight: 10 };
    assert_eq!(score(signal), 179);
    assert_eq!(classify(signal), "accept");
}
