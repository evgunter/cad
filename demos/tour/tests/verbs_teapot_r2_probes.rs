//! **VERBS-TEAPOT R2 review probes (ordinal 100)** — claims-to-falsify.
//!
//! Not part of the PR under review; lives on the probe branch only.
//! Every row here is built from fixtures the PR does NOT enumerate, so
//! a green row is independent signal and not a re-run of their table.
//!
//! **The #1082 rows (R2-1, R2-2, R2-3, R2-4, R2-10) are kept VERBATIM
//! after the repair.** They were written as instruments rather than as
//! assertions — they print the shape the surgery left and assert only
//! that a body comes back — so what they measure now is the fixed rim:
//! one annular face with one disjoint ring on an axis-touching cap,
//! TWO disjoint annuli on an annular one, both meshing. Nothing here
//! needed re-pinning, and the rows are worth more unedited: they are
//! the measurement that re-scoped the class, and they still run the
//! same fixtures against the door. The numbers that ARE pinned live
//! beside them — `verbs_teapot::the_opened_rim_is_an_annulus_on_every_
//! revolve` and `..::the_annular_mouth_opens_to_two_disjoint_rims` —
//! and R2-5, the box control, is unchanged and still green, which is
//! the differential that says the repair did not move the case that
//! was always right.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;

use pncad::authoring::{p2, validated};
use pncad::geom::{Curve3, Surface};
use pncad::geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use pncad::prelude::{Open, Start};
use pncad::profile::{ProfileLoop, SketchPlane};
use pncad::sweep::{
    Extrusion, Revolution, RevolveAxis, TubeWindow, extrude, revolve, tube_along_arc,
};
use pncad::topo::{Body, FaceKey, LoopBoundary, ReplaceFaceError, ShellError};

/// A closed polygon through `$first` and the rest, on the `path`
/// lattice (`RawLoop::polygon` is deliberately off pncad's presented
/// surface, so the chain is unrolled here).
macro_rules! poly {
    ($tol:expr, $first:expr, $($rest:expr),+ $(,)?) => {{
        let b = Open.at($first);
        $( let b = b.line_to($rest, $tol).expect("side"); )+
        let lp: ProfileLoop<f64> = b.line_to(Start, $tol).expect("close").into();
        lp
    }};
}

const FIT_TOL: f64 = 1e-6;

fn revolved(lp: ProfileLoop<f64>, tol: Tol) -> Body<f64> {
    revolve(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("meridian validates"),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("meridian revolves")
    .body
}

fn revolved_partial(lp: ProfileLoop<f64>, theta: f64, tol: Tol) -> Body<f64> {
    revolve(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("meridian validates"),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(theta),
        tol,
    )
    .expect("meridian revolves partially")
    .body
}

fn extruded(lp: ProfileLoop<f64>, h: f64, tol: Tol) -> Body<f64> {
    extrude(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("footprint validates"),
        Extrusion::Distance(h),
        tol,
    )
    .expect("footprint extrudes")
    .body
}

/// **One of NINE copies of this helper across five crates (#1123).**
/// `demos/tour` is a separate workspace and an integration test cannot
/// import a binary's module, so no existing home covers them all; the
/// issue carries the list and the shared-test-support fix.
fn rings(body: &Body<f64>) -> usize {
    body.faces().map(|(_, f)| f.rings.len()).sum()
}

fn genus(body: &Body<f64>) -> i64 {
    let (v, e, f) = (
        body.vertices().count() as i64,
        body.edges().count() as i64,
        body.faces().count() as i64,
    );
    body.shells().count() as i64 - (v - e + f - rings(body) as i64) / 2
}

/// Every planar face whose plane origin sits at station `y`.
fn plane_chart_at(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

/// The refusal's class name plus what it measured, off the payload.
fn offset_refusal(e: &ShellError<f64>) -> String {
    match e {
        ShellError::Face { error, .. } => match &**error {
            ReplaceFaceError::ReanchorOffCarrier { gap, .. } => {
                format!("ReanchorOffCarrier(gap={gap})")
            }
            ReplaceFaceError::CarrierLaneUnsupported { what, .. } => {
                format!("CarrierLaneUnsupported({what})")
            }
            other => format!("OTHER-FACE-DOOR({other})"),
        },
        other => format!("NOT-THE-OFFSET-DOOR({other})"),
    }
}

// =====================================================================
// #1082 — the validated wrong body, and what these rows read after the
// repair. Every claim below is written in the PAST tense where it
// describes the defect: these rows measured it, the fix landed on that
// measurement, and a row whose prose still says "is wrong" would be
// stating a falsehood about the door it runs against.
// =====================================================================

/// **R2-1: MY OWN revolved profile, not the pot's and not the drum's.**
/// A three-step waisted silhouette on non-dyadic stations, opened at a
/// chart the PR never touches. Written to reproduce #1082 on a fixture
/// outside their enumeration, which it did; it now instruments the
/// REPAIRED rim on that same fixture, and its printout is what a
/// reader compares against the numbers pinned in `verbs_teapot`.
#[test]
fn r2_my_own_revolve_opens_at_a_chart_they_never_touch() {
    let tol = Tol::witness();
    let top = 0.37;
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(0.11, 0.0), tol)
        .expect("base")
        .line_to(Point2::new(0.11, 0.09), tol)
        .expect("foot")
        .line_to(Point2::new(0.17, 0.09), tol)
        .expect("out")
        .line_to(Point2::new(0.17, 0.23), tol)
        .expect("belly")
        .line_to(Point2::new(0.13, 0.23), tol)
        .expect("in")
        .line_to(Point2::new(0.13, top), tol)
        .expect("neck")
        .line_to(Point2::new(0.0, top), tol)
        .expect("mouth")
        .line_to(Start, tol)
        .expect("axis")
        .into();
    let body = revolved(lp, tol);
    let chart = plane_chart_at(&body, top);
    println!("[r2-1] my mouth chart is {} face(s)", chart.len());
    let cup = pncad::topo::shell_open(&body, 0.021, &chart, FIT_TOL, tol)
        .expect("shell_open returns a body");
    println!(
        "[r2-1] tier3 = {:?}; shells = {}; rings = {}; genus = {}",
        pncad::topo::validate_geometric(&cup, tol),
        cup.shells().count(),
        rings(&cup),
        genus(&cup),
    );
    let mesh = pncad::mesh::tessellate(&cup, 1e-3, tol);
    println!(
        "[r2-1] tessellate = {:?}",
        mesh.map(|m| m.patches.iter().map(|q| q.triangles.len()).sum::<usize>())
    );
    let props = pncad::topo::mass_properties(&cup, tol);
    println!(
        "[r2-1] props = {:?}",
        props.map(|p| (p.volume, p.volume_pad))
    );
}

/// **R2-2: the variable the PR's mechanism story names — is it the SEAM
/// SPLIT, or is it that the mouth touches the AXIS?**
///
/// Their claim: "an extrusion's cap is ONE face; a full revolve's is
/// TWO half-discs sharing a chart. That path has never had a consumer."
/// Every fixture in their sweep is a full revolve of an AXIS-TOUCHING
/// profile, so the two variables move together. This row separates
/// them: a revolved TUBE (annular meridian) has a mouth chart of two
/// half-ANNULI — still a full revolve, still two faces on one chart,
/// but no axis apex and the designated faces already carry a hole's
/// worth of boundary.
#[test]
fn r2_revolved_tube_separates_seam_from_axis() {
    let tol = Tol::witness();
    let (ri, ro, h) = (0.30, 0.50, 0.40);
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(ri, 0.0))
        .line_to(Point2::new(ro, 0.0), tol)
        .expect("base annulus")
        .line_to(Point2::new(ro, h), tol)
        .expect("outer wall")
        .line_to(Point2::new(ri, h), tol)
        .expect("top annulus")
        .line_to(Start, tol)
        .expect("bore")
        .into();
    let body = revolved(lp, tol);
    let chart = plane_chart_at(&body, h);
    println!(
        "[r2-2] revolved tube: {} faces, mouth chart {} face(s)",
        body.faces().count(),
        chart.len()
    );
    match pncad::topo::shell_open(&body, 0.05, &chart, FIT_TOL, tol) {
        Err(e) => println!("[r2-2] REFUSED: {e}"),
        Ok(cup) => {
            println!(
                "[r2-2] Ok: tier3 = {:?}; shells = {}; rings = {}; genus = {}",
                pncad::topo::validate_geometric(&cup, tol),
                cup.shells().count(),
                rings(&cup),
                genus(&cup),
            );
            println!(
                "[r2-2] tessellate = {:?}",
                pncad::mesh::tessellate(&cup, 1e-3, tol).map(|m| m
                    .patches
                    .iter()
                    .map(|q| q.triangles.len())
                    .sum::<usize>())
            );
        }
    }
}

/// **R2-3: a PARTIAL revolve — one cap face, still a revolve.** The
/// other half of the same separation: a wedge's top is ONE planar face
/// (no seam split at all) and it touches the axis. The question it was
/// written to settle — if the opened rim were right here the seam
/// split would be the variable, and if wrong here too then "two
/// half-discs sharing a chart" was not the mechanism — was settled
/// against the seam-split story, and the repair was built to what this
/// and R2-2 measured rather than to that story.
#[test]
fn r2_partial_revolve_one_cap_face() {
    let tol = Tol::witness();
    let (r, h) = (0.5, 0.4);
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(r, 0.0), tol)
        .expect("base")
        .line_to(Point2::new(r, h), tol)
        .expect("wall")
        .line_to(Point2::new(0.0, h), tol)
        .expect("top")
        .line_to(Start, tol)
        .expect("axis")
        .into();
    let body = revolved_partial(lp, FRAC_PI_2, tol);
    let chart = plane_chart_at(&body, h);
    println!(
        "[r2-3] wedge: {} faces; top chart = {} face(s)",
        body.faces().count(),
        chart.len()
    );
    // The SEALED arm first: the wedge's meridian caps are planes
    // CONTAINING the cylinder's axis, which their stated surviving
    // class ("a plane NORMAL to a cylinder's axis") excludes.
    match pncad::topo::shell(&body, 0.05, FIT_TOL, tol) {
        Err(e) => println!("[r2-3] SEALED refuses: {}", offset_refusal(&e)),
        Ok(b) => println!(
            "[r2-3] SEALED hollows: shells = {}, genus = {}",
            b.shells().count(),
            genus(&b)
        ),
    }
    if chart.len() == 1 {
        match pncad::topo::shell_open(&body, 0.05, &chart, FIT_TOL, tol) {
            Err(e) => println!("[r2-3] OPEN refuses: {e}"),
            Ok(cup) => println!(
                "[r2-3] OPEN Ok: rings = {}, genus = {}, mesh = {:?}",
                rings(&cup),
                genus(&cup),
                pncad::mesh::tessellate(&cup, 1e-3, tol).map(|m| m
                    .patches
                    .iter()
                    .map(|q| q.triangles.len())
                    .sum::<usize>())
            ),
        }
    }
}

/// **R2-4: ANATOMY of the "spurious full ring".** Is the extra loop a
/// FULL circle (which would be geometrically impossible inside a half
/// disc), or the cavity half-disc's own boundary re-labelled as a ring?
/// Does it share vertices with the face's outer loop? The PR's word is
/// "a full RING"; #1082 repeats it. This row measures which it is.
#[test]
fn r2_ring_anatomy_on_a_drum() {
    let tol = Tol::witness();
    let (r, h, t) = (0.5, 0.4, 0.05);
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(r, 0.0), tol)
        .expect("base")
        .line_to(Point2::new(r, h), tol)
        .expect("wall")
        .line_to(Point2::new(0.0, h), tol)
        .expect("top")
        .line_to(Start, tol)
        .expect("axis")
        .into();
    let body = revolved(lp, tol);
    let chart = plane_chart_at(&body, h);
    let cup = pncad::topo::shell_open(&body, t, &chart, FIT_TOL, tol).expect("the drum opens");
    println!(
        "[r2-4] drum cup: V/E/F = {}/{}/{}, shells = {}, rings = {}, genus = {}",
        cup.vertices().count(),
        cup.edges().count(),
        cup.faces().count(),
        cup.shells().count(),
        rings(&cup),
        genus(&cup)
    );
    println!(
        "[r2-4] tier3 = {:?}",
        pncad::topo::validate_geometric(&cup, tol)
    );
    let walk = |lk| -> Vec<(String, Point3<f64>)> {
        let LoopBoundary::Cycle { first } = cup.get_loop(lk).expect("loop").boundary else {
            return vec![];
        };
        cup.loop_cycle(first)
            .expect("cycle")
            .iter()
            .map(|&he| {
                let hd = cup.get_half_edge(he).expect("he");
                let p = *cup
                    .get_point(cup.get_vertex(hd.start).expect("v").point)
                    .expect("pt");
                let ed = cup.get_edge(hd.edge).expect("e");
                let carrier = cup
                    .get_curve_geom(ed.curve)
                    .and_then(|g| g.certified())
                    .map(|c| match c.carrier() {
                        Curve3::Line { .. } => "line".to_string(),
                        Curve3::Circle { center, radius, .. } => {
                            format!("circle(c.y={:.4}, r={:.4})", center.y, radius)
                        }
                        Curve3::Ellipse { .. } => "ellipse".to_string(),
                        Curve3::Nurbs(_) => "nurbs".to_string(),
                    })
                    .unwrap_or_else(|| "?".to_string());
                (carrier, p)
            })
            .collect()
    };
    for (k, f) in cup.faces() {
        if f.rings.is_empty() {
            continue;
        }
        let plane = matches!(cup.get_surface(f.surface),
            Some(Surface::Plane { origin, .. }) if (origin.y - h).abs() < 1e-12);
        println!(
            "[r2-4] face {k:?} (mouth plane = {plane}) rings = {}",
            f.rings.len()
        );
        println!("[r2-4]   OUTER: {:?}", walk(f.outer));
        for r in &f.rings {
            println!("[r2-4]   RING : {:?}", walk(*r));
        }
    }
    println!(
        "[r2-4] tessellate(1e-3) = {:?}",
        pncad::mesh::tessellate(&cup, 1e-3, tol).map(|m| m
            .patches
            .iter()
            .map(|q| q.triangles.len())
            .sum::<usize>())
    );
    println!(
        "[r2-4] props = {:?}",
        pncad::topo::mass_properties(&cup, tol).map(|p| (p.volume, p.volume_pad, p.surface_area))
    );
    // The closed form a correct cup would have: the drum minus a
    // cavity that runs to the top.
    let want = core::f64::consts::PI * (r * r * h - (r - t) * (r - t) * (h - t));
    println!("[r2-4] a correct cup's closed-form volume = {want}");
}

/// **R2-5: the box control, on MY box and MY thickness.** The PR's
/// control is `boxy(0.2, 0.3, 0.25)` at t = 0.02. Different numbers,
/// same question.
#[test]
fn r2_box_control_is_right() {
    let tol = Tol::witness();
    let (w, d, h, t) = (1.3, 0.7, 0.9, 0.11);
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(w, 0.0), tol)
        .expect("a")
        .line_to(Point2::new(w, d), tol)
        .expect("b")
        .line_to(Point2::new(0.0, d), tol)
        .expect("c")
        .line_to(Start, tol)
        .expect("d")
        .into();
    let body = extruded(lp, h, tol);
    let top: Vec<FaceKey> = body
        .faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if normal.x.abs() < 1e-9 && normal.y.abs() < 1e-9
                        && (origin.z - h).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(top.len(), 1, "an extrusion's cap is ONE face");
    let cup =
        pncad::topo::shell_open(&body, t, &top, FIT_TOL, tol).expect("a box opens at its top");
    println!(
        "[r2-5] box cup: rings = {}, genus = {}, shells = {}, tier3 = {:?}",
        rings(&cup),
        genus(&cup),
        cup.shells().count(),
        pncad::topo::validate_geometric(&cup, tol)
    );
    let m = pncad::mesh::tessellate(&cup, 1e-3, tol);
    println!(
        "[r2-5] tessellate = {:?}",
        m.map(|x| x.patches.iter().map(|q| q.triangles.len()).sum::<usize>())
    );
    let props = pncad::topo::mass_properties(&cup, tol).expect("props");
    let want = w * d * h - (w - 2.0 * t) * (d - 2.0 * t) * (h - t);
    println!(
        "[r2-5] volume {} vs the cup's closed form {want} (pad {})",
        props.volume, props.volume_pad
    );
    assert_eq!(
        (rings(&cup), genus(&cup)),
        (1, 0),
        "the control must be right"
    );
}

// =====================================================================
// #1081 — the one-junction-shape class
// =====================================================================

/// **R2-6: fixtures OUTSIDE their enumeration.** A hexagonal prism
/// (120° dihedrals, all planes, CONVEX, and its inward offset is a
/// trivially correct smaller hexagon), a box with ONE bevelled edge
/// (135°), and a 4-sided kite prism. Their class rule predicts all
/// three refuse `ReanchorOffCarrier`; a success on any of them would
/// mean the class is narrower than "oblique".
#[test]
fn r2_oblique_plane_prisms_outside_their_table() {
    let tol = Tol::witness();
    let hx = |i: i32| {
        let a = core::f64::consts::TAU * f64::from(i) / 6.0;
        Point2::new(0.5 * a.cos(), 0.5 * a.sin())
    };
    // A regular HEXAGONAL prism: every dihedral 120 degrees, every face
    // a plane, and the inward offset of the footprint is a trivially
    // correct smaller hexagon.
    let hex = poly!(tol, hx(0), hx(1), hx(2), hx(3), hx(4), hx(5));
    // A box with ONE bevelled corner: two 135-degree dihedrals, the
    // other three square.
    let bevel = poly!(
        tol,
        Point2::new(0.0, 0.0),
        Point2::new(0.6, 0.0),
        Point2::new(0.8, 0.2),
        Point2::new(0.8, 0.5),
        Point2::new(0.0, 0.5),
    );
    // A kite: four planes, no right angle anywhere.
    let kite = poly!(
        tol,
        Point2::new(0.0, 0.0),
        Point2::new(0.4, -0.25),
        Point2::new(0.9, 0.0),
        Point2::new(0.4, 0.25),
    );
    // The control the class predicts SURVIVES: a non-convex CROSS whose
    // every dihedral is square (four of them reflex).
    let cross = poly!(
        tol,
        Point2::new(0.2, 0.0),
        Point2::new(0.4, 0.0),
        Point2::new(0.4, 0.2),
        Point2::new(0.6, 0.2),
        Point2::new(0.6, 0.4),
        Point2::new(0.4, 0.4),
        Point2::new(0.4, 0.6),
        Point2::new(0.2, 0.6),
        Point2::new(0.2, 0.4),
        Point2::new(0.0, 0.4),
        Point2::new(0.0, 0.2),
        Point2::new(0.2, 0.2),
    );
    for (what, lp) in [
        ("a hexagonal prism (120 deg)", hex),
        ("a box with ONE bevelled edge (135 deg)", bevel),
        ("a kite prism (no right angle)", kite),
        ("a CROSS prism (all square, non-convex)", cross),
    ] {
        let body = extruded(lp, 0.3, tol);
        match pncad::topo::shell(&body, 0.02, FIT_TOL, tol) {
            Err(e) => println!("[r2-6] {what}: REFUSES {}", offset_refusal(&e)),
            Ok(h) => println!(
                "[r2-6] {what}: HOLLOWS (shells {}, genus {})",
                h.shells().count(),
                genus(&h)
            ),
        }
    }
}

/// **R2-7: the tangent bullet's second door, and WHICH of its two
/// `what` strings fires.** The PR attributes the row to the mapped-arc
/// description authored by `.tangent().tangent_arc_to(..)`. That is one
/// of two `CarrierLaneUnsupported` sites in `replace_face.rs`; the
/// other is a carrier that is neither a line nor a circle. The payload
/// carries the string, so this is measurable and the PR did not
/// measure it (its table records only the variant).
#[test]
fn r2_tangent_bullet_which_door() {
    let tol = Tol::witness();
    let r = 3.0 / 64.0;
    let top = 8.0 / 64.0;
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(r, 0.0), tol)
        .expect("base")
        .line_to(Point2::new(r, top), tol)
        .expect("wall")
        .tangent()
        .tangent_arc_to(Point2::new(0.0, top + r), tol)
        .expect("dome")
        .line_to(Start, tol)
        .expect("axis")
        .into();
    let body = revolved(lp, tol);
    match pncad::topo::shell(&body, 1.0 / 128.0, FIT_TOL, tol) {
        Err(e) => println!("[r2-7] bullet: {}", offset_refusal(&e)),
        Ok(h) => println!("[r2-7] bullet HOLLOWS (genus {})", genus(&h)),
    }
    // The SAME curved pair (sphere against cylinder) reached by the
    // ORDINARY `Center` arc instead of the tangent door: the centre is
    // lifted off the wall's top so the junction is definitely NOT
    // tangent. If this refuses `ReanchorOffCarrier`, the bullet's
    // different door tracks the AUTHORING ROUTE rather than the pair.
    let d = 0.02;
    let rr = (r * r + d * d).sqrt();
    let lp2: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(r, 0.0), tol)
        .expect("base")
        .line_to(Point2::new(r, top), tol)
        .expect("wall")
        .arc_to(
            pncad::profile::Center {
                c: Point2::new(0.0, top + d),
                winding: pncad::profile::ArcSweep::Ccw,
                p: Point2::new(0.0, top + d + rr),
            },
            tol,
        )
        .expect("a non-tangent dome authors")
        .line_to(Start, tol)
        .expect("axis")
        .into();
    let b2 = revolved(lp2, tol);
    match pncad::topo::shell(&b2, 1.0 / 128.0, FIT_TOL, tol) {
        Err(e) => println!(
            "[r2-7] NON-tangent dome (Center arc): {}",
            offset_refusal(&e)
        ),
        Ok(h) => println!("[r2-7] NON-tangent dome HOLLOWS (genus {})", genus(&h)),
    }
}

/// **R2-8: the acceptance corpus, rebuilt from its own descriptions.**
/// The PR's "why nothing caught this" says `verbs_shell.rs`'s three
/// fixtures — a box, a cylinder between two caps, a tube between two
/// caps — all sit inside the surviving class.
#[test]
fn r2_acceptance_corpus_sits_inside_the_class() {
    let tol = Tol::witness();
    let boxy = extruded(
        poly!(
            tol,
            Point2::new(0.0, 0.0),
            Point2::new(2.0, 0.0),
            Point2::new(2.0, 3.0),
            Point2::new(0.0, 3.0),
        ),
        4.0,
        tol,
    );
    let vessel = revolved(
        poly!(
            tol,
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(1.0, 2.0),
            Point2::new(0.0, 2.0),
        ),
        tol,
    );
    let tube = revolved(
        poly!(
            tol,
            Point2::new(0.6, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(1.0, 2.0),
            Point2::new(0.6, 2.0),
        ),
        tol,
    );
    for (what, body, t) in [
        ("the box", boxy, 0.25),
        ("the vessel", vessel, 0.2),
        ("the tube", tube, 0.1),
    ] {
        match pncad::topo::shell(&body, t, FIT_TOL, tol) {
            Ok(h) => println!(
                "[r2-8] {what} hollows (shells {}, genus {})",
                h.shells().count(),
                genus(&h)
            ),
            Err(e) => println!("[r2-8] {what} REFUSES {}", offset_refusal(&e)),
        }
    }
}

/// **R2-9: both union walls, on MY operands.** A torus tube against a
/// plain cylinder can, and a cone frustum against the same can. The
/// gate scans `Operand::A` then `B`, offenders in ARENA order against
/// the other body's faces in ARENA order, and returns the FIRST padded
/// box overlap — so swapping the operands must swap which side is
/// named.
#[test]
fn r2_the_two_union_walls_on_my_operands() {
    let tol = Tol::witness();
    let can = revolved(
        poly!(
            tol,
            Point2::new(0.0, 0.0),
            Point2::new(0.5, 0.0),
            Point2::new(0.5, 1.0),
            Point2::new(0.0, 1.0),
        ),
        tol,
    );
    let handle = tube_along_arc::<f64>(
        Point3 {
            x: 0.5,
            y: 0.5,
            z: 0.0,
        },
        Vec3::unit_z(),
        Vec3::unit_x(),
        0.3,
        TubeWindow::Arc { t0: -2.0, t1: 2.0 },
        0.08,
        tol,
    )
    .expect("tube builds")
    .body;
    println!(
        "[r2-9] can u handle: {:?}",
        pncad::topo::union(&can, &handle, tol).err()
    );
    println!(
        "[r2-9] handle u can: {:?}",
        pncad::topo::union(&handle, &can, tol).err()
    );
    let cone = revolved(
        poly!(
            tol,
            Point2::new(0.0, 0.0),
            Point2::new(0.25, 0.0),
            Point2::new(0.1, 0.7),
            Point2::new(0.0, 0.7),
        ),
        tol,
    );
    let cone = pncad::topo::transform_rigid(
        &cone,
        &pncad::geom_core::Affine3::from_parts(
            pncad::geom_core::Mat3::identity(),
            Vec3::new(0.45, 0.2, 0.0),
        ),
        tol,
    )
    .expect("placed");
    println!(
        "[r2-9] can u cone: {:?}",
        pncad::topo::union(&can, &cone, tol).err()
    );
    println!(
        "[r2-9] cone u can: {:?}",
        pncad::topo::union(&cone, &can, tol).err()
    );
}

/// **R2-10: the ANNULAR-mouth revolve, in full.** R2-2 showed a
/// revolved tube's mouth chart is ONE face (a full revolve of a CLOSED
/// off-axis profile closes its seam) and that opening it ALSO
/// PRODUCED an untessellatable body — with different wrong numbers —
/// so "two half-discs sharing a chart" was never the discriminator.
/// This row dumps what the surgery leaves, and checks the SEALED arm
/// on the same body as the control; what it dumps now is the face
/// SPLIT the repair builds, pinned with its radii and closed-form
/// volume in `verbs_teapot::the_annular_mouth_opens_to_two_disjoint_rims`.
#[test]
fn r2_annular_mouth_anatomy() {
    let tol = Tol::witness();
    let (ri, ro, h, t) = (0.30, 0.50, 0.40, 0.05);
    let body = revolved(
        poly!(
            tol,
            Point2::new(ri, 0.0),
            Point2::new(ro, 0.0),
            Point2::new(ro, h),
            Point2::new(ri, h),
        ),
        tol,
    );
    println!(
        "[r2-10] operand: V/E/F = {}/{}/{}, rings = {}, genus = {}",
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
        rings(&body),
        genus(&body)
    );
    let sealed = pncad::topo::shell(&body, t, FIT_TOL, tol).expect("the tube hollows");
    println!(
        "[r2-10] SEALED: shells = {}, rings = {}, genus = {}, mesh = {:?}",
        sealed.shells().count(),
        rings(&sealed),
        genus(&sealed),
        pncad::mesh::tessellate(&sealed, 1e-3, tol).map(|m| m
            .patches
            .iter()
            .map(|q| q.triangles.len())
            .sum::<usize>())
    );
    let chart = plane_chart_at(&body, h);
    let cup = pncad::topo::shell_open(&body, t, &chart, FIT_TOL, tol).expect("the tube opens");
    println!(
        "[r2-10] OPEN: V/E/F = {}/{}/{}, shells = {}, rings = {}, genus = {}, tier3 = {:?}",
        cup.vertices().count(),
        cup.edges().count(),
        cup.faces().count(),
        cup.shells().count(),
        rings(&cup),
        genus(&cup),
        pncad::topo::validate_geometric(&cup, tol)
    );
    for (k, f) in cup.faces() {
        let s = match cup.get_surface(f.surface) {
            Some(Surface::Plane { origin, .. }) => format!("plane(y={:.3})", origin.y),
            Some(Surface::Cylinder { radius, .. }) => format!("cyl(r={radius:.3})"),
            other => format!("{other:?}"),
        };
        println!("[r2-10]   face {k:?}: {s}, rings = {}", f.rings.len());
    }
    println!(
        "[r2-10] mesh = {:?}",
        pncad::mesh::tessellate(&cup, 1e-3, tol).map(|m| m
            .patches
            .iter()
            .map(|q| q.triangles.len())
            .sum::<usize>())
    );
    // The correct rim here is TWO disjoint annuli (ro-t..ro and
    // ri..ri+t), which is a face SPLIT that `kfmrh` cannot express —
    // so this case cannot be a ring-count bug at all.
    let want = core::f64::consts::PI
        * ((ro * ro - ri * ri) * h - ((ro - t).powi(2) - (ri + t).powi(2)) * (h - t));
    println!(
        "[r2-10] props = {:?} vs a correct cup's {want}",
        pncad::topo::mass_properties(&cup, tol).map(|p| (p.volume, p.volume_pad))
    );
}

/// **R2-11: the STEP frontier the scene declares.** A sealed hollow
/// CYLINDER-and-PLANE pot must refuse `CurvedShellClassification`, and
/// the register row names `kind: "circle"`.
#[test]
fn r2_step_frontier_kind() {
    let tol = Tol::witness();
    let pot = revolved(
        poly!(
            tol,
            Point2::new(0.0, 0.0),
            Point2::new(0.5, 0.0),
            Point2::new(0.5, 1.0),
            Point2::new(0.0, 1.0),
        ),
        tol,
    );
    let hollow = pncad::topo::shell(&pot, 0.1, FIT_TOL, tol).expect("hollows");
    let e = pncad::step_export::step_string(
        &hollow,
        &pncad::step_export::StepOptions {
            product_name: "r2pot".to_string(),
            ..Default::default()
        },
        tol,
    )
    .err();
    println!("[r2-11] STEP on a sealed hollow revolve: {e:?}");
}

/// **R2-12: the scene's own vessel, rebuilt from its stations**, to
/// check the four numbers the panel's note states as measurements: the
/// sense-bit count, the antiparallel clearance, the capacity in litres,
/// and the two union payloads on the REAL operands.
#[test]
fn r2_the_scene_numbers() {
    let tol = Tol::witness();
    let (rf, rb, rn) = (3.0 / 64.0, 5.0 / 64.0, 3.0 / 64.0);
    let (yf, ys, ym) = (1.0 / 64.0, 6.0 / 64.0, 8.0 / 64.0);
    let wall = 1.0 / 128.0;
    let sharp = revolved(
        poly!(
            tol,
            Point2::new(0.0, 0.0),
            Point2::new(rf, 0.0),
            Point2::new(rf, yf),
            Point2::new(rb, yf),
            Point2::new(rb, ys),
            Point2::new(rn, ys),
            Point2::new(rn, ym),
            Point2::new(0.0, ym),
        ),
        tol,
    );
    println!(
        "[r2-12] sharp: V/E/F = {}/{}/{}",
        sharp.vertices().count(),
        sharp.edges().count(),
        sharp.faces().count()
    );
    let planes: Vec<(f64, f64, f64)> = sharp
        .faces()
        .filter_map(|(_, f)| match sharp.get_surface(f.surface) {
            Some(Surface::Plane { origin, normal, .. }) => Some((
                origin.y,
                normal.y,
                if f.sense { normal.y } else { -normal.y },
            )),
            _ => None,
        })
        .collect();
    let stored_plus = planes.iter().filter(|p| p.1 >= 0.0).count();
    println!(
        "[r2-12] planar faces = {}, storing +y = {stored_plus}; senses = {:?}",
        planes.len(),
        planes.iter().map(|p| (p.0, p.2)).collect::<Vec<_>>()
    );
    let mut clearance = f64::INFINITY;
    let mut blind = f64::INFINITY;
    for (i, a) in planes.iter().enumerate() {
        for b in &planes[i + 1..] {
            if a.2 * b.2 < 0.0 {
                clearance = clearance.min((b.0 - a.0).abs());
            }
            if a.1 * b.1 < 0.0 {
                blind = blind.min((b.0 - a.0).abs());
            }
        }
    }
    println!(
        "[r2-12] sense-aware clearance = {clearance}; sense-BLIND = {blind} (inf = none found)"
    );
    let pot = pncad::topo::shell(&sharp, wall, FIT_TOL, tol).expect("the pot hollows");
    println!(
        "[r2-12] pot: V/E/F = {}/{}/{}, shells = {}, genus = {}, tier3 = {:?}",
        pot.vertices().count(),
        pot.edges().count(),
        pot.faces().count(),
        pot.shells().count(),
        genus(&pot),
        pncad::topo::validate_geometric(&pot, tol)
    );
    let classes = pncad::topo::classify_shells(&pot, tol).expect("classify");
    for c in &classes {
        println!(
            "[r2-12]   shell role {:?}, volume {} ({} L)",
            c.role,
            c.volume,
            -c.volume * 1000.0
        );
    }
    // The handle and the spout, on the scene's own parameters.
    let handle = tube_along_arc::<f64>(
        Point3 {
            x: rb,
            y: (yf + ys) / 2.0,
            z: 0.0,
        },
        Vec3::unit_z(),
        Vec3::unit_x(),
        6.0 / 256.0,
        TubeWindow::Arc {
            t0: -(core::f64::consts::FRAC_PI_2 + 0.5),
            t1: core::f64::consts::FRAC_PI_2 + 0.5,
        },
        1.0 / 128.0,
        tol,
    )
    .expect("handle")
    .body;
    println!(
        "[r2-12] pot u handle: {:?}",
        pncad::topo::union(&pot, &handle, tol).err()
    );
    let spout = revolved(
        poly!(
            tol,
            Point2::new(6.0 / 256.0 - 1.0 / 256.0, 0.0),
            Point2::new(6.0 / 256.0, 0.0),
            Point2::new(3.0 / 256.0, 8.0 / 64.0),
            Point2::new(3.0 / 256.0 - 1.0 / 256.0, 8.0 / 64.0),
        ),
        tol,
    );
    let spout = pncad::topo::transform_rigid(
        &spout,
        &pncad::geom_core::Affine3::from_parts(
            pncad::geom_core::Mat3::from_cols(
                Vec3::new(0.6, 0.8, 0.0),
                Vec3::new(-0.8, 0.6, 0.0),
                Vec3::unit_z(),
            ),
            Vec3::new(-1.0 / 32.0, 3.0 / 64.0, 0.0),
        ),
        tol,
    )
    .expect("spout placed");
    let e = pncad::topo::union(&pot, &spout, tol).err();
    println!("[r2-12] pot u spout: {e:?}");
    if let Some(pncad::topo::BooleanError::CurvedPairUnsupported { other_face, .. }) = &e {
        let s = pot
            .get_face(*other_face)
            .and_then(|f| pot.get_surface(f.surface));
        println!("[r2-12]   the face the gate NAMED on the pot: {s:?}");
    }
}
