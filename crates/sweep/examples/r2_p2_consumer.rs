//! R2 consumer probe for PCURVE P-2 (PR #1177) — the MINT door.
//!
//! Independent of the unit's own rows. It rebuilds the unit's fixture
//! from public API only and asks the questions the unit's row does not:
//!
//!  1. Does the mint pass's OWN mate lookup find an operand pair for
//!     the interior column? (`mint_face` calls `mate_surface`; the
//!     unit's row hand-picks the plane instead, and says so.)
//!  2. What does `certify_general` do with the mate the mint would
//!     actually hand it?
//!  3. Does the WIDENED cap-rim branch's `IsoLine` output certify?
//!     (The whole-body mint refuses during the WALK, so no row in the
//!     tree ever puts that output through `run_iso_checks`.)
//!
//! Run: cargo run -p sweep --example r2_p2_consumer
#![allow(clippy::print_stdout, clippy::too_many_lines)]

use std::sync::Arc;

use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, PcurveCache};
use topo::{Body, FaceSurface, Pcurve};
use profile::RawLoop;

fn prism(scale: f64) -> Body<f64> {
    let square = move || -> sweep::Section {
        let v = |x: f64, y: f64| profile::ProfileVertex::new(Point2::new(x, y), 0.0);
        vec![profile::ProfileLoop::new(vec![
            v(-scale, -scale),
            v(scale, -scale),
            v(scale, scale),
            v(-scale, scale),
        ])]
    };
    let sections = vec![square(), square(), square()];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.5 * scale, 0.0, 1.0 * scale)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0 * scale)),
    ];
    sweep::loft_body::<f64>(&sections, &places, 2, Tol::witness()).expect("prism builds").body
}

fn is_flat(body: &Body<f64>, key: topo::SurfaceKey, scale: f64) -> bool {
    matches!(body.get_surface(key), Some(Surface::Nurbs(n))
        if !n.is_placeholder() && n.control().iter().all(|p| p.y == -scale))
}
fn is_bowed(body: &Body<f64>, key: topo::SurfaceKey, scale: f64) -> bool {
    matches!(body.get_surface(key), Some(Surface::Nurbs(n))
        if !n.is_placeholder() && n.control().iter().any(|p| p.y != -scale)
            && n.control().iter().any(|p| p.x.abs() == scale))
}

fn widened_u_chart(n: &NurbsSurface<f64>) -> Surface<f64> {
    let (_nu, nv) = n.control_counts();
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1).unwrap();
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for i in 0..4 {
        for j in 0..nv {
            let (a, b) = (n.control()[j], n.control()[nv + j]);
            control.push(match i {
                0 => a + (a - b),
                1 => a,
                2 => b,
                _ => b + (b - a),
            });
            weights.push(n.weights()[if i <= 1 { j } else { nv + j }]);
        }
    }
    Surface::Nurbs(Arc::new(
        NurbsSurface::new(ku, n.knots_v().clone(), control, weights).unwrap(),
    ))
}

const SCALE: f64 = 1.0 / 1024.0;

fn main() {
    let tol = Tol::witness();
    let eps = tol.get().eps;
    let band = Band::linear(tol).unwrap();
    println!("eps = {eps:e}, scale = {SCALE}");

    let mut body = prism(SCALE);
    // The flat/bowed seam.
    let (edge, flat, bowed, he_bowed) = {
        let mut found = None;
        for (ek, edge) in body.edges() {
            let (Some(hp), Some(hm)) = (
                body.get_half_edge(edge.he_plus),
                body.get_half_edge(edge.he_minus),
            ) else {
                continue;
            };
            let sp = body.get_face(body.get_loop(hp.parent_loop).unwrap().face).unwrap().surface;
            let sm = body.get_face(body.get_loop(hm.parent_loop).unwrap().face).unwrap().surface;
            let spline = matches!(
                body.get_curve_geom(edge.curve),
                Some(topo::CurveGeom::Certified(c)) if matches!(c.carrier(), geom::Curve3::Nurbs(_))
            );
            if !spline {
                continue;
            }
            if is_flat(&body, sp, SCALE) && is_bowed(&body, sm, SCALE) {
                found = Some((ek, sp, sm, edge.he_minus));
                break;
            }
            if is_flat(&body, sm, SCALE) && is_bowed(&body, sp, SCALE) {
                found = Some((ek, sm, sp, edge.he_plus));
                break;
            }
        }
        found.expect("the prism has a flat/bowed seam")
    };
    let flat_face = body.faces().find(|(_, f)| f.surface == flat).unwrap().0;
    let (carrier, t0, t1) = {
        let Some(topo::CurveGeom::Certified(c)) =
            body.get_curve_geom(body.get_edge(edge).unwrap().curve)
        else {
            panic!()
        };
        let (a, b) = c.params();
        (c.carrier().clone(), a, b)
    };
    let plane = body
        .set_face_surface(
            flat_face,
            FaceSurface::New(Surface::Plane {
                origin: Point3::new(0.0, -SCALE, 0.0),
                normal: Vec3::new(0.0, -1.0, 0.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }),
        )
        .expect("flat wall restates as a plane");
    body.set_edge_curve_nurbs_lane(
        edge,
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1: plane,
                s2: bowed,
                witness: carrier.eval((t0 + t1) * 0.5),
            },
            carrier,
            param_start: t0,
            param_end: t1,
        },
        tol,
    )
    .expect("the seam re-describes");
    body.detach_pcurve(he_bowed);

    // Widen the bowed wall's chart.
    let old_chart = match body.get_surface(bowed) {
        Some(Surface::Nurbs(n)) => n.as_ref().clone(),
        _ => panic!(),
    };
    let widened = widened_u_chart(&old_chart);
    let bowed_face = body.faces().find(|(_, f)| f.surface == bowed).unwrap().0;
    let new_key = body
        .set_face_surface(bowed_face, FaceSurface::New(widened))
        .expect("rechart");
    let chart = match body.get_surface(new_key) {
        Some(Surface::Nurbs(n)) => n.as_ref().clone(),
        _ => panic!(),
    };
    println!("widened chart u domain = {:?}", chart.knots_u().domain());

    // ---- Q1: would the mint's OWN mate lookup find an operand pair? ----
    // `mate_surface` (private) reads `Intersection{s1,s2}` from the
    // edge's description and requires the FACE'S CURRENT surface key
    // to be one of the pair. Replicated here verbatim.
    let desc_pair = {
        let e = body.get_edge(body.get_half_edge(he_bowed).unwrap().edge).unwrap();
        match body.get_curve_geom(e.curve) {
            Some(topo::CurveGeom::Certified(c)) => match *c.description() {
                geom_brep::EdgeDescription::Intersection { s1, s2, .. } => Some((s1, s2)),
                _ => None,
            },
            _ => None,
        }
    };
    let own = body
        .get_face(body.get_loop(body.get_half_edge(he_bowed).unwrap().parent_loop).unwrap().face)
        .unwrap()
        .surface;
    let mate_found = match desc_pair {
        Some((s1, s2)) => own == s1 || own == s2,
        None => false,
    };
    println!(
        "\nQ1  description pair = {desc_pair:?}, face's CURRENT surface = {own:?}"
    );
    println!(
        "Q1  mate_surface would return: {}",
        if mate_found { "Some(mate)" } else { "None  <-- the mint hands certify_general None" }
    );

    // ---- Q2: the derivation, and the mint's own certification call. ----
    let out = topo::pcurve_of(&body, he_bowed, band);
    match &out {
        Ok(Pcurve::General(image)) => {
            println!("\nQ2  pcurve_of(seam) = Ok(General), {} controls, control u = {:?}",
                image.control().len(),
                image.control().iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max));
            let (cc, ct0, ct1) = {
                let e = body.get_edge(body.get_half_edge(he_bowed).unwrap().edge).unwrap();
                let Some(topo::CurveGeom::Certified(c)) = body.get_curve_geom(e.curve) else { panic!() };
                let (a, b) = c.params();
                (c.carrier().clone(), a, b)
            };
            let window = out.as_ref().unwrap().chart_box(ct0, ct1);
            let surf = Surface::Nurbs(Arc::new(chart.clone()));
            // (a) with the mate the MINT would supply (None):
            let with_mint_mate = PcurveCache::certify_general(
                Arc::clone(image), ct0, ct1, &cc, &surf, None, window, band,
            );
            println!("Q2  certify_general(mate = what mint_face supplies) -> {:?}",
                with_mint_mate.as_ref().map(|_| "Ok(cache)").map_err(|e| format!("{e:?}")));
            // (b) with the hand-picked plane the unit's row supplies:
            let plane_surf = body.get_surface(plane).cloned().unwrap();
            let with_hand_mate = PcurveCache::certify_general(
                Arc::clone(image), ct0, ct1, &cc, &surf, Some(&plane_surf), window, band,
            );
            println!("Q2  certify_general(mate = hand-picked plane)        -> {}",
                match &with_hand_mate {
                    Ok(c) => format!("Ok, envelope {:e}", c.certificate().envelope),
                    Err(e) => format!("{e:?}"),
                });
        }
        other => println!("\nQ2  pcurve_of(seam) = {other:?}"),
    }

    // ---- Q3: the whole-body mint. ----
    let mut body2 = body.clone();
    let mint = topo::mint_pcurves(&mut body2, tol);
    println!("\nQ3  mint_pcurves -> {mint:?}");

    // ---- Q4: every half-edge of the widened face, derived, and the
    //          rim arms' output put through the CERTIFICATION door. ----
    println!("\nQ4  the widened face's loop, half-edge by half-edge (caches cleared):");
    let mut cleared = body.clone();
    for (he, _) in cleared.half_edges().map(|(k, v)| (k, v.clone())).collect::<Vec<_>>() {
        cleared.detach_pcurve(he);
    }
    let face_data = cleared.get_face(bowed_face).unwrap().clone();
    let first = match cleared.get_loop(face_data.outer).unwrap().boundary {
        topo::LoopBoundary::Cycle { first } => first,
        topo::LoopBoundary::Empty { .. } => panic!("empty loop"),
    };
    let cycle = cleared.loop_cycle(first).unwrap();
    let surf = Surface::Nurbs(Arc::new(chart.clone()));
    let mut boxes: Vec<(topo::HalfEdgeKey, Pcurve<f64>, f64, f64)> = Vec::new();
    for he in cycle {
        let (cc, ct0, ct1) = {
            let e = cleared.get_edge(cleared.get_half_edge(he).unwrap().edge).unwrap();
            let Some(topo::CurveGeom::Certified(c)) = cleared.get_curve_geom(e.curve) else {
                println!("  {he:?}: carrier not certified");
                continue;
            };
            let (a, b) = c.params();
            (c.carrier().clone(), a, b)
        };
        let d = topo::pcurve_of(&cleared, he, band);
        let kind = match &d {
            Ok(Pcurve::General(_)) => "General".to_string(),
            Ok(Pcurve::IsoLine { p0, pl }) => format!("IsoLine p0={p0:?} pl={pl:?}"),
            Ok(p) => format!("{p:?}"),
            Err(e) => format!("REFUSED {e:?}"),
        };
        println!("  {he:?}: carrier {} -> {kind}",
            match &cc { geom::Curve3::Line{..} => "Line", geom::Curve3::Nurbs(_) => "Nurbs",
                        geom::Curve3::Circle{..} => "Circle", _ => "other" });
        if let Ok(p) = d {
            boxes.push((he, p, ct0, ct1));
        }
    }
    if boxes.len() == cleared.loop_cycle(first).unwrap().len() {
        // Every half-edge derived: build the face window exactly as
        // `mint_face` does, then certify each one.
        let mut window = None;
        for (_, p, a, b) in &boxes {
            let bx = p.chart_box(*a, *b);
            window = Some(match window { None => bx, Some(acc) => geom_brep::ChartWindow::<f64>::hull(acc, bx) });
        }
        let window = window.unwrap();
        println!("\nQ4  all derived — now the CERTIFICATION door on each:");
        for (he, p, a, b) in &boxes {
            let e = cleared.get_edge(cleared.get_half_edge(*he).unwrap().edge).unwrap();
            let Some(topo::CurveGeom::Certified(c)) = cleared.get_curve_geom(e.curve) else { continue };
            let cc = c.carrier().clone();
            let r = match p {
                Pcurve::General(img) => {
                    let mate = None;
                    PcurveCache::certify_general(Arc::clone(img), *a, *b, &cc, &surf, mate, window, band)
                }
                other => PcurveCache::certify(other.clone(), *a, *b, &cc, &surf, window, band),
            };
            println!("  {he:?}: {}", match &r {
                Ok(cache) => format!("CERTIFIED envelope {:e}", cache.certificate().envelope),
                Err(e) => format!("REFUSED {e:?}"),
            });
        }
    } else {
        println!("\nQ4  {} of {} half-edges derived; the walk cannot complete, so NO half-edge of this face reaches the certification door in `mint_face`.",
            boxes.len(), cleared.loop_cycle(first).unwrap().len());
        // Q5: force the question anyway — build the window from the
        // half-edges that DID derive (the true trim region up to the
        // one refusing seam, whose image is the other interior column
        // and lies inside it) and put each derived pcurve through the
        // certification door by hand. This is the measurement no row
        // in the tree takes: does the WIDENED cap-rim branch's output
        // actually certify?
        let mut window: Option<geom_brep::ChartWindow<f64>> = None;
        for (_, p, a, b) in &boxes {
            let bx = p.chart_box(*a, *b);
            window = Some(match window { None => bx, Some(acc) => geom_brep::ChartWindow::<f64>::hull(acc, bx) });
        }
        let window = window.unwrap();
        println!("\nQ5  the certification door, forced (window = hull of the 3 derived boxes):");
        for (he, p, a, b) in &boxes {
            let e = cleared.get_edge(cleared.get_half_edge(*he).unwrap().edge).unwrap();
            let Some(topo::CurveGeom::Certified(c)) = cleared.get_curve_geom(e.curve) else { continue };
            let cc = c.carrier().clone();
            let (label, r) = match p {
                Pcurve::General(img) => (
                    "General (mate = what mint_face supplies: None)",
                    PcurveCache::certify_general(Arc::clone(img), *a, *b, &cc, &surf, None, window, band),
                ),
                other => (
                    "IsoLine from the WIDENED cap-rim branch",
                    PcurveCache::certify(other.clone(), *a, *b, &cc, &surf, window, band),
                ),
            };
            println!("  {he:?}  {label}\n      -> {}", match &r {
                Ok(cache) => format!("CERTIFIED, envelope {:e}", cache.certificate().envelope),
                Err(e) => format!("REFUSED {e:?}"),
            });
        }
    }
}
