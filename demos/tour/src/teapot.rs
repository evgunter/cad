//! **The teapot** — `shell`'s designated demo, and what it measured.
//!
//! PLACEHOLDER MODULE DOCS — filled in after the assertions settle.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use pncad::authoring::{p2, validated};
use pncad::geom::{Curve3, Surface};
use pncad::geom_brep::SurfaceKind;
use pncad::geom_core::{Affine3, Band, Mat3, Point2, Point3, Tol, Vec2, Vec3};
use pncad::prelude::{Open, Start, fillet_edges};
use pncad::profile::{ArcSweep, Center, ProfileLoop, SketchPlane};
use pncad::sweep::{Revolution, RevolveAxis, TubeWindow, revolve, tube_along_arc};
use pncad::topo::{Body, BooleanError, EdgeKey, FaceKey, Operand, ReplaceFaceError, ShellError};

use crate::{SceneBody, Stop, View};

// ---------------------------------------------------------------------
// The vessel's meridian. Every station is a dyadic rational in meters,
// so the profile's own arithmetic is exact and the closed forms below
// are compared against numbers no rounding entered.
// ---------------------------------------------------------------------

/// The foot's radius, and the neck's: the pot is waisted at both ends.
const R_FOOT: f64 = 3.0 / 64.0;
/// The belly's radius — the pot's widest wall.
const R_BELLY: f64 = 5.0 / 64.0;
/// The neck's radius. Equal to the foot's, which is a choice about the
/// silhouette and not a constraint; the lid's rim reads it.
const R_NECK: f64 = R_FOOT;
/// Where the foot ends and the lower shoulder turns out.
const Y_FOOT: f64 = 1.0 / 64.0;
/// Where the belly ends and the upper shoulder turns in.
const Y_SHOULDER: f64 = 6.0 / 64.0;
/// The mouth's plane — the top of the neck.
const Y_MOUTH: f64 = 8.0 / 64.0;

/// The wall thickness. A tenth of the belly's radius, and an eighth of
/// the narrowest wall the shell has to survive (the neck's), so every
/// per-face reach margin below is definite by a wide margin rather
/// than by a hair.
const WALL: f64 = 1.0 / 128.0;

/// The NURBS fit tolerance `shell` would hand its approximating lane.
/// Unread here: every wall of this pot is analytic, so the offsets are
/// closed forms and nothing is fitted.
const FIT_TOL: f64 = 1e-6;

// ---------------------------------------------------------------------
// The lid: a second solid of revolution, lifted.
// ---------------------------------------------------------------------

/// How far above the mouth the lid renders — the exploded-view gap.
/// No mate is authored and none is implied; see the module docs.
const LIFT: f64 = 1.0 / 32.0;
/// The lid's underside plane.
const LID_BASE: f64 = Y_MOUTH + LIFT;
/// The dome's sphere radius, and its centre's station: the 5-12-13
/// triple, so the dome passes through the lid's rim at `R_NECK` and
/// carries the knob's foot at `R_KNOB` with both stations exact.
const DOME_R: f64 = 13.0 / 256.0;
/// The dome sphere's centre, BELOW the lid's underside.
const DOME_C: f64 = LID_BASE - 5.0 / 256.0;
/// The knob's radius — the 5-12-13 point of the dome circle.
const R_KNOB: f64 = 5.0 / 256.0;
/// Where the dome ends and the knob's wall begins.
const Y_KNOB: f64 = LID_BASE + 7.0 / 256.0;
/// The knob's top.
const Y_TOP: f64 = LID_BASE + 12.0 / 256.0;
/// The steam vent bored through the finial — which is also what makes
/// the lid's profile ANNULAR, and therefore what makes its latitude
/// rims closed edges. See the module docs' finding 3.
const R_VENT: f64 = 1.0 / 256.0;
/// The roll on the knob's top rim.
const ROLL: f64 = 2.0 / 256.0;

// ---------------------------------------------------------------------
// The spout and the handle.
// ---------------------------------------------------------------------

/// The spout's length along its own axis.
const SPOUT_LEN: f64 = 8.0 / 64.0;
/// The spout's outer radius at the root, and at the tip.
const SPOUT_R0: f64 = 6.0 / 256.0;
const SPOUT_R1: f64 = 3.0 / 256.0;
/// The spout's wall.
const SPOUT_WALL: f64 = 1.0 / 256.0;
/// The spout's root, inside the belly.
const SPOUT_ROOT: Point3<f64> = Point3 {
    x: -1.0 / 32.0,
    y: 3.0 / 64.0,
    z: 0.0,
};
/// The spout's axis: the 3-4-5 direction, so the placement's rotation
/// matrix is exact in binary and `transform_rigid`'s orthonormality
/// decide has nothing to round.
const SPOUT_DIR: Vec3<f64> = Vec3 {
    x: -0.8,
    y: 0.6,
    z: 0.0,
};

/// The handle's spine radius — half the chord between its two roots on
/// the belly wall, so the unextended window is exactly a semicircle.
const HANDLE_R: f64 = 6.0 / 256.0;
/// The handle's tube radius.
const HANDLE_TUBE: f64 = 1.0 / 128.0;
/// The handle's spine centre, ON the belly wall at the belly's own
/// mid-height.
const HANDLE_C: Point3<f64> = Point3 {
    x: R_BELLY,
    y: 14.0 / 256.0,
    z: 0.0,
};
/// How far past the semicircle each end of the handle runs, in radians
/// of its own spine. The overshoot is what buries each root INSIDE the
/// belly wall rather than tangent to it, which is what makes the union
/// attempted below a real request rather than a touching one.
const HANDLE_OVER: f64 = 0.5;

/// The bellied pot's foot radius. Its belly is a sphere zone of the
/// stepped pot's OWN widest radius (`R_BELLY`) centred on the axis at
/// `Y_MOUTH / 2`, and it meets the foot and the mouth at that sphere's
/// two 3-4-5 stations — so the two pots share their widest radius,
/// their mouth radius and both of those stations, and differ only in
/// whether the shoulders are turned or squared.
const BELLIED_FOOT: f64 = 4.0 / 64.0;

/// The scene's chord budget.
const DELTA: f64 = 2e-4;

// ---------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------

/// One full revolve of `lp` about the sketch's own `+y` axis.
fn revolved(lp: ProfileLoop<f64>, tol: Tol) -> Body<f64> {
    revolve(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("the meridian validates"),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("the meridian fully revolves")
    .body
}

/// **The vessel's meridian**: base, foot, lower shoulder, belly, upper
/// shoulder, neck, mouth — every junction a right angle, which is the
/// scene's finding 1 written as geometry.
fn vessel_meridian(tol: Tol) -> ProfileLoop<f64> {
    Open.at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(R_FOOT, 0.0), tol)
        .expect("the base disc")
        .line_to(Point2::new(R_FOOT, Y_FOOT), tol)
        .expect("the foot")
        .line_to(Point2::new(R_BELLY, Y_FOOT), tol)
        .expect("the lower shoulder")
        .line_to(Point2::new(R_BELLY, Y_SHOULDER), tol)
        .expect("the belly")
        .line_to(Point2::new(R_NECK, Y_SHOULDER), tol)
        .expect("the upper shoulder")
        .line_to(Point2::new(R_NECK, Y_MOUTH), tol)
        .expect("the neck")
        .line_to(Point2::new(0.0, Y_MOUTH), tol)
        .expect("the mouth disc")
        .line_to(Start, tol)
        .expect("the axis closes the meridian")
        .into()
}

/// **The meridian the model wanted**: the same pot with the shoulders
/// and the belly replaced by one arc about a centre ON the axis — a
/// sphere zone, the shape every teapot in the world has. Built only to
/// be shelled, in wall 1.
fn bellied_meridian(tol: Tol) -> ProfileLoop<f64> {
    Open.at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(BELLIED_FOOT, 0.0), tol)
        .expect("the base disc")
        .line_to(Point2::new(BELLIED_FOOT, Y_FOOT), tol)
        .expect("the foot")
        .arc_to(
            Center {
                c: Point2::new(0.0, 1.0 / 16.0),
                winding: ArcSweep::Ccw,
                p: Point2::new(R_NECK, Y_MOUTH),
            },
            tol,
        )
        .expect("the belly rides a sphere centred on the axis")
        .line_to(Point2::new(0.0, Y_MOUTH), tol)
        .expect("the mouth disc")
        .line_to(Start, tol)
        .expect("the axis closes the meridian")
        .into()
}

/// **The lid's meridian**: underside annulus, dome, knob wall, knob
/// top annulus, vent bore.
fn lid_meridian(tol: Tol) -> ProfileLoop<f64> {
    Open.at(Point2::new(R_VENT, LID_BASE))
        .line_to(Point2::new(R_NECK, LID_BASE), tol)
        .expect("the underside seats on the mouth rim")
        .arc_to(
            Center {
                c: Point2::new(0.0, DOME_C),
                winding: ArcSweep::Ccw,
                p: Point2::new(R_KNOB, Y_KNOB),
            },
            tol,
        )
        .expect("the dome rides a sphere centred on the axis")
        .line_to(Point2::new(R_KNOB, Y_TOP), tol)
        .expect("the knob's wall")
        .line_to(Point2::new(R_VENT, Y_TOP), tol)
        .expect("the knob's top")
        .line_to(Start, tol)
        .expect("the vent closes the meridian")
        .into()
}

/// **The spout's meridian**: an annular trapezoid — a cone frustum
/// with a cone frustum bored out of it, one wall thick.
fn spout_meridian(tol: Tol) -> ProfileLoop<f64> {
    Open.at(Point2::new(SPOUT_R0 - SPOUT_WALL, 0.0))
        .line_to(Point2::new(SPOUT_R0, 0.0), tol)
        .expect("the root annulus")
        .line_to(Point2::new(SPOUT_R1, SPOUT_LEN), tol)
        .expect("the outer cone")
        .line_to(Point2::new(SPOUT_R1 - SPOUT_WALL, SPOUT_LEN), tol)
        .expect("the tip annulus")
        .line_to(Start, tol)
        .expect("the bore closes the meridian")
        .into()
}

/// The placement that takes the spout's own `+y` axis onto
/// [`SPOUT_DIR`] and its root onto [`SPOUT_ROOT`]. The rotation is the
/// 3-4-5 turn about `+z`, so every entry is exact.
fn spout_placement() -> Affine3<f64> {
    Affine3::from_parts(
        Mat3::from_cols(
            Vec3::new(SPOUT_DIR.y, -SPOUT_DIR.x, 0.0),
            SPOUT_DIR,
            Vec3::unit_z(),
        ),
        Vec3::new(SPOUT_ROOT.x, SPOUT_ROOT.y, SPOUT_ROOT.z),
    )
}

/// Every planar face of `body` whose plane sits at station `y` — the
/// CHART GROUP, not a face: a full revolve cuts each wall at two seam
/// meridians, so the mouth is two half-discs sharing one plane, and
/// `shell_open` lifts a chart as one (`ShellError::OpenFaceChartPartial`
/// is what a half designation gets).
fn plane_chart_at(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

/// The one closed latitude rim of `body` whose circle sits at station
/// `y` with radius `r` — the selection said BY DESCRIPTION, by hand,
/// because a directly revolved body has no selector (the register's
/// standing gap; `bud::rims_between` and `klein::corner_edges` are the
/// same scan).
fn rim_at(body: &Body<f64>, y: f64, r: f64) -> EdgeKey {
    let hits: Vec<EdgeKey> = body
        .edges()
        .filter(|(k, _)| {
            let Some(c) = body
                .get_curve_geom(body.get_edge(*k).expect("the edge").curve)
                .and_then(|g| g.certified())
            else {
                return false;
            };
            matches!(*c.carrier(), Curve3::Circle { center, radius, .. }
                if (center.y - y).abs() < 1e-12 && (radius - r).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(
        hits.len(),
        1,
        "the description (station {y}, radius {r}) names exactly one rim"
    );
    hits[0]
}

// ---------------------------------------------------------------------
// The closed forms — derived HERE from the authored stations, never
// read back from the bodies they check.
// ---------------------------------------------------------------------

/// The volume of a stack of coaxial cylindrical segments `(radius,
/// height)`.
fn stack_volume(segments: &[(f64, f64)]) -> f64 {
    PI * segments.iter().map(|(r, h)| r * r * h).sum::<f64>()
}

/// A spherical zone's own two closed forms about a sphere of radius
/// `r` centred at station `c`, between stations `y0` and `y1`:
/// `(∫π x² dy, Archimedes' 2πr·Δy)`.
fn zone(r: f64, c: f64, y0: f64, y1: f64) -> (f64, f64) {
    let (a, b) = (y0 - c, y1 - c);
    (
        PI * (r * r * (b - a) - (b * b * b - a * a * a) / 3.0),
        2.0 * PI * r * (y1 - y0),
    )
}

/// A right cone frustum's volume between radii `r0` and `r1` over
/// height `h`.
fn frustum_volume(r0: f64, r1: f64, h: f64) -> f64 {
    PI * h * (r0 * r0 + r0 * r1 + r1 * r1) / 3.0
}

/// A right cone frustum's LATERAL area between radii `r0` and `r1`
/// over height `h`.
fn frustum_lateral(r0: f64, r1: f64, h: f64) -> f64 {
    PI * (r0 + r1) * ((r0 - r1) * (r0 - r1) + h * h).sqrt()
}

/// An annulus's area.
fn annulus(ro: f64, ri: f64) -> f64 {
    PI * (ro * ro - ri * ri)
}

/// A door's answer, as one line for the panel's note — the refusal's
/// own payload rather than a sentence about it (the wall-7 lesson: a
/// refusal's TEXT is not evidence of its cause; the payload and the
/// raising site are).
fn describe<T, E: core::fmt::Debug>(outcome: &Result<T, E>) -> String {
    match outcome {
        Ok(_) => "COMPOSED".to_string(),
        Err(e) => format!("{e:?}"),
    }
}


// ---------------------------------------------------------------------
// The scene
// ---------------------------------------------------------------------

pub fn stops(tol: Tol) -> Vec<Stop> {
    let band = Band::linear(tol).expect("the run's band");

    // ---- the vessel, before the wall ----
    let sharp = revolved(vessel_meridian(tol), tol);
    assert_eq!(
        (
            sharp.vertices().count(),
            sharp.edges().count(),
            sharp.faces().count(),
        ),
        (14, 26, 14),
        "seven revolved meridian segments (the eighth is the axis and sweeps nothing), \
         each cut at the two seam meridians into a pair of half-walls"
    );

    // ---- the gates, MEASURED off the body before the verb runs ----
    //
    // A plane's own reach is unbounded, so its per-face collapse margin
    // is vacuous and `wall_clearance` is what stands in for it: every
    // antiparallel non-adjacent planar pair must clear 2t. The curved
    // faces carry the reach margins that ARE definite. Both are read
    // off the operand rather than recited from the stations above.
    let planes: Vec<(f64, f64)> = sharp
        .faces()
        .filter_map(|(_, f)| match sharp.get_surface(f.surface) {
            Some(Surface::Plane { origin, normal, .. }) => Some((origin.y, normal.y)),
            _ => None,
        })
        .collect();
    let mut clearance = f64::INFINITY;
    for (i, a) in planes.iter().enumerate() {
        for b in &planes[i + 1..] {
            if a.1 * b.1 < 0.0 {
                clearance = clearance.min((b.0 - a.0).abs());
            }
        }
    }
    assert!(
        clearance > 2.0 * WALL,
        "the closest antiparallel planar pair clears {clearance} m and two walls need {}",
        2.0 * WALL
    );
    let reach = sharp
        .faces()
        .filter_map(|(_, f)| match sharp.get_surface(f.surface) {
            Some(Surface::Cylinder { radius, .. }) => Some(*radius - WALL),
            _ => None,
        })
        .fold(f64::INFINITY, f64::min);
    assert!(
        reach > 0.0,
        "every cylinder's realized inner radius is positive; the tightest has {reach} m left"
    );

    // ---- the mouth's CHART, and the opened shell ----
    let mouth = plane_chart_at(&sharp, Y_MOUTH);
    assert_eq!(
        mouth.len(),
        2,
        "the mouth is one plane worn by TWO half-disc faces — a full revolve's seam \
         cut — and the rim lift moves a chart as one"
    );
    let cup = pncad::topo::shell_open(&sharp, WALL, &mouth, FIT_TOL, band, tol)
        .unwrap_or_else(|e| panic!("the pot opens at its mouth, got {e}"));
    let (cv, ce, cf) = (
        cup.vertices().count(),
        cup.edges().count(),
        cup.faces().count(),
    );
    assert_eq!(
        (cv, ce, cf),
        (28, 52, 26),
        "the operand's 14/26/14 doubled, less the mouth's two half-discs, which die in \
         the rim surgery: the cavity's counterpart is what is consumed, not the \
         designated face"
    );
    assert_eq!(
        cup.shells().count(),
        1,
        "the rim FUSES the cavity into the boundary — a cup is closed and has one shell; \
         nothing opens (D1's manifold-first stance is untouched)"
    );
    let rings: usize = cup.faces().map(|(_, f)| f.rings.len()).sum();
    assert_eq!(
        rings,
        2,
        "one ring per designated HALF-disc: the chart is lifted as one and each of its \
         two faces comes back annular"
    );
    for k in &mouth {
        assert_eq!(
            cup.get_face(*k)
                .expect("the designated face survives as the rim")
                .rings
                .len(),
            1,
            "each mouth half-disc carries exactly one ring"
        );
    }

    // ---- the cup against its closed forms ----
    //
    // The outer solid is a stack of three cylindrical segments; the
    // cavity is the same stack shrunk by one wall in every direction,
    // with the neck's segment running all the way to the MOUTH PLANE
    // because that is where the lift put the cavity's counterpart.
    let v_out = stack_volume(&[
        (R_FOOT, Y_FOOT),
        (R_BELLY, Y_SHOULDER - Y_FOOT),
        (R_NECK, Y_MOUTH - Y_SHOULDER),
    ]);
    let v_cav = stack_volume(&[
        (R_FOOT - WALL, Y_FOOT),
        (R_BELLY - WALL, Y_SHOULDER - Y_FOOT - 2.0 * WALL),
        (R_NECK - WALL, Y_MOUTH - Y_SHOULDER + WALL),
    ]);
    let v_want = v_out - v_cav;
    let a_want = {
        let outer = PI * R_FOOT * R_FOOT
            + 2.0 * PI * R_FOOT * Y_FOOT
            + annulus(R_BELLY, R_FOOT)
            + 2.0 * PI * R_BELLY * (Y_SHOULDER - Y_FOOT)
            + annulus(R_BELLY, R_NECK)
            + 2.0 * PI * R_NECK * (Y_MOUTH - Y_SHOULDER);
        let inner = PI * (R_FOOT - WALL) * (R_FOOT - WALL)
            + 2.0 * PI * (R_FOOT - WALL) * Y_FOOT
            + annulus(R_BELLY - WALL, R_FOOT - WALL)
            + 2.0 * PI * (R_BELLY - WALL) * (Y_SHOULDER - Y_FOOT - 2.0 * WALL)
            + annulus(R_BELLY - WALL, R_NECK - WALL)
            + 2.0 * PI * (R_NECK - WALL) * (Y_MOUTH - Y_SHOULDER + WALL);
        outer + annulus(R_NECK, R_NECK - WALL) + inner
    };
    let cup_props = pncad::topo::mass_properties(&cup, tol).expect("the cup's props");
    assert!(
        ((cup_props.volume - v_want) / v_want).abs() < 1e-12,
        "cup V = {} vs the wall's own closed form {v_want}",
        cup_props.volume
    );
    assert!(
        ((cup_props.surface_area - a_want) / a_want).abs() < 1e-12,
        "cup A = {} vs the closed form {a_want}",
        cup_props.surface_area
    );
    assert_eq!(cup_props.volume_pad, 0.0, "closed forms need no pad");

    // ---- the lid ----
    let plain_lid = revolved(lid_meridian(tol), tol);
    assert_eq!(
        (
            plain_lid.vertices().count(),
            plain_lid.edges().count(),
            plain_lid.faces().count(),
        ),
        (5, 10, 5),
        "an ANNULAR profile mints ONE full wall per segment — five walls, five closed \
         latitude rims and five seam meridians — where the pot's axis-touching profile \
         mints half-walls and open arcs"
    );
    let knob_rim = rim_at(&plain_lid, Y_TOP, R_KNOB);
    let rolled = fillet_edges(&plain_lid, &[knob_rim], ROLL, band, tol)
        .unwrap_or_else(|e| panic!("the knob's cylinder x plane top rim rolls, got {e:?}"));
    assert_eq!(
        (
            rolled.body.vertices().count(),
            rolled.body.edges().count(),
            rolled.body.faces().count(),
        ),
        (6, 12, 6),
        "the annulus band's own census delta: +1 vertex, +2 edges, +1 face"
    );
    assert_eq!(rolled.band_faces.len(), 1, "one rim, one band");
    let (band_major, band_minor) = {
        let f = rolled.band_faces[0];
        match rolled
            .body
            .get_surface(rolled.body.get_face(f).expect("the band face").surface)
        {
            Some(Surface::Torus {
                major_radius,
                minor_radius,
                center,
                ..
            }) => {
                assert!(
                    center.x.abs() < 1e-12
                        && center.z.abs() < 1e-12
                        && (center.y - (Y_TOP - ROLL)).abs() < 1e-12,
                    "the band's spine circle sits one roll below the knob's top, on the \
                     axis: got {center:?}"
                );
                (*major_radius, *minor_radius)
            }
            other => panic!("a coaxial cylinder x plane band is a torus, got {other:?}"),
        }
    };
    // The band re-derived from the two tangency conditions rather than
    // read back from the arm: on a cylinder of radius R_KNOB the ball's
    // centre rides at R_KNOB - r; on a plane ⊥ the axis it rides r
    // below it. Both are lines in the meridian and they cross once.
    assert!(
        (band_minor - ROLL).abs() < 1e-12 && (band_major - (R_KNOB - ROLL)).abs() < 1e-12,
        "the band is the torus (R_KNOB - roll, roll) = ({}, {ROLL}); got ({band_major}, \
         {band_minor})",
        R_KNOB - ROLL
    );
    assert!(
        rolled
            .body
            .get_face(rolled.band_faces[0])
            .expect("the band face")
            .rings
            .is_empty(),
        "a curved band is ring-free: one cycle, two closed trim circles and a slit"
    );

    // The SHARP lid against its closed forms — the dome by Archimedes,
    // the rest by the stack — and the roll then bounded against that
    // twin rather than against a remembered number.
    let (v_dome, a_dome) = zone(DOME_R, DOME_C, LID_BASE, Y_KNOB);
    let v_lid = v_dome + stack_volume(&[(R_KNOB, Y_TOP - Y_KNOB)])
        - stack_volume(&[(R_VENT, Y_TOP - LID_BASE)]);
    let a_lid = a_dome
        + 2.0 * PI * R_KNOB * (Y_TOP - Y_KNOB)
        + annulus(R_KNOB, R_VENT)
        + annulus(R_NECK, R_VENT)
        + 2.0 * PI * R_VENT * (Y_TOP - LID_BASE);
    let sharp_lid_props = pncad::topo::mass_properties(&plain_lid, tol).expect("the lid's props");
    assert!(
        ((sharp_lid_props.volume - v_lid) / v_lid).abs() < 1e-12,
        "lid V = {} vs the closed form {v_lid}",
        sharp_lid_props.volume
    );
    assert!(
        ((sharp_lid_props.surface_area - a_lid) / a_lid).abs() < 1e-12,
        "lid A = {} vs the closed form {a_lid}",
        sharp_lid_props.surface_area
    );
    let lid_props = pncad::topo::mass_properties(&rolled.body, tol).expect("the rolled lid");
    let dv_lid = sharp_lid_props.volume - lid_props.volume;
    let pappus_cap = 2.0 * PI * R_KNOB * ROLL * ROLL;
    assert!(
        dv_lid > 0.0 && dv_lid < pappus_cap,
        "a convex rim's roll removes material, and less than the corner square swept \
         round the rim: ΔV = {dv_lid} against the bound {pappus_cap}"
    );

    // ---- the spout: built about its own axis, then placed ----
    let spout = pncad::topo::transform_rigid(
        &revolved(spout_meridian(tol), tol),
        &spout_placement(),
        tol,
    )
    .expect("the spout is placed by a rigid map");
    let v_spout = frustum_volume(SPOUT_R0, SPOUT_R1, SPOUT_LEN)
        - frustum_volume(SPOUT_R0 - SPOUT_WALL, SPOUT_R1 - SPOUT_WALL, SPOUT_LEN);
    let a_spout = frustum_lateral(SPOUT_R0, SPOUT_R1, SPOUT_LEN)
        + frustum_lateral(SPOUT_R0 - SPOUT_WALL, SPOUT_R1 - SPOUT_WALL, SPOUT_LEN)
        + annulus(SPOUT_R0, SPOUT_R0 - SPOUT_WALL)
        + annulus(SPOUT_R1, SPOUT_R1 - SPOUT_WALL);
    let spout_props = pncad::topo::mass_properties(&spout, tol).expect("the spout's props");
    // Asserted AFTER the placement, which is the point: the closed
    // forms are stated in the spout's OWN frame and the rigid map is
    // what has to have left them alone.
    assert!(
        ((spout_props.volume - v_spout) / v_spout).abs() < 1e-12,
        "spout V = {} vs the frustum difference {v_spout}",
        spout_props.volume
    );
    assert!(
        ((spout_props.surface_area - a_spout) / a_spout).abs() < 1e-12,
        "spout A = {} vs the closed form {a_spout}",
        spout_props.surface_area
    );

    // ---- the handle ----
    let sweep = 2.0 * (FRAC_PI_2 + HANDLE_OVER);
    let handle = tube_along_arc::<f64>(
        HANDLE_C,
        Vec3::unit_z(),
        Vec3::unit_x(),
        HANDLE_R,
        TubeWindow::Arc {
            t0: -(FRAC_PI_2 + HANDLE_OVER),
            t1: FRAC_PI_2 + HANDLE_OVER,
        },
        HANDLE_TUBE,
        tol,
    )
    .expect("the handle's arc tube builds")
    .body;
    let v_handle = sweep * HANDLE_R * PI * HANDLE_TUBE * HANDLE_TUBE;
    let a_handle = sweep * HANDLE_R * 2.0 * PI * HANDLE_TUBE + 2.0 * PI * HANDLE_TUBE * HANDLE_TUBE;
    let handle_props = pncad::topo::mass_properties(&handle, tol).expect("the handle's props");
    assert!(
        ((handle_props.volume - v_handle) / v_handle).abs() < 1e-12,
        "handle V = {} vs Pappus on the disc {v_handle}",
        handle_props.volume
    );
    assert!(
        ((handle_props.surface_area - a_handle) / a_handle).abs() < 1e-12,
        "handle A = {} vs the closed form {a_handle}",
        handle_props.surface_area
    );

    // ---- the walls ----
    //
    // Every one of these is ATTEMPTED here, on every pass, and pinned
    // by its own EXACT typed refusal: a different refusal or a success
    // fails the tour, so the findings list above cannot rot behind a
    // frontier that moved.

    // WALL 1 — the pot the model wanted. The stepped meridian above is
    // not a stylistic choice; it is what survives, and this is where
    // that is measured rather than claimed.
    let bellied = revolved(bellied_meridian(tol), tol);
    let bellied_mouth = plane_chart_at(&bellied, Y_MOUTH);
    crate::walls::wall(
        "teapot",
        1,
        "shell the pot with a BELLY — the same widest radius, the same mouth, the \
         shoulders turned into one sphere zone about a centre on the axis",
        pncad::topo::shell_open(&bellied, WALL, &bellied_mouth, FIT_TOL, band, tol),
        |e| {
            matches!(
                e,
                ShellError::Face { error: b, .. }
                    if matches!(**b, ReplaceFaceError::ReanchorOffCarrier { .. })
            )
        },
        "replace `vessel_meridian` with `bellied_meridian` — one arc for three \
         segments — re-derive the cup's closed forms from the zone rather than the \
         stack, and delete this probe together with the junction table in \
         `tests/verbs_teapot.rs`",
    );

    // WALL 2 — the handle joined to the pot. A curved x curved pair at
    // the operand gate; the germ roster has no arm for it.
    //
    // Each union is run ONCE and its refusal is carried into the
    // panel's note verbatim, so the number in the caption and the
    // number the probe pinned cannot be two different measurements.
    let handle_union = pncad::topo::union(&cup, &handle, tol);
    let handle_refusal = describe(&handle_union);
    crate::walls::wall(
        "teapot",
        2,
        "join the handle to the vessel (union, both roots buried in the belly wall)",
        handle_union,
        |e| {
            matches!(
                e,
                BooleanError::CurvedPairUnsupported {
                    op: None,
                    operand: Operand::B,
                    kind: SurfaceKind::Torus,
                    other_kind: SurfaceKind::Cylinder,
                    ..
                }
            )
        },
        "make the teapot ONE solid: union the handle and the spout into the vessel, \
         drop these two probes, and re-state the montage caption, which currently says \
         four solids",
    );

    // WALL 3 — the spout joined to the pot. A DIFFERENT pair, and it
    // is not the pair the model cares about: the gate is pair-scoped
    // and box-conservative, so the face it names is the first whose
    // box MAY meet one of the other operand's — here the spout's outer
    // cone against the pot's base disc, not the belly wall the spout
    // actually pierces.
    let spout_union = pncad::topo::union(&cup, &spout, tol);
    let spout_refusal = describe(&spout_union);
    crate::walls::wall(
        "teapot",
        3,
        "join the spout to the vessel (union, the root buried in the belly)",
        spout_union,
        |e| {
            matches!(
                e,
                BooleanError::CurvedPairUnsupported {
                    op: None,
                    operand: Operand::B,
                    kind: SurfaceKind::Cone,
                    other_kind: SurfaceKind::Plane,
                    ..
                }
            )
        },
        "make the teapot ONE solid: union the handle and the spout into the vessel, \
         drop these two probes, and re-state the montage caption, which currently says \
         four solids",
    );

    // WALL 4 — the SEALED pot as STEP. The cup shipped below is one
    // shell and exports; seal the same shell instead of opening it and
    // the body is a multi-shell CURVED solid, which the writer's
    // outward/void classifier has closed forms for planar faces only.
    // This is the fourth live probe of ONE gate, and they retire
    // together.
    let sealed = pncad::topo::shell(&sharp, WALL, FIT_TOL, band, tol)
        .unwrap_or_else(|e| panic!("the pot seals, got {e}"));
    assert_eq!(
        sealed.shells().count(),
        2,
        "sealed: the outer boundary and the cavity, in one solid"
    );
    crate::walls::wall(
        "teapot",
        4,
        "export the SEALED pot (a curved two-shell solid) as AP214 STEP",
        pncad::step_export::step_string(
            &sealed,
            &pncad::step_export::StepOptions {
                product_name: "teapotsealed".into(),
                ..Default::default()
            },
            tol,
        ),
        |e| {
            matches!(
                e,
                pncad::step_export::StepExportError::CurvedShellClassification { .. }
            )
        },
        "retire ALL FOUR probes of this one gate together: klein's WALL 6, the `ring` \
         scene's `step_at_frontier`, `tubewall::hollowtorus`'s, and this one — and \
         update docs/KERNEL-VERBS.md's hollow-ring STEP row, which names them",
    );

    vec![Stop {
        name: "teapot",
        caption: "THE TEAPOT (FOUR solids: the shelled pot, the lid lifted, and the \
                  spout and handle it will not join)"
            .to_string(),
        montage: true,
        story: "shell's designated demo. The pot is ONE revolved profile hollowed by \
                `shell_open` and opened at its mouth, where the wall's thickness shows \
                as an annular rim. The lid is a SECOND solid, rendered lifted — an \
                exploded view, not a mate. The spout and the handle are two more \
                solids: their unions with the pot are attempted on every pass and \
                REFUSE, so what the montage shows is an assembly of four bodies \
                sitting where a teapot's parts sit",
        ops: "revolve(meridian, +y, Full) -> shell_open(pot, t = 7.8125 mm, [the \
              mouth's CHART]) for the vessel; revolve + fillet_edges(the knob's top \
              rim) for the lid; revolve + transform_rigid for the spout; \
              tube_along_arc for the handle. Four walls pinned: the belly shell, both \
              unions, and the SEALED pot's STEP export",
        delta: DELTA,
        note: Some(format!(
            "THE VESSEL. {cv} vertices, {ce} edges, {cf} faces in ONE shell — the \
             sharp pot's 14/26/14 doubled, less the mouth's two half-discs, which the \
             rim surgery consumes. The mouth is one PLANE worn by two half-disc faces \
             (a full revolve cuts every wall at its two seam meridians), so the \
             designation is a CHART and the lift moves it as one; each half comes back \
             with exactly one ring, {rings} in the body. V = {:.9} m³ and A = {:.9} m² \
             against the closed forms of the wall — the outer stack of three \
             cylindrical segments less the cavity's, whose neck segment runs to the \
             MOUTH PLANE because that is where the lift put the cavity's counterpart. \
             Zero enclosure pad. The gates, measured on the operand before the verb \
             ran: the closest antiparallel planar pair clears {clearance} m where two \
             walls need {:.7} m, so `wall_clearance` — which is what stands in for a \
             plane's vacuous reach margin — does not bind; the tightest cylinder has \
             {reach} m of realized inner radius left. THE BELLY IS NOT AUTHORED AS AN \
             ARC, AND THAT IS THIS SCENE'S FIRST FINDING: the same pot with its \
             shoulders turned into one sphere zone REFUSES (wall 1), and so does a \
             cone frustum, and so does a triangular prism — the class is OBLIQUE \
             junctions, not curvature (`tests/verbs_teapot.rs` carries the table). \
             THE LID. 5/10/5 sharp — an ANNULAR profile mints one FULL wall per \
             segment where the pot's axis-touching profile mints half-walls — and \
             6/12/6 rolled, the annulus band's own (+1, +2, +1). The band is the torus \
             ({band_major}, {band_minor}), which is ({}, {ROLL}) re-derived here from \
             the two tangency lines rather than read back from the arm that minted it, \
             ring-free, centred one roll below the knob's top. ΔV = {dv_lid:.9} m³, \
             inside the corner-square bound {pappus_cap:.9}. The steam vent is what \
             makes that rim a CLOSED edge at all: bore the finial and the profile is \
             annular; leave it solid and the rim is two arcs over two half-discs, which \
             the one-edge annulus band does not carve. NO MATE IS AUTHORED — the lid \
             renders {LIFT} m above the mouth and the two bodies are strangers to the \
             kernel; declared contact is M9's. THE SPOUT AND THE HANDLE. Both build \
             and both check against closed forms — the spout as a difference of cone \
             frusta, asserted AFTER `transform_rigid` placed it, so the map's isometry \
             is part of the receipt; the handle by Pappus on its own disc. Neither \
             JOINS. handle ∪ vessel: {handle_refusal}. spout ∪ vessel: {spout_refusal}. \
             Both are the pair-scoped operand gate naming a germ PAIR with no wired \
             arm, and note what the second one names — the spout's outer CONE against \
             a PLANE of the pot, not the belly wall the spout actually pierces: box \
             overlap is a MAY, and the gate reports the first pair whose boxes may \
             meet. The schedule is the banked germ-chord lanes (DESIGN frontier (d)) \
             and #1057's two C5 arms. A lofted or canal-swept spout — the shape a \
             potter would draw — is not authorable at all: the register carries that \
             as a note, and this scene does not hack around it",
            cup_props.volume,
            cup_props.surface_area,
            2.0 * WALL,
            R_KNOB - ROLL,
        )),
        // The teapot's axis is +y and its spout, handle and lid knob
        // all lie in the world z = 0 plane, so a camera near -z sees
        // the silhouette a teapot is recognised by. Off it by 10
        // degrees toward the spout, and 22 up: enough elevation that
        // the mouth's annular RIM reads as an ellipse rather than a
        // line, and little enough that the lid's lift stays a visible
        // GAP rather than foreshortening onto the rim.
        view: View {
            elev: 22.0,
            azim: -100.0,
            up: 'y',
        },
        bodies: vec![
            SceneBody::plain("teapotvessel", [0.78, 0.74, 0.68], cup),
            SceneBody::plain("teapotlid", [0.62, 0.66, 0.72], rolled.body),
            SceneBody::plain("teapotspout", [0.78, 0.74, 0.68], spout),
            SceneBody::plain("teapothandle", [0.62, 0.66, 0.72], handle),
        ],
    }]
}
