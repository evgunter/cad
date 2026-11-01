//! **A section ellipse nearly along a cylinder's axis splits at every ε
//! row**, once each chord seams it at the principal vertex nearest its
//! arc. A plane `θ` off the axis cuts an ellipse of semi-axis about
//! `r/θ` along it, and the face's part of it sits at its minor vertices:
//! seamed at a major vertex, the parameter there resolves only to
//! `ulp(π/2)·r/θ`, 7.3e-12 at `θ = 3e-5`, and the chord's ends missed
//! their vertices in band at ε = 1e-12 whatever the vertex's offset.
//!
//! - **Planes through a seam vertex**, `(1, 0, 0)` or `(−1, 0, 1)`, with
//!   normal `(0, 1, θ)`, the vertex `c·ε` off along the normal: the
//!   dual review of PR 4810's coin-flip family, 66 poses.
//! - **Planes through the axis**, normal `(cos φ, sin φ, θ)`,
//!   `θ = 1e-6 … 3e-4`: 96 poses.
//!
//! **Every split side is validated at rest**, and each row states every
//! pose's outcome exactly: both sides valid, or the refusal named in
//! its table, so a pose that flips either way reds the row. The split
//! gates its own sides at rest and refuses on the first side tier 3
//! refuses (`at-rest gate <side> <findings>`). Those refusals are tier
//! 3's frame check on a section ellipse
//! (`UnrepresentableCurveDatum { Ellipse, URef, Length | Tilt }`): its
//! margin is `ε − δ·A`, the frame's own rounding `δ` amplified by the
//! semi-axis `A ≈ r/θ`, up to 1e6 here ([`FRAME_RESIDUE`]). At
//! ε = 1e-6 the `θ = 3e-5` coin-flip planes cut a wall sliver of in-band
//! width at the seam ruling, and each side builds: the split merges the
//! sliver into the wall face across that seam, so no side keeps it
//! ([`SLIVER_RESIDUE`]). Upstream refusals are the input's readings at
//! that row: in band of along the axis, or a sector at the vertex in
//! band. No pose at any row refuses at a chord's attachment.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use topo::splitting::{SplitError, SplitFinishError, split};
use topo::{AtRestBody, Body, BooleanError, SplitJoinError, ValidationError};

/// The row that owns the at-rest frame refusals these tables name.
const FRAME_RESIDUE: &str = "work/restfront/tier-3s-frame-check-reads-a-section-ellipses-rounding-amplified-by-its-semi-axis.md";

/// The row that owns the at-rest volume refusal of an in-band curved
/// sliver. No pose here keeps one: each sliver lies beside the wall face
/// across a seam, and the split merges the two.
const SLIVER_RESIDUE: &str =
    "work/cleave/a-split-leaves-an-in-band-sliver-wall-face-that-refuses-only-at-rest.md";

/// The unit cylinder about `z`, `0 ≤ z ≤ 1`, its seam vertices at
/// `(±1, 0, 0)` and `(±1, 0, 1)`.
fn cylinder(tol: Tol) -> AtRestBody<f64> {
    AtRestBody::validate(
        sweep::test_support::prism(
            vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)],
            1.0,
            tol,
        ),
        tol,
    )
    .expect("a valid operand")
}

fn cut(
    body: &AtRestBody<f64>,
    origin: Point3<f64>,
    normal: Vec3<f64>,
    tol: Tol,
) -> Result<topo::SplitResult<f64>, SplitError> {
    let plane = topo::test_support::split_plane(origin, normal / normal.norm(), tol);
    split(body, &plane, tol)
}

/// Why tier 3 refuses one side, or `None` where it is valid at rest.
fn at_rest(body: &Body<f64>, tol: Tol) -> Option<String> {
    Some(reasons(&AtRestBody::validate(body.clone(), tol).err()?))
}

/// Tier 3's findings on one side: a frame finding by its datum, measure
/// and end, any other by its variant.
fn reasons(errors: &[ValidationError]) -> String {
    errors
        .iter()
        .map(|e| match e {
            ValidationError::UnrepresentableCurveDatum {
                kind: geom::CurveKind::Ellipse,
                datum,
                measure,
                end,
                ..
            } => format!("{datum:?} {measure:?} {end:?}"),
            other => variant(&format!("{other:?}"), 1),
        })
        .collect::<Vec<_>>()
        .join(" & ")
}

/// A pose's outcome: `ok` where both sides are valid at rest, else each
/// refused side and why, or the split's own refusal by its stage and
/// variant. The split's at-rest gate refuses on the first side tier 3
/// refuses (above before below), named with that side's findings.
fn outcome(r: Result<topo::SplitResult<f64>, SplitError>, tol: Tol) -> String {
    match r {
        Ok(res) => {
            let sides: Vec<String> = [("above", &res.above), ("below", &res.below)]
                .into_iter()
                .filter_map(|(side, part)| Some(format!("{side} {}", at_rest(part.body()?, tol)?)))
                .collect();
            if sides.is_empty() {
                "ok".to_owned()
            } else {
                sides.join(", ")
            }
        }
        Err(SplitError::Join(SplitJoinError::Euler(e))) => format!("chord {e:?}"),
        Err(SplitError::Finish(SplitFinishError::ResultNotAtRest { side, errors })) => {
            format!("at-rest gate {} {}", side.word(), reasons(&errors))
        }
        Err(e) => format!("refused {}", variant(&format!("{e:?}"), 2)),
    }
}

/// The first `depth` variant names of a `Debug` rendering.
fn variant(text: &str, depth: usize) -> String {
    text.split(['(', ' ', '{'])
        .filter(|w| !w.is_empty())
        .take(depth)
        .collect::<Vec<_>>()
        .join(" ")
}

fn is_eps(eps: f64, row: f64) -> bool {
    (eps - row).abs() <= 1e-3 * row
}

/// The table for this run's ε row.
fn row<'a>(eps: f64, rows: [&'a [(&'a str, &'a str)]; 3]) -> &'a [(&'a str, &'a str)] {
    if is_eps(eps, 1e-6) {
        rows[0]
    } else if is_eps(eps, 1e-9) {
        rows[1]
    } else {
        rows[2]
    }
}

/// Every pose not in `want` is `ok`; every pose in it is as it says.
fn check(got: &[(String, String)], want: &[(&str, &str)]) {
    let mut wrong = Vec::new();
    for (tag, out) in got {
        let expected = want
            .iter()
            .find(|(t, _)| t == tag)
            .map_or("ok", |(_, w)| *w);
        if out != expected {
            wrong.push(format!("{tag}: {out} (want {expected})"));
        }
    }
    for (tag, _) in want {
        assert!(
            got.iter().any(|(t, _)| t == tag),
            "the table names a pose the row never cuts: {tag}"
        );
    }
    let table: Vec<String> = got
        .iter()
        .filter(|(_, o)| o != "ok")
        .map(|(t, o)| format!("(\"{t}\", \"{o}\"),"))
        .collect();
    assert!(
        wrong.is_empty(),
        "poses off their table (at-rest frame refusals are {FRAME_RESIDUE}'s, \
         at-rest volume refusals {SLIVER_RESIDUE}'s):\n{}\n\nthe run's table:\n{}",
        wrong.join("\n"),
        table.join("\n"),
    );
}

/// The coin-flip family at each row.
#[test]
fn planes_nearly_along_the_axis_through_a_seam_vertex_split() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let cyl = cylinder(tol);
    let mut got = Vec::new();
    for th in [3e-5, 1e-4, 3e-4] {
        for (vx, vz) in [(1.0, 0.0), (-1.0, 1.0)] {
            for c in [
                0.0, 0.5, -0.5, 0.8, -0.8, 0.9, -0.9, 0.99, -0.99, 50.0, -50.0,
            ] {
                let n = Vec3::new(0.0, 1.0, th);
                let o = Point3::new(vx, 0.0, vz) + n / n.norm() * (c * eps);
                got.push((
                    format!("θ {th:e} ({vx}, {vz}) c {c}"),
                    outcome(cut(&cyl, o, n, tol), tol),
                ));
            }
        }
    }
    check(
        &got,
        row(eps, [COIN_FLIP_1E6, COIN_FLIP_1E9, COIN_FLIP_1E12]),
    );
}

/// The through-axis family at each row.
#[test]
fn planes_through_the_axis_nearly_along_it_split() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let cyl = cylinder(tol);
    let mut got = Vec::new();
    for th in [1e-6, 3e-6, 1e-5, 3e-5, 1e-4, 3e-4] {
        for k in 0..8 {
            let phi = 0.4 + f64::from(k) * 0.7;
            for z in [0.5, 0.3] {
                let n = Vec3::new(phi.cos(), phi.sin(), th);
                got.push((
                    format!("θ {th:e} φ {phi:.1} z {z}"),
                    outcome(cut(&cyl, Point3::new(0.0, 0.0, z), n, tol), tol),
                ));
            }
        }
    }
    check(&got, row(eps, [THROUGH_1E6, THROUGH_1E9, THROUGH_1E12]));
}

/// **A section ellipse reads back its major direction**, wherever its
/// seam is: a chord nearly along the axis is seamed at a minor vertex,
/// across the axis, and `edge_pose`'s `u_ref` is still the direction
/// along it.
#[test]
fn a_near_axial_sections_readback_u_ref_is_its_major_direction() {
    let tol = Tol::witness();
    let cyl = cylinder(tol);
    let res = cut(
        &cyl,
        Point3::new(0.0, 0.0, 0.5),
        Vec3::new(0.4, 0.9, 1e-3),
        tol,
    )
    .expect("the plane splits the cylinder");
    let mut seamed_across = 0;
    for body in [res.above.body(), res.below.body()].into_iter().flatten() {
        for (edge, _) in body.edges() {
            let Ok(&geom::Curve3::Ellipse { u_ref, .. }) =
                topo::readback::edge_carrier_ref(body, edge)
            else {
                continue;
            };
            if u_ref.z.abs() < 0.5 {
                seamed_across += 1;
            }
            let pose = topo::readback::edge_pose(body, edge).expect("an ellipse has a pose");
            let major = pose.u_ref.expect("an ellipse fixes a u_ref");
            assert!(
                major.z.abs() > 0.999,
                "edge {edge:?}: u_ref {major:?} is not along the axis"
            );
        }
    }
    assert!(
        seamed_across > 0,
        "no section chord is seamed across the axis: the row reads nothing"
    );
}

/// **FLUX's quadrature pose builds**: the cylinder against a slab whose
/// inner face is tilted `±1e-2` about `y` through a point just inside
/// the seam ruling, in both orders, for ∪, ∩ and ∖, at every row.
#[test]
fn a_slab_face_nearly_along_a_ruling_builds() {
    type Op = fn(
        &AtRestBody<f64>,
        &AtRestBody<f64>,
        Tol,
    ) -> Result<topo::BooleanResult<f64>, BooleanError>;
    let tol = Tol::witness();
    let eps = tol.eps();
    let cyl = cylinder(tol);
    let rect = |x0: f64, y0: f64, x1: f64, y1: f64| {
        [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
            .map(|(x, y)| (Point2::new(x, y), 0.0))
            .to_vec()
    };
    let ops: [(&str, Op); 3] = [
        ("∪", topo::union),
        ("∩", topo::intersect),
        ("∖", topo::subtract),
    ];
    let mut wrong = Vec::new();
    for dd in [-50.0 * eps, -1e3 * eps] {
        for t in [1e-2, -1e-2] {
            let slab = AtRestBody::validate(
                sweep::test_support::prism_at(rect(1.0 + dd, -2.0, 3.0, 2.0), 0.25, 0.5, tol),
                tol,
            )
            .expect("a valid slab");
            let turn = Affine3::rotation_about_axis(
                Point3::new(1.0 + dd, 0.0, 0.5),
                Vec3::new(0.0, 1.0, 0.0),
                t,
            );
            let slab = AtRestBody::validate(
                topo::transform_rigid(&slab, &geom_core::test_support::rigid(&turn, tol), tol)
                    .expect("a rigid turn"),
                tol,
            )
            .expect("a valid tilted slab");
            for (order, a, b) in [("cyl, slab", &cyl, &slab), ("slab, cyl", &slab, &cyl)] {
                for (name, op) in ops {
                    if let Err(e) = op(a, b, tol) {
                        wrong.push(format!("dd {dd:e} t {t:e} {name}({order}): {e:?}"));
                    }
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

const COIN_FLIP_1E6: &[(&str, &str)] = &[];
const COIN_FLIP_1E9: &[(&str, &str)] = &[
    ("θ 1e-4 (1, 0) c 0", "refused Reduce SliverSector"),
    ("θ 1e-4 (1, 0) c 0.5", "refused Reduce SliverSector"),
    ("θ 1e-4 (1, 0) c 0.8", "refused Reduce SliverSector"),
    ("θ 1e-4 (1, 0) c 0.9", "refused Reduce SliverSector"),
    ("θ 1e-4 (1, 0) c 0.99", "refused Reduce SliverSector"),
    ("θ 1e-4 (1, 0) c 50", "refused Reduce SliverSector"),
    ("θ 1e-4 (-1, 1) c 0", "refused Reduce SliverSector"),
    ("θ 1e-4 (-1, 1) c -0.5", "refused Reduce SliverSector"),
    ("θ 1e-4 (-1, 1) c -0.8", "refused Reduce SliverSector"),
    ("θ 1e-4 (-1, 1) c -0.9", "refused Reduce SliverSector"),
    ("θ 1e-4 (-1, 1) c -0.99", "refused Reduce SliverSector"),
    ("θ 1e-4 (-1, 1) c -50", "refused Reduce SliverSector"),
];
const COIN_FLIP_1E12: &[(&str, &str)] = &[];
const THROUGH_1E6: &[(&str, &str)] = &[
    ("θ 1e-6 φ 0.4 z 0.5", "refused Reduce SliverSector"),
    ("θ 1e-6 φ 0.4 z 0.3", "refused Reduce SliverSector"),
    ("θ 1e-6 φ 1.1 z 0.5", "refused Join Escalated"),
    ("θ 1e-6 φ 1.1 z 0.3", "refused Join Escalated"),
    ("θ 1e-6 φ 1.8 z 0.5", "refused Join Escalated"),
    ("θ 1e-6 φ 1.8 z 0.3", "refused Join Escalated"),
    ("θ 1e-6 φ 2.5 z 0.5", "refused Join Escalated"),
    ("θ 1e-6 φ 2.5 z 0.3", "refused Join Escalated"),
    ("θ 1e-6 φ 3.2 z 0.5", "refused Reduce SliverSector"),
    ("θ 1e-6 φ 3.2 z 0.3", "refused Reduce SliverSector"),
    ("θ 1e-6 φ 3.9 z 0.5", "refused Join Escalated"),
    ("θ 1e-6 φ 3.9 z 0.3", "refused Join Escalated"),
    ("θ 1e-6 φ 4.6 z 0.5", "refused Join Escalated"),
    ("θ 1e-6 φ 4.6 z 0.3", "refused Join Escalated"),
    ("θ 1e-6 φ 5.3 z 0.5", "refused Join Escalated"),
    ("θ 1e-6 φ 5.3 z 0.3", "refused Join Escalated"),
    ("θ 3e-6 φ 0.4 z 0.5", "refused Reduce SliverSector"),
    ("θ 3e-6 φ 0.4 z 0.3", "refused Reduce SliverSector"),
    ("θ 3e-6 φ 1.1 z 0.5", "refused Reduce SliverSector"),
    ("θ 3e-6 φ 1.1 z 0.3", "refused Reduce SliverSector"),
    ("θ 3e-6 φ 1.8 z 0.5", "refused Join Escalated"),
    ("θ 3e-6 φ 1.8 z 0.3", "refused Join Escalated"),
    ("θ 3e-6 φ 2.5 z 0.5", "refused Reduce SliverSector"),
    ("θ 3e-6 φ 2.5 z 0.3", "refused Reduce SliverSector"),
    ("θ 3e-6 φ 3.2 z 0.5", "refused Reduce SliverSector"),
    ("θ 3e-6 φ 3.2 z 0.3", "refused Reduce SliverSector"),
    ("θ 3e-6 φ 3.9 z 0.5", "refused Reduce SliverSector"),
    ("θ 3e-6 φ 3.9 z 0.3", "refused Reduce SliverSector"),
    ("θ 3e-6 φ 4.6 z 0.5", "refused Join Escalated"),
    ("θ 3e-6 φ 4.6 z 0.3", "refused Join Escalated"),
    ("θ 3e-6 φ 5.3 z 0.5", "refused Reduce SliverSector"),
    ("θ 3e-6 φ 5.3 z 0.3", "refused Reduce SliverSector"),
    ("θ 1e-5 φ 0.4 z 0.5", "refused Reduce SliverSector"),
    ("θ 1e-5 φ 0.4 z 0.3", "refused Reduce SliverSector"),
    ("θ 1e-5 φ 1.1 z 0.5", "refused Reduce SliverSector"),
    ("θ 1e-5 φ 1.1 z 0.3", "refused Reduce SliverSector"),
    ("θ 1e-5 φ 1.8 z 0.5", "refused Reduce SliverSector"),
    ("θ 1e-5 φ 1.8 z 0.3", "refused Reduce SliverSector"),
    ("θ 1e-5 φ 2.5 z 0.5", "refused Reduce SliverSector"),
    ("θ 1e-5 φ 2.5 z 0.3", "refused Reduce SliverSector"),
    ("θ 1e-5 φ 3.2 z 0.5", "refused Reduce SliverSector"),
    ("θ 1e-5 φ 3.2 z 0.3", "refused Reduce SliverSector"),
    ("θ 1e-5 φ 3.9 z 0.5", "refused Reduce SliverSector"),
    ("θ 1e-5 φ 3.9 z 0.3", "refused Reduce SliverSector"),
    ("θ 1e-5 φ 4.6 z 0.5", "refused Reduce SliverSector"),
    ("θ 1e-5 φ 4.6 z 0.3", "refused Reduce SliverSector"),
    ("θ 1e-5 φ 5.3 z 0.5", "refused Reduce SliverSector"),
    ("θ 1e-5 φ 5.3 z 0.3", "refused Reduce SliverSector"),
    ("θ 3e-5 φ 0.4 z 0.3", "refused Reduce SliverSector"),
    ("θ 3e-5 φ 1.1 z 0.5", "refused Reduce SliverSector"),
    ("θ 3e-5 φ 1.1 z 0.3", "refused Reduce SliverSector"),
    ("θ 3e-5 φ 1.8 z 0.5", "refused Reduce SliverSector"),
    ("θ 3e-5 φ 1.8 z 0.3", "refused Reduce SliverSector"),
    ("θ 3e-5 φ 2.5 z 0.3", "refused Reduce SliverSector"),
    ("θ 3e-5 φ 3.9 z 0.3", "refused Reduce SliverSector"),
    ("θ 3e-5 φ 4.6 z 0.5", "refused Reduce SliverSector"),
    ("θ 3e-5 φ 4.6 z 0.3", "refused Reduce SliverSector"),
    ("θ 3e-5 φ 5.3 z 0.5", "refused Reduce SliverSector"),
    ("θ 3e-5 φ 5.3 z 0.3", "refused Reduce SliverSector"),
    ("θ 1e-4 φ 1.8 z 0.3", "refused Reduce SliverSector"),
    ("θ 1e-4 φ 4.6 z 0.5", "refused Reduce SliverSector"),
    ("θ 1e-4 φ 4.6 z 0.3", "refused Reduce SliverSector"),
];
const THROUGH_1E9: &[(&str, &str)] = &[];
const THROUGH_1E12: &[(&str, &str)] = &[
    (
        "θ 1e-6 φ 3.9 z 0.5",
        "at-rest gate above URef Length Upper & URef Length Upper",
    ),
    (
        "θ 1e-6 φ 3.9 z 0.3",
        "at-rest gate above URef Length Upper & URef Length Upper",
    ),
    (
        "θ 1e-6 φ 5.3 z 0.5",
        "at-rest gate above URef Tilt Lower & URef Tilt Upper",
    ),
    (
        "θ 1e-6 φ 5.3 z 0.3",
        "at-rest gate above URef Tilt Lower & URef Tilt Upper",
    ),
    (
        "θ 3e-6 φ 1.1 z 0.5",
        "at-rest gate above URef Length Upper & URef Length Upper",
    ),
    (
        "θ 3e-6 φ 1.1 z 0.3",
        "at-rest gate above URef Length Upper & URef Length Upper",
    ),
    (
        "θ 3e-6 φ 1.8 z 0.5",
        "at-rest gate above URef Tilt Upper & URef Tilt Lower",
    ),
    (
        "θ 3e-6 φ 1.8 z 0.3",
        "at-rest gate above URef Tilt Upper & URef Tilt Lower",
    ),
    (
        "θ 3e-6 φ 2.5 z 0.5",
        "at-rest gate above URef Length Lower & URef Length Lower",
    ),
    (
        "θ 3e-6 φ 2.5 z 0.3",
        "at-rest gate above URef Length Lower & URef Length Lower",
    ),
    (
        "θ 3e-6 φ 3.9 z 0.5",
        "at-rest gate above URef Tilt Upper & URef Tilt Lower",
    ),
    (
        "θ 3e-6 φ 3.9 z 0.3",
        "at-rest gate above URef Tilt Upper & URef Tilt Lower",
    ),
    (
        "θ 3e-6 φ 5.3 z 0.5",
        "at-rest gate above URef Tilt Upper & URef Tilt Lower",
    ),
    (
        "θ 3e-6 φ 5.3 z 0.3",
        "at-rest gate above URef Tilt Upper & URef Tilt Lower",
    ),
    (
        "θ 1e-5 φ 0.4 z 0.5",
        "at-rest gate above URef Length Lower & URef Length Lower",
    ),
    (
        "θ 1e-5 φ 0.4 z 0.3",
        "at-rest gate above URef Length Lower & URef Length Lower",
    ),
    (
        "θ 1e-5 φ 2.5 z 0.5",
        "at-rest gate above URef Length Lower & URef Length Lower",
    ),
    (
        "θ 1e-5 φ 2.5 z 0.3",
        "at-rest gate above URef Length Lower & URef Length Lower",
    ),
    (
        "θ 3e-5 φ 0.4 z 0.5",
        "at-rest gate above URef Tilt Upper & URef Tilt Lower",
    ),
    (
        "θ 3e-5 φ 0.4 z 0.3",
        "at-rest gate above URef Tilt Upper & URef Tilt Lower",
    ),
    (
        "θ 3e-5 φ 1.1 z 0.5",
        "at-rest gate above URef Length Lower & URef Length Lower",
    ),
    (
        "θ 3e-5 φ 1.1 z 0.3",
        "at-rest gate above URef Length Lower & URef Length Lower",
    ),
    (
        "θ 3e-5 φ 2.5 z 0.5",
        "at-rest gate above URef Length Lower & URef Length Lower",
    ),
    (
        "θ 3e-5 φ 2.5 z 0.3",
        "at-rest gate above URef Length Lower & URef Length Lower",
    ),
    (
        "θ 3e-5 φ 3.9 z 0.5",
        "at-rest gate above URef Tilt Lower & URef Tilt Upper",
    ),
    (
        "θ 3e-5 φ 3.9 z 0.3",
        "at-rest gate above URef Tilt Lower & URef Tilt Upper",
    ),
    (
        "θ 3e-5 φ 5.3 z 0.5",
        "at-rest gate above URef Length Lower & URef Length Lower",
    ),
    (
        "θ 3e-5 φ 5.3 z 0.3",
        "at-rest gate above URef Length Lower & URef Length Lower",
    ),
    (
        "θ 1e-4 φ 1.1 z 0.5",
        "at-rest gate above URef Length Lower & URef Length Lower",
    ),
    (
        "θ 1e-4 φ 1.1 z 0.3",
        "at-rest gate above URef Length Lower & URef Length Lower",
    ),
    (
        "θ 1e-4 φ 2.5 z 0.5",
        "at-rest gate above URef Length Lower & URef Length Lower",
    ),
    (
        "θ 1e-4 φ 2.5 z 0.3",
        "at-rest gate above URef Length Lower & URef Length Lower",
    ),
];
