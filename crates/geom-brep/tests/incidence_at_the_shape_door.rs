//! **A circle off its face's surface is an incidence miss at the shape
//! door, not a verdict on the face's shape.** Each loop's first edge is a
//! circle no point of which lies on the surface it is stored against;
//! the door answers `OffSurface`, a defect of the stored data, where it
//! used to answer `NotIsoRectangle`, the valid face outside the
//! inventory, whose caller routes it to a lane.

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
