//! **SURVEY probe** (TESS, lane `tess/approx-face-survey`, 2026-09-22):
//! evidence for `work/tess/tessellate-refuses-approx-face-without-caches.md`.
//!
//! Reporting only — it prints and asserts nothing about the answers it
//! prints, because its job is to MEASURE what the two doors say on the
//! `Approx`-capped box today, whether `mint_pcurves` still refuses the
//! seam class over a straight carrier, and what changes if it does not.
//!
//! Run: `cargo test -p sweep --test all survey_ -- --nocapture`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_core::Tol;

use crate::common;
use common::approx::{approx_walls, box_with_approx_cap, prism};

/// One face's boundary half-edges, in loop order.
fn outer_hes(body: &topo::Body<f64>, face: topo::FaceKey) -> Vec<topo::HalfEdgeKey> {
    let outer = body.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("outer loop is a cycle");
    };
    body.loop_cycle(first).unwrap()
}

/// Everything the mint and the two consumers key on, per half-edge.
fn dump_loop(body: &topo::Body<f64>, face: topo::FaceKey, tag: &str) {
    for he in outer_hes(body, face) {
        let hed = body.get_half_edge(he).unwrap();
        let edge = body.get_edge(hed.edge).unwrap();
        let curve = body
            .get_curve_geom(edge.curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap();
        let carrier = match curve.carrier() {
            Curve3::Line { .. } => "Line",
            Curve3::Circle { .. } => "Circle",
            Curve3::Ellipse { .. } => "Ellipse",
            Curve3::Spiric { .. } => "Spiric",
            Curve3::Nurbs(_) => "Nurbs",
        };
        let desc = match curve.description() {
            geom_brep::EdgeDescription::Chart(c) => {
                format!("Chart({:?}, seam={})", c.pcurve, c.seam)
            }
            geom_brep::EdgeDescription::Intersection { .. } => "Intersection".to_string(),
            geom_brep::EdgeDescription::TangentIntersection { .. } => {
                "TangentIntersection".to_string()
            }
            geom_brep::EdgeDescription::Scaffold(_) => "Scaffold".to_string(),
        };
        let cache = body
            .pcurve(he)
            .map_or_else(|| "NONE".to_string(), |c| format!("{:?}", c.pcurve()));
        println!("[{tag}] {he:?} carrier={carrier} desc={desc} cache={cache}");
    }
}

/// The chart the cap wears, in the terms the seam class reads.
fn dump_chart(body: &topo::Body<f64>, face: topo::FaceKey, tag: &str) {
    let sk = body.get_face(face).unwrap().surface;
    match body.get_surface(sk) {
        Some(Surface::Approx(a)) => {
            let fit = a.fit();
            println!(
                "[{tag}] fit degree u/v = {}/{}",
                fit.knots_u().degree(),
                fit.knots_v().degree()
            );
            println!(
                "[{tag}] fit domain u = {:?} v = {:?}",
                fit.knots_u().domain(),
                fit.knots_v().domain()
            );
            println!("[{tag}] fit knots u = {:?}", fit.knots_u().knots());
            println!("[{tag}] fit knots v = {:?}", fit.knots_v().knots());
            println!(
                "[{tag}] fit weights all one = {}",
                fit.weights().iter().all(|w| *w == 1.0)
            );
            println!("[{tag}] fit control count = {}", fit.control().len());
        }
        other => println!("[{tag}] surface is {other:?}"),
    }
}

/// Both consumer doors, printed as they answer.
fn dump_doors(body: &topo::Body<f64>, tag: &str) {
    match mesh::tessellate(body, 0.05, Tol::witness()) {
        Ok(m) => println!(
            "[{tag}] tessellate = Ok({} patches, {} positions)",
            m.patches.len(),
            m.positions.len()
        ),
        Err(e) => println!("[{tag}] tessellate = Err({e:?})\n[{tag}]   display: {e}"),
    }
    match topo::mass_properties(body, Tol::witness()) {
        Ok(p) => println!("[{tag}] mass_properties = Ok(volume {})", p.volume),
        Err(e) => println!("[{tag}] mass_properties = Err({e:?})\n[{tag}]   display: {e}"),
    }
    match topo::validate(body) {
        Ok(()) => println!("[{tag}] validate (structural) = Ok"),
        Err(e) => println!("[{tag}] validate (structural) = Err({e:?})"),
    }
    match topo::validate_geometric(body, Tol::witness()) {
        Ok(()) => println!("[{tag}] validate_geometric (check 7) = Ok"),
        Err(e) => println!("[{tag}] validate_geometric (check 7) = Err({e:?})"),
    }
}

/// **The row's reproduction**: the `Approx`-capped box built through the
/// public doors, both consumers before and after a `mint_pcurves` call.
#[test]
fn survey_approx_capped_box() {
    let (mut body, face) = box_with_approx_cap(0.05, 1e-9);
    dump_chart(&body, face, "box");
    dump_loop(&body, face, "box/before");
    dump_doors(&body, "box/before");

    let mint = topo::mint_pcurves(&mut body, Tol::witness());
    println!("[box] mint_pcurves = {mint:?}");
    if mint.is_ok() {
        dump_loop(&body, face, "box/after");
        dump_doors(&body, "box/after");
    }
}

/// **The three-eps sweep of the same class**: does the seam-class mint
/// over a straight carrier hold at every tolerance the tree runs, and
/// at both signs of `d`?
#[test]
fn survey_approx_cap_three_eps() {
    for d in [0.05_f64, -0.05] {
        for target in [1e-6_f64, 1e-9, 1e-12] {
            let (mut body, face) = box_with_approx_cap(d, target);
            let mint = topo::mint_pcurves(&mut body, Tol::witness());
            let tess = mesh::tessellate(&body, 0.05, Tol::witness())
                .map(|m| (m.patches.len(), m.positions.len()));
            let props = topo::mass_properties(&body, Tol::witness()).map(|p| p.volume);
            let tier = topo::validate_geometric(&body, Tol::witness()).is_ok();
            println!(
                "[sweep] d = {d}, target = {target:e}: mint = {mint:?}, tess = {tess:?}, \
                 volume = {props:?}, check7 = {tier}, face = {face:?}"
            );
        }
    }
}

/// The same, on the ONE `Approx`-faced body that meshes today — the
/// loft whose walls were minted with their caches.
#[test]
fn survey_approx_walled_prism() {
    let mut body = prism();
    let faces = approx_walls(&mut body, 0.05, 1e-9);
    dump_chart(&body, faces[0], "prism");
    dump_loop(&body, faces[0], "prism");
    dump_doors(&body, "prism");
}

/// **The isolating probe**: the cap's surgery on the box, but with
/// `mint_pcurves` called on the cap FACE alone, so the refusal (if any)
/// is attributed to the cap's own four half-edges rather than to the
/// whole body's.
#[test]
fn survey_cap_face_mint_alone() {
    let (mut body, face) = box_with_approx_cap(0.05, 1e-9);
    let one = topo::mint_pcurves_of(&mut body, &[face], Tol::witness());
    println!("[cap-only] mint_pcurves_of(cap) = {one:?}");
    if one.is_ok() {
        dump_loop(&body, face, "cap-only");
    }
}
