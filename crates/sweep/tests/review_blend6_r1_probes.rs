//! **BLEND-6 R1 review probes** — the verb-neutral contract, held
//! red-capably over the WHOLE refusal battery, not only the arms the
//! chamfer fixtures reach.
//!
//! The review's mutation pass found that `blend6_verb_vocab.rs`'s
//! purity rows guard exactly the arms its fixtures reach: a shared arm
//! outside that set (`UnsupportedChain` was the witness) can regain a
//! hard `"fillet chain: "` prefix and 1673 sweep+editor-core tests
//! stay green. The contract those rows execute — mod.rs's
//! "Verb-neutral by contract: no arm here names a verb" — is stated
//! over EVERY arm, and `recourse_tests::seeds()` already renders every
//! arm; what was missing is the verb assertion over those renders.
//! Probe 1 supplies it.
//!
//! Probe 2 pins what a fillet caller reads at a nonpositive size — the
//! door asymmetry the PR discloses (`NonpositiveSize` is minted at the
//! chamfer door only) — so the current nonsense sentence is a measured
//! fact rather than a rumor, and the row goes red the day the fillet
//! door gains the check (at which point it should flip to assert the
//! `NonpositiveSize` refusal, as the chamfer's own row does).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, BandError, Indeterminate, MarginDiag, Tol};
use sweep::blend::build::fillet_edges;
use sweep::blend::{BlendError, BlendKind, BlendRefusal, BlendSite, Convexity, CornerConfig};
use sweep::test_support::cube;
use topo::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};

/// One value of every `BlendError` variant, in declaration order —
/// the same shape as `recourse_tests::seeds()` (which is `#[cfg(test)]`
/// inside the crate and not importable here; a divergence shows up as
/// an arm this list misses when the enum grows, exactly as it does for
/// the in-crate list).
fn seeds() -> Vec<BlendError> {
    let band = Band::new(1e-9, 1e-6).expect("a band");
    vec![
        BlendError::Band(BandError::Empty {
            zero: 1.0,
            escalate: 0.5,
        }),
        BlendError::ChainNotConnected {
            edge: EdgeKey::default(),
        },
        BlendError::RadiusHeadroom {
            face: FaceKey::default(),
            margin: -1e-3,
            radius: 0.5,
        },
        BlendError::FaceClearanceUncertified {
            face: FaceKey::default(),
            margin: -1e-3,
            gap: 0.2,
            cross_chain: false,
        },
        BlendError::FaceClearanceUncertified {
            face: FaceKey::default(),
            margin: -1e-3,
            gap: 0.2,
            cross_chain: true,
        },
        BlendError::TangentialEdge {
            edge: EdgeKey::default(),
            margin: 0.0,
        },
        BlendError::SpineIrregular {
            margin: -1e-3,
            radius: 0.5,
        },
        BlendError::ChainNotG1 {
            vertex: VertexKey::default(),
            margin: -1e-3,
            arm: 0.5,
        },
        BlendError::ConvexitySignFlip {
            edge: EdgeKey::default(),
            margin: -1e-3,
            chain: Convexity::Convex,
        },
        BlendError::UnsupportedCorner {
            vertex: VertexKey::default(),
            corner: CornerConfig::NEdgeVertex { valence: 4 },
            policy: CornerConfig::NEdgeVertex { valence: 4 }.policy(),
        },
        BlendError::UnsupportedCorner {
            vertex: VertexKey::default(),
            corner: CornerConfig::SeamVertex,
            policy: CornerConfig::SeamVertex.policy(),
        },
        BlendError::SpineUnsupported {
            edge: EdgeKey::default(),
            supports: "a support pair with no analytic arm",
        },
        BlendError::ChamferArmUnsupported {
            edge: EdgeKey::default(),
            supports: "non-(plane–plane)",
        },
        BlendError::Escalated {
            site: BlendSite::Chain,
            source: Indeterminate {
                margin: MarginDiag::Value(0.0),
                band,
                predicate: Some("fillet3_ring_clearance"),
            },
        },
        BlendError::Escalated {
            site: BlendSite::Chain,
            source: Indeterminate {
                margin: MarginDiag::Value(0.0),
                band,
                predicate: Some("fillet3_radius_headroom"),
            },
        },
        BlendError::RepeatedEdge {
            edge: EdgeKey::default(),
        },
        BlendError::NonpositiveSize { size: 0.0 },
        BlendError::UnsupportedBody {
            solids: 2,
            shells: 2,
        },
        BlendError::UnsupportedChain {
            edge: EdgeKey::default(),
            detail: "a chain shape that is not built",
        },
        BlendError::UnsupportedRunOut {
            at: EntityId::Vertex(VertexKey::default()),
            detail: "a termination the request does not cover",
        },
        BlendError::UnsupportedGeometry {
            at: EntityId::Face(FaceKey::default()),
            detail: "a stored shape the closed forms do not cover",
        },
        BlendError::BodyNotIntact {
            at: EntityId::HalfEdge(HalfEdgeKey::default()),
            detail: "a reference the plan followed",
        },
        BlendError::RingClearance {
            face: FaceKey::default(),
            margin: -1e-3,
        },
        BlendError::Certify {
            site: "blend face pcurves",
            source: topo::PcurveMintError::Corrupt,
        },
        BlendError::Op {
            site: "strut mev",
            source: topo::EulerOpError::StaleKey {
                key: EntityId::Edge(EdgeKey::default()),
            },
        },
    ]
}

/// Whether a seeded arm is allowed to speak a verb's word mid-text —
/// the PR's own dispositions: ball facts keep ball language
/// (`RadiusHeadroom`, `SpineIrregular`, and an escalation routed to a
/// ball predicate's recourse), the chamfer's own arm speaks as the
/// chamfer, and three recourse constants carry deliberately
/// CONDITIONED per-verb clauses ("for a fillet, …", "a chamfer has no
/// closed-chain band") on `UnsupportedChain`, `UnsupportedGeometry`
/// and the seam-vertex corner tag.
fn verb_words_allowed(err: &BlendError) -> bool {
    match err {
        BlendError::RadiusHeadroom { .. }
        | BlendError::SpineIrregular { .. }
        | BlendError::ChamferArmUnsupported { .. }
        | BlendError::UnsupportedChain { .. }
        | BlendError::UnsupportedGeometry { .. } => true,
        BlendError::UnsupportedCorner { corner, .. } => {
            matches!(corner, CornerConfig::SeamVertex)
        }
        BlendError::Escalated { source, .. } => matches!(
            source.predicate,
            Some("fillet3_radius_headroom" | "fillet3_spine_regularity")
        ),
        _ => false,
    }
}

/// **Probe 1a — no arm renders a verb PREFIX, over the whole battery.**
///
/// The composed refusal must render each door's verb exactly once for
/// EVERY seeded arm, and the inner text must not open with either
/// verb's word — which is the exact regression shape issue 917 fixed
/// (`"fillet assembly: …"`, `"fillet chain: …"` inside the shared
/// Display). The shipped suite holds this only on the arms its chamfer
/// fixtures reach; this row holds it on all of them, so re-inserting
/// the old prefix on ANY shared arm goes red here.
#[test]
fn no_arm_renders_a_verb_prefix_over_the_whole_battery() {
    for seed in seeds() {
        let inner = seed.to_string();
        assert!(
            !inner.starts_with("fillet") && !inner.starts_with("chamfer"),
            "an inner Display must not open with a verb (the door supplies it): {inner}"
        );
        for verb in [BlendKind::Fillet, BlendKind::Chamfer] {
            let composed = BlendRefusal {
                verb,
                error: seed.clone(),
            }
            .to_string();
            let prefix = format!("{verb}: ");
            assert!(
                composed.starts_with(&prefix),
                "the composed refusal opens with its door's verb: {composed}"
            );
            assert_eq!(
                composed.matches(&prefix).count(),
                1,
                "the verb prefix renders exactly once on every arm: {composed}"
            );
        }
    }
}

/// **Probe 1b — mid-text verb words appear only where a disposition
/// covers them.** Outside the allowlisted arms (ball facts, the
/// chamfer's own arm, the conditioned per-verb clauses), an arm's
/// rendered text carries neither verb's word at all — modulo the
/// `fillet3_*` predicate names, which are roster carriers both verbs
/// meter under and are stripped first, as the shipped suite strips
/// them.
#[test]
fn verb_words_appear_only_where_a_disposition_covers_them() {
    for seed in seeds() {
        if verb_words_allowed(&seed) {
            continue;
        }
        let text = seed.to_string().replace("fillet3_", "");
        assert!(
            !text.contains("fillet") && !text.contains("chamfer"),
            "a shared arm with no per-verb disposition must be verb-free: {seed:?} — {text}"
        );
    }
}

/// **Probe 2 — what a fillet caller actually reads at size zero**,
/// executed. The chamfer door refuses `NonpositiveSize` before
/// anything is metered; the fillet door has no such check (disclosed
/// in the BLEND-6 PR as behavior unchanged), so a zero radius reaches
/// predicate 1 and refuses `RadiusHeadroom` with a sentence that is
/// false in both halves for this input: radius 0 does not "exceed"
/// anything, and "reduce the fillet radius" is advice with nowhere to
/// go from zero. Pinned as a characterization: this row documents the
/// current behavior and goes red the day the fillet door gains the
/// door check — flip it then to assert `NonpositiveSize`, as the
/// chamfer's own row in the blend6 suite does.
#[test]
fn a_zero_radius_fillet_reads_a_headroom_refusal_with_unfollowable_advice() {
    let t = Tol::witness();
    let body = cube(1.0, t);
    let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    let err = fillet_edges(&body, &edges, 0.0, t).expect_err("a zero radius does not build");
    assert!(
        matches!(err.error, BlendError::RadiusHeadroom { radius, .. } if radius == 0.0),
        "today the zero radius reaches predicate 1, not a door check: {err:?}"
    );
    let text = err.to_string();
    assert!(
        text.contains("reduce the fillet radius"),
        "and the recourse tells the caller to reduce zero: {text}"
    );
}
