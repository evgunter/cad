//! Verifier probes for PR 4839 at bd8fda83 (review/4839-verify).
use super::*;
use geom_core::Tol;

fn chan(c0: f64, ca: f64, cb: f64, cl: f64) -> HarmChan {
    HarmChan { c0: pt(c0), ca: pt(ca), cb: pt(cb), cl: pt(cl) }
}
fn line(u: HarmChan, v: HarmChan, span: f64, forward: bool) -> TrimEdgeQ {
    TrimEdgeQ { u, v, t0: pt(0.0), t1: pt(span), forward, trig0: (pt(1.0), pt(0.0)), trig1: None, env: pt(0.0) }
}
fn both(loops: &[&[TrimEdgeQ]]) -> [(&'static str, Result<FaceCutBounds, PropsError>); 2] {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let eps = Tol::witness().get().eps;
    [
        ("f64", cylinder_cut_face::<f64>(pt(1.0), Interval::zero(), loops, eps, band)),
        ("Interval", cylinder_cut_face::<Interval>(pt(1.0), Interval::zero(), loops, eps, band)),
    ]
}
fn show(r: &Result<FaceCutBounds, PropsError>, exact: f64) -> String {
    match r {
        Ok(b) => format!(
            "OK area=[{:.17e},{:.17e}] contains_exact={} err_over_halfwidth={:.3}",
            b.area.lo(), b.area.hi(), b.area.lo() <= exact && exact <= b.area.hi(),
            ((b.area.lo() + b.area.hi()) * 0.5 - exact).abs() / (b.area.width() * 0.5)
        ),
        Err(PropsError::Escalated { cause, check }) => format!("Escalated {check:?} {:?}", cause.predicate),
        Err(e) => format!("{e:?}"),
    }
}

/// Rectangle [0,2]×[0,1] with its top stored at 1 + d: two junctions open by d.
#[test]
fn probe_rectangle_openings_near_slack() {
    let eps = Tol::witness().get().eps;
    for mult in [0.0, 1.0, 2.0, 3.0, 3.9, 4.0, 4.5, 5.0, 10.0, 12.9, 13.5, 14.0, 20.0, 100.0] {
        let d = mult * eps;
        let outer = [
            line(chan(0.0, 0.0, 0.0, 1.0), chan(0.0, 0.0, 0.0, 0.0), 2.0, true),
            line(chan(2.0, 0.0, 0.0, 0.0), chan(0.0, 0.0, 0.0, 1.0), 1.0, true),
            line(chan(0.0, 0.0, 0.0, 1.0), chan(1.0 + d, 0.0, 0.0, 0.0), 2.0, false),
            line(chan(0.0, 0.0, 0.0, 0.0), chan(0.0, 0.0, 0.0, 1.0), 1.0, false),
        ];
        for (s, r) in both(&[&outer]) {
            eprintln!("PROBE-OPEN-V eps={eps:e} d={mult}eps {s}: {}", show(&r, 2.0));
        }
    }
}

/// The ellipse band, 30 m tall, a km up, its top rim stopping d short of
/// a whole turn: one junction open by d in u, at height ~15 m off the anchor.
#[test]
fn probe_band_open_in_u_far_up() {
    let eps = Tol::witness().get().eps;
    let tau = core::f64::consts::TAU;
    for lift in [0.0, 1000.0] {
        for mult in [0.0, 2.0, 4.0, 10.0, 14.0, 100.0] {
            let d = mult * eps;
            let bottom = TrimEdgeQ { u: chan(0.0, 0.0, 0.0, 1.0), v: chan(lift, 0.0, 0.0, 0.0), t0: pt(0.0), t1: pt(tau), forward: true, trig0: (pt(1.0), pt(0.0)), trig1: None, env: pt(0.0) };
            let top = TrimEdgeQ { u: chan(0.0, 0.0, 0.0, 1.0 - d / tau), v: chan(lift + 30.0, 0.4, 0.0, 0.0), t0: pt(0.0), t1: pt(tau), forward: false, trig0: (pt(1.0), pt(0.0)), trig1: None, env: pt(0.0) };
            for (s, r) in both(&[&[bottom], &[top]]) {
                eprintln!("PROBE-OPEN-U eps={eps:e} lift={lift} d={mult}eps {s}: {}", show(&r, 30.0 * tau));
            }
        }
    }
}

/// The junction decide reads the gap's LOWER bound: a side edge whose
/// end level is a wide enclosure (a whole-circle trig bracket, as a
/// line edge's is, with a cos amplitude a) reads closed against a top
/// stored g above where the side truly ends, for any g < ~a.
#[test]
fn probe_lo_read_admits_a_gap_inside_the_enclosure() {
    let eps = Tol::witness().get().eps;
    let whole = (Interval::from_bounds(-1.0, 1.0), Interval::from_bounds(-1.0, 1.0));
    for a in [3.5e-10, 1e-6, 1e-3] {
        for frac in [0.0, 0.3, 0.45] {
            let g = frac * a;
            // v(t) = -a + a cos t + c t, with v(1) = 1 exactly in truth.
            let c = 1.0 - a * (1.0_f64.cos() - 1.0);
            let side = |u0: f64, fwd: bool| TrimEdgeQ { u: chan(u0, 0.0, 0.0, 0.0), v: chan(-a, a, 0.0, c), t0: pt(0.0), t1: pt(1.0), forward: fwd, trig0: whole, trig1: None, env: pt(0.0) };
            let outer = [
                line(chan(0.0, 0.0, 0.0, 1.0), chan(0.0, 0.0, 0.0, 0.0), 2.0, true),
                side(2.0, true),
                line(chan(0.0, 0.0, 0.0, 1.0), chan(1.0 + g, 0.0, 0.0, 0.0), 2.0, false),
                side(0.0, false),
            ];
            for (s, r) in both(&[&outer]) {
                eprintln!("PROBE-LO a={a:e} g={g:e} (g/eps={:.1}) {s}: {}", g / eps, show(&r, 2.0));
            }
        }
    }
}

/// A rim winding +1 that is split across two edges with a 2π jump at one
/// junction and a matching -2π at the other (net 0 turns from the junctions),
/// alongside the matching rim: the loop really winds once but Σm says... probe.
#[test]
fn probe_turns_split_rim() {
    let tau = core::f64::consts::TAU;
    let pi = core::f64::consts::PI;
    // bottom rim winds +1 as two half-turn edges stored on the chart [0,2π)
    let h = |u0: f64, v: f64, fwd: bool| TrimEdgeQ { u: chan(u0, 0.0, 0.0, 1.0), v: chan(v, 0.0, 0.0, 0.0), t0: pt(0.0), t1: pt(pi), forward: fwd, trig0: (pt(1.0), pt(0.0)), trig1: None, env: pt(0.0) };
    let bottom = [h(0.0, 0.0, true), h(pi, 0.0, true)];
    let top = [h(pi, 2.0, false), h(0.0, 2.0, false)];
    for (s, r) in both(&[&bottom, &top]) {
        eprintln!("PROBE-SPLIT {s}: {}", show(&r, 2.0 * tau));
    }
    // same but second bottom half stored a period up (u0 = 3π): still one turn.
    let bottom2 = [h(0.0, 0.0, true), h(3.0 * pi, 0.0, true)];
    for (s, r) in both(&[&bottom2, &top]) {
        eprintln!("PROBE-SPLIT-SHIFT {s}: {}", show(&r, 2.0 * tau));
    }
}
