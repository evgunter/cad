//! **An edge off its face's surface is an incidence miss at the shape
//! door, not a verdict on the face's shape.** Each loop's one edge lies
//! off the surface it is stored against — a circle, or a line, an
//! ellipse or a spiric of a kind that lies on no such surface; the door
//! answers `OffSurface`, a defect of the stored data, where it used to
//! answer `NotIsoRectangle`, the valid face outside the inventory, whose
//! caller routes it to a lane. Where the band cannot tell, it escalates.

#![allow(clippy::expect_used)]

use crate::shared::point::{p3, v3};
use crate::shared::surf;
use crate::shared::tol::band;
use crate::shared::topo::edge;
use geom::Curve3;
use geom::Surface;
use geom_brep::props::{PropsError, require_iso_rectangle};

/// The door's answer on a one-edge loop of `circle` against `surface`.
fn door(surface: &Surface<f64>, circle: Curve3<f64>) -> Result<(), PropsError> {
    require_iso_rectangle(surface, &[edge(circle, 0.0, 1.0, 0, 1)], band())
}

/// A circle in a plane that holds the cylinder's axis — no rim by its
/// axis class — of radius 5 against the unit cylinder: its radius misses
/// the cylinder's, decided on the refusing path by `props_rim_fit`.
#[test]
fn a_circle_in_an_axial_plane_off_the_cylinder_is_off_its_surface() {
    let circle = Curve3::circle(p3(0.0, 0.0, 0.0), v3(1.0, 0.0, 0.0), 5.0, v3(0.0, 1.0, 0.0));
    assert_eq!(
        door(&surf::cylinder(1.0), circle),
        Err(PropsError::OffSurface {
            what: "props_rim_fit"
        })
    );
}

/// The same circle against the unit sphere: no great circle
/// (`props_meridian_great`), and off the sphere, which the refusing path
/// decides by `props_rim_fit`. A small circle ON the sphere whose plane
/// holds the axis is the valid one, no iso curve of the chart.
#[test]
fn a_circle_in_an_axial_plane_off_the_sphere_is_off_its_surface() {
    let off = Curve3::circle(p3(0.0, 0.0, 0.0), v3(1.0, 0.0, 0.0), 5.0, v3(0.0, 1.0, 0.0));
    assert_eq!(
        door(&surf::sphere(1.0), off),
        Err(PropsError::OffSurface {
            what: "props_rim_fit"
        })
    );
    let on = Curve3::circle(p3(0.6, 0.0, 0.0), v3(1.0, 0.0, 0.0), 0.8, v3(0.0, 1.0, 0.0));
    assert_eq!(
        door(&surf::sphere(1.0), on),
        Err(PropsError::NotIsoRectangle {
            what: "props_meridian_great"
        })
    );
}

/// On the torus `(2, 0.5)`: a coaxial circle of radius 7 misses every
/// rim (`props_rim_fit`), and a circle of radius 3 in an axial plane
/// misses every meridian (`props_meridian_fit`) — each off the torus.
#[test]
fn circles_off_the_torus_are_off_its_surface() {
    let torus = surf::torus(2.0, 0.5);
    let coaxial = Curve3::circle(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 7.0, v3(1.0, 0.0, 0.0));
    assert_eq!(
        door(&torus, coaxial),
        Err(PropsError::OffSurface {
            what: "props_rim_fit"
        })
    );
    let axial = Curve3::circle(p3(0.0, 0.0, 0.0), v3(0.0, 1.0, 0.0), 3.0, v3(1.0, 0.0, 0.0));
    assert_eq!(
        door(&torus, axial),
        Err(PropsError::OffSurface {
            what: "props_meridian_fit"
        })
    );
}

/// **The cylinder's own radius does not put a circle on it.** A circle
/// of radius 1 in a plane holding the unit cylinder's axis, centred on
/// it: its radius is the cylinder's, and half its points are on the axis.
/// The refusing path reads its distance from the cylinder at four points
/// a quarter turn apart.
#[test]
fn a_circle_of_the_cylinders_radius_in_an_axial_plane_is_off_its_surface() {
    let circle = Curve3::circle(p3(0.0, 0.0, 0.0), v3(1.0, 0.0, 0.0), 1.0, v3(0.0, 1.0, 0.0));
    assert_eq!(
        door(&surf::cylinder(1.0), circle),
        Err(PropsError::OffSurface {
            what: "props_rim_fit"
        })
    );
}

/// **The norm alone does not put a circle on the sphere.** A circle in
/// the plane `x = 0` (normal `x`) centred at `(0, 0.6, 0)`, radius 0.8:
/// `‖w‖² + r² = R²` as for a small circle on the unit sphere, but its
/// centre is off its own normal, so its points reach `‖w‖ + r` from the
/// centre. The refusing path reads `‖w × n‖` beside the norm.
#[test]
fn a_circle_with_the_spheres_norm_but_off_its_normal_is_off_its_surface() {
    let circle = Curve3::circle(p3(0.0, 0.6, 0.0), v3(1.0, 0.0, 0.0), 0.8, v3(0.0, 1.0, 0.0));
    assert_eq!(
        door(&surf::sphere(1.0), circle),
        Err(PropsError::OffSurface {
            what: "props_rim_fit"
        })
    );
}

/// **A kind that lies on no such surface is decided by its distance.**
/// A chord of the unit sphere and of the torus `(2, 0.5)`, an ellipse on
/// each, and a spiric of a torus against the unit cylinder: each kind
/// lies on no such surface unless it is within band of it, which nine
/// points of its span decide (`props_carrier_on_surface`).
#[test]
fn a_line_ellipse_or_spiric_on_a_surface_it_cannot_lie_on_is_off_it() {
    let off = Err(PropsError::OffSurface {
        what: "props_carrier_on_surface",
    });
    let chord = |a: geom_core::Point3<f64>, b: geom_core::Point3<f64>| Curve3::line(a, b - a);
    assert_eq!(
        door(
            &surf::sphere(1.0),
            chord(p3(1.0, 0.0, 0.0), p3(0.0, 1.0, 0.0))
        ),
        off,
        "a chord of the sphere"
    );
    assert_eq!(
        door(
            &surf::torus(2.0, 0.5),
            chord(p3(2.5, 0.0, 0.0), p3(0.0, 2.5, 0.0))
        ),
        off,
        "a chord of the torus's outer equator"
    );
    let ellipse = Curve3::ellipse(
        p3(0.0, 0.0, 0.0),
        v3(0.0, 0.0, 1.0),
        1.0,
        0.5,
        v3(1.0, 0.0, 0.0),
        band(),
    )
    .expect("an ellipse");
    assert_eq!(
        door(&surf::sphere(1.0), ellipse.clone()),
        off,
        "an ellipse on the sphere"
    );
    let wide = Curve3::ellipse(
        p3(0.0, 0.0, 0.0),
        v3(0.0, 0.0, 1.0),
        2.5,
        1.5,
        v3(1.0, 0.0, 0.0),
        band(),
    )
    .expect("an ellipse");
    assert_eq!(
        door(&surf::torus(2.0, 0.5), wide),
        off,
        "an ellipse on the torus"
    );
    let spiric = Curve3::spiric(
        p3(0.0, 0.0, 0.0),
        v3(0.0, 0.0, 1.0),
        v3(1.0, 0.0, 0.0),
        2.0,
        0.5,
        0.3,
        band(),
    )
    .expect("the spiric's numbers are in regime");
    assert_eq!(
        door(&surf::cylinder(1.0), spiric),
        off,
        "a torus's spiric on a cylinder"
    );
}

/// **An undecided incidence is the answer, not "no rim".** A circle about
/// the torus `(2, 0.5)`'s axis, radius 7 so it misses every rim's fit,
/// with its centre three zero-band widths off the axis: whether it is
/// coaxial (off the torus) or not (a Villarceau circle, on it) is a
/// decision in the band, and the door answers that escalation where it
/// used to read it as a decided "not coaxial" and answer the fit's
/// verdict.
#[test]
fn a_rim_whose_incidence_the_band_cannot_decide_escalates_on_the_torus() {
    let off = 3.0 * band().zero();
    let circle = Curve3::circle(p3(off, 0.0, 0.0), v3(0.0, 0.0, 1.0), 7.0, v3(1.0, 0.0, 0.0));
    let got = door(&surf::torus(2.0, 0.5), circle);
    assert!(matches!(&got, Err(PropsError::Escalated { .. })), "{got:?}");
}

/// **A poisoned sample is no reading.** A line whose direction carries a
/// NaN, stored on the unit sphere: its sampled distance is poison, which
/// the incidence decision's poison arm answers as an escalation rather
/// than a fold dropping it and reading the finite samples alone.
#[test]
fn a_carrier_that_evaluates_to_no_finite_point_escalates() {
    let line = Curve3::line(p3(1.0, 0.0, 0.0), v3(-1.0, f64::NAN, 0.0));
    let got = door(&surf::sphere(1.0), line);
    assert!(
        matches!(&got, Err(PropsError::Escalated { cause, .. }) if cause.predicate == Some("props_carrier_on_surface")),
        "{got:?}"
    );
}
