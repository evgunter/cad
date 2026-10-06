//! **The project box with one cylindrical feature, split by the
//! cutaway's tilted plane** — `work/tquery/split-refuses-cylindrical-feature-box.md`.
//!
//! The tour's enclosure (`demos/tour/src/projectbox.rs`, the 15-op
//! chain) with one round feature, cut by the plane through
//! `(1.5, 1, 0.75)` whose authored normal is `(0.75, 0.1875, 1)` —
//! a direction, not a unit vector (`|n| = 1.264`). Two features, one
//! per orientation:
//!
//! - a cable gland: the middle near-wall vent replaced by a bore
//!   `r = 0.1875` along `y` at `x = 1.5, z = 0.875`;
//! - a screw standoff: a round boss `r = 0.1875` along `z` at
//!   `(2, 1)`, unioned onto the cavity floor from `z = 0.1875` to
//!   `0.875`.
//!
//! The plane meets each feature's cylinder in an ellipse whose closed
//! form is `a = r / |n̂·axis|`, `b = r`, and that is what each row
//! reads off the halves: the conic section is where the normal's
//! LENGTH enters (its components are read as direction cosines), so a
//! normal that reached the section unnormalized read the standoff's
//! tilt as zero (`a = r / 1`, refused as a circle) and the gland's as
//! `a = r / 0.1875` (an ellipse off the cylinder, refused at the
//! join's endpoint certification).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom_core::{Band, Point2, Point3, Tol, UnitVec3, Vec3};
use profile::{SketchPlane, circle_split};
use sweep::test_support::{brick, extruded, finished, sketch_at, sketch_from_axes};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{Body, DATUM_UNIT_NORM, subtract, union};

const R: f64 = 0.1875;

fn tol() -> Tol {
    Tol::witness()
}

fn slab(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    brick(x, y, z, tol())
}

fn sub(what: &str, a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let (a, b) = (
        finished(what, a.clone(), tol()),
        finished(what, b.clone(), tol()),
    );
    subtract(&a, &b, tol())
        .unwrap_or_else(|e| panic!("the {what} subtract succeeds: {e:?}"))
        .body()
        .unwrap_or_else(|| panic!("the {what} subtract leaves material"))
        .body
        .clone()
        .into_body()
}

fn uni(what: &str, a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let (a, b) = (
        finished(what, a.clone(), tol()),
        finished(what, b.clone(), tol()),
    );
    union(&a, &b, tol())
        .unwrap_or_else(|e| panic!("the {what} union succeeds: {e:?}"))
        .body()
        .unwrap_or_else(|| panic!("the {what} union leaves material"))
        .body
        .clone()
        .into_body()
}

/// The tour's 15-op enclosure, less the vent at `(column, wall)` when
/// `skip` names one: a copy of `projectbox::build` in
/// `demos/tour/src/projectbox.rs`, which lives in a detached cargo
/// root this crate cannot call. A change to that body is a change to
/// this one.
fn project_box(skip: Option<(usize, usize)>) -> Body<f64> {
    let outer = slab((0.0, 3.0), (0.0, 2.0), (0.0, 1.5));
    let cavity = slab((0.25, 2.75), (0.25, 1.75), (0.25, 2.0));
    let mut acc = sub("cavity", &outer, &cavity);
    let xs = [(0.5, 0.875), (1.3125, 1.6875), (2.125, 2.5)];
    for (i, &x) in xs.iter().enumerate() {
        for (j, y) in [(-0.25, 0.5), (1.5, 2.25)].into_iter().enumerate() {
            if skip != Some((i, j)) {
                acc = sub("vent", &acc, &slab(x, y, (0.5, 1.25)));
            }
        }
    }
    let bx = [(0.4375, 0.8125), (2.1875, 2.5625)];
    let by = [(0.4375, 0.8125), (1.1875, 1.5625)];
    for &x in &bx {
        for &y in &by {
            acc = uni("boss", &acc, &slab(x, y, (0.1875, 0.875)));
        }
    }
    for &x in &bx {
        for &y in &by {
            let px = (x.0 + 0.09375, x.1 - 0.09375);
            let py = (y.0 + 0.09375, y.1 - 0.09375);
            acc = sub("pocket", &acc, &slab(px, py, (0.5625, 1.0625)));
        }
    }
    acc
}

/// The cutaway's authored normal direction (not unit).
fn authored_normal() -> Vec3<f64> {
    Vec3::new(0.75, 0.1875, 1.0)
}

/// The cutaway plane, its normal minted the way a caller holding a
/// direction mints one.
fn cutaway_plane() -> SplitPlane<f64> {
    let band = Band::linear(tol()).expect("the witness tolerance forms a band");
    SplitPlane {
        origin: Point3::new(1.5, 1.0, 0.75),
        normal: UnitVec3::new(authored_normal(), DATUM_UNIT_NORM, band)
            .expect("the cutaway normal has a length"),
    }
}

/// A round of radius [`R`] about `c` on `plane`, three arcs of one
/// carrier, extruded `h` along the plane normal.
fn round(plane: SketchPlane<f64>, c: Point2<f64>, h: f64) -> Body<f64> {
    let rim = circle_split(c, R, 3, 0.0, tol()).expect("the three-arc rim authors");
    extruded(plane, vec![rim.into()], h, tol())
}

/// Splits `body` by the cutaway plane and checks what every split
/// owes: material on both sides, each half valid at tiers 1 and 3,
/// and the halves partitioning the volume. Returns the halves.
fn halves(what: &str, body: &Body<f64>) -> [Body<f64>; 2] {
    let v = topo::mass_properties(body, tol())
        .expect("the body's volume")
        .volume;
    let cut = split(body, &cutaway_plane(), tol())
        .unwrap_or_else(|e| panic!("{what}: the cutaway plane splits it: {e:?}"));
    let mut sum = 0.0;
    let out = [("above", cut.above), ("below", cut.below)].map(|(side, part)| {
        let SplitPart::Body(half) = part else {
            panic!("{what} {side}: the plane crosses the box, so both sides hold material");
        };
        assert_eq!(topo::validate(&half), Ok(()), "{what} {side}: tier 1");
        assert_eq!(
            topo::validate_geometric(&half, tol()),
            Ok(()),
            "{what} {side}: tier 3"
        );
        sum += topo::mass_properties(&half, tol())
            .expect("a half's volume")
            .volume;
        half
    });
    assert!(
        (sum - v).abs() < 1e-9,
        "{what}: the halves partition the volume ({sum} against {v})"
    );
    out
}

/// The `(major, minor)` of every ellipse edge carrier in `half`.
fn ellipse_axes(half: &Body<f64>) -> Vec<(f64, f64)> {
    half.edges()
        .filter_map(|(_, e)| {
            half.get_curve_geom(e.curve)?
                .certified()
                .map(|c| c.carrier().clone())
        })
        .filter_map(|c| match c {
            Curve3::Ellipse { major, minor, .. } => Some((major, minor)),
            _ => None,
        })
        .collect()
}

/// Every ellipse edge on either half is the feature's section, with
/// the closed form's semi-axes: `a = r / |n̂·axis|`, `b = r`.
fn assert_sections_are_the_closed_form(what: &str, halves: &[Body<f64>; 2], axis: Vec3<f64>) {
    let n = authored_normal();
    let a = R / (n.dot(axis) / n.norm()).abs();
    for (side, half) in ["above", "below"].iter().zip(halves) {
        let axes = ellipse_axes(half);
        assert!(
            !axes.is_empty(),
            "{what} {side}: the feature's section is an ellipse"
        );
        for (major, minor) in axes {
            assert!(
                (major - a).abs() < 1e-12 && (minor - R).abs() < 1e-12,
                "{what} {side}: section ellipse ({major}, {minor}), closed form ({a}, {R})"
            );
        }
    }
}

/// The cable gland: the plane meets the `y` bore at `|n̂·ŷ| = 0.148`,
/// a grazing cut whose ellipse is `a = 1.264` on a `0.1875` bore.
#[test]
fn a_box_with_a_bore_along_y_splits_on_the_cutaway_plane() {
    // u = x, v = z: the sketch normal is x × z = −y, so a sketch at
    // y = 0.5 extruded 0.75 runs the bore through y ∈ [−0.25, 0.5].
    let plane = sketch_from_axes(
        Point3::new(0.0, 0.5, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        tol(),
    );
    let bore = round(plane, Point2::new(1.5, 0.875), 0.75);
    let body = sub("gland bore", &project_box(Some((1, 0))), &bore);
    let halves = halves("bored box", &body);
    assert_sections_are_the_closed_form("bored box", &halves, Vec3::unit_y());
}

/// The screw standoff: `|n̂·ẑ| = 0.7911`, so `a − b = 0.0495` — the
/// margin the refusal read as zero.
#[test]
fn a_box_with_a_round_standoff_along_z_splits_on_the_cutaway_plane() {
    let standoff = round(sketch_at(0.1875), Point2::new(2.0, 1.0), 0.875 - 0.1875);
    let body = uni("standoff", &project_box(None), &standoff);
    let halves = halves("standoff box", &body);
    assert_sections_are_the_closed_form("standoff box", &halves, Vec3::unit_z());
}
