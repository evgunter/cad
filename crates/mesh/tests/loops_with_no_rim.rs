//! **A curved face whose loop has NO RIM**, and what makes such a loop
//! bound a face — measured on every chart kind the walk has one for.
//!
//! A rim-free loop is all meridians, so it gets no u-extent from a rim
//! row: the walk's band arm derives the whole width from the columns its
//! iso-side OPENINGS carry, and an opening's column belongs to its EDGE.
//! Two openings on two distinct edges therefore bound a domain and one
//! edge's openings bound nothing —
//! [`mesh::TessellateError::SingleColumnCurvedFace`] is that premise
//! refused, and this file is the class it covers:
//!
//! * **no junction on the chart axis, so one opening** — a torus face
//!   bounded by one meridian circle, and a cylinder face bounded by one
//!   generator. Neither chart has a pole for a meridian to end on;
//!   nothing can move the column, and the loop is a single iso side.
//! * **a junction on the axis, arrived at and left along one edge** — the
//!   one-face sphere whose loop is a single seam walked both ways, and
//!   the one-face cone whose loop is a single generator from the apex.
//!   Both openings state the same edge's column, bitwise.
//! * **the positive controls**, which must still mesh: the ball's two
//!   pole-to-pole bands and the rimless lune, whose two openings stand on
//!   two edges, and the spur-wearing cap in
//!   `loops_the_meridian_guard_admits.rs`, whose loop carries a rim.
//!
//! One member of the class is refused earlier and by another door: the
//! sphere cut along a whole great circle through both poles is tier-3
//! VALID, and props' branch door (`require_one_chart_branch`) refuses it
//! `UnsupportedCurvedShape` because a meridian arc of it carries a pole
//! mid-edge. It is rowed here because the guard's premise would also
//! refuse it, and which door answers first is a fact worth pinning.
//!
//! Without the guard each refusing row answers with a hole where the face
//! is: the cross-face census catches it where debug assertions run, and
//! `tessellate` returns `Ok` with an empty patch where they do not
//! (`work/tess/rim-free-loop-on-a-poleless-chart-meshes-as-a-hole.md` has
//! both measurements). The bodies come through the Euler doors.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use common::witness_bodies::one_circle_cut;
use common::{ball, sphere_wedge};
use core::f64::consts::PI;
use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, SurfaceKind};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{Body, FaceSurface, MevSite};

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn z_axis() -> Vec3<f64> {
    Vec3::new(0.0, 0.0, 1.0)
}
fn x_axis() -> Vec3<f64> {
    Vec3::new(1.0, 0.0, 0.0)
}

fn unit_sphere() -> Surface<f64> {
    Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: z_axis(),
        u_ref: x_axis(),
    }
}

/// The great circle in the xz plane from the north pole:
/// `eval(t) = (sin t, 0, cos t)`.
fn meridian_from_the_pole() -> Curve3<f64> {
    Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        u_ref: z_axis(),
    }
}

/// The same great circle by latitude: `eval(t) = (cos t, 0, sin t)`.
fn meridian_by_latitude() -> Curve3<f64> {
    Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, -1.0, 0.0),
        radius: 1.0,
        u_ref: x_axis(),
    }
}

/// **The one-face, one-seam sphere** (V2 / E1 / F1).
fn one_seam_sphere() -> Body<f64> {
    let tol = Tol::witness();
    let seam = meridian_from_the_pole();
    let mut body = Body::<f64>::new();
    let start = body.mvfs(seam.eval(0.0)).unwrap();
    body.set_face_surface(start.face, FaceSurface::New(unit_sphere()))
        .unwrap();
    body.mev(
        MevSite::Lone {
            r#loop: start.r#loop,
        },
        seam.eval(PI),
        EdgeCurveSpec::arc_of_circle(seam, 0.0, PI).unwrap(),
        tol,
    )
    .unwrap();
    body
}

/// **The torus face bounded by one meridian circle**, and its complement.
fn torus_on_one_meridian_circle() -> Body<f64> {
    let torus = Surface::Torus {
        center: p3(0.0, 0.0, 0.0),
        axis: z_axis(),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: x_axis(),
    };
    let minor = Curve3::Circle {
        center: p3(2.0, 0.0, 0.0),
        axis: Vec3::new(0.0, -1.0, 0.0),
        radius: 0.5,
        u_ref: x_axis(),
    };
    one_circle_cut(&minor, torus, None)
}

/// **The sphere cut along a whole great circle through both poles**,
/// split at the two equator points the cut's parameterization gives.
fn sphere_on_one_great_circle() -> Body<f64> {
    one_circle_cut(&meridian_by_latitude(), unit_sphere(), None)
}

/// **The one-seam cylinder**: one generator, walked up and back.
fn one_seam_cylinder() -> Body<f64> {
    let tol = Tol::witness();
    let cyl = Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: z_axis(),
        radius: 1.0,
        u_ref: x_axis(),
    };
    let mut body = Body::<f64>::new();
    let start = body.mvfs(p3(1.0, 0.0, 0.0)).unwrap();
    body.set_face_surface(start.face, FaceSurface::New(cyl))
        .unwrap();
    body.mev(
        MevSite::Lone {
            r#loop: start.r#loop,
        },
        p3(1.0, 0.0, 1.0),
        EdgeCurveSpec::line_between(p3(1.0, 0.0, 0.0), p3(1.0, 0.0, 1.0)),
        tol,
    )
    .unwrap();
    body
}

/// **The one-seam cone**: one generator from the apex, walked out and back.
fn one_seam_cone() -> Body<f64> {
    let tol = Tol::witness();
    let cone = Surface::Cone {
        apex: p3(0.0, 0.0, 0.0),
        axis: z_axis(),
        half_angle: core::f64::consts::FRAC_PI_4,
        u_ref: x_axis(),
    };
    let mut body = Body::<f64>::new();
    let start = body.mvfs(p3(0.0, 0.0, 0.0)).unwrap();
    body.set_face_surface(start.face, FaceSurface::New(cone))
        .unwrap();
    body.mev(
        MevSite::Lone {
            r#loop: start.r#loop,
        },
        p3(1.0, 0.0, 1.0),
        EdgeCurveSpec::line_between(p3(0.0, 0.0, 0.0), p3(1.0, 0.0, 1.0)),
        tol,
    )
    .unwrap();
    body
}

/// The first face, in arena order, on a surface of `kind` — the face
/// `tessellate`'s fold answers for when several refuse.
fn first_face_on(body: &Body<f64>, kind: SurfaceKind) -> topo::FaceKey {
    body.faces()
        .find(|(_, f)| SurfaceKind::of(body.get_surface(f.surface).unwrap()) == kind)
        .map(|(fk, _)| fk)
        .expect("the body carries a face of that kind")
}

/// Both doors `curved::require_iso_rectangle_face` cites, on the first
/// face of `kind` — the measurement that says the refusal below is this
/// lane's to make and not a door's.
fn doors(body: &Body<f64>, kind: SurfaceKind) -> (bool, bool) {
    let band = Band::linear(Tol::witness()).unwrap();
    let (_, f) = body
        .faces()
        .find(|(_, f)| SurfaceKind::of(body.get_surface(f.surface).unwrap()) == kind)
        .expect("the body carries a face of that kind");
    let surface = body.get_surface(f.surface).unwrap();
    let (outer, _) = topo::props::loop_edges(body, f.outer).unwrap();
    (
        geom_brep::props::require_iso_rectangle(surface, &outer, band).is_ok(),
        geom_brep::props::require_one_chart_branch(surface, &outer, band).is_ok(),
    )
}

/// `tessellate` refuses `body` with the single-column arm, naming the
/// first face on a `kind` surface and that kind — and both doors in front
/// of the walk admitted the face, so the refusal is the walk's.
fn assert_refuses_single_column(name: &str, body: &Body<f64>, kind: SurfaceKind, delta: f64) {
    assert_eq!(
        doors(body, kind),
        (true, true),
        "{name}: both doors admit, so the walk is what refuses"
    );
    assert_eq!(
        mesh::tessellate(body, delta, Tol::witness()).map(|_| ()),
        Err(mesh::TessellateError::SingleColumnCurvedFace {
            face: first_face_on(body, kind),
            surface: kind,
        }),
        "{name}"
    );
}

/// **No junction on the chart axis: the torus and the cylinder.** A
/// torus chart's axis misses the surface and a cylinder chart's axis lies
/// inside it, so no junction of either loop can be at one: every
/// consecutive pair of meridians continues a single iso side, the loop
/// opens exactly one, and nothing can move its column. The torus member
/// is the one the row was filed on (both of its faces are in this state,
/// and the first in arena order is what the fold answers for); the
/// cylinder member is the same statement one chart down, and it is where
/// the count guard behind the walk (`polygon.len() < 3`) used to answer
/// `MissingEntity` for it.
///
/// Neither body is tier-3 valid, and neither could be: a torus cut along
/// one circle is an annulus, which needs two loops, and a cylinder with
/// no rim is unbounded. `tessellate` does not re-validate, so the Euler
/// doors and any caller that meshes without validating reach this.
#[test]
fn a_rim_free_loop_on_a_chart_with_no_pole_refuses_single_column() {
    let tol = Tol::witness();
    for (name, body, kind) in [
        (
            "torus bounded by one meridian circle",
            torus_on_one_meridian_circle(),
            SurfaceKind::Torus,
        ),
        (
            "cylinder bounded by one generator",
            one_seam_cylinder(),
            SurfaceKind::Cylinder,
        ),
    ] {
        assert!(
            topo::validate_geometric(&body, tol).is_err(),
            "{name} is not tier-3 valid"
        );
        assert_refuses_single_column(name, &body, kind, 0.05);
    }
}

/// **A junction on the axis, left along the edge it was arrived on: the
/// one-seam sphere and the one-seam cone.** Both loops DO open two iso
/// sides — the sphere at each pole, the cone at its apex and index 0 —
/// and both state one edge's column at every opening, because a column is
/// the edge carrier's mid-azimuth and the walking direction does not
/// touch it. This is the member the row said "no rim and no pole" would
/// not close, and the edge-identity premise is what closes it.
#[test]
fn a_rim_free_loop_that_turns_at_a_pole_along_its_own_edge_refuses_single_column() {
    let tol = Tol::witness();
    for (name, body, kind) in [
        ("one-seam sphere", one_seam_sphere(), SurfaceKind::Sphere),
        ("one-seam cone", one_seam_cone(), SurfaceKind::Cone),
    ] {
        assert!(
            topo::validate_geometric(&body, tol).is_err(),
            "{name} is not tier-3 valid"
        );
        assert_refuses_single_column(name, &body, kind, 0.05);
    }
}

/// **The member another door owns.** The sphere cut along a whole great
/// circle through both poles is tier-3 VALID and its two faces are
/// hemispheres — a legitimate decomposition this lane cannot walk,
/// because each bounding arc carries a pole in its interior where the
/// azimuth it holds constant jumps by π. Props' branch door says exactly
/// that and `tessellate` answers `UnsupportedCurvedShape`, ahead of the
/// walk. The recourse is to state the poles as vertices, which turns each
/// face into the two-column band the lane meshes.
#[test]
fn a_hemisphere_pair_is_refused_at_the_branch_door_not_by_the_walk() {
    let body = sphere_on_one_great_circle();
    assert_eq!(
        topo::validate_geometric(&body, Tol::witness()),
        Ok(()),
        "a sphere cut along a great circle is tier-3 valid"
    );
    assert_eq!(
        doors(&body, SurfaceKind::Sphere),
        (true, false),
        "the shape door admits it; the branch door is what refuses"
    );
    let got = mesh::tessellate(&body, 0.05, Tol::witness()).map(|_| ());
    assert!(
        matches!(
            got,
            Err(mesh::TessellateError::UnsupportedCurvedShape { .. })
        ),
        "{got:?}"
    );
}

/// **The positive controls: two openings on two edges mesh.** The ball's
/// two pole-to-pole bands and the rimless lune are rim-free loops whose
/// two meridians are two edges meeting at the poles, and they mesh
/// watertight. Their u-extents are the two columns' spread — π and the
/// wedge angle — so nothing about the guard reads a width: these pass it
/// on the same incidence question the refusals fail.
#[test]
fn a_rim_free_loop_whose_openings_are_two_edges_still_meshes() {
    let tol = Tol::witness();
    for (name, body) in [
        ("ball, two pole-to-pole bands", ball()),
        ("rimless lune over theta = 2", sphere_wedge(2.0)),
    ] {
        assert_eq!(topo::validate_geometric(&body, tol), Ok(()), "{name}");
        let mesh = mesh::tessellate(&body, 0.05, tol).unwrap_or_else(|e| panic!("{name}: {e:?}"));
        mesh::validate::check_mesh(&mesh).unwrap_or_else(|e| panic!("{name}: {e:?}"));
        for patch in &mesh.patches {
            assert!(
                !patch.triangles.is_empty(),
                "{name}: every face emits triangles"
            );
        }
    }
}
