//! **VERBS-TEAPOT R2 review probes (ordinal 100)** — claims-to-falsify.
//!
//! Not part of the PR under review; lives on the probe branch only.
//! Every row here is built from fixtures the PR does NOT enumerate, so
//! a green row is independent signal and not a re-run of their table.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;

use pncad::authoring::{p2, validated};
use pncad::geom::{Curve3, Surface};
use pncad::geom_core::{Band, Point2, Point3, Tol, Vec2, Vec3};
use pncad::prelude::{Open, Start};
use pncad::profile::{ProfileLoop, SketchPlane};
use pncad::sweep::{
    Extrusion, Revolution, RevolveAxis, TubeWindow, extrude, revolve, tube_along_arc,
};
use pncad::topo::{Body, FaceKey, LoopBoundary, ReplaceFaceError, ShellError};

const FIT_TOL: f64 = 1e-6;

fn band(tol: Tol) -> Band {
    Band::linear(tol).expect("band")
}

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
// #1082 — the validated wrong body
// =====================================================================

/// **R2-1: MY OWN revolved profile, not the pot's and not the drum's.**
/// A three-step waisted silhouette on non-dyadic stations, opened at a
/// chart the PR never touches. If #1082 is real this reproduces.
#[test]
fn r2_my_own_revolve_opens_wrong() {
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
    let cup = pncad::topo::shell_open(&body, 0.021, &chart, FIT_TOL, band(tol), tol)
        .expect("shell_open returns a body");
    println!(
        "[r2-1] tier3 = {:?}; shells = {}; rings = {}; genus = {}",
        pncad::topo::validate_geometric(&cup, tol),
        cup.shells().count(),
        rings(&cup),
        genus(&cup),
    );
    let mesh = pncad::mesh::tessellate(&cup, 1e-3, tol);
    println!("[r2-1] tessellate = {:?}", mesh.map(|m| m.triangles.len()));
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
    match pncad::topo::shell_open(&body, 0.05, &chart, FIT_TOL, band(tol), tol) {
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
                pncad::mesh::tessellate(&cup, 1e-3, tol).map(|m| m.triangles.len())
            );
        }
    }
}

/// **R2-3: a PARTIAL revolve — one cap face, still a revolve.** The
/// other half of the same separation: a wedge's top is ONE planar face
/// (no seam split at all) and it touches the axis. If the opened rim is
/// right here, the seam split is the variable; if it is wrong here too,
/// "two half-discs sharing a chart" is not the mechanism.
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
    match pncad::topo::shell(&body, 0.05, FIT_TOL, band(tol), tol) {
        Err(e) => println!("[r2-3] SEALED refuses: {}", offset_refusal(&e)),
        Ok(b) => println!(
            "[r2-3] SEALED hollows: shells = {}, genus = {}",
            b.shells().count(),
            genus(&b)
        ),
    }
    if chart.len() == 1 {
        match pncad::topo::shell_open(&body, 0.05, &chart, FIT_TOL, band(tol), tol) {
            Err(e) => println!("[r2-3] OPEN refuses: {e}"),
            Ok(cup) => println!(
                "[r2-3] OPEN Ok: rings = {}, genus = {}, mesh = {:?}",
                rings(&cup),
                genus(&cup),
                pncad::mesh::tessellate(&cup, 1e-3, tol).map(|m| m.triangles.len())
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
    let cup =
        pncad::topo::shell_open(&body, t, &chart, FIT_TOL, band(tol), tol).expect("the drum opens");
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
        pncad::mesh::tessellate(&cup, 1e-3, tol).map(|m| m.triangles.len())
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
    let cup = pncad::topo::shell_open(&body, t, &top, FIT_TOL, band(tol), tol)
        .expect("a box opens at its top");
    println!(
        "[r2-5] box cup: rings = {}, genus = {}, shells = {}, tier3 = {:?}",
        rings(&cup),
        genus(&cup),
        cup.shells().count(),
        pncad::topo::validate_geometric(&cup, tol)
    );
    let m = pncad::mesh::tessellate(&cup, 1e-3, tol);
    println!("[r2-5] tessellate = {:?}", m.map(|x| x.triangles.len()));
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
    let hex: Vec<(f64, f64)> = (0..6)
        .map(|i| {
            let a = core::f64::consts::TAU * f64::from(i) / 6.0;
            (0.5 * a.cos(), 0.5 * a.sin())
        })
        .collect();
    let bevel: Vec<(f64, f64)> = vec![(0.0, 0.0), (0.6, 0.0), (0.8, 0.2), (0.8, 0.5), (0.0, 0.5)];
    let kite: Vec<(f64, f64)> = vec![(0.0, 0.0), (0.4, -0.25), (0.9, 0.0), (0.4, 0.25)];
    for (what, pts) in [
        ("a hexagonal prism", hex),
        ("a box with ONE bevelled edge", bevel),
        ("a kite prism", kite),
    ] {
        let mut b = Open.at(Point2::new(pts[0].0, pts[0].1));
        for p in &pts[1..] {
            b = b.line_to(Point2::new(p.0, p.1), tol).expect("side");
        }
        let lp: ProfileLoop<f64> = b.line_to(Start, tol).expect("close").into();
        let body = extruded(lp, 0.3, tol);
        match pncad::topo::shell(&body, 0.02, FIT_TOL, band(tol), tol) {
            Err(e) => println!("[r2-6] {what}: REFUSES {}", offset_refusal(&e)),
            Ok(h) => println!(
                "[r2-6] {what}: HOLLOWS (shells {}, genus {})",
                h.shells().count(),
                genus(&h)
            ),
        }
    }
    // And the control the class predicts SURVIVES: a right prism on a
    // rectangle with different numbers, plus an axis-aligned cross.
    let cross: Vec<(f64, f64)> = vec![
        (0.2, 0.0),
        (0.4, 0.0),
        (0.4, 0.2),
        (0.6, 0.2),
        (0.6, 0.4),
        (0.4, 0.4),
        (0.4, 0.6),
        (0.2, 0.6),
        (0.2, 0.4),
        (0.0, 0.4),
        (0.0, 0.2),
        (0.2, 0.2),
    ];
    let mut b = Open.at(Point2::new(cross[0].0, cross[0].1));
    for p in &cross[1..] {
        b = b.line_to(Point2::new(p.0, p.1), tol).expect("side");
    }
    let lp: ProfileLoop<f64> = b.line_to(Start, tol).expect("close").into();
    let body = extruded(lp, 0.3, tol);
    match pncad::topo::shell(&body, 0.02, FIT_TOL, band(tol), tol) {
        Err(e) => println!(
            "[r2-6] a CROSS prism (all square): REFUSES {}",
            offset_refusal(&e)
        ),
        Ok(h) => println!(
            "[r2-6] a CROSS prism (all square): hollows (genus {})",
            genus(&h)
        ),
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
    match pncad::topo::shell(&body, 1.0 / 128.0, FIT_TOL, band(tol), tol) {
        Err(e) => println!("[r2-7] bullet: {}", offset_refusal(&e)),
        Ok(h) => println!("[r2-7] bullet HOLLOWS (genus {})", genus(&h)),
    }
    // The same dome geometry reached WITHOUT the tangent door: a
    // hemisphere sitting on a cylinder of a DIFFERENT radius makes the
    // junction non-tangent, so the arc is authored by `Center` and the
    // description is not a mapped tangent arc. If that refuses
    // ReanchorOffCarrier, the bullet's door change is about the
    // AUTHORING route, exactly as the PR declines to claim.
    let lp2: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(r, 0.0), tol)
        .expect("base")
        .line_to(Point2::new(r, top), tol)
        .expect("wall")
        .arc_to(
            pncad::profile::Center {
                c: Point2::new(0.0, top),
                winding: pncad::profile::ArcSweep::Ccw,
                p: Point2::new(0.0, top + r * 0.8),
            },
            tol,
        )
        .map(|b| b.line_to(Start, tol).expect("axis").into())
        .map_err(|e| format!("{e:?}"));
    match lp2 {
        Err(e) => println!("[r2-7] the non-tangent dome does not even author: {e}"),
        Ok(lp2) => {
            let b2 = revolved(lp2, tol);
            match pncad::topo::shell(&b2, 1.0 / 128.0, FIT_TOL, band(tol), tol) {
                Err(e) => println!("[r2-7] non-tangent dome: {}", offset_refusal(&e)),
                Ok(h) => println!("[r2-7] non-tangent dome HOLLOWS (genus {})", genus(&h)),
            }
        }
    }
}

/// **R2-8: the acceptance corpus, re-run here.** The PR's "why nothing
/// caught this" says `verbs_shell.rs`'s three fixtures all sit inside
/// the surviving class. Rebuilt from their own descriptions.
#[test]
fn r2_acceptance_corpus_sits_inside_the_class() {
    let tol = Tol::witness();
    // box
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(2.0, 0.0), tol)
        .expect("a")
        .line_to(Point2::new(2.0, 3.0), tol)
        .expect("b")
        .line_to(Point2::new(0.0, 3.0), tol)
        .expect("c")
        .line_to(Start, tol)
        .expect("d")
        .into();
    let boxy = extruded(lp, 4.0, tol);
    // vessel: cylinder between two caps
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(1.0, 0.0), tol)
        .expect("a")
        .line_to(Point2::new(1.0, 2.0), tol)
        .expect("b")
        .line_to(Point2::new(0.0, 2.0), tol)
        .expect("c")
        .line_to(Start, tol)
        .expect("d")
        .into();
    let vessel = revolved(lp, tol);
    // tube: annular meridian
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.6, 0.0))
        .line_to(Point2::new(1.0, 0.0), tol)
        .expect("a")
        .line_to(Point2::new(1.0, 2.0), tol)
        .expect("b")
        .line_to(Point2::new(0.6, 2.0), tol)
        .expect("c")
        .line_to(Start, tol)
        .expect("d")
        .into();
    let tube = revolved(lp, tol);
    for (what, body, t) in [
        ("the box", boxy, 0.25),
        ("the vessel", vessel, 0.2),
        ("the tube", tube, 0.1),
    ] {
        match pncad::topo::shell(&body, t, FIT_TOL, band(tol), tol) {
            Ok(h) => println!(
                "[r2-8] {what} hollows (shells {}, genus {})",
                h.shells().count(),
                genus(&h)
            ),
            Err(e) => println!("[r2-8] {what} REFUSES {}", offset_refusal(&e)),
        }
    }
}

// =====================================================================
// The union walls
// =====================================================================

/// **R2-9: both refusals, on MY operands.** A torus tube against a
/// plain cylinder (no pot), and a cone frustum against the same
/// cylinder. Confirms the pair the gate names and the wrong-pair
/// caveat's mechanism: the gate returns the FIRST offender × other-face
/// pair in ARENA order whose padded boxes overlap.
#[test]
fn r2_the_two_union_walls_on_my_operands() {
    let tol = Tol::witness();
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(0.5, 0.0), tol)
        .expect("a")
        .line_to(Point2::new(0.5, 1.0), tol)
        .expect("b")
        .line_to(Point2::new(0.0, 1.0), tol)
        .expect("c")
        .line_to(Start, tol)
        .expect("d")
        .into();
    let can = revolved(lp, tol);
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
        "[r2-9] can ∪ handle: {:?}",
        pncad::topo::union(&can, &handle, tol).err()
    );
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(0.25, 0.0), tol)
        .expect("a")
        .line_to(Point2::new(0.1, 0.7), tol)
        .expect("cone")
        .line_to(Point2::new(0.0, 0.7), tol)
        .expect("c")
        .line_to(Start, tol)
        .expect("d")
        .into();
    let cone = revolved(lp, tol);
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
        "[r2-9] can ∪ cone: {:?}",
        pncad::topo::union(&can, &cone, tol).err()
    );
    // Order matters to the gate (Operand::A is scanned first): the same
    // pair with the operands swapped names operand A.
    println!(
        "[r2-9] cone ∪ can: {:?}",
        pncad::topo::union(&cone, &can, tol).err()
    );
}
