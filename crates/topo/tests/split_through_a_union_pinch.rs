//! **A split through a union's pinch fuses its cones on a side whose
//! section joins them.**
//!
//! The L-prism ∪ the tilted cube pinches at the L's reflex corner
//! (1, 1, 1): two vertices on one point key, one per cone. The plane
//! x = 1 passes through the corner and between the cones, and on the
//! side x ≥ 1 the material joins them there: each of the section's two
//! corners at the point (the prism's quadrant and the cube's half-plane)
//! runs from a germ of one cone to a germ of the other. Each side keeps
//! one vertex at the corner, measures at its closed form, and passes
//! tier 3 and tier 3′, either way up.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{finished, split_plane};
use geom_core::{Point3, Tol, Vec3};
use topo::{
    AtRestBody, BooleanBody, SplitPart, split, validate_geometric, validate_pseudomanifold,
};

fn t() -> Tol {
    Tol::witness()
}

/// The L's reflex top corner, where the cube's bottom face meets it.
pub(crate) const PINCH_AT: [f64; 3] = [1.0, 1.0, 1.0];

/// The normal of the cube's bottom face, unnormalised.
pub(crate) const CUBE_NORMAL: [f64; 3] = [1.0, 1.3, 0.7];

pub(crate) fn unit(v: [f64; 3]) -> [f64; 3] {
    let n = v.iter().map(|c| c * c).sum::<f64>().sqrt();
    v.map(|c| c / n)
}

pub(crate) fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub(crate) fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// The cube's frame: its bottom face's normal `m`, and `u` (the tilt of
/// +z into that face) and `w = m × u` spanning it.
pub(crate) fn cube_frame() -> [[f64; 3]; 3] {
    frame_along(CUBE_NORMAL)
}

/// [`cube_frame`] for the bottom face's normal `normal`.
fn frame_along(normal: [f64; 3]) -> [[f64; 3]; 3] {
    let m = unit(normal);
    let u = unit([-m[2] * m[0], -m[2] * m[1], 1.0 - m[2] * m[2]]);
    [m, u, cross(m, u)]
}

/// The prism over the L, a unit tall.
pub(crate) fn ell() -> AtRestBody<f64> {
    let prism = common::prism::<f64>(
        &[
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ],
        1.0,
        t(),
    );
    finished("the prism", prism.body, t())
}

/// The cube of side 4 whose bottom face lies on the plane through
/// [`PINCH_AT`] with normal [`CUBE_NORMAL`], on its side, centred on the
/// corner there.
pub(crate) fn cube() -> AtRestBody<f64> {
    cube_on(cube_frame())
}

/// [`cube`] on the frame `[m, u, w]`.
fn cube_on([m, u, w]: [[f64; 3]; 3]) -> AtRestBody<f64> {
    let v = PINCH_AT;
    let body = common::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (4.0 * (x - 0.5), 4.0 * (y - 0.5), 4.0 * z);
            Point3::new(
                v[0] + a * u[0] + b * w[0] + c * m[0],
                v[1] + a * u[1] + b * w[1] + c * m[1],
                v[2] + a * u[2] + b * w[2] + c * m[2],
            )
        },
        t(),
    );
    finished("the cube", body, t())
}

/// A halfspace `n · x ≤ c`.
pub(crate) type Half = ([f64; 3], f64);

/// The box `[lo, hi]` as halfspaces.
pub(crate) fn box_halves(lo: [f64; 3], hi: [f64; 3]) -> Vec<Half> {
    (0..3)
        .flat_map(|i| {
            let mut e = [0.0; 3];
            e[i] = 1.0;
            [(e, hi[i]), (e.map(|c| -c), -lo[i])]
        })
        .collect()
}

/// The cube as halfspaces.
pub(crate) fn cube_halves() -> Vec<Half> {
    let [m, u, w] = cube_frame();
    let at = |d: [f64; 3]| dot(d, PINCH_AT);
    let neg = |d: [f64; 3]| d.map(|c| -c);
    vec![
        (neg(m), -at(m)),
        (m, at(m) + 4.0),
        (u, at(u) + 2.0),
        (neg(u), 2.0 - at(u)),
        (w, at(w) + 2.0),
        (neg(w), 2.0 - at(w)),
    ]
}

/// The volume of the bounded convex polytope `halves` cut out, by the
/// divergence theorem over its faces: each face's polygon is the
/// polytope's vertices on its plane, taken round their centroid.
pub(crate) fn convex_volume(halves: &[Half]) -> f64 {
    // A plane listed twice bounds one face, counted once.
    let mut planes: Vec<Half> = Vec::new();
    for &(n, c) in halves {
        let k = dot(n, n).sqrt();
        let (n, c) = (n.map(|x| x / k), c / k);
        if !planes
            .iter()
            .any(|&(m, d)| (0..3).all(|r| (m[r] - n[r]).abs() < 1e-12) && (d - c).abs() < 1e-12)
        {
            planes.push((n, c));
        }
    }
    let halves = &planes[..];
    let inside = |p: [f64; 3]| halves.iter().all(|&(n, c)| dot(n, p) <= c + 1e-9);
    let mut points: Vec<[f64; 3]> = Vec::new();
    for i in 0..halves.len() {
        for j in i + 1..halves.len() {
            for k in j + 1..halves.len() {
                let (a, b, c) = (halves[i], halves[j], halves[k]);
                let det = dot(a.0, cross(b.0, c.0));
                if det.abs() < 1e-12 {
                    continue;
                }
                let (bc, ca, ab) = (cross(b.0, c.0), cross(c.0, a.0), cross(a.0, b.0));
                let p = [0, 1, 2].map(|q| (a.1 * bc[q] + b.1 * ca[q] + c.1 * ab[q]) / det);
                let known = points
                    .iter()
                    .any(|&q| (0..3).all(|r| (p[r] - q[r]).abs() < 1e-9));
                if inside(p) && !known {
                    points.push(p);
                }
            }
        }
    }
    let mut volume = 0.0;
    for &(n, c) in halves {
        let on: Vec<[f64; 3]> = points
            .iter()
            .copied()
            .filter(|&p| (dot(n, p) - c).abs() < 1e-9)
            .collect();
        if on.len() < 3 {
            continue;
        }
        let g = [0, 1, 2].map(|r| on.iter().map(|p| p[r]).sum::<f64>() / on.len() as f64);
        let nn = unit(n);
        let e = unit([0, 1, 2].map(|r| on[0][r] - g[r]));
        let f = cross(nn, e);
        let mut ring: Vec<(f64, [f64; 3])> = on
            .iter()
            .map(|&p| {
                let d = [0, 1, 2].map(|r| p[r] - g[r]);
                (dot(d, f).atan2(dot(d, e)), p)
            })
            .collect();
        ring.sort_by(|x, y| x.0.total_cmp(&y.0));
        let area: f64 = (0..ring.len())
            .map(|i| dot(nn, cross(ring[i].1, ring[(i + 1) % ring.len()].1)))
            .sum::<f64>()
            / 2.0;
        volume += c / dot(n, n).sqrt() * area / 3.0;
    }
    volume
}

/// The union's volume on each side of x = 1, `[x ≥ 1, x ≤ 1]`: the L
/// is the boxes `[0, 2] × [0, 1]` and `[0, 1] × [1, 2]`, which meet on
/// a face, so each side is the prism's part plus the cube's less their
/// overlap, every term a convex polytope.
fn halves_volumes() -> [f64; 2] {
    let boxes = [
        box_halves([0.0, 0.0, 0.0], [2.0, 1.0, 1.0]),
        box_halves([0.0, 1.0, 0.0], [1.0, 2.0, 1.0]),
    ];
    let cube = cube_halves();
    [([-1.0, 0.0, 0.0], -1.0), ([1.0, 0.0, 0.0], 1.0)].map(|cut: Half| {
        let with = |parts: &[&[Half]]| {
            let mut all = vec![cut];
            for p in parts {
                all.extend_from_slice(p);
            }
            convex_volume(&all)
        };
        boxes
            .iter()
            .map(|b| with(&[b]) - with(&[b, &cube]))
            .sum::<f64>()
            + with(&[&cube])
    })
}

/// The edges at [`PINCH_AT`] of [`wedge`], each inside the gap the L's
/// reflex notch leaves under the cube's bottom face (x, y > 1, z < 1,
/// below the face), in a right-handed order.
pub(crate) const WEDGE: [[f64; 3]; 3] = [[0.1, 0.1, -1.0], [0.1, 1.0, -2.5], [1.0, 0.1, -2.0]];

/// The parallelepiped on [`WEDGE`]'s edges scaled by 0.3 from the
/// corner: it touches the union only there, a third cone.
pub(crate) fn wedge() -> AtRestBody<f64> {
    let body = common::mapped_cube::<f64>(
        |x, y, z| {
            let q = [0, 1, 2]
                .map(|i| PINCH_AT[i] + 0.3 * (x * WEDGE[0][i] + y * WEDGE[1][i] + z * WEDGE[2][i]));
            Point3::new(q[0], q[1], q[2])
        },
        t(),
    );
    finished("the wedge", body, t())
}

fn united(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> BooleanBody<f64> {
    match topo::union(a, b, t()) {
        Ok(topo::BooleanResult::Body(b)) => b,
        other => panic!("the union builds: {other:?}"),
    }
}

/// The union split by x = 1 either way up, with and without the wedge
/// as a third cone: each side at its closed form, its vertices at the
/// corner (the cones it keeps apart there), tier 3, and tier 3′ against
/// the union's records carried through the split, a record of a fused
/// cone's vertex landing on the vertex that survives it.
#[test]
fn a_split_through_a_union_pinch_fuses_its_cones_on_the_side_that_joins_them() {
    let two = united(&ell(), &cube());
    let three = united(&two.body, &wedge());
    let band = geom_core::Band::linear(t()).expect("a band at the run's tolerance");
    let at_corner = |b: &AtRestBody<f64>| {
        b.vertex_points()
            .filter(|(_, p)| [p.x, p.y, p.z] == PINCH_AT)
            .count()
    };
    let [beyond, short] = halves_volumes();
    let wedge_volume = 0.027 * dot(WEDGE[0], cross(WEDGE[1], WEDGE[2]));
    // (what, the union, its cones, [x ≥ 1, x ≤ 1] as (volume, cones)).
    let cases = [
        ("two cones", &two, 2, [(beyond, 1), (short, 1)]),
        (
            "three cones",
            &three,
            3,
            [(beyond + wedge_volume, 2), (short, 1)],
        ),
    ];
    for (union, u, cones, sides) in cases {
        assert_eq!(at_corner(&u.body), cones, "{union}: one vertex per cone");
        for up in [true, false] {
            let s = if up { 1.0 } else { -1.0 };
            let plane = split_plane(Point3::new(1.0, 0.0, 0.0), Vec3::new(s, 0.0, 0.0), t());
            let r = split(&u.body, &plane, t())
                .unwrap_or_else(|e| panic!("{union}, up {up}: refused: {e:?}"));
            let carried = r
                .carry_contacts(&u.contacts, band, Some(topo::RegionLane::certified()))
                .unwrap_or_else(|e| panic!("{union}, up {up}: the records carry: {e:?}"));
            let parts = if up {
                [(&r.above, &carried.above), (&r.below, &carried.below)]
            } else {
                [(&r.below, &carried.below), (&r.above, &carried.above)]
            };
            for ((part, records), (want, vertices), what) in parts
                .into_iter()
                .zip(sides)
                .zip(["x ≥ 1", "x ≤ 1"])
                .map(|((p, s), w)| (p, s, w))
            {
                let what = format!(
                    "{union}, {what}, the normal {}",
                    if up { "+x" } else { "−x" }
                );
                let SplitPart::Body(b) = part else {
                    panic!("{what}: empty");
                };
                let v = topo::test_support::pinned_volume("the side", b, t()).volume;
                assert!(
                    (v - want).abs() < 1e-9 * want + t().eps(),
                    "{what}: volume {v}, closed form {want}"
                );
                assert_eq!(at_corner(b), vertices, "{what}: vertices at the corner");
                assert_eq!(validate_geometric(b, t()), Ok(()), "{what}: tier 3");
                assert_eq!(
                    validate_pseudomanifold(b, records, t()),
                    Ok(()),
                    "{what}: tier 3′"
                );
            }
        }
    }
}

/// **No tilt in the band reaches the fusion's escalation**
/// (`SplitFinishError::ConeCornerEscalated`'s doc). The cube's normal
/// `(1, δ, 0.7)` puts its trace on x = 1 within an angle of about δ of
/// the prism top's at the corner, the trace of the other cone; at δ a
/// few ε the fusion's wedge would read the two in band. Each such union
/// refuses before the split's fusion reads anything, in the union or in
/// the reduction, or splits. Red where the escalation is reached: the
/// variant then has a witness, and its doc and this row change.
#[test]
fn no_tilt_in_the_band_reaches_the_cone_corner_escalation() {
    let mut split_in_band = 0;
    for c in [0.5, 1.0, 2.0, 5.0, 10.0, 30.0] {
        let delta = c * t().eps();
        let Ok(topo::BooleanResult::Body(u)) =
            topo::union(&ell(), &cube_on(frame_along([1.0, delta, 0.7])), t())
        else {
            continue;
        };
        split_in_band += 1;
        for s in [1.0, -1.0] {
            let plane = split_plane(Point3::new(1.0, 1.0, 1.0), Vec3::new(s, 0.0, 0.0), t());
            let r = split(&u.body, &plane, t());
            assert!(
                !matches!(
                    r,
                    Err(topo::SplitError::Finish(
                        topo::SplitFinishError::ConeCornerEscalated { .. }
                    ))
                ),
                "δ = {c} ε, normal {s} x: the fusion's escalation is reached: {:?}",
                r.err()
            );
        }
    }
    assert!(
        split_in_band > 0,
        "no union in the band built, so no split reached the fusion"
    );
}
