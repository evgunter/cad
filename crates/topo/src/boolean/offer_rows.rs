//! **Every tolerance a Boolean refusal offers, executed** (D4 ¶1 (i)).
//!
//! A refusal that ends "if this size is intended, tighten the tolerance
//! below v m" claims that the same operation passes at a smaller
//! tolerance. Each case here is a raise at a band-decided margin: its
//! child row raises it at whatever tolerance its process runs at, and
//! [`test_utils::offer::execute`] re-runs it just below the value the
//! refusal offered: true of its decision where that decision no longer
//! refuses there and the operation passes further down, every refusal
//! met on the way telling its own true story or logged under the row
//! that owns it (the module docs there state the rule). Each case also
//! states the margin its geometry gives, and the value it quotes and
//! offers is checked against it.
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
//! hand a declared-`Tangent` question its read directly (`tangent_side`,
//! `tangent_side_of`) say so, on the plane x cylinder pair that door
//! admits.
//!
//! The coincfr4 review's poses are adopted here (its wedges W1–W6, its
//! turned corner cc2, its bent prism, its off-axis seam) and in `sweep`'s
//! `offer_rows` (its dome on a tube, its off-axis spheres, its balls by a
//! slab, its brick below a tube): a harness that passes only on its own
//! poses was that review's finding. The census below is keyed per
//! decision, and [`every_site_names_the_decision_it_raises`] keys the
//! sites.
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
use core::f64::consts::FRAC_1_SQRT_2;
use geom_brep::OutwardNormal;
use geom_core::{Band, Indeterminate, MarginDiag, Point3, Tol, Vec3};
use test_utils::offer::{DESIGN_EPS, Executed, Outcome, Verdict, execute, report, run};

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
/// (`|m|/K`, the offer the arm would make were it valued).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Because {
    /// It refuses on this decision: F1 where it is the case's own, F2
    /// where it is a defect, F3 where it offers no tolerance and nothing
    /// smaller passes.
    Refuses(&'static str),
    /// It passes on this pose: the offer is withdrawn for a pose where it
    /// would be false, which the case's comment names.
    Passes,
}

/// One raise.
struct Case {
    /// The child row's name.
    name: &'static str,
    /// The decision its first raise refuses on
    /// (`crate::test_support::offer_key`).
    key: &'static str,
    /// The margin its first raise quotes, computed from its geometry
    /// (signed): the offer a valued case makes is this margin's `|m|/K`,
    /// and a withdrawn case executes that value.
    margin: f64,
    door: Door,
    offer: Offer,
}

impl Case {
    /// The side of zero its refused margin lies on.
    fn positive(&self) -> bool {
        self.margin > 0.0
    }
}

/// Declares the cases: one `#[ignore]`d child row each, which raises
/// its refusal and reports it, and the table the parents read.
macro_rules! cases {
    ($($name:ident: $key:literal, $margin:expr, $door:expr, $offer:expr => $raise:expr;)*) => {
        $(
            #[test]
            #[ignore = "a child row: every_offered_tolerance_passes_just_below_it runs it"]
            fn $name() {
                report(stringify!($name), &outcome($raise));
            }
        )*
        fn cases() -> Vec<Case> {
            vec![$(Case {
                name: stringify!($name),
                key: $key,
                margin: $margin,
                door: $door,
                offer: $offer,
            }),*]
        }
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
const WALL_ROOT_SITE: Door = Door::Site(
    "the wall's axis-parallel rung is read inside the edge sweep's wall crossing, on a line and \
     a run set directly: the prism fixtures' edges run across a wall's axis or exactly along it, \
     never off it by a drift in band",
);

use Door::Public;
use Offer::{Valued, Withdrawn};

/// `x` degrees' sine.
fn sin_deg(x: f64) -> f64 {
    x.to_radians().sin()
}

cases! {
    // The reviewer's C1 table (coincfr3), adopted.
    lever_arm_sector_side_in_band: "LeverArm(SectorSide)", D * FRAC_1_SQRT_2, SECTOR_SITE,
        Valued => sector_side(Vec3::new(1.0, 0.0, 1.0), Reach::Extent(D));
    lever_arm_sector_side_in_the_zero_band: "LeverArm(SectorSide)", Z * FRAC_1_SQRT_2,
        SECTOR_SITE, Valued => sector_side(Vec3::new(1.0, 0.0, 1.0), Reach::Extent(Z));
    // A bound in the face's plane: its departure is exactly zero, so the
    // arm binds and its length is quoted (the coincfr4 review's in-plane
    // pose, whose offer the previous head withdrew).
    lever_arm_sector_side_in_the_plane: "LeverArm(SectorSide)", D, SECTOR_SITE, Valued =>
        sector_side(Vec3::new(1.0, 0.0, 0.0), Reach::Extent(D));
    sector_side_above: "Coincidence(SectorSide)", D, SECTOR_SITE, Valued =>
        sector_side(Vec3::new(1.0, 0.0, D), Reach::Extent(1.0));
    sector_side_below: "Coincidence(SectorSide)", -D, SECTOR_SITE, Valued =>
        sector_side(Vec3::new(1.0, 0.0, -D), Reach::Extent(1.0));
    pierce_curvature_short_of_its_bend: "PierceCurvature", D, SECTOR_SITE, Valued =>
        pierce_curvature(D);
    tangent_side_in_band: "Coincidence(TangentSide)", -D, TANGENT_SITE, Valued =>
        tangent_side(false);
    lever_arm_sector_curving_in_band: "LeverArm(SectorCurving)", D, TANGENT_SITE, Valued =>
        tangent_side(true);
    line_clear_of_a_wall: "Coincidence(EdgeOnCurvedFace)", D, CURVED_ARM_SITE, Valued =>
        line_against_a_wall(1.0 + D, 0.0);
    line_inside_a_wall: "Coincidence(EdgeOnCurvedFace)", -D, CURVED_ARM_SITE, Valued =>
        line_against_a_wall(1.0 - D, 0.0);
    endpoint_clear_of_a_wall: "Coincidence(VertexOnCurvedFace)", D, CURVED_ARM_SITE, Valued =>
        line_against_a_wall(1.0, (2.0 * D).sqrt());
    edge_drifting_off_a_walls_axis: "WallRoots(AxisParallel)", D, WALL_ROOT_SITE, Valued =>
        line_drifting_off_a_walls_axis(D);
    endpoint_inside_a_wall: "Coincidence(VertexOnCurvedFace)", -D, CURVED_ARM_SITE, Valued =>
        line_run([(1.0 - D, 0.0), (2.0, 0.0), (2.0, 2.0), (1.0 - D, 2.0)]);
    // Declared `Rest` through the door, the walls are one carrier and the
    // arc's ends are read; a smaller tolerance decides the radii apart.
    arc_ends_clear_of_a_covered_wall: "Coincidence(VertexOnCoveredFace)", D, CURVED_ARM_SITE,
        Withdrawn(Because::Refuses("ContactContradicted")) =>
        arc_against_a_wall(1.0 + D, Some(ContactClass::Rest));
    thin_wedge_on_a_block: "Coincidence(Sectors)", D * sin_deg(5.0), Public, Valued =>
        wedge_on_a_block(super::super::BooleanOp::Union, 5.0, D);
    thin_wedge_cut_from_a_block: "Coincidence(Sectors)", D * sin_deg(5.0), Public, Valued =>
        wedge_on_a_block(super::super::BooleanOp::Subtract, 5.0, D);
    thin_wedge_meeting_a_block: "Coincidence(Sectors)", D * sin_deg(5.0), Public, Valued =>
        wedge_on_a_block(super::super::BooleanOp::Intersect, 5.0, D);
    // Tilted past the band at the arm, so the normals decide it off, while
    // its bounds still read On: the bounds' margin decides it.
    thinner_wedge_tilted_past_the_band: "Coincidence(Sectors)", 1.5e-8 * sin_deg(2.0), Public,
        Valued => wedge_on_a_block(super::super::BooleanOp::Union, 2.0, 1.5e-8);
    // The coincfr4 review's wedges (W1–W4), on its own poses: turned,
    // scaled, hanging from the bottom face, and tilted into the block,
    // each through the three public ops.
    wedge_turned_on_a_block_union: "Coincidence(Sectors)", 2.0 * D * sin_deg(3.0), Public,
        Valued => turned_wedge(0, true, 37.0, 3.0, 2.0, D);
    wedge_turned_on_a_block_subtract: "Coincidence(Sectors)", 2.0 * D * sin_deg(3.0), Public,
        Valued => turned_wedge(1, true, 37.0, 3.0, 2.0, D);
    wedge_turned_on_a_block_intersect: "Coincidence(Sectors)", 2.0 * D * sin_deg(3.0), Public,
        Valued => turned_wedge(2, true, 37.0, 3.0, 2.0, D);
    small_wedge_on_a_block_union: "Coincidence(Sectors)", 0.5 * 5e-9 * sin_deg(8.0), Public,
        Valued => turned_wedge(0, true, 120.0, 8.0, 0.5, 5e-9);
    small_wedge_on_a_block_subtract: "Coincidence(Sectors)", 0.5 * 5e-9 * sin_deg(8.0), Public,
        Valued => turned_wedge(1, true, 120.0, 8.0, 0.5, 5e-9);
    small_wedge_on_a_block_intersect: "Coincidence(Sectors)", 0.5 * 5e-9 * sin_deg(8.0), Public,
        Valued => turned_wedge(2, true, 120.0, 8.0, 0.5, 5e-9);
    wedge_under_a_block_union: "Coincidence(Sectors)", D * sin_deg(5.0), Public, Valued =>
        turned_wedge(0, false, 0.0, 5.0, 1.0, D);
    wedge_under_a_block_subtract: "Coincidence(Sectors)", D * sin_deg(5.0), Public, Valued =>
        turned_wedge(1, false, 0.0, 5.0, 1.0, D);
    wedge_under_a_block_intersect: "Coincidence(Sectors)", D * sin_deg(5.0), Public, Valued =>
        turned_wedge(2, false, 0.0, 5.0, 1.0, D);
    // Its re-run meets the crossing insertion, whose own offer is true.
    wedge_tilted_into_a_block_union: "Coincidence(Sectors)", D * sin_deg(5.0), Public, Valued =>
        turned_wedge(0, true, 200.0, 5.0, 1.0, -D);
    wedge_tilted_into_a_block_subtract: "Coincidence(Sectors)", D * sin_deg(5.0), Public,
        Valued => turned_wedge(1, true, 200.0, 5.0, 1.0, -D);
    wedge_tilted_into_a_block_intersect: "Coincidence(Sectors)", D * sin_deg(5.0), Public,
        Valued => turned_wedge(2, true, 200.0, 5.0, 1.0, -D);
    // The arms this pass withdrew, each on the raise that showed its
    // offer false.
    // The tilt, `D/4` a metre, levered at the declared pair's extent:
    // the ball around both faces' 3 m × 3 m footprint, `1.5·√2`.
    tangent_screen_of_a_tilted_block: "Coincidence(Planes)",
        D / 4.0 * 1.5 * core::f64::consts::SQRT_2, Public,
        Withdrawn(Because::Refuses("UnsupportedDeclarationClass")) =>
        tilted_block_declared_tangent();
    membership_along_a_curved_flank: "Coincidence(CurvedFlankSense)", D, Door::Site(
        "the membership tie is read inside the edge-edge resolution, on hand-built sectors: \
         an in-band sense needs a flanker arm inside the band, which the corner's arm rung \
         refuses first on a public raise",
    ), Withdrawn(Because::Refuses("CurvedBooleanUnsupported")) => curved_flank_membership(D);
    pierce_germ_line_in_band: "Coincidence(Sectors)", D, Door::Site(
        "the germ line reads in band at a transition sector whose vertices stand off their own \
         face by up to the zero band, which a valid body allows and no public raise here builds",
    ), Valued => pierce_germ_line();
    // A vertex's side of a plane face: withdrawn, since the sweep refuses
    // at the first vertex it reads in band. True on these poses alone.
    vertex_hovering_over_a_face: "Coincidence(VertexOnFace)", D, Public,
        Withdrawn(Because::Passes) => block_on_a_block(1.0 + D);
    vertex_sunk_into_a_face: "Coincidence(VertexOnFace)", -D, Public,
        Withdrawn(Because::Passes) => block_on_a_block(1.0 - D);
    wedge_with_a_far_vertex_in_band_union: "Coincidence(VertexOnFace)",
        1.5 * 3e-9 * sin_deg(20.0), Public, Withdrawn(Because::Passes) =>
        turned_wedge(0, true, 75.0, 20.0, 1.5, 3e-9);
    wedge_with_a_far_vertex_in_band_subtract: "Coincidence(VertexOnFace)",
        1.5 * 3e-9 * sin_deg(20.0), Public, Withdrawn(Because::Passes) =>
        turned_wedge(1, true, 75.0, 20.0, 1.5, 3e-9);
    // The coincfr4 review's cc2: the first margin met is not the binding
    // one, and another vertex refuses below its value.
    corner_turned_on_a_corner: "Coincidence(VertexOnFace)", -3e-9 * 2.0 / CC2_AXIS_NORM,
        Public, Withdrawn(Because::Refuses("Coincidence(VertexOnFace)")) =>
        corner_on_a_corner(Vec3::new(-2.0, 1.0, 0.5), 3e-9);
    // The coincfr4 review's W6: below the vertex's value it builds. Its
    // shell's witnesses read in band there until the ray's parallel test
    // against a face's plane was levered by the selection's reach.
    wide_wedge_with_a_far_vertex_union: "Coincidence(VertexOnFace)", 1.1e-8 * sin_deg(30.0),
        Public, Withdrawn(Because::Passes) =>
        turned_wedge(0, true, 10.0, 30.0, 1.0, 1.1e-8);
    wide_wedge_with_a_far_vertex_intersect: "Coincidence(VertexOnFace)",
        1.1e-8 * sin_deg(30.0), Public, Withdrawn(Because::Passes) =>
        turned_wedge(2, true, 10.0, 30.0, 1.0, 1.1e-8);
    // The rest of the census, one per arm and side.
    edge_nearly_along_an_edge: "Coincidence(EdgeOnEdge)", D, CORNER_SITE, Valued =>
        super::super::recl::parallel_same_dir(
            Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, D, 0.0), 1.0, band()
        ).map(|_| ());
    direction_along_a_short_arm: "DirectionSense", D, CORNER_SITE, Valued =>
        super::super::sectors::direction_sense(
            Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), D, band()
        ).map(|_| ());
    direction_against_a_short_arm: "DirectionSense", -D, CORNER_SITE, Valued =>
        super::super::sectors::direction_sense(
            Vec3::new(1.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), D, band()
        ).map(|_| ());
    direction_just_outside_a_sector: "Coincidence(Sectors)", -D, CORNER_SITE, Valued =>
        within_a_quarter(Vec3::new(1.0, -D, 0.0));
    tangent_side_curving_away: "Coincidence(TangentSide)", D, TANGENT_SITE, Valued =>
        tangent_side_of(1.0);
    corner_with_a_short_arm: "Corner(Arm)", D, CORNER_SITE, Valued =>
        corner(Vec3::new(D, 0.0, 0.0), at_degrees(90.0, 1.0), false);
    corner_folding_back_at_the_band: "Corner(Straight { full_circle: false })",
        -ARM * FRAC_1_SQRT_2, CORNER_SITE, Valued =>
        corner(Vec3::new(ARM, 0.0, 0.0), at_degrees(135.0, ARM), false);
    circle_corner_folding_forward: "Corner(Straight { full_circle: true })",
        ARM * FRAC_1_SQRT_2, CORNER_SITE, Valued =>
        corner(Vec3::new(ARM, 0.0, 0.0), at_degrees(45.0, ARM), true);
    circle_corner_folding_back: "Corner(Straight { full_circle: true })",
        -ARM * FRAC_1_SQRT_2, CORNER_SITE, Valued =>
        corner(Vec3::new(ARM, 0.0, 0.0), at_degrees(135.0, ARM), true);
    torus_with_a_thin_tube: "Torus(Tube)", D, NORMAL_SITE, Valued =>
        pierced_torus(1.0, D);
    horn_torus_at_the_band: "Torus(Ring)", D, NORMAL_SITE, Valued =>
        pierced_torus(1.0, 1.0 - D);
    // No door reaches this guard today: the join's dispatch hands
    // `cs_pair_frame` `CoaxialEvidence::None` for every cylinder × sphere
    // pair (`join::germ_frame`), so the declared-coaxial arm this raise
    // takes runs only here. It stays because that arm is the frame the
    // coaxial declaration is read into once a door passes one
    // (`boolean-coincidence-route-still-holds-join-and-self-check-decisions`,
    // the `Radius` fork), and its offer is executed against the guard
    // it would meet then.
    coaxial_thin_cylinder: "Radius(Cylinder)", D, FRAME_SITE, Valued =>
        coaxial_frame(D, 1.0);
    // The cylinder's guard asks first, so a sphere radius in band beside
    // a clear cylinder radius is a pair that does not meet: decided, it
    // is no section, which a real germ pair never is (the frame's defect).
    coaxial_tiny_sphere: "Radius(Sphere)", D, FRAME_SITE,
        Withdrawn(Because::Refuses("JoinDesync")) =>
        coaxial_frame(0.5, D);
    arc_root_just_inside_its_span: "Crossing(OnEdge)", D, ROOT_SITE, Valued =>
        circle_roots(1.0 - D);
    arc_root_just_outside_its_span: "Crossing(OnEdge)", -D, ROOT_SITE, Valued =>
        circle_roots(1.0 + D);
    vertex_near_a_vertex_of_a_curved_face: "VertexOnVertex", D, CURVED_ARM_SITE, Valued =>
        vertex_near_a_sheet_corner();
    germs_nearly_facing: "Coincidence(Join)", D, JOIN_SITE, Valued => germ_facing(D);
    germs_nearly_turned_away: "Coincidence(Join)", -D, JOIN_SITE, Valued => germ_facing(-D);
    // Overlapping plane flanks lie on one plane, which the door then asks
    // to be one face.
    flanks_along_a_short_arm: "Coincidence(FlankSense)", D, FLANK_SITE,
        Withdrawn(Because::Refuses("UndeclaredCoincidence")) =>
        planar_flank_membership(false, false);
    // Declared `Rest`, the one face the door verified.
    flanks_along_a_short_arm_declared_rest: "Coincidence(FlankSense)", D, FLANK_SITE, Valued =>
        planar_flank_membership(false, true);
    // Coincident planes facing opposite ways, read at a short arm: the
    // offset rung asks next, and refuses an undeclared pair.
    planes_facing_at_a_short_arm: "PlaneOrientation", -D, FLANK_SITE,
        Withdrawn(Because::Refuses("UndeclaredCoincidence")) => shared_side_plane(D);
    flanks_against_a_short_arm: "Coincidence(FlankSense)", -D, FLANK_SITE, Valued =>
        planar_flank_membership(true, false);
    neighbours_bent_at_the_band: "Neighbours(Parallel)", D, GATE_SITE, Valued =>
        bent_neighbours(D);
    // The coincfr4 review's bent prism, through a public union: its
    // re-run meets the containment (CONTACT's row).
    neighbours_bent_in_a_union: "Neighbours(Parallel)", D, Public, Valued =>
        bent_neighbours_in_a_union(D);
    // Valid bodies only (the coincv5 review's NF-2 poses): on one, the
    // offset the gate reads is the bend `Neighbours(Parallel)` reads
    // first, so it is offered from the zero band. A prism whose wall
    // turns by a zero-band angle at a short edge, beside a far brick and
    // crossed by one through each public op; and a split top bent about
    // its diagonal, its plane's origin far along the plane.
    neighbours_kinked_beside_a_far_brick: "CoplanarNeighbours", -KINK * KINK_HEIGHT, Public,
        Valued => kinked_prism(None, KINK);
    neighbours_kinked_crossed_union: "CoplanarNeighbours", -KINK * KINK_HEIGHT, Public,
        Valued => kinked_prism(Some(0), KINK);
    neighbours_kinked_crossed_subtract: "CoplanarNeighbours", -KINK * KINK_HEIGHT, Public,
        Valued => kinked_prism(Some(1), KINK);
    neighbours_kinked_crossed_intersect: "CoplanarNeighbours", -KINK * KINK_HEIGHT, Public,
        Valued => kinked_prism(Some(2), KINK);
    neighbours_kinked_twice_as_far_beside_a_far_brick: "CoplanarNeighbours",
        -2.0 * KINK * KINK_HEIGHT, Public, Valued => kinked_prism(None, 2.0 * KINK);
    neighbours_kinked_twice_as_far_crossed_union: "CoplanarNeighbours",
        -2.0 * KINK * KINK_HEIGHT, Public, Valued => kinked_prism(Some(0), 2.0 * KINK);
    neighbours_bent_far_origin_beside_a_far_brick: "CoplanarNeighbours",
        KINK * FRAC_1_SQRT_2, Public, Valued => bent_split_far_origin(None, 10.0);
    // Its re-run meets a corner's side of a face, whose own offer is true.
    neighbours_bent_far_origin_crossed_union: "CoplanarNeighbours", KINK * FRAC_1_SQRT_2,
        Public, Valued => bent_split_far_origin(Some(0), 10.0);
    neighbours_bent_far_origin_crossed_subtract: "CoplanarNeighbours", KINK * FRAC_1_SQRT_2,
        Public, Valued => bent_split_far_origin(Some(1), -10.0);
    rim_just_above_a_face: "Coincidence(EdgeOnPlane)", D, RIM_PLANE_SITE, Valued =>
        rim_over_a_brick(1.0 - D);
    rim_just_below_a_face: "Coincidence(EdgeOnPlane)", -D, RIM_PLANE_SITE, Valued =>
        rim_over_a_brick(1.0 + D);
    roots_a_hair_apart: "Crossing(Order)", -D, ROOT_SITE, Valued => ellipse_roots(false);
    roots_a_hair_apart_across_the_window: "Crossing(Order)", D, ROOT_SITE, Valued =>
        ellipse_roots(true);
    // At an angle other than a right one, the wedge the arm meters is
    // `sin θ · arm`, and that is the margin quoted.
    seam_over_a_short_edge: "LeverArm(Seam)", D * 1.0_f64.sin(), SEAM_SITE, Valued =>
        seam(1.0, D, Vec3::new(0.0, 0.0, 1.0));
    seam_over_an_edge_in_the_zero_band: "LeverArm(Seam)", Z * 0.5_f64.sin(), SEAM_SITE,
        Valued => seam(0.5, Z, Vec3::new(0.0, 0.0, 1.0));
    // The coincfr4 review's off-axis seam, whose arm offer met the wedge.
    seam_over_a_short_edge_off_axis: "LeverArm(Seam)", 7e-9 * 1.0_f64.sin(), SEAM_SITE,
        Valued => seam(1.0, 7e-9, Vec3::new(0.3, 1.0, 0.2));
    // A wedge that reads zero at the arm's own tolerance leaves the arm
    // binding (the coincv5 review's NF-1 poses): a tangent cylinder and
    // plane, whose `sin θ` is rounding, and planes at 3e-3, 1e-7 and
    // 1e-9 rad, each over an arm of 7e-9.
    seam_tangent_over_a_short_edge: "LeverArm(Seam)", 7e-9, SEAM_SITE, Valued =>
        seam_tangent(7e-9);
    seam_with_a_wedge_in_the_zero_band: "LeverArm(Seam)", 7e-9, SEAM_SITE, Valued =>
        seam(3e-3, 7e-9, Vec3::new(0.3, 1.0, 0.2));
    seam_nearly_flat_over_a_short_edge: "LeverArm(Seam)", 7e-9, SEAM_SITE, Valued =>
        seam(1e-7, 7e-9, Vec3::new(0.3, 1.0, 0.2));
    seam_flatter_still_over_a_short_edge: "LeverArm(Seam)", 7e-9, SEAM_SITE, Valued =>
        seam(1e-9, 7e-9, Vec3::new(0.3, 1.0, 0.2));
    seam_barely_creased: "SeamWedge", D, SEAM_SITE, Valued =>
        seam(D, 1.0, Vec3::new(0.0, 0.0, 1.0));
    seam_barely_bending_apart: "SeamJet", D, SEAM_SITE, Valued => seam_bend(D);
    sphere_barely_leaning: "Sphere(RecutAlign)", D, Door::Site(
        "a re-cut sphere's lean is read on a crossing-free escape, where an axis near the escape \
         normal carries a seam across the escape plane that the crossing layer meets first: no \
         public raise is known to reach it in band"
    ), Valued => recut(D);
}

/// The norm of the coincfr4 review's cc2 turning axis, `(−2, 1, ½)`.
const CC2_AXIS_NORM: f64 = 2.291_287_847_477_92;

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
const TANGENT_SITE: Door = Door::Site(
    "a declared-Tangent pair's second-order side is read inside the vertex neighbourhood walk; \
     the case hands it the read its door gives a cylinder resting on a floor along a ruling, \
     the plane x cylinder pair the door admits, and the arm and the direction are set directly",
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
    side_code(dir, reach, n, NO_CURVATURE(), band()).map(|_| ())
}

/// A bisector of reach ½ leaving a face of bend radius 1 at the slope
/// `s` whose peak separation from the face, `s²·R/4`, is `margin`.
fn pierce_curvature(margin: f64) -> Result<(), BooleanError> {
    let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
    let c = 2.0 * margin.sqrt();
    let dir = Vec3::new((1.0 - c * c).sqrt(), 0.0, c);
    side_code(dir, Reach::Bisector(0.5), n, 1.0, band()).map(|_| ())
}

/// A unit cylinder along `y` resting on a floor from the side `side`
/// (`+1` above), the plane x cylinder pair a declared-`Tangent` door
/// admits (along a ruling).
fn resting_cylinder(side: f64) -> geom::Surface<f64> {
    geom::Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, side),
        axis: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// A cylinder under a floor, handed the read a declared-`Tangent` door
/// gives: the second-order side across its ruling at the arm whose
/// sagitta is `D`, or the arm gate at an arm of `D`.
fn tangent_side(arm_gate: bool) -> Result<(), BooleanError> {
    let (p, d) = (Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
    let ball = resting_cylinder(-1.0);
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
    let read = DeclarationRead::Spent(BooleanCoincidence::TANGENT);
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

/// A line from inside the unit wall about `z`, off the axis direction
/// by `drift` over the unit run its edge covers.
fn line_drifting_off_a_walls_axis(drift: f64) -> Result<(), BooleanError> {
    let z = Vec3::new(0.0, 0.0, 1.0);
    super::super::solid_contain::line_wall_roots(
        Point3::new(0.5, 0.0, 0.0),
        Vec3::new(drift, 0.0, 1.0),
        Point3::new(0.0, 0.0, 0.0),
        z,
        1.0,
        1.0,
        band(),
    )
    .map(|_| ())
    .map_err(|fault| BooleanError::Escalated {
        decision: BooleanDecision::WallRoots(fault.rung),
        diag: fault.diag,
    })
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
    let declared =
        DeclaredPairs::<f64>::without_struts(&BooleanDeclarations::none(), Default::default());
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
    let declared = DeclaredPairs::build(&decls, one, &x, &y, band())?;
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
    let (block, wedge) = lane_wedge(opening_deg, tilt);
    let tol = Tol::witness();
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

/// The block and the wedge of [`wedge_on_a_block`].
fn lane_wedge(opening_deg: f64, tilt: f64) -> (crate::body::Body<f64>, crate::body::Body<f64>) {
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
    (brick((-1.0, 3.0), (-2.0, 2.5), (0.0, 1.0), tol), wedge)
}

/// A block tilted by an in-band angle on a block's top face, the pair
/// declared `Tangent` (the review's G2 pose).
fn tilted_block_declared_tangent() -> Result<(), BooleanError> {
    block_declared_tangent_at(D / 2.0)
}

/// The same block, its top rising by `tilt` over each metre along `x`.
fn block_declared_tangent_at(tilt: f64) -> Result<(), BooleanError> {
    use crate::test_support_fixtures::{brick, mapped_cube};
    let tol = Tol::witness();
    let a = brick((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), tol);
    let b = mapped_cube::<f64>(
        move |u, v, w| Point3::new(1.0 + 2.0 * u, 1.0 + 2.0 * v, 1.0 + w + tilt * u),
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
/// flankers' arm `arm`: the membership tie on a curved flank (the
/// review's probe, undeclared).
fn curved_flank_membership(arm: f64) -> Result<(), BooleanError> {
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
        arm,
    };
    let records = [PairRecord {
        a: 0,
        b: 0,
        sa: (super::super::SideCode::On, super::super::SideCode::Out),
        sb: (super::super::SideCode::Out, super::super::SideCode::In),
        intersect: true,
    }];
    let corner = |face| [sector(z, x, face), sector(x, z, face)];
    let declared =
        DeclaredPairs::<f64>::without_struts(&BooleanDeclarations::none(), Default::default());
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

/// A declared-`Tangent` cylinder resting on a floor along a ruling from
/// the side `side` (`+1` above), at the arm whose sagitta is `D`.
fn tangent_side_of(side: f64) -> Result<(), BooleanError> {
    let (p, d) = (Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
    let ball = resting_cylinder(side);
    let floor = crate::test_support_fixtures::plane(
        &[p, Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
        Tol::witness(),
    );
    let accel = |s: &geom::Surface<f64>| {
        -geom_brep::implicit_hessian_form(s, p, d) / geom_brep::implicit_gradient(s, p).dot(n.vec())
    };
    let arm = (2.0 * D / (accel(&ball) - accel(&floor)).abs()).sqrt();
    let read = DeclarationRead::Spent(BooleanCoincidence::TANGENT);
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

/// A cylinder of radius 1 and the plane tangent to it along a ruling,
/// the pair turned generically so the two normals agree only to rounding,
/// read as a seam of the result at a point of the ruling over `extent`
/// (the coincv5 review's `seam_tangent_noise`).
fn seam_tangent(extent: f64) -> Result<(), BooleanError> {
    let k = Vec3::new(0.37, -0.81, 0.45).normalize();
    let a = 0.913_f64;
    let r = |v: Vec3<f64>| v * a.cos() + k.cross(v) * a.sin() + k * (k.dot(v) * (1.0 - a.cos()));
    let c0 = Point3::new(0.4, -1.3, 2.2);
    let cylinder = geom::Surface::Cylinder {
        origin: c0 + r(Vec3::new(0.0, 1.0, 0.0)),
        axis: r(Vec3::new(0.0, 0.0, 1.0)),
        radius: 1.0,
        u_ref: r(Vec3::new(1.0, 0.0, 0.0)),
    };
    let plane = plane_through(
        c0,
        r(Vec3::new(1.0, 0.0, 0.0)),
        r(Vec3::new(0.0, -1.0, 0.0)),
    );
    let p = c0 + r(Vec3::new(0.0, 0.0, 0.3));
    super::super::ops::seam_class(&cylinder, &plane, p, extent, band()).map(|_| ())
}

/// Two planes through a line along `axis` at `angle` to one another,
/// read as a seam of the result over `extent` (the coincfr4 review's
/// `seam_site`, off the origin).
fn seam(angle: f64, extent: f64, axis: Vec3<f64>) -> Result<(), BooleanError> {
    let o = Point3::new(0.2, -0.3, 0.7);
    let k = axis.normalize();
    let across = if k.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let n1 = k.cross(across).normalize();
    let n2 = n1 * angle.cos() + k.cross(n1) * angle.sin();
    let (s1, s2) = (plane_through(o, k, n1), plane_through(o, k, n2));
    super::super::ops::seam_class(&s1, &s2, o, extent, band()).map(|_| ())
}

/// A unit cylinder resting on the floor `z = 0` along the `y` axis,
/// read as a smooth seam of the result along that ruling over the
/// extent whose sagitta under the cylinder's bend is `sagitta`: the
/// must-carry rule's second-order reading at every station.
fn seam_bend(sagitta: f64) -> Result<(), BooleanError> {
    let o = Point3::new(0.0, 0.0, 0.0);
    let y = Vec3::new(0.0, 1.0, 0.0);
    let floor = plane_through(o, y, Vec3::new(0.0, 0.0, 1.0));
    let extent = (2.0 * sagitta).sqrt();
    let ruling = geom::Curve3::Line { origin: o, dir: y };
    super::super::ops::seam_must_carry(
        &floor,
        &resting_cylinder(1.0),
        &ruling,
        0.0,
        extent,
        extent,
        band(),
    )
    .map(|_| ())
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

/// Two germs a metre apart along `x`, the second facing the first, the
/// first leaning off the chord so its facing reads `lean` metres.
fn germ_facing(lean: f64) -> Result<(), BooleanError> {
    use super::super::HalfGerm;
    let germ = |dir| HalfGerm {
        he: crate::entity::HalfEdgeKey::default(),
        a_face: crate::entity::FaceKey::default(),
        b_face: crate::entity::FaceKey::default(),
        a_locus: super::super::Locus::InFace(crate::entity::FaceKey::default()),
        b_locus: super::super::Locus::InFace(crate::entity::FaceKey::default()),
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
    planar_flank_membership_at(against, rest, D)
}

/// [`planar_flank_membership`] at the flankers' arm `arm`.
fn planar_flank_membership_at(against: bool, rest: bool, arm: f64) -> Result<(), BooleanError> {
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
        arm,
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
    let declared = DeclaredPairs::build(&decls, one, &pa.body, &pb.body, band())?;
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
    let declared =
        DeclaredPairs::<f64>::without_struts(&BooleanDeclarations::none(), Default::default());
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

/// The same bent prism beside a far brick, through a public union (the
/// coincfr4 review's `split_bent_public`).
fn bent_neighbours_in_a_union(margin: f64) -> Result<(), BooleanError> {
    let body = super::tests::top_split_redescribed(|p0, along, diagonal| {
        let theta = margin / diagonal;
        let up = Vec3::new(0.0, 0.0, 1.0);
        plane_through(p0, along, up * theta.cos() + along.cross(up) * theta.sin())
    });
    let far = crate::test_support_fixtures::brick::<f64>(
        (5.0, 6.0),
        (0.0, 1.0),
        (0.0, 1.0),
        Tol::witness(),
    );
    super::super::union(&body, &far, Tol::witness()).map(|_| ())
}

/// The public op `k` (0 union, 1 subtract, else intersect) of `a` and
/// `b`, `decls` declared.
fn public_op(
    k: u8,
    a: &crate::body::Body<f64>,
    b: &crate::body::Body<f64>,
    decls: &BooleanDeclarations,
) -> Result<(), BooleanError> {
    let op = match k {
        0 => super::super::BooleanOp::Union,
        1 => super::super::BooleanOp::Subtract,
        _ => super::super::BooleanOp::Intersect,
    };
    super::super::boolean_op_with(
        op,
        a,
        b,
        decls,
        super::super::SweepStrategy::Realized,
        Tol::witness(),
    )
    .map(|_| ())
}

/// `v` turned by `deg` degrees about `z`.
fn turned_z(deg: f64, v: Vec3<f64>) -> Vec3<f64> {
    let (s, c) = deg.to_radians().sin_cos();
    Vec3::new(v.x * c - v.y * s, v.x * s + v.y * c, v.z)
}

/// The coincfr4 review's wedge (`wedge_body`): a wedge-cornered block of
/// opening `opening` degrees and scale `s`, standing on (`top`) or
/// hanging from the big block's face, its corner on it, its second
/// in-face edge rising off the face by `tilt · s · sin(opening)` away
/// from the block (into it where negative), the whole turned by `rot`
/// degrees about `z`.
fn review_wedge(top: bool, rot: f64, opening: f64, s: f64, tilt: f64) -> crate::body::Body<f64> {
    use crate::test_support_fixtures::mapped_cube;
    let tol = Tol::witness();
    let phi = opening.to_radians();
    let up = if top { 1.0 } else { -1.0 };
    let ea = turned_z(rot, Vec3::new(s, 0.0, 0.0));
    let eb = turned_z(rot, Vec3::new(s * phi.cos(), s * phi.sin(), 0.0))
        + Vec3::new(0.0, 0.0, up * tilt * s * phi.sin());
    let p = Point3::new(0.3, -0.2, if top { 1.0 } else { 0.0 });
    let h = Vec3::new(0.0, 0.0, up);
    if top {
        mapped_cube::<f64>(move |u, v, w| p + ea * u + eb * v + h * w, tol)
    } else {
        mapped_cube::<f64>(move |u, v, w| p + eb * u + ea * v + h * w, tol)
    }
}

/// The review's big block, `[−4, 4]² × [0, 1]`.
fn big_block() -> crate::body::Body<f64> {
    crate::test_support_fixtures::brick((-4.0, 4.0), (-4.0, 4.0), (0.0, 1.0), Tol::witness())
}

/// The review's wedge on its big block, through the public op `k`.
fn turned_wedge(
    k: u8,
    top: bool,
    rot: f64,
    opening: f64,
    s: f64,
    tilt: f64,
) -> Result<(), BooleanError> {
    let wedge = review_wedge(top, rot, opening, s, tilt);
    public_op(k, &big_block(), &wedge, &BooleanDeclarations::none())
}

/// The plane face of `body` whose outward normal is `±z` (`up`) through
/// height `z`.
fn z_face(body: &crate::body::Body<f64>, z: f64, up: bool) -> crate::entity::FaceKey {
    body.faces()
        .map(|(k, _)| k)
        .find(|&k| {
            matches!(super::super::face_carrier(body, k),
                Some(super::super::CarrierDesc::Plane { normal, origin, .. })
                    if (normal.z > 0.9) == up && normal.z.abs() > 0.9
                        && (origin.z - z).abs() < 1e-3)
        })
        .unwrap()
}

/// A wedge on the block `block` (its top at `z = 1`), the wedge's bottom
/// declared `Rest` on the block's top through the public door, by op `k`.
fn declared_rest_wedge(
    k: u8,
    block: &crate::body::Body<f64>,
    wedge: &crate::body::Body<f64>,
) -> Result<(), BooleanError> {
    let decls = BooleanDeclarations {
        coincident_faces: vec![FacePairDeclaration::new(
            z_face(block, 1.0, true),
            z_face(wedge, 1.0, false),
            ContactClass::Rest,
        )],
        ..BooleanDeclarations::none()
    };
    public_op(k, block, wedge, &decls)
}

/// A unit block whose corner sits on the corner `(1, 1, 1)` of the unit
/// block, turned about `axis` through it by `theta`: their union (the
/// coincfr4 review's `corner_on_corner`).
fn corner_on_a_corner(axis: Vec3<f64>, theta: f64) -> Result<(), BooleanError> {
    use crate::test_support_fixtures::{brick, mapped_cube};
    let tol = Tol::witness();
    let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let k = axis.normalize();
    let c = Point3::new(1.0, 1.0, 1.0);
    let turn = move |v: Vec3<f64>| {
        v * theta.cos() + k.cross(v) * theta.sin() + k * (k.dot(v) * (1.0 - theta.cos()))
    };
    let b = mapped_cube::<f64>(move |u, v, w| c + turn(Vec3::new(u, v, w)), tol);
    public_op(0, &a, &b, &BooleanDeclarations::none())
}

/// The zero-band angle the neighbour cases turn by.
const KINK: f64 = 5.5e-10;

/// The height of [`kinked_prism`], the length of its kinked edge.
const KINK_HEIGHT: f64 = 0.1;

/// The public op `k` of `body` and the brick `cross`, or, where `k` is
/// `None`, the union of `body` and a far brick.
fn beside_or_crossed(
    body: &crate::body::Body<f64>,
    k: Option<u8>,
    cross: &crate::body::Body<f64>,
) -> Result<(), BooleanError> {
    match k {
        None => {
            let far = crate::test_support_fixtures::brick::<f64>(
                (20.0, 21.0),
                (0.0, 1.0),
                (0.0, 1.0),
                Tol::witness(),
            );
            public_op(0, body, &far, &BooleanDeclarations::none())
        }
        Some(k) => public_op(k, body, cross, &BooleanDeclarations::none()),
    }
}

/// A valid prism of height [`KINK_HEIGHT`] whose profile turns by `theta`
/// at `(0.1, 0)`: the wall along `y = 0` and the one tilted by `theta` out
/// to `x = 10.1` share the vertical edge there. Beside a far brick (`k`
/// `None`) or crossed on its long wall by the public op `k` (the coincv5
/// review's `kinked_prism`).
fn kinked_prism(k: Option<u8>, theta: f64) -> Result<(), BooleanError> {
    let tol = Tol::witness();
    let h = KINK_HEIGHT;
    let profile = [
        (0.0, 0.0),
        (0.1, 0.0),
        (10.1, 10.0 * theta.tan()),
        (10.1, 1.0),
        (0.0, 1.0),
    ];
    let body = prism_z::<f64>(&profile, 0.0, h, tol).body;
    let cross = crate::test_support_fixtures::brick::<f64>(
        (4.0, 5.0),
        (-0.5, 0.5),
        (0.02, 0.5 * h + 0.3),
        tol,
    );
    beside_or_crossed(&body, k, &cross)
}

/// The brick that crosses a split top.
fn split_top_crossing() -> crate::body::Body<f64> {
    crate::test_support_fixtures::brick::<f64>((0.3, 2.0), (0.2, 0.7), (0.5, 1.5), Tol::witness())
}

/// A valid split top: the re-described half bent by [`KINK`] about the
/// diagonal (its edges stay on it), its plane's origin `l` from the
/// diagonal within the plane. Beside a far brick (`k` `None`) or crossed
/// by the public op `k` (the coincv5 review's `split_bent_far_origin`).
fn bent_split_far_origin(k: Option<u8>, l: f64) -> Result<(), BooleanError> {
    let body = super::tests::top_split_redescribed(|p0, along, _| {
        let up = Vec3::new(0.0, 0.0, 1.0);
        let n = up * KINK.cos() + along.cross(up) * KINK.sin();
        plane_through(p0 + n.cross(along) * l, along, n)
    });
    beside_or_crossed(&body, k, &split_top_crossing())
}

/// **A stranded split top, crossed by a brick, reaches the classification
/// invariant, at a clear offset, through each public op**: the half
/// re-described on the parallel plane `1000 ε` above leaves its own edges
/// `1000 ε` off it, which no valid body does, and the operation ends on a
/// kernel invariant rather than a typed refusal. The offset is far past
/// the band at every tolerance, so the invariant is the stranded body's,
/// not any offer's (the coincv5 review's NF-2; filed as
/// `work/hone/a-stranded-operand-reaches-the-classification-invariant.md`).
/// The `CoplanarNeighbours` offers run on valid bodies, in the cases.
#[test]
fn a_stranded_split_top_crossed_by_a_brick_reaches_the_classification_invariant() {
    let up = Vec3::new(0.0, 0.0, 1.0);
    let offset = 1e3 * Tol::witness().get().eps;
    let body = super::tests::top_split_redescribed(|p0, along, _| {
        plane_through(p0 + up * offset, along, up)
    });
    for k in 0..3 {
        let err = public_op(
            k,
            &body,
            &split_top_crossing(),
            &BooleanDeclarations::none(),
        )
        .expect_err("a stranded operand does not pass");
        assert_eq!(
            err.kind(),
            BooleanErrorKind::ClassificationInvariant,
            "op {k}: {err}"
        );
    }
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

/// The point margin a refusal quotes, read off its payload.
fn quoted_margin(text: &str) -> Option<f64> {
    let (_, tail) = text.split_once("margin ")?;
    tail.split_whitespace().next()?.parse::<f64>().ok()
}

/// Whether `got` is `want` to the precision a fixed pose's margin is
/// computed to.
fn near(got: f64, want: f64) -> bool {
    (got - want).abs() <= 1e-4 * want.abs() + 1e-15
}

/// **The value a case's first raise quotes and offers is its computed
/// margin's**: the quoted margin is the one the case computes from its
/// geometry, and a valued offer is that margin's `|m|/K` at the design
/// band (the coincfr4 review's MY1: a chain's outcome alone cannot see
/// an inflated value that still reaches a pass).
fn check_value(case: &Case, text: &str) -> Result<(), String> {
    let quoted = quoted_margin(text);
    if !quoted.is_some_and(|m| near(m, case.margin)) {
        return Err(format!(
            "{}: quotes margin {quoted:?}, computed {:e}: {text}",
            case.name, case.margin
        ));
    }
    let offered = test_utils::offer::offered_below(text);
    let want = (case.offer == Valued).then(|| design_band().tolerance_deciding(case.margin));
    match (offered, want) {
        (Some(v), Some(w)) if near(v, w) => Ok(()),
        (None, None) => Ok(()),
        _ => Err(format!(
            "{}: offers {offered:?}, its margin gives {want:?}: {text}",
            case.name
        )),
    }
}

/// The rows a chain's later refusals are logged under, and whether each
/// names a file in this repository.
fn logged_rows(laters: &[test_utils::offer::Later]) -> Result<Vec<String>, String> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let logged = test_utils::offer::judge_laters(
        laters,
        geom_core::COINCIDENCE_RECOURSE,
        crate::test_support::LATER_STORIES_OWNED,
    )?;
    logged
        .into_iter()
        .map(|(key, row)| {
            if root.join(row).is_file() {
                Ok(format!("{key} logs {row}"))
            } else {
                Err(format!("{key}: its owning row {row} is not a file"))
            }
        })
        .collect()
}

/// Runs `f` over `items` on a few threads at a time: each child is a
/// process, and one thread per case spawns every one at once.
fn on_a_few_threads<I: Sync, O: Send>(items: &[I], f: impl Fn(&I) -> O + Sync) -> Vec<O> {
    let next = std::sync::atomic::AtomicUsize::new(0);
    let done = std::sync::Mutex::new(Vec::new());
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| {
                loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let Some(item) = items.get(i) else { break };
                    let out = f(item);
                    done.lock().unwrap().push((i, out));
                }
            });
        }
    });
    let mut done = done.into_inner().unwrap();
    done.sort_by_key(|(i, _)| *i);
    done.into_iter().map(|(_, o)| o).collect()
}

/// **Every offered tolerance is true of its decision** (the module docs
/// of `test_utils::offer`), on every case whose arm offers one: its value
/// is its computed margin's, and re-run just below it, the same raise
/// passes (T1) or refuses on a different decision (T2) past which it
/// passes, every refusal met further along telling its own true story or
/// logged under the row that owns it.
#[test]
fn every_offered_tolerance_passes_just_below_it() {
    let cases = cases();
    let valued: Vec<&Case> = cases.iter().filter(|c| c.offer == Valued).collect();
    let verdicts = on_a_few_threads(&valued, |case| {
        execute(
            &row(case),
            case.key,
            crate::test_support::offer_same_decision,
        )
    });
    let mut false_offers = Vec::new();
    for (case, verdict) in valued.iter().zip(verdicts) {
        let Executed { chain, verdict } = match verdict {
            Ok(executed) => executed,
            Err(why) => {
                false_offers.push(why);
                continue;
            }
        };
        let Outcome::Refused { text, .. } = &chain[0].outcome else {
            unreachable!("execute returns a chain that starts with its refusal");
        };
        if let Err(why) = check_value(case, text) {
            false_offers.push(why);
            continue;
        }
        let (kind, logs) = match &verdict {
            Verdict::T1 => ("T1", Vec::new()),
            Verdict::T2 { laters } => match logged_rows(laters) {
                Ok(logs) => ("T2", logs),
                Err(why) => {
                    false_offers.push(format!("{}: {why}", case.name));
                    continue;
                }
            },
        };
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
        let stories = match &verdict {
            Verdict::T1 => String::new(),
            Verdict::T2 { laters } => laters
                .iter()
                .map(|l| format!(" {}: {:?};", l.key, l.story))
                .collect(),
        };
        println!(
            "OFFER {} {kind}: {}{stories} {} [{door}]",
            case.name,
            path.join(" -> "),
            logs.join("; ")
        );
    }
    assert!(
        false_offers.is_empty(),
        "false offers:\n{}",
        false_offers.join("\n\n")
    );
}

/// The value a refusal's quoted point margin gives at the band its child
/// ran at ([`DESIGN_EPS`], the default `K`): the offer the arm would make
/// were it valued, through the renderer's own home
/// (`Band::tolerance_deciding`).
fn would_offer(case: &Case) -> f64 {
    design_band().tolerance_deciding(case.margin)
}

/// **A withdrawn tolerance stays withdrawn**: each case whose arm offers
/// none refuses on its decision at [`DESIGN_EPS`], quoting its computed
/// margin with no tolerance; re-run just below the value that margin
/// gives, it meets what its [`Because`] states.
#[test]
fn every_withdrawn_tolerance_stays_withdrawn() {
    for case in cases() {
        let Offer::Withdrawn(because) = case.offer else {
            continue;
        };
        let Outcome::Refused { key, defect, text } = run(&row(&case), DESIGN_EPS) else {
            panic!("{}: passes at the design tolerance", case.name);
        };
        assert_eq!(key, case.key, "{}: {text}", case.name);
        assert!(!defect, "{}: {text}", case.name);
        check_value(&case, &text).unwrap_or_else(|why| panic!("{why}"));
        assert!(
            !text.contains("tighten"),
            "{}: offers a tolerance: {text}",
            case.name
        );
        let eps = test_utils::offer::BELOW * would_offer(&case);
        let below = run(&row(&case), eps);
        let met = match (&below, because) {
            (Outcome::Refused { key, .. }, Because::Refuses(want)) => key == want,
            (Outcome::Pass, Because::Passes) => true,
            _ => false,
        };
        assert!(
            met,
            "{}: at {eps:e} the withdrawn offer meets {because:?}: {below:?}",
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
        | BooleanErrorKind::InsideOutOperand
        | BooleanErrorKind::NonMaximalFaces
        | BooleanErrorKind::NonFiniteSectorChord
        | BooleanErrorKind::UnderflowedSectorChord
        | BooleanErrorKind::DeclarationContradicted
        | BooleanErrorKind::ContactContradicted
        | BooleanErrorKind::ContinuationContradicted
        | BooleanErrorKind::UnsupportedDeclarationClass
        | BooleanErrorKind::SeamContradicted
        | BooleanErrorKind::RimCuspArmUnbuilt
        | BooleanErrorKind::TangentSlitArmUnbuilt
        | BooleanErrorKind::InvalidDeclaration
        | BooleanErrorKind::PairingMismatch
        | BooleanErrorKind::SharedVertexCrossings
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
        | BooleanErrorKind::ShellWitnessExhausted
        | BooleanErrorKind::CoincidentShell
        | BooleanErrorKind::SeamOrientation
        | BooleanErrorKind::ZipCorrespondence
        | BooleanErrorKind::ResultInvalid
        | BooleanErrorKind::ResultVolumeImplausible
        | BooleanErrorKind::VolumeCorrupt
        | BooleanErrorKind::VolumeUndecided
        | BooleanErrorKind::UnrepresentableResult
        | BooleanErrorKind::NonManifoldResult
        // Nest another module's refusal, whose offers are that module's
        // to execute: this census does not reach them.
        | BooleanErrorKind::CrossingInsertion
        | BooleanErrorKind::Containment
        | BooleanErrorKind::Revert
        | BooleanErrorKind::Merge
        | BooleanErrorKind::Pcurves
        | BooleanErrorKind::Euler
        | BooleanErrorKind::Join
        | BooleanErrorKind::VolumeUnmeasured
        | BooleanErrorKind::GraftRecertify
        | BooleanErrorKind::Pieces => Vec::new(),
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
    let mut executed: std::collections::BTreeSet<(String, bool)> = cases()
        .iter()
        .filter(|c| c.offer == Valued)
        .map(|c| (c.key.to_owned(), c.positive()))
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

// ------------------------------------------------------------------
// A declared `Rest` at a decided tilt (the coincfr4 review's MAJOR-1).
// ------------------------------------------------------------------

/// Declares the C2 rows: one `#[ignore]`d child each, run at
/// [`DESIGN_EPS`] by [`a_declared_rest_at_a_decided_tilt_is_contradicted`].
macro_rules! declared_rows {
    ($($name:ident => $raise:expr;)*) => {
        $(
            #[test]
            #[ignore = "a child row: a_declared_rest_at_a_decided_tilt_is_contradicted runs it"]
            fn $name() {
                report(stringify!($name), &outcome($raise));
            }
        )*
        const DECLARED_ROWS: &[&str] = &[$(stringify!($name)),*];
    };
}

declared_rows! {
    // The lane's own pose, whose normals decide its tilt at 1e-9.
    declared_rest_on_a_wedge_tilted_past_the_band_union => {
        let (block, wedge) = lane_wedge(2.0, 1.5e-8);
        declared_rest_wedge(0, &block, &wedge)
    };
    declared_rest_on_a_wedge_tilted_past_the_band_subtract => {
        let (block, wedge) = lane_wedge(2.0, 1.5e-8);
        declared_rest_wedge(1, &block, &wedge)
    };
    declared_rest_on_a_wedge_tilted_past_the_band_intersect => {
        let (block, wedge) = lane_wedge(2.0, 1.5e-8);
        declared_rest_wedge(2, &block, &wedge)
    };
    // The review's W1: the door's ladder bridges the tilt, and the
    // sector's normals decide it at the arm.
    declared_rest_on_a_turned_wedge_union => {
        declared_rest_wedge(0, &big_block(), &review_wedge(true, 37.0, 3.0, 2.0, D))
    };
    declared_rest_on_a_turned_wedge_subtract => {
        declared_rest_wedge(1, &big_block(), &review_wedge(true, 37.0, 3.0, 2.0, D))
    };
}

/// **A declared `Rest` pair whose sectors' tilt is decided is
/// contradicted, and offers neither a declaration nor a tolerance**
/// (C2, declared through `boolean_op_with`): a decided tilt is no
/// coincidence a declaration settles, so the decided-tilt arm of `vtxfac`
/// reads none and refuses the declared pair as the door does at a
/// smaller tolerance. Removed, the declaration leaves the undeclared
/// cases `thinner_wedge_tilted_past_the_band` and
/// `wedge_turned_on_a_block_*`, whose offers are executed true.
#[test]
fn a_declared_rest_at_a_decided_tilt_is_contradicted() {
    let module = module_path!()
        .split_once("::")
        .map_or(module_path!(), |(_, m)| m);
    for name in DECLARED_ROWS {
        let got = run(&format!("{module}::{name}"), DESIGN_EPS);
        let Outcome::Refused { key, defect, text } = &got else {
            panic!("{name}: a declared Rest at a decided tilt builds: {got:?}");
        };
        assert!(
            key == "ContactContradicted"
                && !defect
                && text.contains("the declared planes are not parallel")
                && !text.contains("declare the coincidence")
                && !text.contains("tighten"),
            "{name}: {got:?}"
        );
    }
    // Undeclared, the same poses offer the tolerance and no declaration.
    for name in [
        "thinner_wedge_tilted_past_the_band",
        "wedge_turned_on_a_block_union",
    ] {
        let got = run(&format!("{module}::{name}"), DESIGN_EPS);
        assert!(
            matches!(&got, Outcome::Refused { key, text, .. }
                if key == "Coincidence(Sectors)"
                    && !text.contains("declare")
                    && text.contains("tighten")),
            "{name}: {got:?}"
        );
    }
}

// ------------------------------------------------------------------
// The site census (the coincfr4 review's MY5).
// ------------------------------------------------------------------

/// **Every decision a Boolean site names, keyed per site**: each mention
/// of a `Coincide`, `BooleanDecision`, `LeverArm`, `SphereQuestion` or
/// `SelfCheck` variant in the Boolean's production code, by file and
/// enclosing function, with its count. The executed census above is keyed
/// per decision, so a site that raises one decision under another
/// decision's key (MY5: the declared-`Tangent` witness's `TangentLocus`
/// raised as `Rim`) keeps every case green; here it moves a row. A new
/// or moved site is a row to add here, and the per-site table of the PR
/// that adds it.
#[test]
fn every_site_names_the_decision_it_raises() {
    use test_utils::source::{code_only, rust_sources};
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/boolean");
    let mut got: std::collections::BTreeMap<(String, String, String), usize> = Default::default();
    for path in rust_sources(&dir) {
        let file = path
            .strip_prefix(&dir)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        if file == "refusal_routes.rs" || file == "offer_rows.rs" {
            continue;
        }
        let source = without_test_modules(&code_only(&std::fs::read_to_string(&path).unwrap()));
        let mut function = String::from("-");
        for line in source.lines() {
            if let Some(name) = top_level_fn(line) {
                function = name;
            }
            for (at, _) in line.match_indices("::") {
                let head = &line[..at];
                let ty = head
                    .rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
                    .next()
                    .unwrap_or("");
                if !SITE_TYPES.contains(&ty) {
                    continue;
                }
                let variant: String = line[at + 2..]
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                *got.entry((file.clone(), function.clone(), format!("{ty}::{variant}")))
                    .or_default() += 1;
            }
        }
    }
    let want: std::collections::BTreeMap<(String, String, String), usize> = SITES
        .iter()
        .map(|&(f, func, d, n)| ((f.to_owned(), func.to_owned(), d.to_owned()), n))
        .collect();
    let moved: Vec<String> = got
        .iter()
        .filter(|(k, n)| want.get(*k) != Some(n))
        .map(|((f, func, d), n)| format!("(\"{f}\", \"{func}\", \"{d}\", {n}),"))
        .chain(
            want.iter()
                .filter(|(k, _)| !got.contains_key(*k))
                .map(|((f, func, d), n)| format!("gone: ({f}, {func}, {d}, {n})")),
        )
        .collect();
    assert!(
        moved.is_empty(),
        "sites whose decision moved:\n{}",
        moved.join("\n")
    );
}

/// The decision types a site names.
const SITE_TYPES: &[&str] = &[
    "Coincide",
    "BooleanDecision",
    "LeverArm",
    "SphereQuestion",
    "SelfCheck",
];

/// `code` (a [`test_utils::source::code_only`] view) with every
/// `#[cfg(test)]` module's body removed.
fn without_test_modules(code: &str) -> String {
    use test_utils::source::balanced_end;
    let mut out = String::new();
    let mut from = 0;
    while let Some(at) = code[from..].find("#[cfg(test)]").map(|i| from + i) {
        // Past the attribute and any after it, to the item.
        let mut item = at;
        while code[item..].trim_start().starts_with("#[") {
            let open = item + code[item..].find('[').unwrap();
            item = balanced_end(code, open).unwrap() + 1;
        }
        let head = code[item..].trim_start();
        let head = head
            .strip_prefix("pub(crate) ")
            .or_else(|| head.strip_prefix("pub "))
            .unwrap_or(head);
        let body = head
            .strip_prefix("mod ")
            .and_then(|rest| {
                let name_end = rest.find(|c: char| !(c.is_alphanumeric() || c == '_'))?;
                rest[name_end..].trim_start().starts_with('{').then_some(())
            })
            .and_then(|()| code[item..].find('{').map(|i| item + i));
        match body.and_then(|open| balanced_end(code, open)) {
            Some(close) => {
                out.push_str(&code[from..at]);
                from = close + 1;
            }
            None => {
                out.push_str(&code[from..item]);
                from = item;
            }
        }
    }
    out.push_str(&code[from..]);
    out
}

/// The name of a function `line` opens at the top level or in an `impl`
/// (indented at most four spaces).
fn top_level_fn(line: &str) -> Option<String> {
    let body = line.trim_start();
    if line.len() - body.len() > 4 {
        return None;
    }
    let mut rest = body;
    for prefix in ["pub(crate) ", "pub(super) ", "pub ", "const "] {
        rest = rest.strip_prefix(prefix).unwrap_or(rest);
    }
    let rest = rest.strip_prefix("fn ")?;
    Some(
        rest.chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect(),
    )
}

/// The per-site table, as the census reads it: `(file, function,
/// decision, mentions)`.
const SITES: &[(&str, &str, &str, usize)] = &[
    (
        "carrier_cross.rs",
        "escalated",
        "BooleanDecision::Crossing",
        1,
    ),
    (
        "circle_cylinder.rs",
        "-",
        "BooleanDecision::ArcCylinderRoots",
        2,
    ),
    (
        "circle_sphere.rs",
        "-",
        "BooleanDecision::ArcSphereRoots",
        1,
    ),
    ("circle_torus.rs", "-", "BooleanDecision::ArcTorusRoots", 1),
    (
        "circle_torus.rs",
        "escalated",
        "BooleanDecision::ArcTorusRoots",
        1,
    ),
    (
        "ellipse_roots.rs",
        "ellipse_roots",
        "BooleanDecision::ArcCylinderRoots",
        1,
    ),
    (
        "ellipse_roots.rs",
        "ellipse_roots",
        "BooleanDecision::ArcSphereRoots",
        1,
    ),
    (
        "finish.rs",
        "weld_pinches",
        "BooleanDecision::VertexOnVertex",
        1,
    ),
    ("insert.rs", "germ_dir", "BooleanDecision::SelfCheck", 1),
    ("insert.rs", "germ_dir", "SelfCheck::GermLine", 1),
    ("insert.rs", "record_germ_dir", "Coincide::TangentLocus", 2),
    ("insert.rs", "strut_order", "Coincide::Sectors", 1),
    ("insert.rs", "walks_after", "Coincide::Sectors", 1),
    ("join.rs", "bool_connect", "Coincide::Section", 1),
    ("join.rs", "frame_refusal", "BooleanDecision::Radius", 1),
    ("join.rs", "frame_refusal", "Coincide::Section", 1),
    (
        "join.rs",
        "germs_face_each_other",
        "BooleanDecision::SelfCheck",
        1,
    ),
    ("join.rs", "germs_face_each_other", "Coincide::Join", 1),
    (
        "join.rs",
        "germs_face_each_other",
        "SelfCheck::ArcFacing",
        1,
    ),
    ("join.rs", "loose_partners", "Coincide::Join", 1),
    ("join.rs", "partners", "Coincide::Join", 1),
    (
        "join.rs",
        "ring_winding_order",
        "BooleanDecision::SelfCheck",
        1,
    ),
    ("join.rs", "ring_winding_order", "SelfCheck::RingWinding", 1),
    ("join.rs", "slots", "Coincide::Join", 1),
    ("mod.rs", "coincidence", "BooleanDecision::Coincidence", 1),
    ("mod.rs", "decision_words", "BooleanDecision::ArcSpan", 1),
    (
        "mod.rs",
        "decision_words",
        "BooleanDecision::Containment",
        1,
    ),
    (
        "mod.rs",
        "decision_words",
        "BooleanDecision::PierceOnFace",
        1,
    ),
    ("mod.rs", "decision_words", "Coincide::EdgeOnCurvedFace", 1),
    ("mod.rs", "decision_words", "Coincide::EdgeOnEdge", 1),
    ("mod.rs", "decision_words", "Coincide::EdgeOnPlane", 1),
    ("mod.rs", "decision_words", "Coincide::Sectors", 1),
    ("mod.rs", "decision_words", "Coincide::VertexOnFace", 1),
    ("mod.rs", "of_lever", "BooleanDecision::of_lever", 1),
    (
        "mod.rs",
        "of_pierced_normal",
        "BooleanDecision::of_normal",
        1,
    ),
    (
        "mod.rs",
        "plane_identity",
        "BooleanDecision::of_plane_rung",
        1,
    ),
    (
        "mod.rs",
        "screen_contradiction",
        "BooleanDecision::SelfCheck",
        1,
    ),
    (
        "mod.rs",
        "screen_contradiction",
        "SelfCheck::CarrierLadder",
        1,
    ),
    ("mod.rs", "unsettled_rest", "Coincide::DeclaredReach", 1),
    ("mod.rs", "tangent_rim_refusal", "Coincide::Rim", 1),
    (
        "mod.rs",
        "verify_tangency_declaration",
        "Coincide::Contact",
        1,
    ),
    ("mod.rs", "verify_tangency_declaration", "Coincide::Rim", 2),
    (
        "mod.rs",
        "verify_tangency_declaration",
        "Coincide::TangentLocus",
        2,
    ),
    (
        "ops.rs",
        "bound_holds",
        "BooleanDecision::VolumeBackstop",
        1,
    ),
    ("ops.rs", "recut_lean", "BooleanDecision::Sphere", 1),
    ("ops.rs", "recut_lean", "SphereQuestion::RecutAlign", 1),
    ("ops.rs", "seam_refusal", "BooleanDecision::SeamJet", 1),
    ("ops.rs", "seam_refusal", "LeverArm::Seam", 1),
    (
        "ops.rs",
        "sphere_extent_scan",
        "BooleanDecision::Containment",
        1,
    ),
    ("ops.rs", "sphere_extent_scan", "BooleanDecision::Sphere", 1),
    (
        "ops.rs",
        "sphere_extent_scan",
        "SphereQuestion::AgainstPlane",
        1,
    ),
    ("ops.rs", "sphere_extent_scan", "SphereQuestion::Apart", 1),
    (
        "ops.rs",
        "sphere_extent_scan",
        "SphereQuestion::EscapeParallel",
        1,
    ),
    ("ops.rs", "sphere_extent_scan", "SphereQuestion::Nested", 1),
    (
        "ops.rs",
        "volume_backstop",
        "BooleanDecision::VolumeBackstop",
        1,
    ),
    ("recl.rs", "parallel_same_dir", "Coincide::EdgeOnEdge", 1),
    ("recl.rs", "recl_sectors", "Coincide::TangentSide", 1),
    (
        "recl.rs",
        "resolve_edge_edge",
        "Coincide::CurvedFlankSense",
        1,
    ),
    ("recl.rs", "resolve_edge_edge", "Coincide::FlankSense", 1),
    ("recl.rs", "resolve_edge_edge", "Coincide::TangentSide", 1),
    (
        "reduce.rs",
        "arc_chain_reaches",
        "Coincide::EdgeOnCurvedFace",
        1,
    ),
    (
        "reduce.rs",
        "curved_face_arm",
        "Coincide::ArcOnCoveredFace",
        1,
    ),
    (
        "reduce.rs",
        "curved_face_arm",
        "Coincide::EdgeOnCurvedFace",
        1,
    ),
    (
        "reduce.rs",
        "curved_face_arm",
        "Coincide::VertexOnCoveredFace",
        1,
    ),
    (
        "reduce.rs",
        "curved_face_arm",
        "Coincide::VertexOnCurvedFace",
        1,
    ),
    ("reduce.rs", "esc", "BooleanDecision::Containment", 1),
    (
        "reduce.rs",
        "line_wall_roots_of",
        "BooleanDecision::SphereRoots",
        1,
    ),
    (
        "reduce.rs",
        "line_wall_roots_of",
        "BooleanDecision::TorusRoots",
        1,
    ),
    (
        "reduce.rs",
        "line_wall_roots_of",
        "BooleanDecision::WallRoots",
        1,
    ),
    (
        "reduce.rs",
        "split_other_at_point",
        "BooleanDecision::ArcSpan",
        1,
    ),
    (
        "reduce.rs",
        "split_other_at_point",
        "BooleanDecision::SplitPointOnCircle",
        1,
    ),
    (
        "reduce.rs",
        "sweep_direction",
        "BooleanDecision::of_conic_root",
        1,
    ),
    ("reduce.rs", "sweep_direction", "Coincide::EdgeOnPlane", 3),
    ("reduce.rs", "sweep_direction", "Coincide::VertexOnFace", 6),
    (
        "reduce.rs",
        "vertex_on_curved_face_at",
        "BooleanDecision::VertexOnVertex",
        1,
    ),
    (
        "reduce.rs",
        "wall_crossing",
        "BooleanDecision::Containment",
        1,
    ),
    ("reduce.rs", "wall_crossing", "BooleanDecision::Crossing", 1),
    ("rest.rs", "enumerate_segments", "Coincide::Join", 1),
    (
        "sectors.rs",
        "bisector_zero_refusal",
        "BooleanDecision::BisectorSide",
        1,
    ),
    ("sectors.rs", "build_sectors", "BooleanDecision::Corner", 1),
    (
        "sectors.rs",
        "direction_sense",
        "BooleanDecision::DirectionSense",
        1,
    ),
    ("sectors.rs", "pair_search", "Coincide::Sectors", 1),
    ("sectors.rs", "parallel_same", "Coincide::Sectors", 1),
    (
        "sectors.rs",
        "side_code",
        "BooleanDecision::PierceCurvature",
        1,
    ),
    ("sectors.rs", "side_code", "Coincide::SectorSide", 1),
    ("sectors.rs", "side_code", "LeverArm::SectorSide", 1),
    ("sectors.rs", "tangent_lump", "Coincide::TangentLocus", 1),
    (
        "sectors.rs",
        "tangent_relative_side",
        "LeverArm::SectorCurving",
        1,
    ),
    ("sectors.rs", "within", "Coincide::Sectors", 1),
    (
        "vtxfac.rs",
        "classify_vertex_on_face",
        "Coincide::Sectors",
        2,
    ),
    (
        "vtxfac.rs",
        "classify_vertex_on_face",
        "Coincide::TangentSide",
        1,
    ),
    ("vtxfac.rs", "pierce_germ_dir", "Coincide::Sectors", 1),
];

// ------------------------------------------------------------------
// The levers the withdrawn arms name, executed (the coincfr4 review's
// MINOR-2 and C3).
// ------------------------------------------------------------------

/// Declares the lever rows: one `#[ignore]`d child each, run at
/// [`DESIGN_EPS`] by [`every_withdrawn_arms_lever_passes_or_it_ends_as_its_frontier`],
/// with the outcome it must have (`None`: it passes).
macro_rules! lever_rows {
    ($($name:ident: $want:expr => $raise:expr;)*) => {
        $(
            #[test]
            #[ignore = "a child row: every_withdrawn_arms_lever_passes_or_it_ends_as_its_frontier runs it"]
            fn $name() {
                report(stringify!($name), &outcome($raise));
            }
        )*
        const LEVER_ROWS: &[(&str, Option<&str>)] = &[$((stringify!($name), $want)),*];
    };
}

lever_rows! {
    // `CurvedFlankSense`: the faces along the edge reshaped to planes that
    // only touch there, at a clear arm.
    curved_flank_reshaped_to_touching_planes: None => planar_flank_membership_at(true, false, 1e-3);
    // `VertexOnCoveredFace` and `ArcOnCoveredFace`: the vertex moved
    // clearly onto the face it is declared to touch (the arc's ends with
    // it) passes; moved clearly clear of it, the declaration is
    // contradicted, which is why the lever no longer names that side.
    covered_vertex_moved_onto_its_face: None =>
        arc_against_a_wall(1.0, Some(ContactClass::Rest));
    covered_vertex_moved_clear_of_its_face: Some("ContactContradicted") =>
        arc_against_a_wall(1.1, Some(ContactClass::Rest));
    // `Coincidence(Planes)`: a declared-`Tangent` pair of planes, clearly
    // parallel (contradicted) or clearly tilted (the class unsupported):
    // no move of the parts passes, so the arm ends as that frontier.
    tangent_planes_made_parallel: Some("ContactContradicted") => block_declared_tangent_at(0.0);
    tangent_planes_clearly_tilted: Some("UnsupportedDeclarationClass") =>
        block_declared_tangent_at(1e-3);
    // `FlankSense`, overlapping and undeclared: lengthened, the sense
    // decides and the undeclared coplanar pair refuses next, whose own
    // story is the declaration, which passes.
    overlapping_flanks_lengthened: Some("UndeclaredCoincidence") =>
        planar_flank_membership_at(false, false, 1e-3);
    overlapping_flanks_lengthened_declared_rest: None =>
        planar_flank_membership_at(false, true, 1e-3);
    touching_flanks_lengthened: None => planar_flank_membership_at(true, false, 1e-3);
    // `PlaneOrientation`: lengthened, the offset rung refuses the
    // undeclared pair next, as above.
    facing_planes_lengthened: Some("UndeclaredCoincidence") => shared_side_plane(1e-3);
    // `VertexOnFace`: the vertices moved clearly off the face.
    vertex_moved_clearly_above_a_face: None => block_on_a_block(1.0 + 1e-3);
    vertex_moved_clearly_into_a_face: None => block_on_a_block(1.0 - 1e-3);
}

/// **Every lever a withdrawn arm names passes, or the arm ends as its
/// frontier**: each row moves the parts as a withdrawn arm's lever says
/// and runs at [`DESIGN_EPS`]; it passes, or refuses on the decision its
/// row names, which is the frontier or the next decision's own story.
#[test]
fn every_withdrawn_arms_lever_passes_or_it_ends_as_its_frontier() {
    let module = module_path!()
        .split_once("::")
        .map_or(module_path!(), |(_, m)| m);
    for &(name, want) in LEVER_ROWS {
        let got = run(&format!("{module}::{name}"), DESIGN_EPS);
        let met = match (&got, want) {
            (Outcome::Pass, None) => true,
            (Outcome::Refused { key, defect, .. }, Some(want)) => key == want && !defect,
            _ => false,
        };
        assert!(met, "{name}: wants {want:?}: {got:?}");
    }
}

// ------------------------------------------------------------------
// The harness's own rule, on synthetic children.
// ------------------------------------------------------------------

mod harness {
    use geom_core::Tol;
    use test_utils::offer::{Executed, Outcome, Story, Verdict, execute, judge_laters, report};

    /// The tolerance a synthetic child runs at, as the process
    /// committed it.
    fn eps() -> f64 {
        Tol::witness().eps()
    }

    /// A refusal of `key` offering `below` (none where `None`), its text
    /// `menu` appended.
    fn refused(key: &str, below: Option<f64>, menu: &str) -> Outcome {
        Outcome::Refused {
            key: key.into(),
            defect: false,
            text: below.map_or_else(
                || format!("x is undecided. Recourse: move it{menu}"),
                |v| {
                    format!(
                        "x is undecided. Recourse: move it, or tighten the tolerance below {v:e} m"
                    )
                },
            ),
        }
    }

    /// Declares synthetic child rows: each reports what its closure makes
    /// of the tolerance it runs at.
    macro_rules! synthetic {
        ($($name:ident => $at:expr;)*) => {
            $(
                #[test]
                #[ignore = "a child row: execute_tells_each_offer_by_the_rule runs it"]
                fn $name() {
                    let at: fn(f64) -> Outcome = $at;
                    report(stringify!($name), &at(eps()));
                }
            )*
        };
    }

    synthetic! {
        // Passes below its offer.
        synthetic_t1 => |e| if e > 6e-10 { refused("K", Some(5e-10), "") } else { Outcome::Pass };
        // Its own decision refuses below its offer.
        synthetic_f1 => |e| refused("K", Some(e / 2.0), "");
        // A defect below its offer.
        synthetic_f2 => |e| if e > 6e-10 {
            refused("K", Some(5e-10), "")
        } else {
            Outcome::Refused { key: "D".into(), defect: true, text: "defect".into() }
        };
        // A different decision below its offer, which no tolerance passes.
        synthetic_f3 => |e| if e > 6e-10 {
            refused("K", Some(5e-10), "")
        } else {
            refused("Frontier", None, "")
        };
        // A different decision below its offer, offering no value but an
        // unvalued menu, and a pass a decade down.
        synthetic_t2_menu => |e| if e > 6e-10 {
            refused("K", Some(5e-10), "")
        } else if e > 1e-10 {
            refused("Other", None, ", or lower the tolerance")
        } else {
            Outcome::Pass
        };
        // A different decision below its offer whose own offer is false.
        synthetic_t2_false_later => |e| if e > 6e-10 {
            refused("K", Some(5e-10), "")
        } else if e > 3e-10 {
            refused("L", Some(e), "")
        } else if e > 1e-11 {
            refused("M", None, "")
        } else {
            Outcome::Pass
        };
    }

    /// **`execute` tells each offer by the rule** (the module docs), on
    /// synthetic children whose outcomes are fixed functions of the
    /// tolerance: T1; F1, F2 and F3 refused; T2 with each later story
    /// recorded, and `judge_laters` asking an owner for an untrue one.
    #[test]
    fn execute_tells_each_offer_by_the_rule() {
        let module = module_path!()
            .split_once("::")
            .map_or(module_path!(), |(_, m)| m);
        let row = |name: &str| format!("{module}::{name}");
        let same = |a: &str, b: &str| a == b;
        assert!(matches!(
            execute(&row("synthetic_t1"), "K", same),
            Ok(Executed {
                verdict: Verdict::T1,
                ..
            })
        ));
        for (name, why) in [
            ("synthetic_f1", "F1"),
            ("synthetic_f2", "F2"),
            ("synthetic_f3", "F3"),
        ] {
            let got = execute(&row(name), "K", same);
            assert!(
                got.as_ref().is_err_and(|e| e.contains(why)),
                "{name}: {got:?}"
            );
        }
        let Ok(Executed {
            verdict: Verdict::T2 { laters },
            ..
        }) = execute(&row("synthetic_t2_menu"), "K", same)
        else {
            panic!("a different decision, then a pass");
        };
        assert_eq!(
            laters
                .iter()
                .map(|l| (l.key.as_str(), l.story))
                .collect::<Vec<_>>(),
            [("Other", Story::NoValue)]
        );
        let menu = "lower the tolerance";
        assert!(judge_laters(&laters, menu, &[]).is_err());
        assert_eq!(
            judge_laters(&laters, menu, &[("Other", "work/row.md")]),
            Ok(vec![("Other".to_owned(), "work/row.md")])
        );
        let Ok(Executed {
            verdict: Verdict::T2 { laters },
            ..
        }) = execute(&row("synthetic_t2_false_later"), "K", same)
        else {
            panic!("a different decision, then a pass");
        };
        assert_eq!(
            laters
                .iter()
                .map(|l| (l.key.as_str(), l.story))
                .collect::<Vec<_>>(),
            [("L", Story::FalseOffer), ("M", Story::NoValue)]
        );
        assert!(judge_laters(&laters, menu, &[]).is_err());
    }
}
