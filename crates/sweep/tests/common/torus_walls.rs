//! **The torus-walled revolves the offset-axial door is measured on**:
//! the barrel and the teapot's belly (full revolves whose one curved
//! wall is a meridian arc about a centre OFF the axis), and the tour's
//! sectioned vessel with the cavity the axial door carves in it (a
//! partial revolve whose rims mint as spirics).
//!
//! `torax_axial` hollows the barrel and the belly and
//! `pis_arc_capped_poses` asks `point_in_solid` of them at six poses;
//! `spiric_rim` reads the vessel cavity's carriers, and
//! `pis_arc_capped_poses` and `contfp_reads_arcs_on_their_carriers`
//! walk its spiric-bounded faces. A row in one and its twin in another
//! are about each other only while they build THE SAME BODY
//! ([`super::cavity`]'s rule). Body authoring, so it routes here
//! ([`super`]'s routing rule).
//!
//! **Deliberately not absorbed**, and the whole of it:
//!
//! - `review_arms3_r1_probes`' `torus_barrel`, a different barrel under
//!   the same name (an annular meridian about another centre);
//! - `shell7_dump`'s `torus_vessel`, the tour's torus vessel revolved a
//!   FULL turn with its band spelled as a bulge — a different body from
//!   [`vessel_quarter`].

use geom_core::{Band, Point2, Tol, Vec2};
use profile::path::{Open, Start};
use profile::{ArcSweep, Center, Profile, ProfileLoop, SketchPlane};
use sweep::test_support::revolved_about_y;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::Body;

use super::bulge;
use super::charts::hollow_moves;

/// **The barrel bulged about a centre OFF the axis.** The same two
/// junction stations and the same `5/64` meridian radius as the tour's
/// sphere-zone barrel, about the OTHER centre on their perpendicular
/// bisector — so the wall is a TORUS: `R = 6/64`, `r = 5/64`,
/// `h_c = 4/64`, a 3-4-5 at each junction with both residuals exactly
/// zero.
pub fn torus_barrel() -> Body<f64> {
    let c = Point2::new(6.0 / 64.0, 1.0 / 16.0);
    let (lo, hi) = (
        Point2::new(3.0 / 64.0, 0.0),
        Point2::new(3.0 / 64.0, 8.0 / 64.0),
    );
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (lo, bulge(lo, hi, c)),
            (hi, 0.0),
            (Point2::new(0.0, 8.0 / 64.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

/// **The teapot's wall-1 belly.** The pot's own foot and mouth, its
/// belly bulged about `(7/64, 5/64)` — off the axis, so a TORUS with
/// `R = 7/64`, `r = 5/64`, `h_c = 5/64`.
pub fn torus_belly() -> Body<f64> {
    let c = Point2::new(7.0 / 64.0, 5.0 / 64.0);
    let (lo, hi) = (
        Point2::new(4.0 / 64.0, 1.0 / 64.0),
        Point2::new(3.0 / 64.0, 8.0 / 64.0),
    );
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(4.0 / 64.0, 0.0), 0.0),
            (lo, bulge(lo, hi, c)),
            (hi, 0.0),
            (Point2::new(0.0, 8.0 / 64.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

/// **The sectioned vessel**: the tour's torus-walled vessel meridian
/// (`demos/tour/src/torusvessel.rs`, the BELLIED centre), spelled from
/// the same stations and revolved a quarter turn, so the sectioned
/// vessel's door is measured on the scene's own body.
pub fn vessel_quarter() -> Body<f64> {
    let tol = Tol::witness();
    let (r_foot, r_band, r_neck) = (5.0 / 64.0, 9.0 / 64.0, 7.0 / 64.0);
    let (y_foot, y_shoulder, y_mouth) = (4.0 / 64.0, 12.0 / 64.0, 24.0 / 64.0);
    let (h_tube, r_bellied) = (8.0 / 64.0, 6.0 / 64.0);
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(r_foot, 0.0), tol)
        .expect("the base disc")
        .line_to(Point2::new(r_foot, y_foot), tol)
        .expect("the foot")
        .line_to(Point2::new(r_band, y_foot), tol)
        .expect("the lower shoulder")
        .arc_to(
            Center {
                c: Point2::new(r_bellied, h_tube),
                winding: ArcSweep::Ccw,
                p: Point2::new(r_band, y_shoulder),
            },
            tol,
        )
        .expect("the band")
        .line_to(Point2::new(r_neck, y_shoulder), tol)
        .expect("the upper shoulder")
        .line_to(Point2::new(r_neck, y_mouth), tol)
        .expect("the neck")
        .line_to(Point2::new(0.0, y_mouth), tol)
        .expect("the mouth disc")
        .line_to(Start, tol)
        .expect("the axis closes the meridian")
        .into();
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        tol,
    )
    .expect("the meridian revolves")
    .body
}

/// **The sectioned vessel and its cavity through the axial door** —
/// `(quarter, cavity)`: the body `shell` builds and stops on at tier 3,
/// taken BEFORE tier 3 so its carriers can be read.
pub fn vessel_cavity(t: f64) -> (Body<f64>, Body<f64>) {
    let quarter = vessel_quarter();
    let mut cavity = quarter.clone();
    let band = Band::linear(Tol::witness()).expect("band");
    topo::offset_charts_together(
        &mut cavity,
        &hollow_moves(&quarter, t),
        band,
        Tol::witness(),
    )
    .expect("the vessel's corners solve and its rims mint");
    (quarter, cavity)
}
