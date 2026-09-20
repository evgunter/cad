//! **The seam class, end to end** — the rows that go red when the
//! frame constructor's comparison stops deciding on the geometry a CAD
//! corpus is made of.
//!
//! `Vec3::orthonormal_basis` crosses the normal with `e_z` when
//! `|n.z| ≤ max(|n.x|, |n.y|)/2` and with `e_y` otherwise. Everything
//! here is about faces whose normals are NEAR that comparison — the
//! chamfer families a corpus does carry, and the seam's own elevation,
//! which it does not.
//!
//! Three rows, each measuring a different door on the same class:
//!
//! * a 45° chamfer face, minted through `newell_plane::<Interval>` from
//!   exact `f64` quad corners, DECIDES and its `u_ref` is still a unit
//!   direction to [`UnitVec3::new`] — the door a derived sketch frame
//!   goes through. On the earlier ratio (`|n.z| = max(|n.x|, |n.y|)`)
//!   this face sat exactly on the seam, the frame came back as the hull
//!   of two orthogonal candidates, and this door escalated on
//!   `datum_unit_norm` with a margin of `[0, 1.4142]`;
//! * a face at the seam's OWN elevation, `atan(1/2) ≈ 26.57°`, where
//!   the comparison cannot decide over an enclosure: the frame is a
//!   hull, and the row states what a hull is and is not — bounded, but
//!   not a unit direction, so the door refuses rather than inventing
//!   one;
//! * an extrude on a TILTED sketch frame (45° and 30° about `x`), whose
//!   four tilted planes are the case R2 measured refusing at the
//!   clearance engine's `refines` door: `clearance` certifies on the
//!   stored charts, with no re-chart anywhere.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;

use editor_core::UnitSym;
use editor_core::analysis::{BoxAxis, ParamBox};
use editor_core::clearance::{ClearanceVerdict, Selection, clearance};
use editor_core::{
    Dimension, Distribution, DocEdit, DocParam, Expr, LoopProgram, Node, ParamName, ProfileDoc,
    ProfileProgram, RecipeNodeId,
};
use geom::Surface;
use geom_brep::newell_plane;
use geom_core::{Band, Bounds, Interval, Point3, Real, Tol, UnitVec3, Vec3};
use topo::DATUM_UNIT_NORM;

use fixture::{Recorder, len, scl};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The analysis box's half-width — the same ε-scaled box every other
/// interval clearance fixture uses, for the same reason: no node's
/// interval replay survives a wider one.
fn half() -> f64 {
    Tol::witness().eps() / 64.0
}

fn box_of(axis: &str) -> ParamBox {
    let mut axes = BTreeMap::new();
    axes.insert(
        ParamName::new(axis),
        BoxAxis::Varying {
            lo: -half(),
            hi: half(),
        },
    );
    ParamBox::from_axes(axes)
}

fn declare(r: &mut Recorder, axis: &str, nominal: f64) {
    r.push(DocEdit::SetDocParam {
        name: ParamName::new(axis),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: nominal,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -half(),
                hi: half(),
            }),
        },
    });
}

/// A quad, as the four corners of one face, at whichever scalar.
fn quad<T: Real>(corners: [(f64, f64, f64); 4]) -> [Point3<T>; 4] {
    corners.map(|(x, y, z)| Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z)))
}

/// **A 45° chamfer face decides, and its frame is still a direction.**
///
/// The face is the top edge of a unit box chamfered at 45°: four exact
/// `f64` corners, whose Newell normal is `(0, 1, 1)/√2`. That normal's
/// `|n.z|` equals `max(|n.x|, |n.y|)`, so under the earlier ratio the
/// comparison was undecided over any enclosure with width — the frame
/// came back as `([0, 0.7071], [0, 1], [−0.7071, 0])`, whose norm
/// encloses `[0, 1.4142]` and which [`UnitVec3::new`] therefore
/// escalated on. At `ρ = 1/2` the same face is a whole quadrant away
/// from the seam and every door downstream of it is quiet.
///
/// The 30° and 60° chamfers of the same edge ride along: the corpus
/// uses all three and none of them may be on a seam.
#[test]
fn a_chamfer_face_decides_and_its_frame_is_a_unit_direction() {
    for (name, (dy, dz)) in [
        ("45°", (1.0, 1.0)),
        ("30°", (3f64.sqrt(), 1.0)),
        ("60°", (1.0, 3f64.sqrt())),
    ] {
        // A strip along +x whose normal is (0, dy, dz) normalized: the
        // chamfer face of a box edge, wound so the normal points out.
        let corners = [
            (0.0, 0.0, 1.0),
            (1.0, 0.0, 1.0),
            (1.0, dz, 1.0 + dy),
            (0.0, dz, 1.0 + dy),
        ];
        let Ok(Surface::Plane { normal, u_ref, .. }) =
            newell_plane::<Interval>(&quad::<Interval>(corners), band())
        else {
            panic!("{name}: newell_plane refused at Interval");
        };
        let d = normal.z.abs() - normal.x.abs().max(normal.y.abs()) * Interval::from_f64(0.5);
        assert!(
            d.hi() <= 0.0 || d.lo() > 0.0,
            "{name} chamfer: the axis comparison did not decide — d = [{}, {}], \
             n = ([{}, {}], [{}, {}], [{}, {}])",
            d.lo(),
            d.hi(),
            normal.x.lo(),
            normal.x.hi(),
            normal.y.lo(),
            normal.y.hi(),
            normal.z.lo(),
            normal.z.hi()
        );
        // The frame is a direction: the door a derived sketch frame
        // goes through accepts it, and its norm is 1 to within the
        // normal's own width.
        UnitVec3::new(u_ref, DATUM_UNIT_NORM, band()).unwrap_or_else(|e| {
            panic!(
                "{name} chamfer: the stored u_ref is not a unit direction: {e:?} \
                 (u_ref = ([{}, {}], [{}, {}], [{}, {}]))",
                u_ref.x.lo(),
                u_ref.x.hi(),
                u_ref.y.lo(),
                u_ref.y.hi(),
                u_ref.z.lo(),
                u_ref.z.hi()
            )
        });
        let norm = u_ref.norm();
        assert!(
            norm.lo() >= 1.0 - 1e-12 && norm.hi() <= 1.0 + 1e-12,
            "{name} chamfer: ‖u_ref‖ = [{}, {}] is not a direction",
            norm.lo(),
            norm.hi()
        );
    }
}

/// **The seam's own elevation: what decides there, and what hulls.**
///
/// The seam's upper edge is the meridian `n.x = 0` at elevation
/// `atan(1/2)`, and it is the worst azimuth on the whole set: with
/// `n.x = 0` the candidates are `c_z = (−n.y, 0, 0)` and
/// `c_y = (n.z, 0, 0)`, both along `±e_x`, so wherever `n.y·n.z > 0`
/// they are ANTIPARALLEL and crossing the seam there is a HALF TURN
/// rather than a quarter. Two rows, on the two things that can happen:
///
/// * a face minted through `newell_plane::<Interval>` from exact `f64`
///   corners has a normal of width ~3e-16, so `d` straddles and the
///   frame is the HULL of the two antiparallel candidates. What the row
///   pins is that the hull is honest: bounded and certified — never the
///   `[−∞, ∞] Trv` a division by an enclosure of zero gives — and not a
///   unit direction, so the door that needs one refuses rather than
///   taking the hull for an answer. That refusal is the design: the
///   geometry on this seam is the geometry no corpus draws (0 of 817);
/// * a normal that is a POINT enclosure on the seam DECIDES. `ρ = 1/2`
///   is dyadic, so `max(|n.x|, |n.y|)/2` is an exact halving and the
///   comparison's two sides are the same number: `d = [0, 0]`, which
///   the door's tie-break sends to the `e_z` arm. `copysign` could not
///   have decided there, and that is the whole reason this door exists.
#[test]
fn the_seam_hulls_over_a_minted_normal_and_decides_at_a_point() {
    // A strip along +x descending in z, so the right-hand normal of the
    // winding is (0, 2, 1)/√5 — both components positive, which is what
    // makes the two candidates antiparallel.
    let corners = [
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (1.0, 1.0, -2.0),
        (0.0, 1.0, -2.0),
    ];
    let r = 2.0 / 5f64.sqrt();
    let Ok(Surface::Plane { normal, u_ref, .. }) =
        newell_plane::<Interval>(&quad::<Interval>(corners), band())
    else {
        panic!("newell_plane refused at Interval");
    };
    assert!(
        (normal.y.lo() - r).abs() <= 1e-12 && (normal.z.lo() - 0.5 * r).abs() <= 1e-12,
        "the fixture is not on the seam: n = (_, [{}, {}], [{}, {}])",
        normal.y.lo(),
        normal.y.hi(),
        normal.z.lo(),
        normal.z.hi()
    );
    let d = normal.z.abs() - normal.x.abs().max(normal.y.abs()) * Interval::from_f64(0.5);
    assert!(
        d.lo() <= 0.0 && d.hi() > 0.0,
        "the minted normal was expected to straddle the seam: d = [{}, {}]",
        d.lo(),
        d.hi()
    );
    for (e, which) in [
        (u_ref.x, "u_ref.x"),
        (u_ref.y, "u_ref.y"),
        (u_ref.z, "u_ref.z"),
    ] {
        assert!(
            e.lo().is_finite() && e.hi().is_finite(),
            "{which} on the seam is unbounded: [{}, {}] — a hull, not a non-real",
            e.lo(),
            e.hi()
        );
        assert!(
            e.is_certified(),
            "{which} on the seam is not certified: [{}, {}]",
            e.lo(),
            e.hi()
        );
    }
    assert!(
        u_ref.x.lo() < -0.9 && u_ref.x.hi() > 0.9,
        "the hull does not span both antiparallel candidates: u_ref.x = [{}, {}]",
        u_ref.x.lo(),
        u_ref.x.hi()
    );
    assert!(
        UnitVec3::new(u_ref, DATUM_UNIT_NORM, band()).is_err(),
        "the direction door accepted a hull spanning two antiparallel candidates"
    );
    // The same direction as a POINT enclosure: the tie decides.
    let point = Vec3::new(
        Interval::from_f64(0.0),
        Interval::from_f64(r),
        Interval::from_f64(0.5 * r),
    );
    let d = point.z.abs() - point.x.abs().max(point.y.abs()) * Interval::from_f64(0.5);
    assert!(
        d.lo() == 0.0 && d.hi() == 0.0,
        "the dyadic halving is not exact: d = [{}, {}]",
        d.lo(),
        d.hi()
    );
    let (b1, _) = point.orthonormal_basis();
    UnitVec3::new(b1, DATUM_UNIT_NORM, band()).expect("a point enclosure on the seam decides");
    assert!(
        b1.x.hi() < 0.0,
        "the tie did not take the e_z arm: b1.x = [{}, {}]",
        b1.x.lo(),
        b1.x.hi()
    );
}

/// A prism extruded from a sketch frame tilted `deg` about `+x`.
fn tilted_prism(deg: f64) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    declare(&mut r, "place", 0.0);
    let (c, s) = deg.to_radians().sin_cos();
    let (s, c) = (c, s);
    let plane = r.insert(fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, c, s]));
    let p = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)].into_iter())
                .expect("finite corners"),
        ],
    }));
    let solid = r.insert(Node::Extrude {
        profile: p,
        distance: len(0.5),
    });
    let placed = r.insert(Node::Transform {
        input: solid,
        translation: [
            Expr::param(ParamName::new("place"), Dimension::Length),
            len(0.0),
            len(0.0),
        ],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite angle"),
    });
    (r.doc, placed)
}

/// **R2's tilted extrude, through the clearance engine.**
///
/// A body extruded from a 45°-tilted sketch frame has four planes whose
/// normals are `(0, ±cos45, ±sin45)` and `(0, ∓sin45, ±cos45)` — every
/// one of them exactly on the earlier ratio's seam. Measured there,
/// four of the six planes were undecided, two walls stored
/// `u_ref = ([−1.0000000000000016, 1.000000000000002], ~0, ~0)`, and
/// `clearance` refused at the `refines` door on `FaceKey(3v1)`: the
/// stored chart could not be narrowed because the frame itself spanned
/// both signs.
///
/// At `ρ = 1/2` those normals are a long way off the seam, every plane
/// decides, and the engine certifies on the stored chart with no
/// re-chart in the tree. The 30° tilt rides along as the second
/// chamfer family a corpus actually uses.
#[test]
fn a_tilted_extrude_certifies_on_its_stored_charts() {
    for deg in [45.0, 30.0] {
        let (doc, placed) = tilted_prism(deg);
        let sel = Selection::body_of(placed);
        let report = clearance(&doc, &box_of("place"), &sel, &sel, 0.1, Tol::witness());
        assert_eq!(
            report.verdict(),
            &ClearanceVerdict::Holds,
            "the {deg}°-tilted extrude did not certify: {}",
            report.serialize()
        );
        let r = report.receipt();
        assert_eq!(
            r.refused,
            0,
            "the {deg}°-tilted extrude refused {} pairs: {}",
            r.refused,
            report.serialize()
        );
    }
}
