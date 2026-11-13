//! **The chamfer over a plane–cylinder rim is a cone frustum.** A circle
//! where a cylinder meets a plane square to its axis, chamfered at equal
//! setbacks `s`, is replaced by the cone coaxial with the cylinder through
//! the circle of radius `R ± s` on the plane and the circle `s` along the
//! axis on the cylinder. The section it removes (convex) or fills
//! (concave) is the right triangle of legs `s`, area `s²/2`, whose
//! centroid lies a third of a leg from the rim corner along each leg. So
//! by Pappus the rim moves the volume by `∓π·s²·ρ̄`, `ρ̄` the centroid's
//! distance from the axis:
//!
//! - `ρ̄ = R + s/3` where the plane runs OUTWARD from the rim: a bore's
//!   mouth (convex, removes), a boss's foot (concave, adds);
//! - `ρ̄ = R − s/3` where it runs INWARD to the axis: a boss's top
//!   (convex, removes), a counterbore's step meeting its wide wall
//!   (concave, adds).
//!
//! The band carves through the fillet's closed-rim walks — the ladder
//! (a ring of the plane), the seam-split annulus (the plane's outer
//! cycle) and the one-edge annulus (a full revolve's walls) — with only
//! its surface, its slit and its trims' description its own.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Bounds, CertifiedBounds, Decide, Interval, Point2, Real, Tol};
use sweep::blend::{BlendError, CornerConfig};
use sweep::chamfer::{Chamfered, chamfer_edges};
use sweep::test_support::{
    assert_naming_totality, at_rest, bored_block_of_arcs, boss_of_arcs, circle_arcs_at_z, corners,
    cube, cylinder_of_arcs_at, prism_at, realized, revolved_about_y_at, rim_arcs_at, z_rim,
};
use topo::boolean::BooleanOp;
use topo::{Body, EdgeKey, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

/// The frustum ring's Pappus volume: the triangle `s²/2` carried round
/// the circle of radius `centroid`.
fn ring(s: f64, centroid: f64) -> f64 {
    PI * centroid * s * s
}

fn volume<T: CertifiedBounds + topo::AtRestPolicy>(body: &Body<T>) -> T {
    let p = topo::mass_properties(
        &topo::test_support::finished("the measured body", topo::Body::clone(body), tol()),
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .expect("mass properties");
    assert_eq!(p.enclosure.volume_pad, 0.0, "closed-form faces only");
    p.enclosure.volume
}

fn chamfer<T: Decide + Bounds + topo::AtRestPolicy>(
    body: &Body<T>,
    edges: &[EdgeKey],
    s: f64,
) -> Result<Chamfered<T>, BlendError> {
    chamfer_edges(&at_rest(body, tol()), edges, T::from_f64(s), tol()).map_err(|r| r.error)
}

/// The request, chamfered: tier-3 valid, `bands` closed-rim bands, the
/// volume moved by `want` — at `f64` to 1e-12, at `Interval` bracketed
/// within a nanometre cubed — and the birth records total over
/// `blended`, the request less any smooth seam it carries.
fn built<T: Decide + CertifiedBounds + topo::AtRestPolicy>(
    what: &str,
    body: &Body<T>,
    (edges, blended): (&[EdgeKey], &[EdgeKey]),
    s: f64,
    bands: usize,
    want: f64,
) -> Chamfered<T> {
    let out = chamfer(body, edges, s).unwrap_or_else(|e| panic!("{what}: builds, got {e:?}"));
    assert_eq!(out.band_faces.len(), bands, "{what}: one frustum per rim");
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier-3, {e:?}"));
    let (v0, v1) = (volume(body), volume(&out.body));
    assert!(
        v0.hi() - v0.lo() < 1e-12,
        "{what}: the source's own enclosure is a point"
    );
    let truth = v0.lo() + want;
    assert!(
        v1.lo() - 1e-12 <= truth && truth <= v1.hi() + 1e-12 && v1.hi() - v1.lo() < 1e-9,
        "{what}: V₁ = [{}, {}] against V₀ + ΔV = {truth} (ΔV = {want})",
        v1.lo(),
        v1.hi()
    );
    for f in &out.band_faces {
        let surface = out.body.face_surface(*f).expect("a band face's surface");
        assert!(
            matches!(surface, geom::Surface::Cone { .. }),
            "{what}: a frustum band is a cone, got {surface:?}"
        );
    }
    assert_naming_totality(body, &out, blended, what);
    out
}

/// A full revolve about `y` of a closed profile `(x = radius, y =
/// height)` that stays off the axis, so every wall is ONE face and
/// every latitude rim ONE closed edge — the one-edge annulus shape.
fn revolved<T: Decide + topo::AtRestPolicy>(profile: &[(f64, f64)]) -> Body<T> {
    revolved_about_y_at(
        profile
            .iter()
            .map(|&(x, y)| (Point2::new(T::from_f64(x), T::from_f64(y)), T::zero()))
            .collect(),
        sweep::Revolution::Full,
        tol(),
    )
}

/// **One-edge annulus rims at both scalars, on both material sides.** A
/// revolved washer's top face carries its bore's mouth as a ring and its
/// outer top edge as its outer cycle (a boss's top), both convex, in one
/// call; a revolved counterbore's step carries its wide wall's foot
/// (concave) and its narrow bore's mouth (convex), in one call.
fn one_edge_annuli<T: Decide + CertifiedBounds + topo::AtRestPolicy>() {
    let s = 0.1;
    let washer = revolved::<T>(&[(0.3, 0.0), (1.0, 0.0), (1.0, 0.5), (0.3, 0.5)]);
    let rims = [
        rim_arcs_at(&washer, 0.3, 0.5),
        rim_arcs_at(&washer, 1.0, 0.5),
    ]
    .concat();
    assert_eq!(rims.len(), 2, "the washer's two top rims, one edge each");
    built(
        "the washer's mouth and top edge",
        &washer,
        (&rims, &rims),
        s,
        2,
        -ring(s, 0.3 + s / 3.0) - ring(s, 1.0 - s / 3.0),
    );

    let counterbore = revolved::<T>(&[
        (0.3, 0.0),
        (1.0, 0.0),
        (1.0, 1.0),
        (0.6, 1.0),
        (0.6, 0.5),
        (0.3, 0.5),
    ]);
    let step = [
        rim_arcs_at(&counterbore, 0.6, 0.5),
        rim_arcs_at(&counterbore, 0.3, 0.5),
    ]
    .concat();
    assert_eq!(step.len(), 2, "the step's two rims, one edge each");
    built(
        "the counterbore's step, concave and convex",
        &counterbore,
        (&step, &step),
        s,
        2,
        ring(s, 0.6 - s / 3.0) - ring(s, 0.3 + s / 3.0),
    );
}

#[test]
fn one_edge_annulus_rims_chamfer_at_their_frustum_rings() {
    one_edge_annuli::<f64>();
}

#[test]
fn one_edge_annulus_rims_chamfer_at_their_frustum_rings_at_the_certified_scalar() {
    one_edge_annuli::<Interval>();
}

/// **The ladder and the seam-split annulus, both material sides.** A
/// bore through a block (a ring of each cap, the wall cut into `n` arcs
/// by its seams): its mouth alone, then both rims in one call; a post's
/// top (the cap's outer cycle, a seam-split annulus); a boss's foot on a
/// block (a ring of the block's top, concave).
#[test]
fn ladder_and_seam_split_rims_chamfer_at_their_frustum_rings() {
    let (r, s) = (0.5, 0.1);
    for n in 2..=4 {
        let body = bored_block_of_arcs(n, 2.0, 1.0, r, tol());
        let mouth = circle_arcs_at_z(&body, 1.0);
        let what = format!("the {n}-arc bore's mouth");
        built(&what, &body, (&mouth, &mouth), s, 1, -ring(s, r + s / 3.0));
        let both = [circle_arcs_at_z(&body, 0.0), mouth].concat();
        let what = format!("both rims of the {n}-arc bore");
        built(
            &what,
            &body,
            (&both, &both),
            s,
            2,
            -2.0 * ring(s, r + s / 3.0),
        );
    }
    let post = cylinder_of_arcs_at(2, r, Point2::new(0.0, 0.0), 0.0, 1.0, tol());
    let top = circle_arcs_at_z(&post, 1.0);
    built(
        "a post's top",
        &post,
        (&top, &top),
        s,
        1,
        -ring(s, r - s / 3.0),
    );

    let r = 0.4;
    let boss = boss_of_arcs(2, 2.0, r, 1.0, 2.0, tol());
    let foot = z_rim(&boss, r, 2.0, true);
    built(
        "a boss's foot",
        &boss,
        (&foot, &foot),
        s,
        1,
        ring(s, r + s / 3.0),
    );
}

/// **A counterbore through a block, in sequence.** Its step is ONE plane
/// face whose outer cycle is the wide wall's foot (an annulus rim,
/// concave) and whose ring is the narrow bore's mouth (a ladder rim,
/// convex). The two share their host, which the closed-rim walks carve
/// in sequential calls, as the fillet's do: in one call the request
/// refuses saying so, and in sequence it builds at both rings.
#[test]
fn a_counterbores_step_chamfers_both_rings_in_sequence() {
    let (r1, r2, s) = (0.6, 0.3, 0.1);
    let wide = cylinder_of_arcs_at(2, r1, Point2::new(1.0, 1.0), 1.5, 1.0, tol());
    let narrow = cylinder_of_arcs_at(2, r2, Point2::new(1.0, 1.0), -0.5, 3.0, tol());
    let body = realized(
        BooleanOp::Subtract,
        &realized(BooleanOp::Subtract, &cube(2.0, tol()), &wide, tol()),
        &narrow,
        tol(),
    );
    let (inner, outer) = (z_rim(&body, r2, 1.5, true), z_rim(&body, r1, 1.5, true));
    let one_call = chamfer(&body, &[inner.clone(), outer].concat(), s);
    assert!(
        matches!(one_call, Err(BlendError::UnsupportedChain { .. })),
        "the step's two rims share their host and ask for two calls, got {one_call:?}"
    );
    let mid = built(
        "the narrow bore's mouth",
        &body,
        (&inner, &inner),
        s,
        1,
        -ring(s, r2 + s / 3.0),
    )
    .body;
    let outer = z_rim(&mid, r1, 1.5, true);
    built(
        "then the wide wall's foot",
        &mid,
        (&outer, &outer),
        s,
        1,
        ring(s, r1 - s / 3.0),
    );
}

/// The 6 × 4 × 0.5 plate, less a two-arc drill at each `(x, y, r)`.
fn plate_with_holes(holes: &[(f64, f64, f64)]) -> Body<f64> {
    let mut body = prism_at(
        corners(&[(0.0, 0.0), (6.0, 0.0), (6.0, 4.0), (0.0, 4.0)]),
        0.0,
        0.5,
        tol(),
    );
    for &(x, y, r) in holes {
        let drill = cylinder_of_arcs_at(2, r, Point2::new(x, y), -0.5, 1.5, tol());
        body = realized(BooleanOp::Subtract, &body, &drill, tol());
    }
    body
}

/// **A bored plate chamfered all round.** Each of the box's twelve edges
/// removes the prism `s²/2` along its length; at each corner the three
/// prisms' ends overlap in the cube of side `s` there, of which the patch
/// keeps the tetrahedron `s³/6` beyond it, so the corner removes `5s³/6`
/// in place of the prisms' `3s³/2` there. Over `ΣL = 42` that is
/// `21s² − 16s³/3`, and the bore's two rims remove a frustum ring each.
/// The request is every edge, the bore wall's two smooth seams among
/// them, which the blend leaves as they are.
#[test]
fn a_bored_plate_chamfers_all_round() {
    let (r, s) = (0.3, 0.05);
    let body = plate_with_holes(&[(3.0, 2.0, r)]);
    let mut all = topo::query::all_edges(&body);
    all.sort_unstable();
    let sharp: Vec<EdgeKey> = all
        .iter()
        .copied()
        .filter(|&e| {
            let (a, b) = topo::readback::edge_sides(&body, e).unwrap().surfaces();
            a != b
        })
        .collect();
    assert_eq!(all.len() - sharp.len(), 2, "the bore wall's two seams");
    let want = -21.0 * s * s + 16.0 / 3.0 * s.powi(3) - 2.0 * ring(s, r + s / 3.0);
    built(
        "the bored plate all round",
        &body,
        (&all, &sharp),
        s,
        2,
        want,
    );
}

/// **The bracket, chamfered all round.** Its bore no longer refuses:
/// "select all edges" reaches the mixed-convexity corner where the boss
/// meets the plate
/// (`work/band/a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds.md`'s
/// ground), and the bore's two rims chamfer inside the bracket at two
/// frustum rings.
#[test]
fn the_bracket_chamfers_its_bore_and_all_round_reaches_only_its_mixed_corner() {
    let s = 0.05;
    let plate = prism_at(
        corners(&[(0.0, 0.0), (6.0, 0.0), (6.0, 4.0), (0.0, 4.0)]),
        0.0,
        0.5,
        tol(),
    );
    let boss = prism_at(
        corners(&[(0.0, 0.5), (0.5, 0.5), (0.5, 3.5), (0.0, 3.5)]),
        0.25,
        1.75,
        tol(),
    );
    let bore = cylinder_of_arcs_at(2, 0.3, Point2::new(3.0, 2.0), -0.5, 1.5, tol());
    let bracket = realized(
        BooleanOp::Subtract,
        &realized(BooleanOp::Union, &plate, &boss, tol()),
        &bore,
        tol(),
    );
    let mut all = topo::query::all_edges(&bracket);
    all.sort_unstable();
    let got = chamfer(&bracket, &all, s);
    assert!(
        matches!(
            got,
            Err(BlendError::UnsupportedCorner {
                corner: CornerConfig::MixedConvexity { .. },
                ..
            })
        ),
        "the bracket all round refuses at its mixed corner alone, got {got:?}"
    );
    let rims = [
        z_rim(&bracket, 0.3, 0.0, true),
        z_rim(&bracket, 0.3, 0.5, true),
    ]
    .concat();
    built(
        "the bracket's bore",
        &bracket,
        (&rims, &rims),
        s,
        2,
        -2.0 * ring(s, 0.3 + s / 3.0),
    );
}

/// **A second hole walked toward the mouth's trim.** The frustum's trim
/// on the plate is the circle `R + s` about the bore; a hole of radius
/// `r₂` whose centre stands `d` off the bore's clears it by
/// `d − r₂ − (R + s)`. The clearance meter reads that margin — the hole
/// is not refused for being there — so the chamfer builds while it is
/// positive and refuses with it as its reading once it is negative.
#[test]
fn a_neighbour_hole_clears_the_mouths_trim_until_its_margin_turns() {
    let (r, r2, s) = (0.3, 0.2, 0.1);
    let at = |d: f64| {
        let body = plate_with_holes(&[(3.0, 2.0, r), (3.0 + d, 2.0, r2)]);
        let rim = z_rim(&body, r, 0.5, true);
        (body, rim)
    };
    for d in [0.75, 0.61, 0.601] {
        let (body, rim) = at(d);
        let what = format!("a neighbour {d} off");
        built(&what, &body, (&rim, &rim), s, 1, -ring(s, r + s / 3.0));
    }
    for d in [0.599, 0.59, 0.55] {
        let (body, rim) = at(d);
        let margin = d - r2 - (r + s);
        match chamfer(&body, &rim, s) {
            Err(BlendError::FaceClearanceUncertified { margin: read, .. }) => {
                let reading = read
                    .reading
                    .diagnostic_f64_for_error_text()
                    .value()
                    .expect("a value");
                assert!(
                    (reading - margin).abs() < 1e-12,
                    "a neighbour {d} off: the meter reads {reading} against the margin {margin}"
                );
            }
            other => panic!("a neighbour {d} off refuses at its clearance, got {other:?}"),
        }
    }
}

/// **An arc inside a chain of lines and arcs keeps its refusal.** A
/// rounded outline's top edges — two lines, an arc, two lines — make one
/// chain whose arc is a plane–cylinder link beside plane–plane ones; the
/// strip and the frustum would meet at a junction, which is not built,
/// so the chain refuses at its arc.
#[test]
fn an_arc_between_lines_refuses_at_the_arc() {
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let body = prism_at(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.0, 1.0), q),
            (Point2::new(1.0, 2.0), 0.0),
            (Point2::new(0.0, 2.0), 0.0),
        ],
        0.0,
        1.0,
        tol(),
    );
    let top: Vec<EdgeKey> = topo::query::all_edges(&body)
        .into_iter()
        .filter(|&e| {
            let (a, b) = body.edge_vertices(e).unwrap();
            let z = |v| body.get_point(body.get_vertex(v).unwrap().point).unwrap().z;
            z(a) > 0.5 && z(b) > 0.5
        })
        .collect();
    let arc: Vec<EdgeKey> = top
        .iter()
        .copied()
        .filter(|&e| {
            let c = body.get_edge(e).and_then(|d| body.get_curve_geom(d.curve));
            matches!(
                c.and_then(|g| g.certified()).map(|c| c.carrier()),
                Some(geom::Curve3::Circle(_))
            )
        })
        .collect();
    assert_eq!(arc.len(), 1, "the outline's one top arc");
    let got = chamfer(&body, &top, 0.1);
    assert!(
        matches!(got, Err(BlendError::ChamferArmUnsupported { edge, .. }) if edge == arc[0]),
        "the chain refuses at its arc, got {got:?}"
    );
}
