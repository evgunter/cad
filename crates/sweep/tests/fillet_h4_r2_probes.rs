//! **FILLET-H4 review probes (lane r2).** Rows the unit's own suite does
//! not carry, each pinning a claim of `docs/FILLET-H4-SPEC.md` at the
//! level the claim is made:
//!
//! - **C0, at the arm.** The waist's `ConeConeTorus` rests the ball on
//!   the VOID side: spine `x_v + r√2`, feet `r/√2` from the waist vertex
//!   along each generator, each trim circle inside its own cone face's
//!   span. Hand numbers, nothing of the kernel enters.
//! - **The band face's sense bit is the stored verdict's fold**, read
//!   back off the carved body on both material sides — the one
//!   material-side fact the closed-rim surgery writes
//!   (`surgery.rs` `set_face_sense(.., blend_sense())`), pinned where
//!   the suite's rows only reach it through tier 3.
//! - **The ladder twin is proved, not inferred from the volume**: the
//!   boolean union is tier-3 valid BEFORE the fillet, and the band's
//!   neighbours are exactly one plane face and two ring-free sphere
//!   faces — the LADDER's own signature.
//! - **A retirement that misses.** The naming rows check that every
//!   recorded death names a source key and that every output entity is
//!   minted or a survivor; neither direction sees a source entity that
//!   VANISHED without a record. This row closes that gap on the concave
//!   band and on its convex sibling.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_core::{Band, Tol, Vec3};
use sweep::blend::battery::{BlendRequest, run_battery};
use sweep::blend::build::fillet_edges;
use sweep::test_support::{ball_poled_z, cube, lantern, rim_arcs_at, waisted};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations, EdgeKey, FaceKey, LoopBoundary, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

const R: f64 = 0.05;

// ------------------------------------------------------------------
// C0 at the arm: the concave rest, by hand.
// ------------------------------------------------------------------

/// **The waist's arm rests the ball in the void.** In the meridian
/// half-plane the waist vertex is `V = (0.5, 0.5)` and the void wedge
/// opens toward `+x` at 90°; the ball tangent to both generators has its
/// centre on the bisector at `r/sin 45° = r√2` from `V`, so the torus
/// spine is the circle of radius `x_v + r√2` at `y = y_v`, and the feet
/// are `r` from `V` along each generator: `(x_v + r/√2, y_v ∓ r/√2)`.
/// The lower cone's face spans `y ≤ 0.5` and the upper's `y ≥ 0.5`, so
/// each trim circle lies INSIDE its support's span — which is exactly
/// what the material-side rest (`x_v − r√2`, trims on the extensions)
/// violated before the fold.
#[test]
fn the_waist_arm_rests_the_ball_on_the_void_side() {
    let body = waisted(tol());
    let arcs = rim_arcs_at(&body, 0.5, 0.5);
    assert_eq!(arcs.len(), 2, "the waist rim is two arcs");
    let verdict = run_battery(
        &BlendRequest {
            body: &body,
            edges: arcs.clone(),
            size: R,
        },
        Band::linear(tol()).unwrap(),
    )
    .expect("the battery resolves the waist rim");
    let sqrt2 = 2f64.sqrt();
    let spine = 0.5 + R * sqrt2;
    let foot_x = 0.5 + R / sqrt2;
    let mut links = 0;
    for chain in &verdict.chains {
        for link in chain.links() {
            links += 1;
            let Surface::Torus {
                center,
                major_radius,
                minor_radius,
                ..
            } = link.blend.surface
            else {
                panic!(
                    "a cone×cone rim's blend is a torus, got {:?}",
                    link.blend.surface
                )
            };
            assert!(
                (major_radius - spine).abs() < 1e-12,
                "the spine is on the VOID side, x_v + r√2 = {spine}, got {major_radius}"
            );
            assert!((center.y - 0.5).abs() < 1e-12 && (minor_radius - R).abs() < 1e-12);
            let mut ys: Vec<f64> = Vec::new();
            for (trim, _) in [&link.blend.trim_a, &link.blend.trim_b] {
                let Curve3::Circle { center, radius, .. } = *trim else {
                    panic!("a coaxial rim's trimline is a circle, got {trim:?}")
                };
                assert!(
                    (radius - foot_x).abs() < 1e-12,
                    "each foot is r/√2 out from the waist vertex: {foot_x}, got {radius}"
                );
                ys.push(center.y);
            }
            ys.sort_by(f64::total_cmp);
            assert!(
                (ys[0] - (0.5 - R / sqrt2)).abs() < 1e-12
                    && (ys[1] - (0.5 + R / sqrt2)).abs() < 1e-12,
                "one foot on each cone, r/√2 below and above the waist: {ys:?}"
            );
        }
    }
    assert_eq!(links, 2, "both arcs carry the arm");
}

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

/// The faces across the edges of `face`'s outer cycle, deduplicated.
fn neighbours(body: &Body<f64>, face: FaceKey) -> Vec<FaceKey> {
    let LoopBoundary::Cycle { first } = body
        .get_loop(body.get_face(face).unwrap().outer)
        .unwrap()
        .boundary
    else {
        panic!("a band's outer loop is a cycle")
    };
    let mut out: Vec<FaceKey> = body
        .loop_cycle(first)
        .unwrap()
        .into_iter()
        .map(|he| {
            let mate = body.mate(he).unwrap();
            body.get_loop(body.get_half_edge(mate).unwrap().parent_loop)
                .unwrap()
                .face
        })
        .filter(|&f| f != face)
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

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
    let around = neighbours(&out.body, band);
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

/// Every source entity absent from the carved body is a RECORDED
/// retirement — a dead edge/vertex, or a rim arc a band row replaced.
fn every_vanished_source_entity_is_recorded(source: &Body<f64>, arcs: &[EdgeKey], what: &str) {
    let out =
        fillet_edges(source, arcs, R, tol()).unwrap_or_else(|e| panic!("{what} carves, got {e:?}"));
    let rec = out.naming.as_ref().expect("the rim phase records");
    let banded: Vec<EdgeKey> = rec
        .bands
        .iter()
        .flat_map(|(_, es)| es.iter().copied())
        .collect();
    for (k, _) in source.edges() {
        if out.body.get_edge(k).is_none() {
            assert!(
                rec.dead.edges.contains(&k) || banded.contains(&k),
                "{what}: source edge {k:?} vanished with no retirement recorded"
            );
        }
    }
    for (k, _) in source.vertices() {
        if out.body.get_vertex(k).is_none() {
            assert!(
                rec.dead.vertices.contains(&k),
                "{what}: source vertex {k:?} vanished with no retirement recorded"
            );
        }
    }
}

/// **A retirement that misses is seen** — on the concave waist and on
/// the convex shoulder the sibling row pins, so the direction neither
/// naming row checks is checked on both material sides.
#[test]
fn a_vanished_source_entity_is_a_recorded_retirement_on_both_sides() {
    let waist = waisted(tol());
    every_vanished_source_entity_is_recorded(
        &waist,
        &rim_arcs_at(&waist, 0.5, 0.5),
        "the concave waist",
    );
    let lant = lantern(tol());
    every_vanished_source_entity_is_recorded(
        &lant,
        &rim_arcs_at(&lant, 0.8, 0.6),
        "the convex shoulder",
    );
}
