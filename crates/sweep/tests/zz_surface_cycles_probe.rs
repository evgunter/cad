//! Scratch probe (not for commit): the curved closed forms over a face's
//! loops against over its surface cycles.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;
use geom_core::{Arc2, Band, Point2, Real, Tol};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{Body, FaceKey, HalfEdgeKey};

fn tol() -> Tol {
    Tol::witness()
}

fn circle<T: Real>(cx: f64, cy: f64, r: f64) -> ProfileLoop<T> {
    let f = T::from_f64;
    RawLoop::new([(
        Point2::new(f(cx + r), f(cy)),
        Segment::Arc(Arc2 {
            centre: Point2::new(f(cx), f(cy)),
            radius: f(r),
            sweep: f(TAU),
        }),
    )])
}

fn both_sides(body: &Body<f64>, he: HalfEdgeKey) -> bool {
    let e = body.get_half_edge(he).unwrap().edge;
    let s = topo::readback::edge_sides(body, e).unwrap();
    let (a, b) = s.faces();
    a == b
}

/// The face's surface cycles, as half-edge lists.
fn cycles(body: &Body<f64>, face: FaceKey) -> Vec<Vec<HalfEdgeKey>> {
    let f = body.get_face(face).unwrap();
    let mut all = Vec::new();
    for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let (_, hes) = topo::props::loop_edges(body, lk).unwrap();
        all.extend(hes);
    }
    let succ = |h: HalfEdgeKey| {
        let mut n = body.get_half_edge(h).unwrap().next;
        while both_sides(body, n) {
            n = body.get_half_edge(body.mate(n).unwrap()).unwrap().next;
        }
        n
    };
    let mut seen = Vec::new();
    let mut out = Vec::new();
    for &h in &all {
        if both_sides(body, h) || seen.contains(&h) {
            continue;
        }
        let mut c = vec![h];
        seen.push(h);
        let mut n = succ(h);
        while n != h {
            c.push(n);
            seen.push(n);
            n = succ(n);
        }
        out.push(c);
    }
    out
}

fn report(body: &Body<f64>, what: &str) {
    let band = Band::linear(tol()).unwrap();
    for (fk, f) in body.faces() {
        let surface = body.get_surface(f.surface).unwrap();
        if matches!(surface, geom::Surface::Plane { .. }) {
            continue;
        }
        let mut by_he = std::collections::HashMap::new();
        let mut full = Vec::new();
        for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
            let (edges, hes) = topo::props::loop_edges(body, lk).unwrap();
            for (e, h) in edges.iter().zip(&hes) {
                by_he.insert(*h, e.clone());
            }
            full.push(edges);
        }
        let cyc: Vec<Vec<_>> = cycles(body, fk)
            .iter()
            .map(|c| c.iter().map(|h| by_he[h].clone()).collect())
            .collect();
        let fl: Vec<&[_]> = full.iter().map(Vec::as_slice).collect();
        let cl: Vec<&[_]> = cyc.iter().map(Vec::as_slice).collect();
        let a = geom_brep::props::curved_face_loops(surface, &fl, f.sense, band);
        let b = geom_brep::props::curved_face_loops(surface, &cl, f.sense, band);
        eprintln!(
            "PROBE {what} {fk:?} kind={:?} loops={:?} cycles={:?}\n  loops:  {a:?}\n  cycles: {b:?}",
            core::mem::discriminant(surface),
            full.iter().map(Vec::len).collect::<Vec<_>>(),
            cyc.iter().map(Vec::len).collect::<Vec<_>>(),
        );
    }
}

fn y_axis<T: Real>() -> sweep::RevolveAxis<T> {
    sweep::RevolveAxis {
        origin: Point2::new(T::zero(), T::zero()),
        dir: geom_core::Vec2::new(T::zero(), T::one()),
    }
}

#[test]
fn zz_probe_surface_cycles_green_forms() {
    let vp = Profile::new(SketchPlane::<f64>::xy(), vec![circle(0.5, -0.25, 1.5)])
        .validate(tol())
        .unwrap();
    let t = extrude(
        &vp,
        Extrusion::Distance {
            depth: 2.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap();
    report(&t.body, "extruded circle (cylinder, seam strut)");
    let vp = Profile::new(SketchPlane::<f64>::xy(), vec![circle(3.0, 0.25, 0.5)])
        .validate(tol())
        .unwrap();
    for turn in [sweep::Revolution::Partial(1.25), sweep::Revolution::Full] {
        let r = sweep::revolve(&vp, y_axis(), turn, tol()).unwrap();
        report(&r.body, &format!("revolved circle {turn:?} (torus)"));
    }
}
