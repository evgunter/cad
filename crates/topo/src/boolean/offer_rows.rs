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
//! The census is by construction: every decision the table renders a
//! valued tolerance for, on each side of zero it offers one on, has a
//! case here, or one `sweep` runs (`OFFERS_EXECUTED_IN_SWEEP`); a case
//! whose arm has withdrawn its tolerance asserts it stays withdrawn.
//!
//! Each case's margin is a fixed length chosen against the band at
//! [`test_utils::offer::DESIGN_EPS`], so a re-run at a smaller tolerance
//! raises the same geometry. A case says which door it raises through:
//! the public Boolean, or the site that asks the question, where no
//! public door is known to reach it with a margin in band.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::super::reduce::{ContactAcc, CurvedEvent, curved_face_arm};
use super::super::sectors::{NO_CURVATURE, Reach, side_code, tangent_relative_side};
use super::super::{BooleanDeclarations, BooleanError, DeclaredPairs, FacePairDeclaration, Operand};
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
    /// No tolerance: the arm withdrew it, and the case pins that.
    Withdrawn,
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
    line_inside_a_wall: "Coincidence(EdgeOnCurvedFace)", false, CURVED_ARM_SITE, Withdrawn =>
        line_against_a_wall(1.0 - D, 0.0);
    endpoint_clear_of_a_wall: "Coincidence(VertexOnCurvedFace)", true, CURVED_ARM_SITE, Valued =>
        line_against_a_wall(1.0, (2.0 * D).sqrt());
    endpoint_inside_a_wall: "Coincidence(VertexOnCurvedFace)", false, CURVED_ARM_SITE, Withdrawn =>
        line_run([(1.0 - D, 0.0), (2.0, 0.0), (2.0, 2.0), (1.0 - D, 2.0)]);
    arc_clear_of_a_wall: "Coincidence(ArcClearsCurvedFace)", true, CURVED_ARM_SITE, Valued =>
        arc_against_a_wall(1.0 + D, None, false);
    arc_on_a_covered_wall: "Coincidence(ArcOnCoveredFace)", true, CURVED_ARM_SITE, Valued =>
        arc_against_a_wall(1.0 + D, Some(ContactClass::Rest), false);
    arc_on_a_tangent_wall: "Coincidence(ArcOnCoveredFace)", true, CURVED_ARM_SITE, Valued =>
        arc_against_a_wall(1.0 + D, Some(ContactClass::Tangent), false);
    arc_ends_clear_of_a_covered_wall: "Coincidence(VertexOnCoveredFace)", true, CURVED_ARM_SITE, Withdrawn =>
        arc_against_a_wall(1.0 + D, Some(ContactClass::Rest), true);
    thin_wedge_on_a_block: "Coincidence(Sectors)", true, Public, Valued =>
        wedge_on_a_block(5.0, D);
    // The arms this pass withdrew, each on the raise that showed its
    // offer false.
    tangent_screen_of_a_tilted_block: "Coincidence(Planes)", true, Public, Withdrawn =>
        tilted_block_declared_tangent();
    membership_along_a_curved_flank: "Coincidence(CurvedFlankSense)", true, Door::Site(
        "the membership tie is read inside the edge-edge resolution, on hand-built sectors: \
         an in-band sense needs a flanker arm inside the band, which the corner's arm rung \
         refuses first on a public raise",
    ), Withdrawn => curved_flank_membership();
    pierce_germ_line_in_band: "Coincidence(Sectors)", true, Door::Site(
        "the germ line is read at a transition sector whose vertices stand off their own face, \
         which a public raise does not place",
    ), Valued => pierce_germ_line();
}

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

/// A declared-`Tangent` ball on a floor: the second-order side at the
/// arm whose sagitta is `D`, or the arm gate at an arm of `D`.
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
    let wall = cyl_wall_sheet(&mut y, CylFrame::canonical(1.0), None, (-0.5, 0.5), (0.0, 1.0), tol);
    let declared = DeclaredPairs::build(&BooleanDeclarations::none(), Default::default());
    let mut acc = ContactAcc::default();
    curved_face_arm(
        &x, &mut y, Operand::A, edge_key, &edge, a, c, wall, pt(a), pt(c), &declared, &mut acc,
        band(), tol,
    )
    .map(|_: CurvedEvent<f64>| ())
}

/// The rim arcs of a radius-`r` wall sheet against a unit wall sheet
/// sharing its axis, the pair declared `class` (and read as one carrier
/// where `one_carrier`): the first arc's raise.
fn arc_against_a_wall(
    r: f64,
    class: Option<ContactClass>,
    one_carrier: bool,
) -> Result<(), BooleanError> {
    let tol = Tol::witness();
    let mut x: crate::body::Body<f64> = crate::body::Body::new();
    let xw = cyl_wall_sheet(&mut x, CylFrame::canonical(r), None, (0.5, 1.0), (0.25, 0.5), tol);
    let sense = x.get_face(xw).unwrap().sense;
    x.set_face_sense(xw, !sense).unwrap();
    let mut y: crate::body::Body<f64> = crate::body::Body::new();
    let yw = cyl_wall_sheet(&mut y, CylFrame::canonical(1.0), None, (0.0, 3.0), (0.0, 1.0), tol);
    let decls = BooleanDeclarations {
        coincident_faces: class
            .map(|class| vec![FacePairDeclaration::new(xw, yw, class)])
            .unwrap_or_default(),
        ..BooleanDeclarations::none()
    };
    let one = if one_carrier {
        [(xw, yw)].into_iter().collect()
    } else {
        Default::default()
    };
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
        &x, &mut y, Operand::A, edge_key, &edge, a, c, yw, pt(a), pt(c), &declared, &mut acc,
        band(), tol,
    )
    .map(|_: CurvedEvent<f64>| ())
}

/// A wedge-cornered block of opening `opening_deg` whose bottom face is
/// tilted by `tilt` about one wedge edge, its corner on a block's top
/// face: the union.
fn wedge_on_a_block(opening_deg: f64, tilt: f64) -> Result<(), BooleanError> {
    use crate::test_support_fixtures::{brick, mapped_cube};
    let tol = Tol::witness();
    let phi = opening_deg.to_radians();
    let (ea, eb) = (
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(phi.cos(), phi.sin(), tilt * phi.sin()),
    );
    let p = Point3::new(0.5, 0.2, 1.0);
    let wedge = mapped_cube::<f64>(move |u, v, w| p + ea * u + eb * v + Vec3::new(0.0, 0.0, w), tol);
    let block = brick((-1.0, 3.0), (-2.0, 2.5), (0.0, 1.0), tol);
    super::super::boolean_op_with(
        super::super::BooleanOp::Union,
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
        start_reach: Reach::Chord { base: o, far: o + x },
        end_reach: Reach::Chord { base: o, far: o + y },
        face: crate::entity::FaceKey::default(),
        normal: OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true),
        arm: 1.0,
    };
    super::super::vtxfac::pierce_germ_dir(&s, Vec3::new(D.sin(), 0.0, D.cos()), band())
        .map(|_| ())
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
    assert!(false_offers.is_empty(), "false offers:\n{}", false_offers.join("\n\n"));
}

/// **A withdrawn tolerance stays withdrawn**: each case whose arm offers
/// none refuses on its decision at [`DESIGN_EPS`] with no tolerance.
#[test]
fn every_withdrawn_tolerance_stays_withdrawn() {
    for case in CASES.iter().filter(|c| c.offer == Withdrawn) {
        match run(&row(case), DESIGN_EPS) {
            Outcome::Refused { key, defect, text } => {
                assert_eq!(key, case.key, "{}: {text}", case.name);
                assert!(!defect, "{}: {text}", case.name);
                assert_eq!(margin_is_positive(&text), Some(case.positive), "{}: {text}", case.name);
                assert!(
                    test_utils::offer::offered_below(&text).is_none() && !text.contains("tighten"),
                    "{}: offers a tolerance: {text}",
                    case.name
                );
            }
            Outcome::Pass => panic!("{}: passes at the design tolerance", case.name),
        }
    }
}

/// The keys and sides the table renders a valued tolerance on, over
/// every decision ([`every_decision`]) at a point margin in band and in
/// the zero band on each side.
fn offering_arms() -> std::collections::BTreeSet<(String, bool)> {
    let band = band();
    let mut out = std::collections::BTreeSet::new();
    for decision in every_decision() {
        for m in [D, -D, Z, -Z] {
            let diag = Indeterminate {
                margin: MarginDiag::value(m),
                band,
                predicate: Some("census"),
                terminal_sliver: false,
            };
            let text = BooleanError::Escalated { decision, diag }.to_string();
            if test_utils::offer::offered_below(&text).is_some() {
                let err = BooleanError::Escalated { decision, diag };
                out.insert((crate::test_support::offer_key(&err).0, m > 0.0));
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
    for case in CASES.iter().filter(|c| c.offer == Withdrawn) {
        assert!(
            !offering.contains(&(case.key.to_owned(), case.positive)),
            "{}: a withdrawn case's arm offers a tolerance in the table",
            case.name
        );
    }
}
