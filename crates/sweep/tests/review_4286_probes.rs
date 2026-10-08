//! Review probes for PR #4286 (a requested ring edge of a planar support
//! metered against its strip). Not for merge: each row prints what the
//! carve did and checks every built body at tier 3, tier 3′, its closed
//! form and by sampled point membership against an analytic model.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Interval, Point2, Point3, Real, Tol};
use sweep::blend::BlendError;
use sweep::test_support::{block, finished, prism_at};
use topo::boolean::{BooleanDeclarations, BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, EdgeKey, SolidContainment, mass_properties, validate_geometric};

use crate::band_planar_cut_off::{D, Verb, edge, tol};

/// Concave or convex family: the outline is subtracted from a 6×5×2
/// block from z = 1 (a pocket, convex mouth edges) or united onto a
/// 6×5×1 block from z = 1 to 2 (a boss, concave base edges).
#[derive(Clone, Copy, Debug, PartialEq)]
enum Fam {
    Pocket,
    Boss,
}

fn lift<T: geom_core::Decide + geom_core::Bounds + topo::AtRestPolicy>(o: &[(f64, f64)]) -> Vec<(Point2<T>, T)> {
    o.iter()
        .map(|&(x, y)| (Point2::new(T::from_f64(x), T::from_f64(y)), T::zero()))
        .collect()
}

fn build<T: geom_core::Decide + geom_core::Bounds + topo::AtRestPolicy>(
    fam: Fam,
    outline: &[(f64, f64)],
    holes: &[Vec<(f64, f64)>],
) -> Body<T> {
    let tol = tol();
    let op = |op, a: &Body<T>, b: &Body<T>| -> Body<T> {
        boolean_op_with(
            op,
            &finished("a", a.clone(), tol),
            &finished("b", b.clone(), tol),
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            tol,
        )
        .unwrap_or_else(|e| panic!("fixture {op:?}: {e}"))
        .body()
        .expect("a body")
        .body
        .clone()
        .into_body()
    };
    let mut body = match fam {
        Fam::Pocket => op(
            BooleanOp::Subtract,
            &block::<T>(6.0, 5.0, 2.0, tol),
            &prism_at(lift::<T>(outline), T::from_f64(1.0), T::from_f64(2.0), tol),
        ),
        Fam::Boss => op(
            BooleanOp::Union,
            &block::<T>(6.0, 5.0, 1.0, tol),
            &prism_at(lift::<T>(outline), T::from_f64(0.5), T::from_f64(1.5), tol),
        ),
    };
    for h in holes {
        // A blind hole 0.3 deep into the face carrying the tab.
        let z0 = match fam {
            Fam::Pocket => 1.7,
            Fam::Boss => 0.7,
        };
        body = op(
            BooleanOp::Subtract,
            &body,
            &prism_at(lift::<T>(h), T::from_f64(z0), T::from_f64(1.0), tol),
        );
    }
    body
}

fn in_poly(p: (f64, f64), poly: &[(f64, f64)]) -> bool {
    let mut inside = false;
    let n = poly.len();
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        if (a.1 > p.1) != (b.1 > p.1) && p.0 < a.0 + (p.1 - a.1) / (b.1 - a.1) * (b.0 - a.0) {
            inside = !inside;
        }
    }
    inside
}

const TAB_LO: f64 = 2.5;
const TAB_HI: f64 = 3.5;
const TIP_Y: f64 = 2.5;

/// The analytic solid after the carve: the source less (convex) or plus
/// (concave) the band's section over the tab's tip, x ∈ [2.5, 3.5].
fn model(fam: Fam, verb: Verb, outline: &[(f64, f64)], holes: &[Vec<(f64, f64)>], q: Point3<f64>) -> bool {
    let (x, y, z) = (q.x, q.y, q.z);
    let inb = |h: f64| (0.0..6.0).contains(&x) && (0.0..5.0).contains(&y) && (0.0..h).contains(&z);
    let in_hole = |z0: f64| z > z0 && holes.iter().any(|h| in_poly((x, y), h));
    let section = |u: f64, w: f64| -> bool {
        // u, w ≥ 0: distances from the two supports, in the corner square.
        if !(0.0..D).contains(&u) || !(0.0..D).contains(&w) {
            return false;
        }
        match verb {
            Verb::Chamfer => u + w < D,
            Verb::Fillet => (u - D).powi(2) + (w - D).powi(2) > D * D,
        }
    };
    let along = (TAB_LO..TAB_HI).contains(&x);
    match fam {
        Fam::Pocket => {
            let src = inb(2.0) && !(z > 1.0 && in_poly((x, y), outline)) && !in_hole(1.7);
            src && !(along && section(y - TIP_Y, 2.0 - z))
        }
        Fam::Boss => {
            let src = (inb(1.0) || (z < 2.0 && in_poly((x, y), outline) && (0.0..6.0).contains(&x)))
                && !in_hole(0.7);
            src || (along && section(y - TIP_Y, z - 1.0))
        }
    }
}

#[derive(Debug)]
enum Got {
    Built(String),
    Refused(String),
}

/// Run one fixture, one verb; check a built body every way.
fn row(name: &str, fam: Fam, verb: Verb, outline: &[(f64, f64)], holes: &[Vec<(f64, f64)>]) -> Got {
    let body = build::<f64>(fam, outline, holes);
    validate_geometric(&body, tol()).expect("fixture tier 3");
    let z = match fam {
        Fam::Pocket => 2.0,
        Fam::Boss => 1.0,
    };
    let e: EdgeKey = edge(&body, [3.5, 2.5, z], [2.5, 2.5, z]);
    let got = match verb.run(&body, &[e]) {
        Err(BlendError::SurgeryInvariant { at, detail }) => {
            Got::Refused(format!("SurgeryInvariant at {at:?}: {detail}"))
        }
        Err(other) => Got::Refused(format!("{other:?}").chars().take(300).collect()),
        Ok(out) => {
            let t3 = validate_geometric(&out.body, tol()).map(|_| ()).map_err(|e| format!("{e:?}"));
            let t3p = topo::validate_pseudomanifold(&out.body, &topo::ContactRecords::default(), tol())
                .map_err(|e| format!("{e:?}"));
            let v0 = mass_properties(&body, tol()).unwrap().volume;
            let v1 = mass_properties(&out.body, tol()).unwrap().volume;
            let want = match fam {
                Fam::Pocket => verb.section(),
                Fam::Boss => -verb.section(),
            };
            let dv = v0 - v1;
            let band = Band::linear(tol()).unwrap();
            let (mut n, mut wrong) = (0, Vec::new());
            for i in 0..41 {
                for j in 0..37 {
                    for k in 0..23 {
                        let q = Point3::new(
                            2.3 + 0.0347 * f64::from(i) + 1e-4 * 0.7071,
                            2.2 + 0.0213 * f64::from(j) + 1e-4 * 0.3183,
                            z - 0.31 + 0.0141 * f64::from(k) + 1e-4 * 0.5772,
                        );
                        let want = model(fam, verb, outline, holes, q);
                        let got = topo::boolean::point_in_solid(&out.body, q, band, tol());
                        n += 1;
                        match got {
                            Ok(SolidContainment::In) if want => {}
                            Ok(SolidContainment::Out) if !want => {}
                            other => wrong.push((q, other.map_err(|e| format!("{e:?}")), want)),
                        }
                    }
                }
            }
            Got::Built(format!(
                "tier3 {:?} | tier3' {} | dV {dv:.15} want {want:.15} (err {:.1e}) | membership {}/{} wrong{}",
                t3.err().map(|s| s.chars().take(200).collect::<String>()),
                match t3p {
                    Ok(()) => "ok".to_string(),
                    Err(e) => e.chars().take(300).collect(),
                },
                (dv - want).abs(),
                wrong.len(),
                n,
                wrong.iter().take(3).map(|w| format!(" {w:?}")).collect::<String>()
            ))
        }
    };
    eprintln!("PROBE {name} {fam:?} {verb:?}: {got:?}");
    got
}

fn tab(extra_right: &[(f64, f64)], extra_left: &[(f64, f64)], extra_bottom: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut o = vec![(1.0, 1.0)];
    o.extend_from_slice(extra_bottom);
    o.extend_from_slice(&[(5.0, 1.0), (5.0, 4.0), (3.5, 4.0)]);
    o.extend_from_slice(extra_right);
    o.extend_from_slice(&[(3.5, 2.5), (2.5, 2.5)]);
    o.extend_from_slice(extra_left);
    o.extend_from_slice(&[(2.5, 4.0), (1.0, 4.0)]);
    o
}

fn spike_right(depth: f64) -> Vec<(f64, f64)> {
    vec![(3.5, 3.95), (3.0625, 2.5 + depth), (3.5, 3.9)]
}

fn spike_left(depth: f64) -> Vec<(f64, f64)> {
    vec![(2.5, 3.9), (2.9375, 2.5 + depth), (2.5, 3.95)]
}

/// A peninsula of the face rising from the pocket's far wall, its tip
/// `gap` below the tab's tip, midway between two screen samples.
fn far(gap: f64) -> Vec<(f64, f64)> {
    vec![(3.025, 1.0), (3.0625, 2.5 - gap), (3.1, 1.0)]
}

fn all_rows(fam: Fam) -> Vec<(String, Vec<(f64, f64)>, Vec<Vec<(f64, f64)>>)> {
    let sq = |x0: f64, y0: f64, x1: f64, y1: f64| vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)];
    let mut rows: Vec<(String, Vec<(f64, f64)>, Vec<Vec<(f64, f64)>>)> = vec![
        ("control".into(), tab(&[], &[], &[]), vec![]),
        // A separate hole ring inside the strip, between screen samples.
        ("hole_in_strip".into(), tab(&[], &[], &[]), vec![sq(3.05, 2.585, 3.075, 2.595)]),
        ("hole_in_strip_wide".into(), tab(&[], &[], &[]), vec![sq(2.9, 2.55, 3.2, 2.58)]),
        ("hole_between".into(), tab(&[], &[], &[]), vec![sq(3.055, 2.585, 3.07, 2.595)]),
        ("hole_clear".into(), tab(&[], &[], &[]), vec![sq(2.9, 2.75, 3.2, 2.9)]),
        // Notches in the requested ring by the station.
        ("notch_clear".into(), tab(&[(3.5, 2.9), (3.4, 2.8), (3.5, 2.7)], &[], &[]), vec![]),
        ("notch_in_strip".into(), tab(&[(3.5, 2.59), (3.45, 2.56), (3.5, 2.53)], &[], &[]), vec![]),
        ("notch_in_strip_short".into(), tab(&[(3.5, 2.59), (3.45, 2.505)], &[], &[]), vec![]),
        // Crossing the strip twice: spikes from both walls.
        ("two_spikes_in".into(), tab(&spike_right(0.09), &spike_left(0.09), &[]), vec![]),
        ("two_spikes_clear".into(), tab(&spike_right(D + 0.05), &spike_left(D + 0.05), &[]), vec![]),
        ("left_spike_in".into(), tab(&[], &spike_left(0.09), &[]), vec![]),
        // The far side of the tip's line: a feature of the same ring the
        // band never touches.
        ("far_side_0.09".into(), tab(&[], &[], &far(0.09)), vec![]),
        ("far_side_0.05".into(), tab(&[], &[], &far(0.05)), vec![]),
        ("far_side_0.2".into(), tab(&[], &[], &far(0.2)), vec![]),
    ];
    if fam == Fam::Pocket {
        // The same far-side peninsula on the OUTER cycle: the pocket opened
        // through the block's left side (as on main).
        let mut o = tab(&[], &[], &far(0.09));
        o[0] = (-1.0, 1.0);
        let n = o.len();
        o[n - 1] = (-1.0, 4.0);
        rows.push(("outer_far_side_0.09".into(), o, vec![]));
        let mut o = tab(&spike_right(0.09), &[], &[]);
        o[0] = (-1.0, 1.0);
        let n = o.len();
        o[n - 1] = (-1.0, 4.0);
        rows.push(("outer_spike_in".into(), o, vec![]));
    }
    rows
}

#[test]
fn review_4286_rows() {
    let only = std::env::var("PROBE_ONLY").ok();
    for fam in [Fam::Pocket, Fam::Boss] {
        for (name, outline, holes) in all_rows(fam) {
            if only.as_ref().is_some_and(|o| !name.contains(o.as_str())) {
                continue;
            }
            for verb in [Verb::Chamfer, Verb::Fillet] {
                let r = std::panic::catch_unwind(|| row(&name, fam, verb, &outline, &holes));
                if r.is_err() {
                    eprintln!("PROBE {name} {fam:?} {verb:?}: PANIC");
                }
            }
        }
    }
}

/// The Interval replay of the control, the strip spike and the far-side
/// peninsula, convex and concave.
#[test]
fn review_4286_interval() {
    let t: Tol = tol();
    for fam in [Fam::Pocket, Fam::Boss] {
        for (name, outline) in [
            ("control", tab(&[], &[], &[])),
            ("spike_in", tab(&spike_right(0.09), &[], &[])),
            ("spike_clear", tab(&spike_right(D + 0.05), &[], &[])),
            ("far_side_0.09", tab(&[], &[], &far(0.09))),
        ] {
            let body = finished("iv", build::<Interval>(fam, &outline, &[]), t);
            let z = match fam {
                Fam::Pocket => 2.0,
                Fam::Boss => 1.0,
            };
            let near = |p: &Point3<Interval>, q: [f64; 3]| {
                [p.x, p.y, p.z]
                    .iter()
                    .zip(q)
                    .all(|(c, w)| c.lo() <= w + 1e-12 && w - 1e-12 <= c.hi())
            };
            use geom_core::Bounds;
            let (a, b) = ([3.5, 2.5, z], [2.5, 2.5, z]);
            let e = topo::query::all_edges(&body)
                .into_iter()
                .find(|&e| {
                    let he = body.get_edge(e).unwrap().he_plus;
                    let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                    let (s, t) = (p(body.get_half_edge(he).unwrap().start), p(body.half_edge_end(he).unwrap()));
                    (near(&s, a) && near(&t, b)) || (near(&s, b) && near(&t, a))
                })
                .expect("tip");
            for round in [false, true] {
                let r = if round {
                    sweep::blend::build::fillet_edges(&body, &[e], crate::common::interval::iv(D), t)
                } else {
                    sweep::blend::build::chamfer_edges(&body, &[e], crate::common::interval::iv(D), t)
                };
                let s = match r {
                    Ok(out) => {
                        let v0 = mass_properties(&body, t).unwrap().volume;
                        let v1 = mass_properties(&out.body, t).unwrap().volume;
                        format!(
                            "built tier3 {:?} dV [{:.12}, {:.12}]",
                            validate_geometric(&out.body, t).is_ok(),
                            (v0 - v1).lo(),
                            (v0 - v1).hi()
                        )
                    }
                    Err(e) => format!("{:?}", e.error).chars().take(200).collect(),
                };
                eprintln!("PROBE-IV {name} {fam:?} round={round}: {s}");
            }
        }
    }
}

/// Claim 3: a plane supporting a RULED link (the mouth line of a through
/// half-cylinder groove along x) that also carries a ring with a planar
/// requested edge (the tabbed pocket's tip). `spike` puts a narrow spike
/// of the pocket ring toward the groove's mouth, tip `0.09` short of it,
/// between the mouth edge's screen samples (0.75 apart).
fn grooved(spike: bool) -> Body<f64> {
    let mut o = tab(&[], &[], &[]);
    if spike {
        // After (2.5, 4.0), (1.0, 4.0): splice a spike on the y = 4 edge
        // between x = 1.0 and 2.5, going right to left.
        let n = o.len();
        o.splice(n - 1..n - 1, [(1.9, 4.0), (1.875, 4.35), (1.85, 4.0)]);
    }
    let pocket = build::<f64>(Fam::Pocket, &o, &[]);
    let plane = sweep::test_support::sketch_from_axes(
        Point3::new(-0.1, 4.6, 2.0),
        geom_core::Vec3::new(0.0, 1.0, 0.0),
        geom_core::Vec3::new(0.0, 0.0, 1.0),
        tol(),
    );
    let pin = sweep::test_support::prism_on(
        plane,
        vec![(Point2::new(-0.2, 0.0), 1.0), (Point2::new(0.2, 0.0), 1.0)],
        6.2,
        tol(),
    );
    sweep::test_support::realized(BooleanOp::Subtract, &pocket, &pin, tol())
}

#[test]
fn review_4286_ruled() {
    for spike in [false, true] {
        let body = grooved(spike);
        validate_geometric(&body, tol()).expect("grooved tier 3");
        let mouth = edge(&body, [0.0, 4.4, 2.0], [6.0, 4.4, 2.0]);
        let tip = edge(&body, [3.5, 2.5, 2.0], [2.5, 2.5, 2.0]);
        for (what, req) in [("mouth", vec![mouth]), ("tip", vec![tip]), ("both", vec![mouth, tip])] {
            for verb in [Verb::Chamfer, Verb::Fillet] {
                let s = match verb.run(&body, &req) {
                    Ok(out) => format!(
                        "built tier3 {:?} tier3' {:?}",
                        validate_geometric(&out.body, tol()).is_ok(),
                        topo::validate_pseudomanifold(&out.body, &topo::ContactRecords::default(), tol())
                            .map_err(|e| format!("{e:?}").chars().take(200).collect::<String>())
                    ),
                    Err(e) => format!("{e:?}").chars().take(220).collect(),
                };
                eprintln!("PROBE-RULED spike={spike} {what} {verb:?}: {s}");
            }
        }
    }
}

#[test]
fn review_4286_ruled_debug() {
    let body = grooved(false);
    let tip = edge(&body, [3.5, 2.5, 2.0], [2.5, 2.5, 2.0]);
    let ed = body.get_edge(tip).unwrap();
    for he in [ed.he_plus, ed.he_minus] {
        let h = body.get_half_edge(he).unwrap();
        let l = body.get_loop(h.parent_loop).unwrap();
        let f = body.get_face(l.face).unwrap();
        eprintln!(
            "DBG he {he:?} loop {:?} face {:?} outer {:?} rings {:?} surface {:?}",
            h.parent_loop, l.face, f.outer, f.rings, body.get_surface(f.surface).map(|s| format!("{s:?}").chars().take(90).collect::<String>())
        );
    }
    for (k, f) in body.faces() {
        eprintln!("DBG face {k:?} rings {}", f.rings.len());
    }
}

/// The filed sibling row: a BLIND half-cylinder groove in a block's top
/// face, so its mouth lines lie in a ring of the plane; fillet one.
#[test]
fn review_4286_blind_groove() {
    let base = block::<f64>(6.0, 5.0, 2.0, tol());
    let plane = sweep::test_support::sketch_from_axes(
        Point3::new(1.0, 2.5, 2.0),
        geom_core::Vec3::new(0.0, 1.0, 0.0),
        geom_core::Vec3::new(0.0, 0.0, 1.0),
        tol(),
    );
    let pin = sweep::test_support::prism_on(
        plane,
        vec![(Point2::new(-0.2, 0.0), 1.0), (Point2::new(0.2, 0.0), 1.0)],
        4.0,
        tol(),
    );
    let body = sweep::test_support::realized(BooleanOp::Subtract, &base, &pin, tol());
    validate_geometric(&body, tol()).expect("blind groove tier 3");
    let mouth = edge(&body, [1.0, 2.3, 2.0], [5.0, 2.3, 2.0]);
    let s = match Verb::Fillet.run(&body, &[mouth]) {
        Ok(out) => format!("built tier3 {:?}", validate_geometric(&out.body, tol()).is_ok()),
        Err(e) => format!("{e:?}").chars().take(260).collect(),
    };
    eprintln!("PROBE-BLIND fillet mouth: {s}");
}
