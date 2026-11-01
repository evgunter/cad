//! **A cylinder through a holed plate, its wall tangent to the hole's
//! along a ruling.** The cylinder `r = 0.3`, `z ∈ [−1, 1]`, passes
//! through the plate `[−1, 1]² × [−0.5, 0]`, which a hole of radius `r`
//! pierces tangent to the cylinder's wall along the ruling through
//! `(0.3, 0)`: outside it, about `(0.3 + r, 0)`, or inside it, about
//! `(0.3 − r, 0)`. The plate's top and bottom faces each cut the wall in
//! a circle that touches the hole's rim at the ruling.
//!
//! A one-segment cylinder's wall is one face, whose wrap edge is the
//! ruling at its vertex. With that vertex at the tangent ruling, each
//! section circle crosses the wrap edge once, at a vertex of the plate
//! face's boundary: a loop with one site, whose null edge hangs from the
//! hole's rim into the face as a strut. The join closes it there
//! (`boolean::join`'s `closes_at_its_site`, a wrap edge against a vertex
//! of a planar face's boundary), walling the circle off in the plate
//! face from the rim's vertex round to itself (`chord_join`'s
//! `strut_half` and `place_strut`). Outside, the circle walls off the
//! disc inside it and the face keeps the hole's ring and the circle's as
//! one ring; inside, the circle encloses the hole, so the disc it walls
//! off holds the face's outer loop and becomes the face's ring, and the
//! hole's ring with the circle's is the crescent between them.
//!
//! With the hole's vertex at the ruling too, its rim is a closed edge
//! tangent to the section circle there; the germ runs into the plate's
//! face, not along the rim, which leaves the cylinder's wall
//! (`boolean::sectors`'s `closed_on_partner`).
//!
//! With the hole's vertex alone at the ruling, it pierces the cylinder's
//! wall there, and its rim, closed at the vertex, is read the same way
//! against the wall (the vertex-on-face pass's germs).
//!
//! With either vertex alone at the ruling, that operand's wrap edge lies
//! along the other's wall from the top face to the bottom: the op that
//! keeps both walls would part the other's wall along it, and refuses
//! `EdgeInFace` with the `CurvedFace` gap, a slit in a curved face that
//! the chart walk does not mesh nor the curved mass properties measure.
//!
//! Every op runs in both operand orders, each run read through
//! `differential::outcome` (tiers 2 and 3′, the certificate, the legal
//! operand, the closed-form volume) and tier 3. Outside, the plate less
//! the cylinder pinches along the ruling, where its two walls touch.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_core::{Affine3, Arc2, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanError, BooleanResult};

use crate::common::differential::{every_op_both_orders, outcome};
use crate::common::poses::torax_pose;

fn tol() -> Tol {
    Tol::witness()
}

/// The cylinder's radius, about the `z` axis.
const R: f64 = 0.3;
/// The cylinder's foot and height.
const CYLINDER: (f64, f64) = (-1.0, 2.0);
/// The plate's foot and thickness, under `[−1, 1]²`.
const PLATE: (f64, f64) = (-0.5, 0.5);

/// A circle of radius `r` about `(cx, cy)`: one segment with its vertex
/// at azimuth `az`, or two arcs with vertices there and opposite.
fn rim(one: bool, (cx, cy): (f64, f64), r: f64, az: f64) -> ProfileLoop<f64> {
    let arc = |a: f64, sweep: f64| {
        (
            Point2::new(cx + r * a.cos(), cy + r * a.sin()),
            Segment::Arc(Arc2 {
                centre: Point2::new(cx, cy),
                radius: r,
                sweep,
            }),
        )
    };
    if one {
        RawLoop::new([arc(az, TAU)])
    } else {
        RawLoop::new([arc(az, PI), arc(az + PI, PI)])
    }
}

/// The plate's outline, `[−1, 1]²`.
fn square() -> ProfileLoop<f64> {
    RawLoop::polygon([
        Point2::new(-1.0, -1.0),
        Point2::new(1.0, -1.0),
        Point2::new(1.0, 1.0),
        Point2::new(-1.0, 1.0),
    ])
}

/// `loops` extruded from `z = z0` up by `h`, turned by `turn`.
fn extruded(loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64, turn: &Affine3<f64>) -> AtRestBody<f64> {
    let profile = Profile::new(SketchPlane::<f64>::xy(), loops)
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: h,
        side: ExtrudeSide::Along,
    };
    let body = extrude(&profile, depth, tol()).unwrap().body;
    let placed = *turn * Affine3::translation(Vec3::new(0.0, 0.0, z0));
    let body = topo::transform_rigid(
        &body,
        &geom_core::test_support::rigid(&placed, tol()),
        tol(),
    )
    .unwrap();
    topo::test_support::finished("an operand", body, tol())
}

/// Where the tangent ruling's vertices are.
#[derive(Clone, Copy, Debug)]
enum Vertices {
    /// Both rims one segment, each vertex on the ruling.
    BothThere,
    /// Both rims one segment, the cylinder's vertex on the ruling and
    /// the hole's opposite it.
    CylindersAlone,
    /// Both rims one segment, the hole's vertex on the ruling and the
    /// cylinder's at azimuth 1.
    HolesAlone,
    /// Both rims two arcs, a vertex of each on the ruling.
    TwoArcs,
}

/// One run's verdict: a body `SOUND` at its closed form that passes
/// tier 3, or, where `slit` says so, the typed refusal of the cylinder's
/// wrap edge lying along the hole's wall.
fn holds(row: &str, r: Result<BooleanResult<f64>, BooleanError>, want: f64, slit: bool) {
    if slit {
        // Pinned red: the `CurvedFace` gap retires when the walk meshes a
        // slit in a curved face
        // (`work/tesslie/the-walk-panics-on-a-slit-in-a-curved-face.md`)
        // and its mass properties measure one
        // (`work/nearpoint/a-zero-area-ring-in-a-cylinder-face-measures-degenerate.md`).
        assert!(
            matches!(
                r,
                Err(BooleanError::EdgeInFace {
                    gap: topo::EdgeInFaceGap::CurvedFace,
                    ..
                })
            ),
            "{row}: {r:?}"
        );
        return;
    }
    if let Ok(BooleanResult::Body(b)) = &r {
        topo::validate_geometric(&b.body, tol()).unwrap_or_else(|e| panic!("{row}: tier 3: {e:?}"));
    }
    let line = outcome(r, want, tol());
    assert!(line.starts_with("OK SOUND"), "{row}: {line}");
}

/// The cylinder through the plate holed by radius `hr`, `inside` or
/// outside its wall, vertices as `vertices` says, and by one-segment
/// holes `others` `(centre, radius)` clear of both walls, all turned by
/// `turn`: every op in both orders.
fn through(
    hr: f64,
    inside: bool,
    vertices: Vertices,
    others: &[((f64, f64), f64)],
    turn: &Affine3<f64>,
) {
    let (hx, hole_at) = if inside { (R - hr, 0.0) } else { (R + hr, PI) };
    let (one, hole_az, cylinder_az) = match vertices {
        Vertices::BothThere => (true, hole_at, 0.0),
        Vertices::CylindersAlone => (true, hole_at + PI, 0.0),
        Vertices::HolesAlone => (true, hole_at, 1.0),
        Vertices::TwoArcs => (false, hole_at, 0.0),
    };
    let what = format!(
        "hole r = {hr} {}, {vertices:?}, {} more",
        if inside { "inside" } else { "outside" },
        others.len()
    );
    let cylinder = extruded(
        vec![rim(one, (0.0, 0.0), R, cylinder_az)],
        CYLINDER.0,
        CYLINDER.1,
        turn,
    );
    let mut loops = vec![square(), rim(one, (hx, 0.0), hr, hole_az)];
    loops.extend(others.iter().map(|&(c, r)| rim(true, c, r, 0.0)));
    let plate = extruded(loops, PLATE.0, PLATE.1, turn);
    let disc = |r: f64| PI * r * r;
    let in_cylinder = |&((x, y), r): &((f64, f64), f64)| x.hypot(y) + r < R;
    let others_in: f64 = others
        .iter()
        .filter(|o| in_cylinder(o))
        .map(|o| disc(o.1))
        .sum();
    let others_all: f64 = others.iter().map(|o| disc(o.1)).sum();
    let va = disc(R) * CYLINDER.1;
    let vb = (4.0 - disc(hr) - others_all) * PLATE.1;
    let hole_in = if inside { disc(hr) } else { 0.0 };
    let vab = (disc(R) - hole_in - others_in) * PLATE.1;
    for (op, r, want) in every_op_both_orders(&cylinder, &plate, (va, vb, vab), tol()) {
        // The op that keeps the cylinder's wall along the hole's.
        let both_walls = matches!((inside, op), (false, "B ∖ A") | (true, "A ∩ B" | "B ∩ A"));
        let slit =
            both_walls && matches!(vertices, Vertices::CylindersAlone | Vertices::HolesAlone);
        holds(&format!("{what}: {op}"), r, want, slit);
    }
}

/// **By pose, at rest**: holes `r = 0.1` and `0.15`, outside the wall
/// and inside it, both vertices on the ruling, the cylinder's alone, the
/// hole's alone, and the two-arc form. Each op builds at its closed
/// form, but, with the vertex of one alone there, the op keeping both
/// walls, which refuses the slit its wrap edge would leave in the
/// other's wall.
#[test]
fn a_cylinder_through_a_hole_tangent_to_its_wall_by_pose() {
    let rest = Affine3::identity();
    for hr in [0.1, 0.15] {
        for inside in [false, true] {
            for vertices in [
                Vertices::BothThere,
                Vertices::CylindersAlone,
                Vertices::HolesAlone,
                Vertices::TwoArcs,
            ] {
                through(hr, inside, vertices, &[], &rest);
            }
        }
    }
}

/// **The one-segment poses turned** rigidly off the axes
/// ([`torax_pose`]): each reads what it reads at rest.
#[test]
fn a_cylinder_through_a_hole_tangent_to_its_wall_turned() {
    let turn = torax_pose();
    for inside in [false, true] {
        for vertices in [
            Vertices::BothThere,
            Vertices::CylindersAlone,
            Vertices::HolesAlone,
        ] {
            through(0.1, inside, vertices, &[], &turn);
        }
    }
}

/// **Beside other holes**: the plate holed besides by `r = 0.05` inside
/// the cylinder's wall, about `(−0.15, 0.05)`, and by `r = 0.1` outside it,
/// about `(−0.6, 0.5)`. Each plate face's section circle walls off the
/// region it encloses, which takes the first hole's ring and leaves the
/// face the second's: the disc where the tangent hole lies outside the
/// wall, the crescent about it where it lies inside.
#[test]
fn a_cylinder_through_a_hole_tangent_to_its_wall_beside_other_holes() {
    let rest = Affine3::identity();
    let others = [((-0.15, 0.05), 0.05), ((-0.6, 0.5), 0.1)];
    for inside in [false, true] {
        for vertices in [Vertices::BothThere, Vertices::TwoArcs] {
            through(0.1, inside, vertices, &others, &rest);
        }
    }
}
