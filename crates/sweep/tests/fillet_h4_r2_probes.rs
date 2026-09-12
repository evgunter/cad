//! **FILLET-H4 review probes (lane r2).** Rows the unit's own suite does
//! not carry, each pinning a claim of `docs/FILLET-H4-SPEC.md` at the
//! level the claim is made:
//!
//! - **The band face's sense bit is the stored verdict's fold**, read
//!   back off the carved body on both material sides — the one
//!   material-side fact the closed-rim surgery writes
//!   (`surgery.rs` `set_face_sense(.., blend_sense())`), pinned where
//!   the suite's rows only reach it through tier 3.
//! - **The ladder twin is proved, not inferred from the volume**: the
//!   boolean union is tier-3 valid BEFORE the fillet, and the band's
//!   neighbours are exactly one plane face and two ring-free sphere
//!   faces — the LADDER's own signature (`test_support::faces_around`,
//!   which the unit's boss row now calls too).
//! - **A retirement that misses.** The naming rows check that every
//!   recorded death names a source key and that every output entity is
//!   minted or a survivor; neither direction sees a source entity that
//!   VANISHED without a record. This row closes that gap on the concave
//!   band and on its convex sibling, through the shared walk
//!   (`test_support::assert_naming_totality`, whose direction (c) this
//!   row contributed).
//!
//! This lane's arm row (the waist's void-side rest at 1e-12) pinned what
//! `review_h4_r1_probes::the_waist_arm_rests_the_ball_on_the_void_side_at_the_hand_value`
//! pins bit-for-bit, and was retired at adoption in its favour.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Tol, Vec3};
use sweep::blend::build::fillet_edges;
use sweep::test_support::{
    assert_naming_totality, ball_poled_z, cube, faces_around, lantern, rim_arcs_at, waisted,
};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{BooleanDeclarations, EdgeKey, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

const R: f64 = 0.05;

// ------------------------------------------------------------------
// The band face's sense bit, on both sides.
// ------------------------------------------------------------------

/// **The band face's sense bit is the stored verdict's fold**, read off
/// the carved body: `false` on the concave waist (the torus's chart
/// normal points into material — the band is a valley wall), `true` on
/// the convex base. Red if `surgery.rs`'s `set_face_sense(..,
/// blend_sense())` on the closed-rim path is ever hardcoded — tier 3
/// does not see that on every fixture, so it is pinned here directly.
#[test]
fn the_band_faces_sense_bit_folds_the_stored_verdict_on_both_sides() {
    let body = waisted(tol());
    for (name, rim_r, rim_y, want) in [
        ("the concave waist", 0.5, 0.5, false),
        ("the convex base", 1.0, 0.0, true),
    ] {
        let arcs = rim_arcs_at(&body, rim_r, rim_y);
        let out = fillet_edges(&body, &arcs, R, tol())
            .unwrap_or_else(|e| panic!("{name} carves, got {e:?}"));
        let [band] = out.band_faces[..] else {
            panic!("{name}: one band")
        };
        let face = out.body.get_face(band).unwrap();
        assert!(
            matches!(
                out.body.get_surface(face.surface),
                Some(Surface::Torus { .. })
            ),
            "{name}: the band is a torus"
        );
        assert_eq!(
            face.sense, want,
            "{name}: the band face's sense is the chain's blend_sense()"
        );
    }
}

// ------------------------------------------------------------------
// The ladder twin, proved.
// ------------------------------------------------------------------

/// **`cube ∪ ball` is tier-3 valid at rest BEFORE the fillet, and its
/// band is the LADDER.** The union is built through the public boolean
/// door and validated as it stands; the boss's rim is then carved and
/// the band's neighbours read off the body: exactly ONE plane face (the
/// top, keeping its key) and TWO sphere faces, each ring-free — the
/// ladder's own shape (a ring of one plane face, two half-caps) rather
/// than an inference from the volume's sign.
#[test]
fn the_boss_union_is_valid_at_rest_and_its_band_is_the_ladder() {
    const SLAB: f64 = 1.0;
    const BALL_R: f64 = 0.09;
    const CAP_H: f64 = 0.05;
    let ball = ball_poled_z(BALL_R, Vec3::new(0.5, 0.5, SLAB - (BALL_R - CAP_H)), tol());
    let boss = boolean_op_with(
        BooleanOp::Union,
        &cube(SLAB, tol()),
        &ball,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap_or_else(|e| panic!("the union builds, got {e}"))
    .body()
    .expect("a body")
    .body
    .clone();
    validate_geometric(&boss, tol())
        .unwrap_or_else(|e| panic!("the union is tier-3 valid at rest, got {e:?}"));
    let arcs: Vec<EdgeKey> = boss
        .edges()
        .filter(|(_, e)| {
            let kind = |he| {
                let l = boss.get_half_edge(he).unwrap().parent_loop;
                let f = boss.get_face(boss.get_loop(l).unwrap().face).unwrap();
                boss.get_surface(f.surface).map(core::mem::discriminant)
            };
            kind(e.he_plus) != kind(e.he_minus)
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(
        arcs.len(),
        2,
        "the boss's rim is two arcs across the cap's seam"
    );
    let top = boss
        .faces()
        .find(|(_, f)| {
            matches!(boss.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. }) if (origin.z - SLAB).abs() < 1e-12 && normal.z > 0.5)
        })
        .map(|(k, _)| k)
        .expect("the top face");

    let out = fillet_edges(&boss, &arcs, 0.02, tol())
        .unwrap_or_else(|e| panic!("the boss carves, got {e:?}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("tier-3 valid, got {e:?}"));
    let [band] = out.band_faces[..] else {
        panic!("one band")
    };
    let around = faces_around(&out.body, band);
    let mut planes = 0;
    let mut spheres = 0;
    for f in &around {
        let fd = out.body.get_face(*f).unwrap();
        match out.body.get_surface(fd.surface) {
            Some(Surface::Plane { .. }) => {
                planes += 1;
                assert_eq!(
                    *f, top,
                    "the plane beside the band is the top face, its key kept"
                );
                assert_eq!(fd.rings.len(), 1, "the top face keeps exactly one ring");
            }
            Some(Surface::Sphere { .. }) => {
                spheres += 1;
                assert!(fd.rings.is_empty(), "a half-cap is ring-free");
            }
            other => panic!("the band's neighbours are the plane and the cap, got {other:?}"),
        }
    }
    assert_eq!(
        (planes, spheres),
        (1, 2),
        "the LADDER: one plane face and two half-caps around the band, got {around:?}"
    );
    assert!(
        !out.body.get_face(band).unwrap().sense,
        "a boss's band is a valley wall: sense false"
    );
}

// ------------------------------------------------------------------
// Naming: the retirement that misses.
// ------------------------------------------------------------------

/// **A retirement that misses is seen** — on the concave waist and on
/// the convex shoulder the sibling row pins, so the direction neither
/// naming row used to check is checked on both material sides, through
/// the one shared walk.
#[test]
fn a_vanished_source_entity_is_a_recorded_retirement_on_both_sides() {
    let waist = waisted(tol());
    let arcs = rim_arcs_at(&waist, 0.5, 0.5);
    let out = fillet_edges(&waist, &arcs, R, tol())
        .unwrap_or_else(|e| panic!("the concave waist carves, got {e:?}"));
    assert_naming_totality(&waist, &out, &arcs, "the concave waist");

    let lant = lantern(tol());
    let arcs = rim_arcs_at(&lant, 0.8, 0.6);
    let out = fillet_edges(&lant, &arcs, R, tol())
        .unwrap_or_else(|e| panic!("the convex shoulder carves, got {e:?}"));
    assert_naming_totality(&lant, &out, &arcs, "the convex shoulder");
}
