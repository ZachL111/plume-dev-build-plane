use plume_dev_build_plane::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 62, slack: 48, drag: 14, confidence: 54 };
    assert_eq!(review_score(case), 184);
    assert_eq!(review_lane(case), "ship");
}
