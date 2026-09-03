// ==================================================================
// R2 REVIEW PROBES (BOOL-11, PR #1520, frozen head 0d285cd3f).
// Appended to crates/profile/tests/bool11_probes.rs for a probe run,
// REVERTED after; this copy is the record. All rows are written in
// terms of the run's own eps/K so they hold on every tolerance leg
// AND under CAD_AMBIGUITY_K variations (no additive 1.0+x rounding in
// any across-channel: with angle(0.0) the departing unit is exactly
// (1,0), so `across == dy` bitwise).
// ==================================================================

/// R2 PROBE 1: the band's edges land exactly where the doctrine puts
/// them, at 1-ulp resolution — accept INCLUSIVE at eps, escalate
/// strictly inside (eps, K*eps), refuse INCLUSIVE at K*eps — and the
/// negative side mirrors. `keps` is spelled `t.k() * t.eps()`, the
/// same arithmetic `linear_band` uses, so the edge is bit-identical.
#[test]
fn r2_probe_band_edges_at_one_ulp() {
    let t = Tol::witness();
    let eps = t.eps();
    let keps = t.k() * t.eps();
    let attempt = |dy: f64| {
        Open.at(p2(0.0, 0.0))
            .angle(0.0, t)
            .unwrap()
            .line(1.0, t)
            .unwrap()
            .continue_to(p2(2.0, dy), t)
    };
    assert!(attempt(eps).is_ok(), "exactly eps must accept (<= zero)");
    assert!(
        matches!(attempt(eps.next_up()), Err(PathError::Escalated { .. })),
        "eps + 1 ulp must escalate"
    );
    assert!(
        matches!(attempt(keps.next_down()), Err(PathError::Escalated { .. })),
        "K*eps - 1 ulp must escalate"
    );
    assert!(
        matches!(
            attempt(keps),
            Err(PathError::ContinuationTargetOffRay { .. })
        ),
        "exactly K*eps must refuse (>= escalate)"
    );
    assert!(attempt(-eps).is_ok(), "the band is two-sided: -eps accepts");
    assert!(
        matches!(
            attempt(-keps),
            Err(PathError::ContinuationTargetOffRay { across, .. }) if across < 0.0
        ),
        "-K*eps refuses with the signed miss"
    );
}

/// R2 PROBE 2: the miss is ABSOLUTE, not angular — the same lateral
/// miss classifies identically under legs nine orders apart, which is
/// the no-lever design's observable content (a threshold that levered
/// on the leg would move with `len`).
#[test]
fn r2_probe_the_threshold_does_not_scale_with_the_leg() {
    let t = Tol::witness();
    let eps = t.eps();
    let keps = t.k() * eps;
    for len in [1e-3, 1.0, 1e6] {
        let attempt = |dy: f64| {
            Open.at(p2(0.0, 0.0))
                .angle(0.0, t)
                .unwrap()
                .line(len, t)
                .unwrap()
                .continue_to(p2(2.0 * len, dy), t)
        };
        assert!(attempt(0.5 * eps).is_ok(), "len={len}: 0.5*eps accepts");
        assert!(
            matches!(
                attempt(2.0 * keps),
                Err(PathError::ContinuationTargetOffRay { .. })
            ),
            "len={len}: 2*K*eps refuses regardless of the leg"
        );
    }
}

/// R2 PROBE 3: escalation probed K-robustly — the GEOMETRIC midpoint
/// sqrt(eps * K*eps) is strictly inside (eps, K*eps) for every legal
/// K > 1 and needs no additive rounding, so this row runs unchanged
/// under CAD_AMBIGUITY_K near its floor.
#[test]
fn r2_probe_escalation_at_the_geometric_midpoint() {
    let t = Tol::witness();
    let mid = (t.eps() * t.k() * t.eps()).sqrt();
    let att = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(p2(2.0, mid), t);
    assert!(
        matches!(att, Err(PathError::Escalated { .. })),
        "the geometric midpoint of the band must escalate: {att:?}"
    );
}

/// R2 PROBE 4: ordering and the along-band. A target both BEHIND and
/// OFF the ray refuses on the lateral miss (the fact the verb is
/// about); a sub-eps step ahead is a degenerate (nonpositive) leg
/// exactly as for line(len); the along extent has its own escalation
/// band too.
#[test]
fn r2_probe_the_along_extent_rides_the_same_band() {
    let t = Tol::witness();
    let eps = t.eps();
    let run = || {
        Open.at(p2(0.0, 0.0))
            .angle(0.0, t)
            .unwrap()
            .line(1.0, t)
            .unwrap()
    };
    assert!(
        matches!(
            run().continue_to(p2(-3.0, 1.0), t),
            Err(PathError::ContinuationTargetOffRay { .. })
        ),
        "behind AND off-ray: the lateral miss wins"
    );
    assert!(
        matches!(
            run().continue_to(p2(1.0 + 0.5 * eps, 0.0), t),
            Err(PathError::NonpositiveLeg { .. })
        ),
        "sub-eps ahead is a degenerate leg"
    );
    let mid = (eps * t.k() * eps).sqrt();
    assert!(
        matches!(
            run().continue_to(p2(1.0 + mid, 0.0), t),
            Err(PathError::Escalated { .. })
        ),
        "the along extent escalates in its band too"
    );
}

/// R2 PROBE 5: the CLOSER rides the same band as the point form. The
/// loop below puts the entry a controlled `dy` off the closing ray
/// (final tip at (1,0) heading exactly +x, entry at (2,dy), so
/// `across == dy` bitwise): at eps it closes with no minted vertex; at
/// K*eps it refuses OffRay before any seam classification.
#[test]
fn r2_probe_the_closer_boundary_is_the_point_forms() {
    let t = Tol::witness();
    let eps = t.eps();
    let keps = t.k() * eps;
    let attempt = |dy: f64| {
        Open.at(p2(2.0, dy))
            .line_to(p2(2.0, 2.0), t)
            .unwrap()
            .line_to(p2(0.0, 2.0), t)
            .unwrap()
            .line_to(p2(0.0, 0.0), t)
            .unwrap()
            .line_to(p2(1.0, 0.0), t)
            .unwrap()
            .continue_to(Start, t)
    };
    let closed = attempt(eps).expect("an entry eps off the closing ray still closes");
    let v = pinned(closed);
    assert_eq!(v.vertices().len(), 5, "no vertex minted at the entry");
    assert!(
        matches!(
            attempt(keps),
            Err(PathError::ContinuationTargetOffRay { .. })
        ),
        "an entry K*eps off the closing ray refuses as the verb's own miss"
    );
}
