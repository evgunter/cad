//! **A planar face pierced by a prism whose walls are arcs.** The slab's
//! top and bottom faces each keep a ring where the prism goes through,
//! and the ring lane winds that ring's run closed along the section it
//! lies on. Where the section is an arc of the prism's wall the closing
//! is that arc, not the straight chord between its ends: a D's run IS
//! its arc's chord, so closing it straight retraces the run and encloses
//! nothing.
//!
//! The profiles are the D (an arc and its diameter) at bulges from a
//! shallow cap past the half-disc, the crescent and the lens (two arcs
//! on one chord, bowing the same way and opposite ways), and the D and
//! the crescent drawn with an arc split at a vertex on its circle, which
//! the extrude sweeps as one wall, so the vertex drawn on the arc stays
//! on the prism's caps and must leave none in the section. Every profile is driven through
//! the slab upright, off the origin, spun about its axis and tilted
//! about one axis and two, and every op is asked in both member orders.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::{brick, finished};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanError};

/// The slab `[−4, 4]² × [−0.5, 0.5]`.
const SLAB: f64 = 64.0;

/// The prism's length; it runs `z ∈ [−2, 2]` before it is posed, so it
/// crosses the slab whole at every pose below.
const LENGTH: f64 = 4.0;

/// A body's `(solids, vertices, edges, faces, loops)`.
type Census = (usize, usize, usize, usize, usize);

fn census(body: &topo::Body<f64>) -> Census {
    (
        body.solids().count(),
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
        body.loops().count(),
    )
}

/// **The census of a slab pierced by an `n`-sided prism whose caps
/// each carry `k` vertices drawn on an arc**, for ∪, ∩, slab ∖ prism and
/// prism ∖ slab. A vertex drawn between two arcs of one circle splits
/// the cap's edge there but no wall, so the slab's sections never meet
/// it: it shows only on the prism's own caps, which ∪ and prism ∖ slab
/// keep.
///
/// - ∪: the slab's two faces each keep a ring of `n` edges and `n`
///   vertices, and the prism's `n` walls are cut into the stub above and
///   the stub below, its `n` long edges into two each;
/// - ∩: the prism one unit long between the slab's planes;
/// - slab ∖ prism: the slab with a hole through it, genus 1;
/// - prism ∖ slab: the two stubs.
fn pierced(n: usize, k: usize) -> [Census; 4] {
    [
        (
            1,
            8 + 4 * n + 2 * k,
            12 + 6 * n + 2 * k,
            8 + 2 * n,
            10 + 2 * n,
        ),
        (1, 2 * n, 3 * n, 2 + n, 2 + n),
        (1, 8 + 2 * n, 12 + 3 * n, 6 + n, 8 + n),
        (2, 4 * n + 2 * k, 6 * n + 2 * k, 4 + 2 * n, 4 + 2 * n),
    ]
}

/// The bulge of an arc turning through `turn` radians, signed so a
/// positive turn bows to the left of its chord: from `(−1, 0)` to `(1, 0)`
/// over the top.
fn bulge(turn: f64) -> f64 {
    -(turn / 4.0).tan()
}

/// One profile: its name, its chain, its area in closed form, the
/// number of sides its prism has, and the number of its vertices drawn
/// on an arc ([`pierced`]).
struct Shape {
    name: String,
    chain: Vec<(Point2<f64>, f64)>,
    area: f64,
    sides: usize,
    on_arc: usize,
}

/// The area between a chord of length 2 and an arc through `turn` on it.
fn segment_area(turn: f64) -> f64 {
    let r = 1.0 / (turn / 2.0).sin();
    0.5 * r * r * (turn - turn.sin())
}

fn shapes() -> Vec<Shape> {
    let p = |x: f64, y: f64| Point2::new(x, y);
    let mut out = Vec::new();
    // The D: from (−1, 0) over the top to (1, 0), then the diameter.
    for turn in [0.6, PI / 2.0, PI, 1.4 * PI] {
        out.push(Shape {
            name: format!("D turning {turn:.4}"),
            chain: vec![(p(-1.0, 0.0), bulge(turn)), (p(1.0, 0.0), 0.0)],
            area: segment_area(turn),
            sides: 2,
            on_arc: 0,
        });
    }
    // The D's half-disc arc split at its apex, and at a third of its turn:
    // a run of two arcs on one circle, which sweeps one wall, so the
    // prism has two sides and the vertex drawn on the arc stays on its
    // caps.
    out.push(Shape {
        name: "D split at its apex".into(),
        chain: vec![
            (p(-1.0, 0.0), bulge(PI / 2.0)),
            (p(0.0, 1.0), bulge(PI / 2.0)),
            (p(1.0, 0.0), 0.0),
        ],
        area: PI / 2.0,
        sides: 2,
        on_arc: 1,
    });
    out.push(Shape {
        name: "D split at a third".into(),
        chain: vec![
            (p(-1.0, 0.0), bulge(PI / 3.0)),
            (p(-0.5, 0.75f64.sqrt()), bulge(2.0 * PI / 3.0)),
            (p(1.0, 0.0), 0.0),
        ],
        area: PI / 2.0,
        sides: 2,
        on_arc: 1,
    });
    // The crescent: over the top on the half-disc's arc, back on a
    // shallower one bowing the same way.
    out.push(Shape {
        name: "crescent".into(),
        chain: vec![(p(-1.0, 0.0), bulge(PI)), (p(1.0, 0.0), bulge(-0.6))],
        area: segment_area(PI) - segment_area(0.6),
        sides: 2,
        on_arc: 0,
    });
    // The crescent with its outer arc split at its apex.
    out.push(Shape {
        name: "crescent split at its apex".into(),
        chain: vec![
            (p(-1.0, 0.0), bulge(PI / 2.0)),
            (p(0.0, 1.0), bulge(PI / 2.0)),
            (p(1.0, 0.0), bulge(-0.6)),
        ],
        area: segment_area(PI) - segment_area(0.6),
        sides: 2,
        on_arc: 1,
    });
    // The lens: over the top, and back under the bottom.
    out.push(Shape {
        name: "lens".into(),
        chain: vec![(p(-1.0, 0.0), bulge(PI)), (p(1.0, 0.0), bulge(0.6))],
        area: segment_area(PI) + segment_area(0.6),
        sides: 2,
        on_arc: 0,
    });
    out
}

/// One pose: its name, its rigid map, and `|d_z|`, the cosine of the
/// prism's axis against the slab's normal, which divides the volume
/// the slab cuts from the prism.
fn poses() -> Vec<(&'static str, Affine3<f64>, f64)> {
    let o = Point3::origin();
    let x = Vec3::new(1.0, 0.0, 0.0);
    let y = Vec3::new(0.0, 1.0, 0.0);
    let z = Vec3::new(0.0, 0.0, 1.0);
    vec![
        ("upright", Affine3::identity(), 1.0),
        (
            "off the origin",
            Affine3::translation(Vec3::new(0.7, -1.3, 0.0)),
            1.0,
        ),
        ("spun", Affine3::rotation_about_axis(o, z, 0.7), 1.0),
        (
            "tilted",
            Affine3::rotation_about_axis(o, x, 0.3),
            0.3f64.cos(),
        ),
        (
            "tilted twice",
            Affine3::rotation_about_axis(o, y, 0.5) * Affine3::rotation_about_axis(o, x, 0.3),
            0.3f64.cos() * 0.5f64.cos(),
        ),
    ]
}

fn prism(shape: &Shape, pose: &Affine3<f64>) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, -LENGTH / 2.0)));
    let profile = Profile::new(plane, vec![bulge_loop(shape.chain.clone())])
        .validate(tol)
        .unwrap_or_else(|e| panic!("{}: the profile validates: {e:?}", shape.name));
    let upright = extrude(
        &profile,
        Extrusion::Distance {
            depth: LENGTH,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    finished(
        "the prism",
        topo::transform_rigid(&upright, pose, tol).unwrap(),
        tol,
    )
}

/// Whether tier 3′'s census can decide a result's parts apart.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Reach {
    /// It must: one part, or parts the census decides apart.
    Decided,
    /// Two parts whose curved faces lie within reach of each other, which
    /// the census cannot yet decide apart (CONTACT's
    /// `census-cross-solid-curved-pairs-undecidable-on-shell-results`):
    /// tier 3′ passes or refuses with `CensusUndecidable` alone.
    Undecided,
}

/// The result of `out`, held to tiers 1–3 and 3′ and to `truth`: exactly
/// where every face is closed-form, within the measured pad where an
/// ellipse arc bounds a face. Returns its census.
fn check(
    label: &str,
    out: Result<topo::BooleanResult<f64>, BooleanError>,
    truth: f64,
    parts_out_of_reach: Reach,
) -> Census {
    let tol = Tol::witness();
    let out = out.unwrap_or_else(|e| panic!("{label}: refused {e:?}"));
    let topo::BooleanResult::Body(out) = out else {
        panic!("{label}: came back empty");
    };
    assert_eq!(topo::validate(&out.body), Ok(()), "{label}: validate");
    assert_eq!(topo::validate_closed(&out.body), Ok(()), "{label}: closed");
    assert_eq!(
        topo::validate_geometric(&out.body, tol),
        Ok(()),
        "{label}: tier 3"
    );
    match topo::validate_pseudomanifold(&out.body, &out.contacts, tol) {
        Ok(()) => {}
        Err(errs) if parts_out_of_reach == Reach::Undecided => {
            assert!(
                out.body.solids().count() == 2
                    && errs
                        .iter()
                        .all(|e| matches!(e, topo::ValidationError::CensusUndecidable { .. })),
                "{label}: tier 3′ refuses only the two parts' curved pairs, got {errs:?}"
            );
            println!(
                "{label}: tier 3′ cannot yet decide the two parts' curved faces apart \
                 ({} pairs; work/contact/census-cross-solid-curved-pairs-undecidable-on-shell-results.md)",
                errs.len()
            );
        }
        Err(errs) => panic!("{label}: tier 3′ {errs:?}"),
    }
    let m = topo::mass_properties(&out.body, tol)
        .unwrap_or_else(|e| panic!("{label}: mass properties {e:?}"));
    assert!(
        (m.volume - truth).abs() <= 1e-12 * truth.max(1.0) + m.volume_pad,
        "{label}: volume {} ± {} against the closed form {truth}",
        m.volume,
        m.volume_pad
    );
    census(&out.body)
}

/// **Every profile through the slab, at every pose, under every op in
/// both member orders**: tiers 3 and 3′, the closed-form volume, and the
/// census [`pierced`] derives from the prism's sides and the vertices
/// drawn on its arcs. An
/// upright, shifted or spun prism meets the slab in circle arcs, and its
/// volume must be exact; a tilted one meets it in ellipse arcs, whose
/// faces measure within a pad, and leaves two stubs whose walls overhang
/// each other, which tier 3′ cannot yet decide apart ([`Reach`]).
#[test]
fn a_prism_with_arc_walls_through_a_slab_builds_every_op() {
    let tol = Tol::witness();
    let slab = finished(
        "the slab",
        brick((-4.0, 4.0), (-4.0, 4.0), (-0.5, 0.5), tol),
        tol,
    );
    for shape in shapes() {
        for (pose, map, dz) in poses() {
            let label = format!("{} {pose}", shape.name);
            let prism = prism(&shape, &map);
            let v = shape.area * LENGTH;
            let shared = shape.area / dz;
            let want = pierced(shape.sides, shape.on_arc);
            // Tilted, the two stubs' walls overhang each other across the
            // slab.
            let stubs = if dz < 1.0 {
                Reach::Undecided
            } else {
                Reach::Decided
            };
            let union = check(
                &format!("{label}, slab ∪ prism"),
                topo::union(&slab, &prism, tol),
                SLAB + v - shared,
                Reach::Decided,
            );
            assert_eq!(
                check(
                    &format!("{label}, prism ∪ slab"),
                    topo::union(&prism, &slab, tol),
                    SLAB + v - shared,
                    Reach::Decided,
                ),
                union,
                "{label}: ∪ census in both member orders"
            );
            let meet = check(
                &format!("{label}, slab ∩ prism"),
                topo::intersect(&slab, &prism, tol),
                shared,
                Reach::Decided,
            );
            assert_eq!(
                check(
                    &format!("{label}, prism ∩ slab"),
                    topo::intersect(&prism, &slab, tol),
                    shared,
                    Reach::Decided,
                ),
                meet,
                "{label}: ∩ census in both member orders"
            );
            let got = [
                union,
                meet,
                check(
                    &format!("{label}, slab ∖ prism"),
                    topo::subtract(&slab, &prism, tol),
                    SLAB - shared,
                    Reach::Decided,
                ),
                check(
                    &format!("{label}, prism ∖ slab"),
                    topo::subtract(&prism, &slab, tol),
                    v - shared,
                    stubs,
                ),
            ];
            assert_eq!(
                got, want,
                "{label}: census of ∪, ∩, slab ∖ prism, prism ∖ slab"
            );
        }
    }
}
