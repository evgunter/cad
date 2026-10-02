//! **JOIN-1's mechanisms, each held by a row that goes red when it is
//! taken away** (the dual review's minor 1). Measured on the fix-pass
//! head, one mutation at a time:
//!
//! - **locus matching** — the chord join pairs a segment's two ends by
//!   the cells their germs lie in (`partners` compares `a_locus` /
//!   `b_locus`). Comparing the germs' recorded faces instead turns
//!   [`matching_reads_the_germs_loci`] red, and nothing else here.
//! - **the structural skip** — on the boolean lanes a chord is not
//!   minted when the edge between the two halves IS the segment's
//!   locus edge (`SegmentEdge::Is`). Never skipping turns
//!   [`the_skip_takes_the_locus_edge_for_the_segment`] red, and it
//!   stays green under the other two mutations.
//! - **the fold direction** — an on-bound joins the In run unless both
//!   neighbours read Out (`sectors::fold_on_bound`). Folding mixed
//!   bounds Out instead turns the dumbbell row
//!   (`germ_torus_doors::the_torus_waisted_union_builds_like_the_cylinder_control`)
//!   and the teapot cup row
//!   (`verbs_1031b_arcwind::the_boolean_after_the_merge_passes_the_join`)
//!   red, which the other two mutations leave green; it also turns
//!   [`matching_reads_the_germs_loci`] red.
//!
//! Each row asserts the body that builds: tiers 2 and 3′, the at-rest
//! certificate and the closed-form volume.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::mate2_common::{collar_at, peg_at, volume, wall_decls};
use geom_core::Tol;
use topo::{BooleanError, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

fn assert_sound(what: &str, r: Result<BooleanResult<f64>, BooleanError>, want: f64) {
    let r = r.unwrap_or_else(|e| panic!("{what}: {e:?}"));
    let bb = r.body().unwrap_or_else(|| panic!("{what}: empty"));
    topo::validate_closed(&bb.body).unwrap_or_else(|e| panic!("{what}: tier 2: {e:?}"));
    topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3′: {e:?}"));
    topo::validate_geometric_certificate(&bb.body, tol())
        .unwrap_or_else(|e| panic!("{what}: certificate: {e:?}"));
    let v = topo::mass_properties(&bb.body, tol()).unwrap().volume;
    assert!((v - want).abs() < 1e-9, "{what}: volume {v} against {want}");
}

/// **Locus matching.** A peg in a collar's bore, flush with the
/// collar's bottom and proud above its top, every wall pair declared
/// `Rest`. The peg's bottom rim arcs lie on the bore's: each section
/// segment there is an edge of both solids, and the loci name the same
/// two edges at both of its ends. The union is the collar with the
/// peg's proud part: additive.
///
/// The declared-REST zip takes over a declared union the join refuses,
/// and builds this one too, so the volume alone cannot tell the lanes
/// apart; the merge door's record can. The zip keeps the bore wall for
/// the door, which records the declared cylinder pair it has no arm
/// for; the join discards the bore wall with its surface, so the door
/// is handed no pair. The row asserts the join's answer.
#[test]
fn matching_reads_the_germs_loci() {
    let (c, p) = (collar_at(0.0), peg_at(0.0, 1.0, 1.5));
    let want = volume(&c) + volume(&p);
    let r = topo::union_with(&c, &p, &wall_decls(&c, &p), tol());
    if let Ok(BooleanResult::Body(bb)) = &r {
        let declared = bb
            .naming
            .merge_skipped
            .iter()
            .filter(|s| {
                matches!(
                    s.reason,
                    topo::MergeCoplanarError::DeclaredCarrierUnsupported { .. }
                )
            })
            .count();
        assert_eq!(
            declared, 0,
            "the chord join builds the union, not the zip that takes over its refusal"
        );
    }
    assert_sound("peg ∪ collar", r, want);
}

/// **The structural skip.** A peg exactly the collar's height, flush
/// at both ends, every wall pair declared `Rest`: the union is the
/// unbored collar. Both rims are edges of both solids. Each solid
/// mints ONE copy of a rim arc, in the face its fold leaves Out, and
/// keeps the rim itself as the other copy: the chord on the far side
/// of the rim is skipped because the edge between the two halves there
/// IS the segment's locus edge.
#[test]
fn the_skip_takes_the_locus_edge_for_the_segment() {
    let (c, p) = (collar_at(0.0), peg_at(0.0, 1.0, 1.0));
    let want = volume(&c) + volume(&p);
    assert_sound(
        "flush peg ∪ collar",
        topo::union_with(&c, &p, &wall_decls(&c, &p), tol()),
        want,
    );
}
