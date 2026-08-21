//! Adversarial e2e review artifact for M0 PR 3 (trilean predicates,
//! 2026-07-15/16), salvaged from the M0 orchestrator session scratchpad
//! and promoted into CI 2026-07-16 (archived at
//! `references/review-artifacts-m0/pr3-review-demos/`). These are
//! **independent derivations** — do not "simplify" them to match shipped
//! fixtures; the independence is the regression value.
//!
//! Salvage adaptations: every band here is built **purely** via
//! `Band::new(1e-9, 1e-8)` — the same numbers the review's default-ε run
//! used through `Band::linear(Tol::witness())` — so this binary never touches the
//! global tolerance (funnel-test discipline, `src/tolerance.rs`) and the
//! probe margins stay meaningful under the CI ε matrix. The review's
//! `envband` demo (linear/angular constructors tracking the global) is
//! dropped as a duplicate of the shipped `tests/band_tolerance.rs`; its
//! `Band::angular()` probes predate the D4 revision that replaced the
//! global angular tolerance with per-call-site `Band::angular_at(Tol::witness(), lever)`.
//!
//! # Leak checks (compile-fail probes, documentary)
//!
//! The review's `leakchecks/` crate proved the pre-decision discipline is
//! structural. Kept in `compile_fail` doctest form to match the repo
//! idiom (see `crates/topo/src/entity.rs`); note rustdoc does not run
//! integration-test doctests, so these document the review's findings
//! rather than executing as gates.
//!
//! LEAK CHECK 1 — generic `Real` code cannot compare:
//!
//! ```compile_fail,E0369
//! use geom_core::Real;
//! fn f<T: Real>(a: T, b: T) -> bool { a < b }
//! ```
//!
//! LEAK CHECK 2 — generic `Real` code cannot call `sign_within`
//! (`Decide` is a separate trait, not implied by `Real`):
//!
//! ```compile_fail,E0599
//! use geom_core::{Band, Real, Decide as _};
//! fn f<T: Real>(a: T, band: Band) -> bool { a.sign_within(band).is_ok() }
//! ```
//!
//! ERGONOMICS PROBE — construction code returning
//! `Result<_, Indeterminate>` cannot `?` a `Band::linear(Tol::witness())` failure:
//! `BandError` does not convert into `Indeterminate` (two error types on
//! the same happy path). Recorded as a finding; the ratified calling
//! convention is that the *operation's* error enum absorbs `BandError`
//! at operation entry (see `Band::linear`'s docs), so the conversion is
//! deliberately absent:
//!
//! ```compile_fail,E0277
//! use geom_core::{Band, Decide, Indeterminate, Sign};
//! fn classify(m: f64) -> Result<Sign, Indeterminate> {
//!     let band = Band::linear(Tol::witness())?; // BandError does not become Indeterminate
//!     m.sign_within(band)
//! }
//! ```
//!
//! LEAK CHECKS 3 and 4 (things that deliberately DO compile — the
//! post-decision diagnostic leak and the concrete-f64 DIY predicate) are
//! pinned as runtime tests below.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, BandError, BandField, DEFAULT_K, Decide, Indeterminate, MarginDiag, Sign};

/// The review-run band: (ε, K·ε) at the ratified default ε = 1e-9,
/// constructed purely (see the module docs).
fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

// ---------------------------------------------------------------------
// The review's `abuse` demo: boundary table, adversarial Band::new,
// Display messages, error objects.
// ---------------------------------------------------------------------

#[test]
fn boundary_table_from_outside_the_crate() {
    let b = band();
    // Closed coincidence region: |m| <= zero.
    assert_eq!(0.0f64.sign_within(b), Ok(Sign::Zero));
    assert_eq!((-0.0f64).sign_within(b), Ok(Sign::Zero));
    assert_eq!(5e-324f64.sign_within(b), Ok(Sign::Zero)); // min subnormal
    assert_eq!(1e-9f64.sign_within(b), Ok(Sign::Zero)); // eps exactly
    assert_eq!((-1e-9f64).sign_within(b), Ok(Sign::Zero)); // -eps exactly
    // Open ambiguity band: strictly between the thresholds.
    for m in [1e-9f64.next_up(), 5e-9, 1e-8f64.next_down()] {
        let e = m.sign_within(b).unwrap_err();
        assert_eq!(e.margin, MarginDiag::Value(m));
        assert_eq!(e.band, b);
        assert_eq!(e.predicate, None);
        let e = (-m).sign_within(b).unwrap_err();
        assert_eq!(e.margin, MarginDiag::Value(-m));
    }
    // Closed definite regions: |m| >= escalate, infinities included.
    assert_eq!(1e-8f64.sign_within(b), Ok(Sign::Positive)); // Keps exactly
    assert_eq!((-1e-8f64).sign_within(b), Ok(Sign::Negative));
    assert_eq!(f64::INFINITY.sign_within(b), Ok(Sign::Positive));
    assert_eq!(f64::NEG_INFINITY.sign_within(b), Ok(Sign::Negative));
    // NaN of either sign bit: Invalid, never a sign.
    for nan in [f64::NAN, -f64::NAN] {
        let e = nan.sign_within(b).unwrap_err();
        assert_eq!(e.margin, MarginDiag::Invalid);
    }
}

#[test]
fn adversarial_band_construction() {
    // Thresholds must be finite and strictly positive; zero checked first.
    // (NaN rejections are matched structurally: MarginDiag-style NaN
    // payloads make the error non-reflexive under ==.)
    for zero in [0.0, -0.0, -1e-9, f64::NAN] {
        assert!(
            matches!(
                Band::new(zero, 1e-8),
                Err(BandError::InvalidValue {
                    field: BandField::Zero,
                    value,
                }) if value.to_bits() == zero.to_bits()
            ),
            "zero = {zero:?}"
        );
    }
    for escalate in [f64::NAN, f64::INFINITY] {
        assert!(
            matches!(
                Band::new(1e-9, escalate),
                Err(BandError::InvalidValue {
                    field: BandField::Escalate,
                    value,
                }) if value.to_bits() == escalate.to_bits()
            ),
            "escalate = {escalate:?}"
        );
    }
    // Both bad: zero is validated first.
    assert_eq!(
        Band::new(-1.0, f64::NAN),
        Err(BandError::InvalidValue {
            field: BandField::Zero,
            value: -1.0,
        })
    );
    // Equal or inverted thresholds: the open band is empty over the reals.
    assert_eq!(
        Band::new(1e-9, 1e-9),
        Err(BandError::Empty {
            zero: 1e-9,
            escalate: 1e-9,
        })
    );
    assert_eq!(
        Band::new(1e-8, 1e-9),
        Err(BandError::Empty {
            zero: 1e-8,
            escalate: 1e-9,
        })
    );
    // Degenerate-but-accepted: adjacent floats (hairline, see below),
    // subnormal pairs, and a maximal escalate are all valid open bands.
    for (zero, escalate) in [(1e-9, 1e-9f64.next_up()), (5e-324, 1e-320), (1.0, f64::MAX)] {
        let b = Band::new(zero, escalate).unwrap();
        assert_eq!((b.zero(), b.escalate()), (zero, escalate));
    }
}

/// The open band (1e-9, next_up(1e-9)) contains NO representable f64 —
/// every f64 input classifies definite or zero; Indeterminate is
/// unreachable at f64 (the interval instantiation can still refuse: an
/// enclosure straddling the hairline is not a representable point).
#[test]
fn hairline_band_admits_no_f64_indeterminate() {
    let hairline = Band::new(1e-9, 1e-9f64.next_up()).unwrap();
    assert_eq!(1e-9f64.sign_within(hairline), Ok(Sign::Zero));
    assert_eq!(1e-9f64.next_up().sign_within(hairline), Ok(Sign::Positive));
}

#[test]
fn display_messages_and_error_objects() {
    let b = band();
    // Bare escalation names no predicate.
    let bare = 5e-9f64.sign_within(b).unwrap_err();
    let msg = bare.to_string();
    assert!(msg.starts_with("sign indeterminate: "), "got: {msg}");
    assert!(msg.contains("ambiguity band"), "got: {msg}");
    // A named predicate leads the message.
    let named = 5e-9f64
        .sign_within(b)
        .unwrap_err()
        .with_predicate("side_of_plane");
    assert_eq!(named.predicate, Some("side_of_plane"));
    let msg = named.to_string();
    assert!(
        msg.starts_with("predicate 'side_of_plane' indeterminate: "),
        "got: {msg}"
    );
    // The invalid-margin message is the poison one.
    let nan = f64::NAN
        .sign_within(b)
        .unwrap_err()
        .with_predicate("transversality");
    let msg = nan.to_string();
    assert!(msg.contains("invalid"), "got: {msg}");
    // Subnormal thresholds print without panicking.
    let tiny = Band::new(5e-324, 5e-323).unwrap();
    let sub = 2e-323f64.sign_within(tiny).unwrap_err();
    assert!(sub.to_string().contains("ambiguity band"));
    // Both error types are std errors usable as trait objects.
    let boxed: Box<dyn std::error::Error> = Box::new(named);
    assert!(boxed.to_string().contains("side_of_plane"));
    let boxed_band: Box<dyn std::error::Error> = Box::new(BandError::Empty {
        zero: 1.0,
        escalate: 0.5,
    });
    assert!(boxed_band.to_string().contains("invalid band"));
}

#[test]
fn sign_and_margin_diag_consumer_semantics() {
    // The ratified default K is 10 (since M2 PR 7 the run value is
    // ε-style configuration, `Tol::witness().get().k`, defaulting to this).
    assert_eq!(DEFAULT_K, 10.0);
    // Sign equality/copy semantics as a consumer sees them.
    let s = Sign::Positive;
    assert_eq!(s.flip().flip(), s);
    assert!(matches!(1e-9f64.sign_within(band()), Ok(Sign::Zero)));
    // MarginDiag exhaustive match forces consumers to handle Invalid; the
    // in-band margin is recoverable for diagnostics.
    match 5e-9f64.sign_within(band()).unwrap_err().margin {
        MarginDiag::Value(v) => assert_eq!(v, 5e-9),
        MarginDiag::Enclosure { .. } | MarginDiag::Invalid => {
            panic!("f64 in-band margin must be MarginDiag::Value")
        }
    }
}

// ---------------------------------------------------------------------
// The review's `consumers` demo: named predicates as the geometry layers
// would actually write them, plus a toy construction that branches on the
// trichotomy and propagates Indeterminate with `?` (the Q1 pattern).
// ---------------------------------------------------------------------

/// Which side of an (implicitly given) plane a point lies on, from its
/// signed distance. The predicate takes T: Decide — evaluation code that
/// computed the distance was Real-generic and could not branch.
fn side_of_plane<T: Decide>(signed_dist: T, band: Band) -> Result<Sign, Indeterminate> {
    signed_dist
        .sign_within(band)
        .map_err(|e| e.with_predicate("side_of_plane"))
}

/// Transversality: is the angle margin between two surface normals large
/// enough to treat their intersection as transversal? The natural
/// formulation classifies the margin itself against an angular band.
fn is_transversal<T: Decide>(angle_margin: T, band: Band) -> Result<bool, Indeterminate> {
    angle_margin
        .sign_within(band)
        .map(|s| !s.is_zero())
        .map_err(|e| e.with_predicate("transversality"))
}

/// Segment-endpoint coincidence from a distance. Note: distance is
/// nonnegative, so Negative is impossible — the trichotomy is wider than
/// this predicate needs.
fn endpoints_coincide<T: Decide>(dist: T, band: Band) -> Result<bool, Indeterminate> {
    dist.sign_within(band)
        .map(Sign::is_zero)
        .map_err(|e| e.with_predicate("endpoint_coincidence"))
}

/// Toy topology outcome of splitting an edge by a plane.
#[derive(Debug, PartialEq)]
enum EdgeSplit {
    /// Edge fully below: keep in the "below" shell.
    Below,
    /// Edge fully above.
    Above,
    /// One endpoint on the plane: vertex-on-face case, no new vertex.
    TouchAtEnd,
    /// Proper crossing: split the edge, insert a vertex.
    Crossed,
}

/// Classify a segment (given the endpoint signed distances) against a
/// plane — the shape of real boolean-classification inner loops.
fn split_edge_by_plane<T: Decide>(d0: T, d1: T, band: Band) -> Result<EdgeSplit, Indeterminate> {
    let s0 = side_of_plane(d0, band)?;
    let s1 = side_of_plane(d1, band)?;
    Ok(match (s0, s1) {
        (Sign::Negative, Sign::Negative) | (Sign::Negative, Sign::Zero) => EdgeSplit::Below,
        (Sign::Positive, Sign::Positive) | (Sign::Positive, Sign::Zero) => EdgeSplit::Above,
        (Sign::Zero, _) => EdgeSplit::TouchAtEnd,
        (Sign::Negative, Sign::Positive) | (Sign::Positive, Sign::Negative) => EdgeSplit::Crossed,
    })
}

#[test]
fn named_predicates_and_question_mark_propagation() {
    let lin = band();
    // An angular band as a predicate would derive it per call site
    // (adaptation: the review used the since-removed global angular
    // tolerance; the composition pattern is unchanged).
    let ang = Band::new(1e-8, 1e-7).unwrap();

    // side_of_plane over the three regions.
    assert_eq!(side_of_plane(-0.25f64, lin), Ok(Sign::Negative));
    assert_eq!(side_of_plane(3e-10f64, lin), Ok(Sign::Zero));
    let e = side_of_plane(4e-9f64, lin).unwrap_err();
    assert_eq!(e.predicate, Some("side_of_plane"));

    // transversality.
    assert_eq!(is_transversal(0.7f64, ang), Ok(true));
    let e = is_transversal(3.0e-8f64, ang).unwrap_err();
    assert_eq!(e.predicate, Some("transversality"));

    // endpoint coincidence.
    assert_eq!(endpoints_coincide(5e-10f64, lin), Ok(true));
    assert_eq!(endpoints_coincide(1e-3f64, lin), Ok(false));

    // The construction, on all paths.
    assert_eq!(
        split_edge_by_plane(-1.0f64, -2.0, lin),
        Ok(EdgeSplit::Below)
    );
    assert_eq!(
        split_edge_by_plane(-1.0f64, 1.0, lin),
        Ok(EdgeSplit::Crossed)
    );
    assert_eq!(
        split_edge_by_plane(1e-10f64, 1.0, lin),
        Ok(EdgeSplit::TouchAtEnd)
    );
    // The escalation path: second endpoint in the band; `?` propagates
    // the leaf's name.
    let e = split_edge_by_plane(-1.0f64, 5e-9, lin).unwrap_err();
    assert_eq!(e.predicate, Some("side_of_plane"));
    assert_eq!(e.margin, MarginDiag::Value(5e-9));
}

// ---------------------------------------------------------------------
// The review's `composition` demo: composing multiple sign tests —
// "inside triangle" (three orientation tests) and "segments intersect"
// (four orientation tests) — early-exit vs collect-all patterns.
// ---------------------------------------------------------------------

type P = [f64; 2];

/// 2-D orientation margin: twice the signed area of (a, b, c).
fn orient2d_margin(a: P, b: P, c: P) -> f64 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

fn orient2d(a: P, b: P, c: P, band: Band) -> Result<Sign, Indeterminate> {
    orient2d_margin(a, b, c)
        .sign_within(band)
        .map_err(|e| e.with_predicate("orient2d"))
}

/// PATTERN A — early exit with `?`. The first indeterminate orientation
/// aborts the whole classification.
#[derive(Debug, PartialEq)]
enum TriLocation {
    Inside,
    Outside,
    OnBoundary,
}

fn locate_in_triangle(p: P, tri: [P; 3], band: Band) -> Result<TriLocation, Indeterminate> {
    let s0 = orient2d(tri[0], tri[1], p, band)?;
    let s1 = orient2d(tri[1], tri[2], p, band)?;
    let s2 = orient2d(tri[2], tri[0], p, band)?;
    let signs = [s0, s1, s2];
    Ok(
        if signs.iter().any(|s| s.is_positive()) && signs.iter().any(|s| s.is_negative()) {
            TriLocation::Outside
        } else if signs.iter().any(|s| s.is_zero()) {
            TriLocation::OnBoundary
        } else {
            TriLocation::Inside
        },
    )
}

/// PATTERN B — collect-all: every marginal test is reported (e.g.
/// diagnostics for the subdivision driver). This is what it costs today
/// without a combinator.
fn segments_intersect_collect(
    (a, b): (P, P),
    (c, d): (P, P),
    band: Band,
) -> Result<bool, Vec<Indeterminate>> {
    let results = [
        orient2d(a, b, c, band),
        orient2d(a, b, d, band),
        orient2d(c, d, a, band),
        orient2d(c, d, b, band),
    ];
    let mut errs = Vec::new();
    let mut signs = Vec::new();
    for r in results {
        match r {
            Ok(s) => signs.push(s),
            Err(e) => errs.push(e),
        }
    }
    if !errs.is_empty() {
        return Err(errs);
    }
    // Proper-intersection test (ignoring collinear touching subtleties —
    // this demo is about composition shape, not segment-intersection
    // completeness).
    let straddles1 = signs[0] != signs[1] && !signs[0].is_zero() && !signs[1].is_zero();
    let straddles2 = signs[2] != signs[3] && !signs[2].is_zero() && !signs[3].is_zero();
    Ok(straddles1 && straddles2)
}

#[test]
fn orientation_composition_early_exit_and_collect_all() {
    let b = band();
    let tri: [P; 3] = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]];

    assert_eq!(
        locate_in_triangle([0.25, 0.25], tri, b),
        Ok(TriLocation::Inside)
    );
    assert_eq!(
        locate_in_triangle([2.0, 2.0], tri, b),
        Ok(TriLocation::Outside)
    );
    // A point 1e-10 above the base edge: within the coincidence threshold.
    assert_eq!(
        locate_in_triangle([0.5, 1e-10], tri, b),
        Ok(TriLocation::OnBoundary)
    );
    // A sliver above the base edge: in-band margin, early exit names the leaf.
    let e = locate_in_triangle([0.5, 4.0e-9], tri, b).unwrap_err();
    assert_eq!(e.predicate, Some("orient2d"));

    // Segments: clean cross, clean miss, marginal (near-collinear).
    let cross = segments_intersect_collect(([0.0, 0.0], [1.0, 1.0]), ([0.0, 1.0], [1.0, 0.0]), b);
    assert_eq!(cross, Ok(true));
    let miss = segments_intersect_collect(([0.0, 0.0], [1.0, 0.0]), ([0.0, 1.0], [1.0, 1.0]), b);
    assert_eq!(miss, Ok(false));
    // Two nearly-collinear segments: every orientation margin is 2e-9,
    // inside the band — the collect-all pattern reports all four.
    let marginal = segments_intersect_collect(
        ([0.0, 0.0], [1.0, 3.0e-9]),
        ([0.0, 2.0e-9], [1.0, 5.0e-9]),
        b,
    )
    .unwrap_err();
    assert_eq!(marginal.len(), 4);
    for e in &marginal {
        assert_eq!(e.predicate, Some("orient2d"));
        assert!(matches!(e.margin, MarginDiag::Value(v) if v.abs() == 2e-9));
    }
}

// ---------------------------------------------------------------------
// The review's `sliver` demo: the K = 10 sliver-band user experience —
// a boss edge a controlled gap away from a plate edge; sliver gaps fail
// construction with an actionable error.
// ---------------------------------------------------------------------

/// The toy operation: classify the boss edge against the plate edge —
/// coincident (merge the edges), clear (keep both faces), or sliver.
fn classify_gap(gap: f64, band: Band) -> Result<&'static str, Indeterminate> {
    let sign = gap
        .sign_within(band)
        .map_err(|e| e.with_predicate("face_gap_classification"))?;
    Ok(match sign {
        Sign::Zero => "coincident",
        Sign::Positive => "clear",
        Sign::Negative => "overlap",
    })
}

#[test]
fn sliver_band_user_experience() {
    let b = band();
    // The user sweeps a design parameter: the gap between boss and edge.
    assert_eq!(classify_gap(1e-3, b), Ok("clear"));
    assert_eq!(classify_gap(1e-6, b), Ok("clear"));
    assert_eq!(classify_gap(5e-8, b), Ok("clear"));
    assert_eq!(classify_gap(8e-10, b), Ok("coincident"));
    assert_eq!(classify_gap(0.0, b), Ok("coincident"));
    // The sliver: construction fails, and the error a user reads carries
    // both thresholds (move the boss >= escalate away, or make it
    // coincident within zero).
    let e = classify_gap(5e-9, b).unwrap_err();
    assert_eq!(e.predicate, Some("face_gap_classification"));
    assert_eq!(e.band.zero(), 1e-9);
    assert_eq!(e.band.escalate(), 1e-8);
    let msg = e.to_string();
    assert!(msg.contains("face_gap_classification"), "got: {msg}");
    // The unified sub-ε_input recourse (two-tolerance principle, D4 ¶1
    // addendum): pinned as a fragment of the shared carrier const.
    assert_eq!(
        msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
        1,
        "got: {msg}"
    );
}

// ---------------------------------------------------------------------
// The review's leak checks 3 and 4: things that deliberately DO compile.
// ---------------------------------------------------------------------

/// LEAK CHECK 3: the post-decision diagnostic leak — a consumer can
/// branch on the raw margin recovered from Indeterminate, implementing
/// their own guess and defeating escalation (the anti-pattern the
/// MarginDiag docs warn against; diagnostic honesty makes it possible).
fn best_guess(m: f64, band: Band) -> Sign {
    match m.sign_within(band) {
        Ok(s) => s,
        Err(e) => match e.margin {
            // Silently "resolving" the ambiguity band — the anti-pattern.
            MarginDiag::Value(v) => {
                if v > 0.0 {
                    Sign::Positive
                } else {
                    Sign::Negative
                }
            }
            MarginDiag::Enclosure { .. } | MarginDiag::Invalid => Sign::Zero,
        },
    }
}

/// LEAK CHECK 4: Band accessors give tolerance-derived f64s that concrete
/// code can compare against directly, bypassing sign_within entirely —
/// pre-decision scalar->bool for concrete f64.
fn diy_predicate(m: f64, band: Band) -> bool {
    m.abs() <= band.zero()
}

#[test]
fn documented_post_decision_leaks_still_compile() {
    let b = band();
    assert_eq!(best_guess(5e-9, b), Sign::Positive);
    assert_eq!(best_guess(-5e-9, b), Sign::Negative);
    assert!(diy_predicate(5e-10, b));
    assert!(!diy_predicate(5e-9, b));
}
