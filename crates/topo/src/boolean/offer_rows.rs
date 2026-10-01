//! **Every tolerance a Boolean refusal offers, executed** (D4 ¶1 (i)).
//!
//! A refusal that ends "if this size is intended, tighten the tolerance
//! below v m" claims that the same operation passes at a smaller
//! tolerance. Each case here is a raise at a band-decided margin: its
//! child row raises it at whatever tolerance its process runs at, and
//! [`test_utils::offer::execute`] re-runs it just below the value the
//! refusal offered, following a chain of different decisions to a pass
//! (the module docs there state what is true and false).
//!
//! The census is by construction: every refusal kind that quotes a
//! margin of its own is placed by an exhaustive match ([`quoting`]),
//! every decision among them enumerated as `want()` enumerates them, and
//! each key and side any of them renders a valued tolerance on has a case
//! here, or one `sweep` runs (`OFFERS_EXECUTED_IN_SWEEP`). A case whose
//! arm has withdrawn its tolerance asserts it stays withdrawn, and
//! executes the offer it withdrew to show what that offer would meet.
//! A case that declares a pair declares it through the Boolean's
//! declaration door (`verify_declared_contacts`); the site cases that
//! hand a declared-`Tangent` question its read directly
//! (`tangent_side`, `tangent_side_of`, `rims`) say so.
//!
//! Each case's margin is a fixed length chosen against the band at
//! [`test_utils::offer::DESIGN_EPS`], so a re-run at a smaller tolerance
//! raises the same geometry. A case says which door it raises through:
//! the public Boolean, or the site that asks the question, where no
//! public door is known to reach it with a margin in band.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::super::reduce::{ContactAcc, CurvedEvent, curved_face_arm};
use super::super::sectors::{NO_CURVATURE, Reach, side_code, tangent_relative_side};
use super::super::{
    BooleanDeclarations, BooleanError, BooleanErrorKind, DeclaredPairs, FacePairDeclaration,
    Operand,
};
use super::tests::every_decision;
use super::*;
use crate::contact::ContactClass;
use crate::entity::VertexKey;
use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet, prism_z};
use geom_brep::OutwardNormal;
use geom_core::{Band, Indeterminate, MarginDiag, Point3, Tol, Vec3};
use test_utils::offer::{DESIGN_EPS, Outcome, execute, report, run};

/// The fixed margin most cases are built at: the middle of the band at
/// [`DESIGN_EPS`].
const D: f64 = 5.5e-9;

/// A margin in the zero band at [`DESIGN_EPS`].
const Z: f64 = 5e-10;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The outcome a child reports for one raise.
fn outcome(got: Result<(), BooleanError>) -> Outcome {
    match got {
        Ok(()) => Outcome::Pass,
        Err(err) => {
            let (key, defect) = crate::test_support::offer_key(&err);
            Outcome::Refused {
                key,
                defect,
                text: err.to_string(),
            }
        }
    }
}

/// Where a case raises its refusal.
#[derive(Clone, Copy, Debug)]
enum Door {
    /// Through a public Boolean door.
    Public,
    /// At the site that asks the question, and why no public door is
    /// used.
    Site(&'static str),
}

/// What the case's arm offers at its first raise.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Offer {
    /// A valued tolerance, which the chain executes.
    Valued,
    /// No tolerance: the arm withdrew it, and the case pins that, with
    /// what the offer it withdrew would have met.
    Withdrawn(Because),
}

/// What a withdrawn offer would have met, executed: the same raise
/// re-run at [`test_utils::offer::BELOW`] × the value its margin gives
/// (`|m|/K`, the offer the arm would make were it valued) refuses on this
/// decision (F1 where it is the case's own, F2 where it is a defect, F3
/// where it offers no tolerance).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Because {
    Refuses(&'static str),
}

/// One raise.
struct Case {
    /// The child row's name.
    name: &'static str,
    /// The decision its first raise refuses on
    /// (`crate::test_support::offer_key`).
    key: &'static str,
    /// The side of zero the refused margin lies on.
    positive: bool,
    door: Door,
    offer: Offer,
}

/// Declares the cases: one `#[ignore]`d child row each, which raises
/// its refusal and reports it, and the table the parents read.
macro_rules! cases {
    ($($name:ident: $key:literal, $positive:literal, $door:expr, $offer:expr => $raise:expr;)*) => {
        $(
            #[test]
            #[ignore = "a child row: every_offered_tolerance_passes_just_below_it runs it"]
            fn $name() {
                report(stringify!($name), &outcome($raise));
            }
        )*
        const CASES: &[Case] = &[$(Case {
            name: stringify!($name),
            key: $key,
            positive: $positive,
            door: $door,
            offer: $offer,
        }),*];
    };
}

/// The site doors' shared reason.
const SECTOR_SITE: Door = Door::Site(
    "a corner's side against a face is read inside the vertex neighbourhood walk, where the \
     reach, the arm and the elevation are set directly",
);
const CURVED_ARM_SITE: Door = Door::Site(
    "the curved sweep arm is the one door for an edge against a cylinder wall sheet, and a \
     solid built around the pose meets other questions first",
);

use Door::Public;
use Offer::{Valued, Withdrawn};

cases! {
    // The reviewer's C1 table (coincfr3), adopted.
    lever_arm_sector_side_in_band: "LeverArm(SectorSide)", true, SECTOR_SITE, Valued =>
        sector_side(Vec3::new(1.0, 0.0, 1.0), Reach::Extent(D));
    lever_arm_sector_side_in_the_zero_band: "LeverArm(SectorSide)", true, SECTOR_SITE, Valued =>
        sector_side(Vec3::new(1.0, 0.0, 1.0), Reach::Extent(Z));
    sector_side_above: "Coincidence(SectorSide)", true, SECTOR_SITE, Valued =>
        sector_side(Vec3::new(1.0, 0.0, D), Reach::Extent(1.0));
    sector_side_below: "Coincidence(SectorSide)", false, SECTOR_SITE, Valued =>
        sector_side(Vec3::new(1.0, 0.0, -D), Reach::Extent(1.0));
    pierce_curvature_short_of_its_bend: "PierceCurvature", true, SECTOR_SITE, Valued =>
        pierce_curvature(D);
    tangent_side_in_band: "Coincidence(TangentSide)", false, SECTOR_SITE, Valued =>
        tangent_side(false);
    lever_arm_sector_curving_in_band: "LeverArm(SectorCurving)", true, SECTOR_SITE, Valued =>
        tangent_side(true);
    line_clear_of_a_wall: "Coincidence(EdgeOnCurvedFace)", true, CURVED_ARM_SITE, Valued =>
        line_against_a_wall(1.0 + D, 0.0);
    line_inside_a_wall: "Coincidence(EdgeOnCurvedFace)", false, CURVED_ARM_SITE,
        Withdrawn(Because::Refuses("WallRoots(Discriminant)")) =>
        line_against_a_wall(1.0 - D, 0.0);
    endpoint_clear_of_a_wall: "Coincidence(VertexOnCurvedFace)", true, CURVED_ARM_SITE, Valued =>
        line_against_a_wall(1.0, (2.0 * D).sqrt());
    endpoint_inside_a_wall: "Coincidence(VertexOnCurvedFace)", false, CURVED_ARM_SITE,
        Withdrawn(Because::Refuses("WallRoots(Discriminant)")) =>
        line_run([(1.0 - D, 0.0), (2.0, 0.0), (2.0, 2.0), (1.0 - D, 2.0)]);
    arc_clear_of_a_wall: "Coincidence(ArcClearsCurvedFace)", true, CURVED_ARM_SITE, Valued =>
        arc_against_a_wall(1.0 + D, None);
    // Declared `Rest` through the door, the walls are one carrier and the
    // arc's ends are read; a smaller tolerance decides the radii apart.
    arc_ends_clear_of_a_covered_wall: "Coincidence(VertexOnCoveredFace)", true, CURVED_ARM_SITE,
        Withdrawn(Because::Refuses("ContactContradicted")) =>
        arc_against_a_wall(1.0 + D, Some(ContactClass::Rest));
    thin_wedge_on_a_block: "Coincidence(Sectors)", true, Public, Valued =>
        wedge_on_a_block(super::super::BooleanOp::Union, 5.0, D);
    thin_wedge_cut_from_a_block: "Coincidence(Sectors)", true, Public, Valued =>
        wedge_on_a_block(super::super::BooleanOp::Subtract, 5.0, D);
    thin_wedge_meeting_a_block: "Coincidence(Sectors)", true, Public, Valued =>
        wedge_on_a_block(super::super::BooleanOp::Intersect, 5.0, D);
    // Tilted past the band at the arm, so the normals decide it off, while
    // its bounds still read On: the bounds' margin decides it.
    thinner_wedge_tilted_past_the_band: "Coincidence(Sectors)", true, Public, Valued =>
        wedge_on_a_block(super::super::BooleanOp::Union, 2.0, 1.5e-8);
    // The arms this pass withdrew, each on the raise that showed its
    // offer false.
    tangent_screen_of_a_tilted_block: "Coincidence(Planes)", true, Public,
        Withdrawn(Because::Refuses("UnsupportedDeclarationClass")) =>
        tilted_block_declared_tangent();
    membership_along_a_curved_flank: "Coincidence(CurvedFlankSense)", true, Door::Site(
        "the membership tie is read inside the edge-edge resolution, on hand-built sectors: \
         an in-band sense needs a flanker arm inside the band, which the corner's arm rung \
         refuses first on a public raise",
    ), Withdrawn(Because::Refuses("CurvedBooleanUnsupported")) => curved_flank_membership();
    pierce_germ_line_in_band: "Coincidence(Sectors)", true, Door::Site(
        "the germ line reads in band at a transition sector whose vertices stand off their own \
         face by up to the zero band, which a valid body allows and no public raise here builds",
    ), Valued => pierce_germ_line();
    // The rest of the census, one per arm and side.
    vertex_hovering_over_a_face: "Coincidence(VertexOnFace)", true, Public, Valued =>
        block_on_a_block(1.0 + D);
    vertex_sunk_into_a_face: "Coincidence(VertexOnFace)", false, Public, Valued =>
        block_on_a_block(1.0 - D);
    edge_nearly_along_an_edge: "Coincidence(EdgeOnEdge)", true, CORNER_SITE, Valued =>
        super::super::recl::parallel_same_dir(
            Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, D, 0.0), 1.0, band()
        ).map(|_| ());
    direction_along_a_short_arm: "DirectionSense", true, CORNER_SITE, Valued =>
        super::super::sectors::direction_sense(
            Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), D, band()
        ).map(|_| ());
    direction_against_a_short_arm: "DirectionSense", false, CORNER_SITE, Valued =>
        super::super::sectors::direction_sense(
            Vec3::new(1.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), D, band()
        ).map(|_| ());
    direction_just_outside_a_sector: "Coincidence(Sectors)", false, CORNER_SITE, Valued =>
        within_a_quarter(Vec3::new(1.0, -D, 0.0));
    tangent_side_curving_away: "Coincidence(TangentSide)", true, SECTOR_SITE, Valued =>
        tangent_side_of(1.0);
    corner_with_a_short_arm: "Corner(Arm)", true, CORNER_SITE, Valued =>
        corner(Vec3::new(D, 0.0, 0.0), at_degrees(90.0, 1.0), false);
    corner_folding_back_at_the_band: "Corner(Straight { full_circle: false })", false, CORNER_SITE, Valued =>
        corner(Vec3::new(ARM, 0.0, 0.0), at_degrees(135.0, ARM), false);
    circle_corner_folding_forward: "Corner(Straight { full_circle: true })", true, CORNER_SITE, Valued =>
        corner(Vec3::new(ARM, 0.0, 0.0), at_degrees(45.0, ARM), true);
    circle_corner_folding_back: "Corner(Straight { full_circle: true })", false, CORNER_SITE, Valued =>
        corner(Vec3::new(ARM, 0.0, 0.0), at_degrees(135.0, ARM), true);
    torus_with_a_thin_tube: "Torus(Tube)", true, NORMAL_SITE, Valued =>
        pierced_torus(1.0, D);
    horn_torus_at_the_band: "Torus(Ring)", true, NORMAL_SITE, Valued =>
        pierced_torus(1.0, 1.0 - D);
    coaxial_thin_cylinder: "Radius(Cylinder)", true, FRAME_SITE, Valued =>
        coaxial_frame(D, 1.0);
    // The cylinder's guard asks first, so a sphere radius in band beside
    // a clear cylinder radius is a pair that does not meet: decided, it
    // is no section, which a real germ pair never is (the frame's defect).
    coaxial_tiny_sphere: "Radius(Sphere)", true, FRAME_SITE,
        Withdrawn(Because::Refuses("JoinDesync")) =>
        coaxial_frame(0.5, D);
    arc_root_just_inside_its_span: "Crossing(OnEdge)", true, ROOT_SITE, Valued =>
        circle_roots(1.0 - D);
    arc_root_just_outside_its_span: "Crossing(OnEdge)", false, ROOT_SITE, Valued =>
        circle_roots(1.0 + D);
    vertex_near_a_vertex_of_a_curved_face: "VertexOnVertex", true, CURVED_ARM_SITE, Valued =>
        vertex_near_a_sheet_corner();
    rims_a_hair_wider: "Coincidence(Rim)", true, RIM_SITE, Valued => rims(1.0 - D);
    rims_a_hair_narrower: "Coincidence(Rim)", false, RIM_SITE, Valued => rims(1.0 + D);
    germs_nearly_facing: "Coincidence(Join)", true, JOIN_SITE, Valued => germ_facing(D);
    germs_nearly_turned_away: "Coincidence(Join)", false, JOIN_SITE, Valued => germ_facing(-D);
    // Overlapping plane flanks lie on one plane, which the door then asks
    // to be one face.
    flanks_along_a_short_arm: "Coincidence(FlankSense)", true, FLANK_SITE,
        Withdrawn(Because::Refuses("UndeclaredCoincidence")) =>
        planar_flank_membership(false, false);
    // Declared `Rest`, the one face the door verified.
    flanks_along_a_short_arm_declared_rest: "Coincidence(FlankSense)", true, FLANK_SITE, Valued =>
        planar_flank_membership(false, true);
    // Coincident planes facing opposite ways, read at a short arm: the
    // offset rung asks next, and refuses an undeclared pair.
    planes_facing_at_a_short_arm: "PlaneOrientation", false, FLANK_SITE,
        Withdrawn(Because::Refuses("UndeclaredCoincidence")) => shared_side_plane(D);
    flanks_against_a_short_arm: "Coincidence(FlankSense)", false, FLANK_SITE, Valued =>
        planar_flank_membership(true, false);
    neighbours_bent_at_the_band: "Neighbours(Parallel)", true, GATE_SITE, Valued =>
        bent_neighbours(D);
    neighbours_offset_above: "CoplanarNeighbours", false, GATE_SITE, Valued =>
        offset_neighbours(D);
    neighbours_offset_below: "CoplanarNeighbours", true, GATE_SITE, Valued =>
        offset_neighbours(-D);
    neighbours_offset_within_the_zero_band: "CoplanarNeighbours", false, GATE_SITE, Valued =>
        offset_neighbours(Z);
    rim_just_above_a_face: "Coincidence(EdgeOnPlane)", true, RIM_PLANE_SITE, Valued =>
        rim_over_a_brick(1.0 - D);
    rim_just_below_a_face: "Coincidence(EdgeOnPlane)", false, RIM_PLANE_SITE, Valued =>
        rim_over_a_brick(1.0 + D);
    roots_a_hair_apart: "Crossing(Order)", false, ROOT_SITE, Valued => ellipse_roots(false);
    roots_a_hair_apart_across_the_window: "Crossing(Order)", true, ROOT_SITE, Valued =>
        ellipse_roots(true);
    seam_over_a_short_edge: "LeverArm(Seam)", true, SEAM_SITE, Valued =>
        seam(core::f64::consts::FRAC_PI_2, D);
    seam_over_an_edge_in_the_zero_band: "LeverArm(Seam)", true, SEAM_SITE, Valued =>
        seam(core::f64::consts::FRAC_PI_2, Z);
    seam_barely_creased: "SeamWedge", true, SEAM_SITE, Valued => seam(D, 1.0);
    sphere_barely_leaning: "Sphere(RecutAlign)", true, Door::Site(
        "a re-cut sphere's lean is read on a crossing-free escape, where an axis near the escape \
         normal carries a seam across the escape plane that the crossing layer meets first: no \
         public raise is known to reach it in band"
    ), Valued => recut(D);
}

/// An arm just above the band's escalation edge at [`DESIGN_EPS`], so a
/// corner's straightness reads in band once its wideness has not
/// decided.
const ARM: f64 = 1.2e-8;

const CORNER_SITE: Door = Door::Site(
    "a corner's own readings are taken inside the vertex neighbourhood walk, where the arm \
     and the directions are set directly",
);
const NORMAL_SITE: Door = Door::Site(
    "a pierced face's normal is read at the pierce door, on a face whose torus is set \
     directly",
);
const FRAME_SITE: Door = Door::Site(
    "the radius guards run on the declared-coaxial cylinder and sphere frame, which no public \
     door passes",
);
const ROOT_SITE: Door =
    Door::Site("the conic root lane is asked of a carrier and a plane, set directly");
const RIM_SITE: Door = Door::Site(
    "the rim identity is read inside a declared-Tangent verification, on two wall sheets' rims \
     set directly",
);
const JOIN_SITE: Door =
    Door::Site("the join's facing is read on germs the reduction left, set directly");
const FLANK_SITE: Door =
    Door::Site("the membership tie is read inside the edge-edge resolution, on hand-built sectors");
const RIM_PLANE_SITE: Door = Door::Site(
    "a rim's offset from a parallel face is read in the sweep, on a wall sheet and a brick run \
     through it directly",
);
const SEAM_SITE: Door = Door::Site(
    "a seam of the result is read as its edges are re-described, on two surfaces set directly",
);
const GATE_SITE: Door = Door::Site(
    "the maximal-faces gate is asked of the operand directly: a solid whose neighbours are bent \
     at the band has a vertex off its face at a smaller tolerance",
);

// ------------------------------------------------------------------
// The raises.
// ------------------------------------------------------------------

/// A corner's bound `dir` read against the face `z = 0` (outward `+z`),
/// behind it `reach`, at arm 1.
fn sector_side(dir: Vec3<f64>, reach: Reach<f64>) -> Result<(), BooleanError> {
    let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
    side_code(dir, reach, n, 1.0, NO_CURVATURE(), band()).map(|_| ())
}

/// A bisector at arm ½ leaving a face of bend radius 1 by `margin` more
/// than the face bends away over the arm.
fn pierce_curvature(margin: f64) -> Result<(), BooleanError> {
    let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
    let c = 0.5 + 2.0 * margin;
    let dir = Vec3::new((1.0 - c * c).sqrt(), 0.0, c);
    side_code(dir, Reach::Bisector(0.5), n, 0.5, 1.0, band()).map(|_| ())
}

/// A ball on a floor, handed the read a declared-`Tangent` door gives:
/// the second-order side at the arm whose sagitta is `D`, or the arm
/// gate at an arm of `D`.
fn tangent_side(arm_gate: bool) -> Result<(), BooleanError> {
    let (p, d) = (Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
    let ball = geom::Surface::Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let floor = crate::test_support_fixtures::plane(
        &[p, Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
        Tol::witness(),
    );
    let accel = |s: &geom::Surface<f64>| {
        -geom_brep::implicit_hessian_form(s, p, d) / geom_brep::implicit_gradient(s, p).dot(n.vec())
    };
    let arm = if arm_gate {
        D
    } else {
        (2.0 * D / (accel(&ball) - accel(&floor)).abs()).sqrt()
    };
    let read = DeclarationRead::Spent(ContactClass::Tangent);
    tangent_relative_side(&ball, &floor, n, p, d, arm, read, band()).map(|_| ())
}

/// The bottom edge of a thin prism, `x0` from the axis of a unit
/// cylinder wall sheet, the prism's near side at `y0`.
fn line_against_a_wall(x0: f64, y0: f64) -> Result<(), BooleanError> {
    let profile = if y0 == 0.0 {
        [(x0, -0.01), (x0 + 1.0, -0.01), (x0 + 1.0, 0.01), (x0, 0.01)]
    } else {
        [(x0, y0), (x0 + 1.0, y0), (x0 + 1.0, 2.0), (x0, 2.0)]
    };
    line_run(profile)
}

fn line_run(profile: [(f64, f64); 4]) -> Result<(), BooleanError> {
    let tol = Tol::witness();
    let prism = prism_z::<f64>(&profile, 0.25, 0.75, tol);
    let x = prism.body;
    let (u, v) = (prism.bottom[3], prism.bottom[0]);
    let pt = |k: VertexKey| *x.get_point(x.get_vertex(k).unwrap().point).unwrap();
    let (edge_key, edge) = x
        .edges()
        .find(|(_, e)| {
            let ends = [e.he_plus, e.he_minus].map(|he| x.get_half_edge(he).unwrap().start);
            ends == [u, v] || ends == [v, u]
        })
        .map(|(k, e)| (k, e.clone()))
        .unwrap();
    let (a, c) = (
        x.get_half_edge(edge.he_plus).unwrap().start,
        x.get_half_edge(edge.he_minus).unwrap().start,
    );
    let mut y: crate::body::Body<f64> = crate::body::Body::new();
    let wall = cyl_wall_sheet(
        &mut y,
        CylFrame::canonical(1.0),
        None,
        (-0.5, 0.5),
        (0.0, 1.0),
        tol,
    );
    let declared = DeclaredPairs::build(&BooleanDeclarations::none(), Default::default());
    let mut acc = ContactAcc::default();
    curved_face_arm(
        &x,
        &mut y,
        Operand::A,
        edge_key,
        &edge,
        a,
        c,
        wall,
        pt(a),
        pt(c),
        &declared,
        &mut acc,
        band(),
        tol,
    )
    .map(|_: CurvedEvent<f64>| ())
}

/// The rim arcs of a radius-`r` wall sheet against a unit wall sheet
/// sharing its axis, the pair declared `class` through the Boolean's
/// declaration door (`verify_declared_contacts`, which also says whether
/// the two are one carrier): the first arc's raise.
fn arc_against_a_wall(r: f64, class: Option<ContactClass>) -> Result<(), BooleanError> {
    let tol = Tol::witness();
    let mut x: crate::body::Body<f64> = crate::body::Body::new();
    let xw = cyl_wall_sheet(
        &mut x,
        CylFrame::canonical(r),
        None,
        (0.5, 1.0),
        (0.25, 0.5),
        tol,
    );
    let sense = x.get_face(xw).unwrap().sense;
    x.set_face_sense(xw, !sense).unwrap();
    let mut y: crate::body::Body<f64> = crate::body::Body::new();
    let yw = cyl_wall_sheet(
        &mut y,
        CylFrame::canonical(1.0),
        None,
        (0.0, 3.0),
        (0.0, 1.0),
        tol,
    );
    let decls = BooleanDeclarations {
        coincident_faces: class
            .map(|class| vec![FacePairDeclaration::new(xw, yw, class)])
            .unwrap_or_default(),
        ..BooleanDeclarations::none()
    };
    let one = super::super::verify_declared_contacts(&x, &y, &decls, band())?;
    let declared = DeclaredPairs::build(&decls, one);
    let (edge_key, edge) = x
        .edges()
        .map(|(k, e)| (k, e.clone()))
        .find(|(_, e)| {
            matches!(
                x.get_curve_geom(e.curve),
                Some(crate::null::CurveGeom::Certified(c))
                    if matches!(c.carrier(), geom::Curve3::Circle { .. })
            )
        })
        .unwrap();
    let start = |he| x.get_half_edge(he).unwrap().start;
    let (a, c) = (start(edge.he_plus), start(edge.he_minus));
    let pt = |k: VertexKey| *x.get_point(x.get_vertex(k).unwrap().point).unwrap();
    let mut acc = ContactAcc::default();
    curved_face_arm(
        &x,
        &mut y,
        Operand::A,
        edge_key,
        &edge,
        a,
        c,
        yw,
        pt(a),
        pt(c),
        &declared,
        &mut acc,
        band(),
        tol,
    )
    .map(|_: CurvedEvent<f64>| ())
}

/// A wedge-cornered block of opening `opening_deg` whose bottom face is
/// tilted by `tilt` about one wedge edge, its corner on a block's top
/// face: `op`.
fn wedge_on_a_block(
    op: super::super::BooleanOp,
    opening_deg: f64,
    tilt: f64,
) -> Result<(), BooleanError> {
    use crate::test_support_fixtures::{brick, mapped_cube};
    let tol = Tol::witness();
    let phi = opening_deg.to_radians();
    let (ea, eb) = (
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(phi.cos(), phi.sin(), tilt * phi.sin()),
    );
    let p = Point3::new(0.5, 0.2, 1.0);
    let wedge = mapped_cube::<f64>(
        move |u, v, w| p + ea * u + eb * v + Vec3::new(0.0, 0.0, w),
        tol,
    );
    let block = brick((-1.0, 3.0), (-2.0, 2.5), (0.0, 1.0), tol);
    super::super::boolean_op_with(
        op,
        &block,
        &wedge,
        &BooleanDeclarations::none(),
        super::super::SweepStrategy::Realized,
        tol,
    )
    .map(|_| ())
}

/// A block tilted by an in-band angle on a block's top face, the pair
/// declared `Tangent` (the review's G2 pose).
fn tilted_block_declared_tangent() -> Result<(), BooleanError> {
    use crate::test_support_fixtures::{brick, mapped_cube};
    let tol = Tol::witness();
    let a = brick((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), tol);
    let b = mapped_cube::<f64>(
        |u, v, w| Point3::new(1.0 + 2.0 * u, 1.0 + 2.0 * v, 1.0 + w + (D / 2.0) * u),
        tol,
    );
    let facing = |body: &crate::body::Body<f64>, up: bool| {
        body.faces()
            .map(|(k, _)| k)
            .find(|&k| {
                matches!(super::super::face_carrier(body, k),
                    Some(super::super::CarrierDesc::Plane { normal, .. })
                        if (normal.z > 0.99) == up && normal.z.abs() > 0.99)
            })
            .unwrap()
    };
    let decls = BooleanDeclarations {
        coincident_faces: vec![FacePairDeclaration::new(
            facing(&a, true),
            facing(&b, false),
            ContactClass::Tangent,
        )],
        ..BooleanDeclarations::none()
    };
    super::super::union_with(&a, &b, &decls, tol).map(|_| ())
}

/// Two corners sharing an edge along `z`, one flanked by a cylinder
/// face and the other by a plane tangent to it along the edge, their
/// flankers' arm `D`: the membership tie on a curved flank (the review's
/// probe, undeclared).
fn curved_flank_membership() -> Result<(), BooleanError> {
    use super::super::recl::resolve_edge_edge;
    use super::super::sectors::{BoolSector, PairRecord};
    let tol = Tol::witness();
    let o = Point3::new(0.0, 0.0, 0.0);
    let (x, y, z) = (
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );
    let face_on = |surface| {
        let st = crate::fixtures::mvfs_state();
        let mut body = st.body;
        body.set_face_surface(
            st.face,
            crate::euler::FaceSurface::New {
                surface,
                sense: true,
            },
        )
        .unwrap();
        (body, st.face)
    };
    let (ca, fca) = face_on(geom::Surface::Cylinder {
        origin: Point3::new(0.0, 1.0, 0.0),
        axis: z,
        radius: 1.0,
        u_ref: x,
    });
    let (cb, fcb) = face_on(crate::test_support_fixtures::plane(&[o, o + x, o + z], tol));
    let sector = |start: Vec3<f64>, end: Vec3<f64>, face| BoolSector {
        he: crate::entity::HalfEdgeKey::default(),
        start,
        end,
        start_reach: Reach::Chord {
            base: o,
            far: o + start,
        },
        end_reach: Reach::Chord {
            base: o,
            far: o + end,
        },
        face,
        normal: OutwardNormal::from_chart(y, true),
        arm: D,
    };
    let records = [PairRecord {
        a: 0,
        b: 0,
        sa: (super::super::SideCode::On, super::super::SideCode::Out),
        sb: (super::super::SideCode::Out, super::super::SideCode::In),
        intersect: true,
    }];
    let corner = |face| [sector(z, x, face), sector(x, z, face)];
    let declared = DeclaredPairs::build(&BooleanDeclarations::none(), Default::default());
    resolve_edge_edge(
        &records,
        &corner(fca),
        &corner(fcb),
        &ca,
        &cb,
        super::super::BooleanOp::Union,
        &declared,
        band(),
        0,
        0,
    )
    .map(|_| ())
}

/// A transition sector whose face parts from the pierced face by an
/// in-band angle at its reach.
fn pierce_germ_line() -> Result<(), BooleanError> {
    use super::super::sectors::BoolSector;
    let o = Point3::new(0.0, 0.0, 0.0);
    let (x, y) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let s = BoolSector {
        he: crate::entity::HalfEdgeKey::default(),
        start: x,
        end: y,
        start_reach: Reach::Chord {
            base: o,
            far: o + x,
        },
        end_reach: Reach::Chord {
            base: o,
            far: o + y,
        },
        face: crate::entity::FaceKey::default(),
        normal: OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true),
        arm: 1.0,
    };
    super::super::vtxfac::pierce_germ_dir(&s, Vec3::new(D.sin(), 0.0, D.cos()), band()).map(|_| ())
}

/// `A` a block `[0, 2]² × [0, 1]`, `B` a unit block standing over its
/// middle with its bottom at `z0`: their union.
fn block_on_a_block(z0: f64) -> Result<(), BooleanError> {
    use crate::test_support_fixtures::brick;
    let tol = Tol::witness();
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), tol);
    let b = brick::<f64>((0.5, 1.5), (0.5, 1.5), (z0, 2.0), tol);
    super::super::union(&a, &b, tol).map(|_| ())
}

/// `dir` against the quarter sector from `x` to `y` about `z`, arm 1.
fn within_a_quarter(dir: Vec3<f64>) -> Result<(), BooleanError> {
    use super::super::sectors::BoolSector;
    let o = Point3::new(0.0, 0.0, 0.0);
    let (x, y) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let s = BoolSector {
        he: crate::entity::HalfEdgeKey::default(),
        start: x,
        end: y,
        start_reach: Reach::Chord {
            base: o,
            far: o + x,
        },
        end_reach: Reach::Chord {
            base: o,
            far: o + y,
        },
        face: crate::entity::FaceKey::default(),
        normal: OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true),
        arm: 1.0,
    };
    super::super::sectors::within(&s, dir, false, DeclarationRead::Moot, band()).map(|_| ())
}

/// A declared-`Tangent` ball touching a floor at the origin from the
/// side `side` (`+1` above), at the arm whose sagitta is `D`.
fn tangent_side_of(side: f64) -> Result<(), BooleanError> {
    let (p, d) = (Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
    let ball = geom::Surface::Sphere {
        center: Point3::new(0.0, 0.0, side),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let floor = crate::test_support_fixtures::plane(
        &[p, Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
        Tol::witness(),
    );
    let accel = |s: &geom::Surface<f64>| {
        -geom_brep::implicit_hessian_form(s, p, d) / geom_brep::implicit_gradient(s, p).dot(n.vec())
    };
    let arm = (2.0 * D / (accel(&ball) - accel(&floor)).abs()).sqrt();
    let read = DeclarationRead::Spent(ContactClass::Tangent);
    tangent_relative_side(&ball, &floor, n, p, d, arm, read, band()).map(|_| ())
}

/// A direction in the `z = 0` plane at `degrees` from `x`, of length
/// `len`.
fn at_degrees(degrees: f64, len: f64) -> Vec3<f64> {
    let t = degrees.to_radians();
    Vec3::new(t.cos(), t.sin(), 0.0) * len
}

/// A corner between chords `own` and `next` about `z`, read as the
/// vertex neighbourhood walk reads it.
fn corner(own: Vec3<f64>, next: Vec3<f64>, full_circle: bool) -> Result<(), BooleanError> {
    use crate::sector_shape::{SectorFault, sector_shape};
    let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
    sector_shape(own, next, n, full_circle, band())
        .map(|_| ())
        .map_err(|fault| match fault {
            SectorFault::Rung { rung, diag } => BooleanError::Escalated {
                decision: BooleanDecision::Corner(rung),
                diag,
            },
            other => panic!("the corner's chords are finite: {other:?}"),
        })
}

/// One face on a torus of radii `major`, `minor`, its normal read at
/// the tube's outer equator.
fn pierced_torus(major: f64, minor: f64) -> Result<(), BooleanError> {
    let st = crate::fixtures::mvfs_state();
    let mut body = st.body;
    body.set_face_surface(
        st.face,
        crate::euler::FaceSurface::New {
            surface: geom::Surface::Torus {
                center: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                major_radius: major,
                minor_radius: minor,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            sense: true,
        },
    )
    .unwrap();
    let p = Point3::new(major + minor, 0.0, 0.0);
    crate::face_normal::face_outward_normal_at(&body, st.face, p, band())
        .map(|_| ())
        .map_err(|refusal| BooleanError::of_pierced_normal(refusal, Operand::B, st.face))
}

/// The declared-coaxial frame of a cylinder of radius `cyl` and a
/// sphere of radius `sph` on one axis.
fn coaxial_frame(cyl: f64, sph: f64) -> Result<(), BooleanError> {
    use super::super::join::{cs_pair_frame, frame_refusal};
    let c = geom::Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: cyl,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let s = geom::Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: sph,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let face = crate::entity::FaceKey::default();
    cs_pair_frame(&c, &s, geom_brep::CoaxialEvidence::Declared, band())
        .map(|_| ())
        .map_err(|e| frame_refusal(e, (face, &c), (face, &s)))
}

/// The arc `[0, 1]` of the unit circle about `z` against the plane
/// `x = cos(t)`: a root at `t`, the conic root lane's.
fn circle_roots(t: f64) -> Result<(), BooleanError> {
    let circle = geom::Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    conic_roots(&circle, (0.0, 1.0), t.cos())
}

/// The semi-axes of [`ellipse_roots`]' ellipse: long along the plane's
/// normal, so its two roots can stand a band apart (metered on the minor
/// axis) while the plane still reaches clearly into it.
const ELLIPSE: (f64, f64) = (1e4, 1e-3);

/// An ellipse's arc about its `+x` tip (or, `beyond`, its `−x` tip)
/// against the plane `x = const` cutting it at two parameters `D` apart
/// in metres on the minor axis: which comes first is the root order's
/// question, read negative about `+x` and positive about `−x` (the
/// window's reduction takes the later root first there).
fn ellipse_roots(beyond: bool) -> Result<(), BooleanError> {
    let (major, minor) = ELLIPSE;
    let ellipse = geom::Curve3::Ellipse {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        major,
        minor,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let half = D / (2.0 * minor);
    let (mid, x) = if beyond {
        (core::f64::consts::PI, -major * half.cos())
    } else {
        (0.0, major * half.cos())
    };
    conic_roots(&ellipse, (mid - 0.5, mid + 0.5), x)
}

/// `carrier` over `span` against the plane `x = x0`, as the conic root
/// lane reads it.
fn conic_roots(carrier: &geom::Curve3<f64>, span: (f64, f64), x0: f64) -> Result<(), BooleanError> {
    match crate::splitting::conic_plane_crossing_roots(
        carrier,
        span.0,
        span.1,
        Point3::new(x0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        band(),
    ) {
        Ok(crate::splitting::ConicPlaneMeet::Roots(Err(fault))) => Err(BooleanError::Escalated {
            decision: BooleanDecision::of_conic_root(fault, DeclarationRead::Moot),
            diag: fault.diag(),
        }),
        Ok(_) => Ok(()),
        Err(()) => panic!("a conic"),
    }
}

/// A half cylinder-wall sheet whose top rim runs at `z = 1`, over a
/// brick whose top face is the plane `z = top`: the sweep, where the
/// rim's plane parallel to that face is read for its offset.
fn rim_over_a_brick(top: f64) -> Result<(), BooleanError> {
    use super::super::reduce::coplanar_conic_rows::{brick_under, split_sheet, sweep};
    sweep(&split_sheet().0, &brick_under(top).0).map(|_| ())
}

/// Two planes through the `z` axis at `angle`, read as a seam of the
/// result over `extent`.
fn seam(angle: f64, extent: f64) -> Result<(), BooleanError> {
    let o = Point3::new(0.0, 0.0, 0.0);
    let z = Vec3::new(0.0, 0.0, 1.0);
    let (s1, s2) = (
        plane_through(o, z, Vec3::new(0.0, 1.0, 0.0)),
        plane_through(o, z, Vec3::new(angle.sin(), angle.cos(), 0.0)),
    );
    super::super::ops::seam_class(&s1, &s2, o, extent, band()).map(|_| ())
}

/// A re-cut sphere of radius 1 whose polar axis leans off the escape
/// normal by the angle whose sine is `lean`.
fn recut(lean: f64) -> Result<(), BooleanError> {
    let (axis, align) = (
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(lean, 0.0, (1.0 - lean * lean).sqrt()),
    );
    super::super::ops::recut_lean(axis, align, 1.0, band())
}

/// A point on a unit wall sheet's carrier well outside its trim, `D`
/// from a corner of a second sheet in the same body.
fn vertex_near_a_sheet_corner() -> Result<(), BooleanError> {
    use super::super::reduce::vertex_on_curved_face;
    let tol = Tol::witness();
    let mut y: crate::body::Body<f64> = crate::body::Body::new();
    let face = cyl_wall_sheet(
        &mut y,
        CylFrame::canonical(1.0),
        None,
        (0.0, 1.0),
        (0.0, 1.0),
        tol,
    );
    cyl_wall_sheet(
        &mut y,
        CylFrame::canonical(1.0),
        None,
        (3.0 + D, 4.0),
        (0.5, 1.0),
        tol,
    );
    let px = Point3::new(3.0_f64.cos(), 3.0_f64.sin(), 0.5);
    let mut acc = ContactAcc::default();
    vertex_on_curved_face(
        Operand::A,
        &mut y,
        VertexKey::default(),
        px,
        face,
        &mut acc,
        band(),
        tol,
    )
    .map(|_| ())
}

/// The rims of a unit wall sheet and of one of radius `r` about the
/// same axis and over the same heights, read for a shared rim.
fn rims(r: f64) -> Result<(), BooleanError> {
    let tol = Tol::witness();
    let mut a: crate::body::Body<f64> = crate::body::Body::new();
    let fa = cyl_wall_sheet(
        &mut a,
        CylFrame::canonical(1.0),
        None,
        (0.0, 1.0),
        (0.0, 1.0),
        tol,
    );
    let mut b: crate::body::Body<f64> = crate::body::Body::new();
    let fb = cyl_wall_sheet(
        &mut b,
        CylFrame::canonical(r),
        None,
        (0.0, 1.0),
        (0.0, 1.0),
        tol,
    );
    super::super::rim_wedge::shared_rim(&a, fa, &b, fb, band())
        .map(|_| ())
        .map_err(|diag| {
            BooleanError::coincidence(
                Coincide::Rim,
                DeclarationRead::Spent(ContactClass::Tangent),
                diag,
            )
        })
}

/// Two germs a metre apart along `x`, the second facing the first, the
/// first leaning off the chord so its facing reads `lean` metres.
fn germ_facing(lean: f64) -> Result<(), BooleanError> {
    use super::super::HalfGerm;
    let germ = |dir| HalfGerm {
        he: crate::entity::HalfEdgeKey::default(),
        a_face: crate::entity::FaceKey::default(),
        b_face: crate::entity::FaceKey::default(),
        dir,
    };
    let (p1, p2) = (Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let g1 = germ(Vec3::new(lean, 1.0, 0.0).normalize());
    let g2 = germ(Vec3::new(-1.0, 0.0, 0.0));
    super::super::join::germs_face_each_other(None, &g1, &g2, p1, p2, band()).map(|_| ())
}

/// The membership tie on two prisms' shared side plane `x = 1`, each
/// corner's flanker along `x` (or against it, for the second corner,
/// where `against`), at an arm of `D`, the pair declared `Rest` through
/// the Boolean's declaration door where `rest`.
fn planar_flank_membership(against: bool, rest: bool) -> Result<(), BooleanError> {
    use super::super::SideCode::{In, On, Out};
    use super::super::recl::resolve_edge_edge;
    use super::super::sectors::{BoolSector, PairRecord};
    let tol = Tol::witness();
    let o = Point3::new(0.0, 0.0, 0.0);
    let (x, y, z) = (
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );
    let square = |x0: f64| [(x0, 0.0), (x0 + 1.0, 0.0), (x0 + 1.0, 1.0), (x0, 1.0)];
    let pa = prism_z::<f64>(&square(0.0), 0.0, 1.0, tol);
    let pb = prism_z::<f64>(&square(1.0), 0.0, 1.0, tol);
    let (fa, fb) = (pa.side_faces[1], pb.side_faces[3]);
    let sector = |start: Vec3<f64>, end: Vec3<f64>, face| BoolSector {
        he: crate::entity::HalfEdgeKey::default(),
        start,
        end,
        start_reach: Reach::Chord {
            base: o,
            far: o + start,
        },
        end_reach: Reach::Chord {
            base: o,
            far: o + end,
        },
        face,
        normal: OutwardNormal::from_chart(y, true),
        arm: D,
    };
    let records = [PairRecord {
        a: 0,
        b: 0,
        sa: (On, Out),
        sb: (Out, In),
        intersect: true,
    }];
    let flank = if against { -x } else { x };
    let decls = BooleanDeclarations {
        coincident_faces: if rest {
            vec![FacePairDeclaration::new(fa, fb, ContactClass::Rest)]
        } else {
            Vec::new()
        },
        ..BooleanDeclarations::none()
    };
    let one = super::super::verify_declared_contacts(&pa.body, &pb.body, &decls, band())?;
    let declared = DeclaredPairs::build(&decls, one);
    resolve_edge_edge(
        &records,
        &[sector(z, x, fa), sector(x, z, fa)],
        &[sector(z, flank, fb), sector(flank, z, fb)],
        &pa.body,
        &pb.body,
        super::super::BooleanOp::Union,
        &declared,
        band(),
        0,
        0,
    )
    .map(|_| ())
}

/// Two unit prisms' shared side plane `x = 1`, undeclared, read as an
/// on-pair of corners at arm `arm`.
fn shared_side_plane(arm: f64) -> Result<(), BooleanError> {
    use super::super::sectors::BoolSector;
    let tol = Tol::witness();
    let o = Point3::new(1.0, 0.0, 0.0);
    let (y, z) = (Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let square = |x0: f64| [(x0, 0.0), (x0 + 1.0, 0.0), (x0 + 1.0, 1.0), (x0, 1.0)];
    let pa = prism_z::<f64>(&square(0.0), 0.0, 1.0, tol);
    let pb = prism_z::<f64>(&square(1.0), 0.0, 1.0, tol);
    let sector = |face| BoolSector {
        he: crate::entity::HalfEdgeKey::default(),
        start: y,
        end: z,
        start_reach: Reach::Chord {
            base: o,
            far: o + y,
        },
        end_reach: Reach::Chord {
            base: o,
            far: o + z,
        },
        face,
        normal: OutwardNormal::from_chart(Vec3::new(1.0, 0.0, 0.0), true),
        arm,
    };
    let declared = DeclaredPairs::build(&BooleanDeclarations::none(), Default::default());
    super::super::recl::require_same(
        &pa.body,
        Operand::A,
        &sector(pa.side_faces[1]),
        &pb.body,
        Operand::B,
        &sector(pb.side_faces[3]),
        &declared,
        arm,
        band(),
    )
    .map(|_| ())
}

/// A unit prism's top split on its diagonal, one half bent about it by
/// an angle whose sine over the diagonal is `margin`: the operand's
/// maximal-faces gate.
fn bent_neighbours(margin: f64) -> Result<(), BooleanError> {
    let body = super::tests::top_split_redescribed(|p0, along, diagonal| {
        let theta = margin / diagonal;
        let up = Vec3::new(0.0, 0.0, 1.0);
        plane_through(p0, along, up * theta.cos() + along.cross(up) * theta.sin())
    });
    super::super::reduce::gate_maximal_faces(&body, Operand::A, band())
}

/// The same, the half re-described on the parallel plane `offset`
/// above.
fn offset_neighbours(offset: f64) -> Result<(), BooleanError> {
    let up = Vec3::new(0.0, 0.0, 1.0);
    let body = super::tests::top_split_redescribed(|p0, along, _| {
        plane_through(p0 + up * offset, along, up)
    });
    super::super::reduce::gate_maximal_faces(&body, Operand::A, band())
}

/// The plane through `p` with unit normal `n`, containing the unit
/// direction `along` (perpendicular to `n`).
fn plane_through(p: Point3<f64>, along: Vec3<f64>, n: Vec3<f64>) -> geom::Surface<f64> {
    crate::test_support_fixtures::plane(&[p, p + along, p + n.cross(along)], Tol::witness())
}

// ------------------------------------------------------------------
// The parents.
// ------------------------------------------------------------------

/// The libtest path of a child row.
fn row(case: &Case) -> String {
    let module = module_path!()
        .split_once("::")
        .map_or(module_path!(), |(_, m)| m);
    format!("{module}::{}", case.name)
}

/// The sign a refusal's quoted margin has, read off its payload.
fn margin_is_positive(text: &str) -> Option<bool> {
    let (_, tail) = text.split_once("margin ")?;
    let value = tail.split_whitespace().next()?;
    value.parse::<f64>().ok().map(|m| m > 0.0)
}

/// **Every offered tolerance passes just below the value offered**, on
/// every case whose arm offers one: executed (the module docs).
#[test]
fn every_offered_tolerance_passes_just_below_it() {
    let valued: Vec<&Case> = CASES.iter().filter(|c| c.offer == Valued).collect();
    let verdicts: Vec<(String, Result<Vec<test_utils::offer::Link>, String>)> =
        std::thread::scope(|scope| {
            let handles: Vec<_> = valued
                .iter()
                .map(|case| {
                    scope.spawn(|| {
                        let verdict = execute(
                            &row(case),
                            case.key,
                            crate::test_support::offer_same_decision,
                        );
                        (case.name.to_owned(), verdict)
                    })
                })
                .collect();
            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });
    let mut false_offers = Vec::new();
    for (case, (name, verdict)) in valued.iter().zip(verdicts) {
        match verdict {
            Ok(chain) => {
                let first = &chain[0].outcome;
                let Outcome::Refused { text, .. } = first else {
                    unreachable!("execute returns a chain that starts with its refusal");
                };
                assert_eq!(
                    margin_is_positive(text),
                    Some(case.positive),
                    "{name}: the refused margin is on the side the case states: {text}"
                );
                let path: Vec<String> = chain
                    .iter()
                    .map(|l| match &l.outcome {
                        Outcome::Pass => format!("{:e}: pass", l.eps),
                        Outcome::Refused { key, .. } => format!("{:e}: {key}", l.eps),
                    })
                    .collect();
                let door = match case.door {
                    Door::Public => "the public Boolean",
                    Door::Site(why) => why,
                };
                println!("OFFER {name}: {} [{door}]", path.join(" -> "));
            }
            Err(why) => false_offers.push(why),
        }
    }
    assert!(
        false_offers.is_empty(),
        "false offers:\n{}",
        false_offers.join("\n\n")
    );
}

/// The value a refusal's quoted point margin gives, `|m|/K` at the band
/// its child ran at ([`DESIGN_EPS`], the default `K`): the offer the arm
/// would make were it valued.
fn would_offer(text: &str) -> Option<f64> {
    let (_, tail) = text.split_once("margin ")?;
    let m = tail.split_whitespace().next()?.parse::<f64>().ok()?;
    let band = design_band();
    Some(m.abs() / (band.escalate() / band.zero()))
}

/// **A withdrawn tolerance stays withdrawn, and was false**: each case
/// whose arm offers none refuses on its decision at [`DESIGN_EPS`] with
/// no tolerance; re-run just below the value its margin gives, it
/// refuses as its [`Because`] states.
#[test]
fn every_withdrawn_tolerance_stays_withdrawn() {
    for case in CASES {
        let Offer::Withdrawn(because) = case.offer else {
            continue;
        };
        let Outcome::Refused { key, defect, text } = run(&row(case), DESIGN_EPS) else {
            panic!("{}: passes at the design tolerance", case.name);
        };
        assert_eq!(key, case.key, "{}: {text}", case.name);
        assert!(!defect, "{}: {text}", case.name);
        assert_eq!(
            margin_is_positive(&text),
            Some(case.positive),
            "{}: {text}",
            case.name
        );
        assert!(
            test_utils::offer::offered_below(&text).is_none() && !text.contains("tighten"),
            "{}: offers a tolerance: {text}",
            case.name
        );
        let eps = test_utils::offer::BELOW
            * would_offer(&text).unwrap_or_else(|| panic!("{}: a point margin: {text}", case.name));
        let below = run(&row(case), eps);
        let Because::Refuses(want) = because;
        assert!(
            matches!(&below, Outcome::Refused { key, .. } if key == want),
            "{}: at {eps:e} the withdrawn offer meets {want}: {below:?}",
            case.name
        );
    }
}

/// The band every case's first raise runs at: [`DESIGN_EPS`] and the
/// default `K`, which the children run at whatever the parent's own
/// tolerance is.
fn design_band() -> Band {
    Band::new(DESIGN_EPS, geom_core::tolerance::DEFAULT_K * DESIGN_EPS).unwrap()
}

/// Every refusal of `kind` that quotes a margin of its own, at `diag`'s
/// margin: an exhaustive match, so a new kind is a compile error here
/// until it is placed.
fn quoting(kind: BooleanErrorKind, diag: Indeterminate) -> Vec<BooleanError> {
    use geom_brep::recourse::Refused;
    let zero = || Classified {
        margin: diag.margin,
        band: diag.band,
    };
    let refused = || {
        [
            Refused::Zero(zero()),
            Refused::Negative {
                margin: diag.margin,
            },
        ]
    };
    let (operand, face) = (Operand::A, crate::entity::FaceKey::default());
    match kind {
        BooleanErrorKind::Escalated => every_decision()
            .into_iter()
            .map(|decision| BooleanError::Escalated { decision, diag })
            .collect(),
        BooleanErrorKind::CoplanarNeighbours => [
            super::NeighbourOffset::Zero(zero()),
            super::NeighbourOffset::Undecided(diag),
        ]
        .into_iter()
        .map(|offset| BooleanError::CoplanarNeighbours {
            operand,
            faces: [face, face],
            offset,
        })
        .collect(),
        BooleanErrorKind::SpheresMeet => refused()
            .into_iter()
            .map(|verdict| BooleanError::SpheresMeet {
                operand,
                face,
                verdict,
            })
            .collect(),
        BooleanErrorKind::CurvedSectorSideUnsupported => refused()
            .into_iter()
            .map(|verdict| BooleanError::CurvedSectorSideUnsupported { verdict })
            .collect(),
        BooleanErrorKind::DegenerateTorus => [
            geom_brep::TorusConvention::Tube,
            geom_brep::TorusConvention::Ring,
        ]
        .into_iter()
        .flat_map(|convention| {
                refused().map(|verdict| BooleanError::DegenerateTorus {
                    operand,
                    face,
                    convention,
                    verdict,
                })
            })
            .collect(),
        // Its margin is exactly zero or unreadable, and its recourse
        // names no value (the filed NOTE-1 residue).
        BooleanErrorKind::UndeclaredCoincidence
        // Quote no margin: a frontier, a declaration refused, a kernel
        // invariant, or a range fault.
        | BooleanErrorKind::Band
        | BooleanErrorKind::CurvedBooleanUnsupported
        | BooleanErrorKind::CurvedPierceUnsupported
        | BooleanErrorKind::CurvedEdgeUnsupported
        | BooleanErrorKind::PointSplitCarrierUnsupported
        | BooleanErrorKind::ArcLoopContainmentUnsupported
        | BooleanErrorKind::ScaffoldingOperand
        | BooleanErrorKind::NonMaximalFaces
        | BooleanErrorKind::NonFiniteSectorChord
        | BooleanErrorKind::UnderflowedSectorChord
        | BooleanErrorKind::DeclarationContradicted
        | BooleanErrorKind::ContactContradicted
        | BooleanErrorKind::UnsupportedDeclarationClass
        | BooleanErrorKind::RimSeamNotDeclarable
        | BooleanErrorKind::RimCuspArmUnbuilt
        | BooleanErrorKind::InvalidDeclaration
        | BooleanErrorKind::PairingMismatch
        | BooleanErrorKind::ClassificationInvariant
        | BooleanErrorKind::CorruptOperand
        | BooleanErrorKind::CurvedPairUnsupported
        | BooleanErrorKind::NurbsExtentUnsupported
        | BooleanErrorKind::FallbackExtentUnsupported
        | BooleanErrorKind::GermFrameUnsupported
        | BooleanErrorKind::GermFrameCylinderPinch
        | BooleanErrorKind::RestZipUnsupported
        | BooleanErrorKind::JoinDesync
        | BooleanErrorKind::TornComponent
        | BooleanErrorKind::SeamOrientation
        | BooleanErrorKind::ZipCorrespondence
        | BooleanErrorKind::ResultInvalid
        | BooleanErrorKind::ResultVolumeImplausible
        | BooleanErrorKind::UnrepresentableResult
        // Nest another module's refusal, whose offers are that module's
        // to execute: this census does not reach them.
        | BooleanErrorKind::CrossingInsertion
        | BooleanErrorKind::Containment
        | BooleanErrorKind::Revert
        | BooleanErrorKind::Merge
        | BooleanErrorKind::Pcurves
        | BooleanErrorKind::Euler
        | BooleanErrorKind::Join
        | BooleanErrorKind::GraftRecertify => Vec::new(),
    }
}

/// The keys and sides the Boolean's refusals render a valued tolerance
/// on: every kind that quotes a margin of its own ([`quoting`]), every
/// decision ([`every_decision`]) among them, at a point margin in band
/// and in the zero band on each side.
fn offering_arms() -> std::collections::BTreeSet<(String, bool)> {
    let band = design_band();
    let mut out = std::collections::BTreeSet::new();
    for kind in <BooleanErrorKind as strum::IntoEnumIterator>::iter() {
        for m in [D, -D, Z, -Z] {
            let diag = Indeterminate {
                margin: MarginDiag::value(m),
                band,
                predicate: Some("census"),
                terminal_sliver: false,
            };
            for err in quoting(kind, diag) {
                if test_utils::offer::offered_below(&err.to_string()).is_some() {
                    out.insert((crate::test_support::offer_key(&err).0, m > 0.0));
                }
            }
        }
    }
    out
}

/// **The census**: every decision arm the table renders a valued
/// tolerance on has a case that executes it, here or in `sweep`, and
/// every case stands for such an arm.
#[test]
fn every_arm_that_offers_a_value_has_an_executed_case() {
    let offering = offering_arms();
    let mut executed: std::collections::BTreeSet<(String, bool)> = CASES
        .iter()
        .filter(|c| c.offer == Valued)
        .map(|c| (c.key.to_owned(), c.positive))
        .collect();
    executed.extend(
        crate::test_support::OFFERS_EXECUTED_IN_SWEEP
            .iter()
            .map(|&(key, positive, _)| (key.to_owned(), positive)),
    );
    let missing: Vec<_> = offering.difference(&executed).collect();
    let stale: Vec<_> = executed.difference(&offering).collect();
    assert!(
        missing.is_empty() && stale.is_empty(),
        "offering arms with no executed case: {missing:?}\ncases standing for no offering arm: \
         {stale:?}"
    );
}
