//! **The general-pose cone rows against a traced section** (a
//! counterexample search, shape 1): a cone against an oblique cylinder
//! and against a tilted or parallel-axis cone, at scales from 1 m to
//! 1 km, each pose generic or built at a signed offset from one
//! degeneracy — the apex on the partner, a tangency of the carriers at a
//! chosen point, a generator along the partner's asymptotic directions —
//! the offset log-uniform from `1e-12` of the scale up to half of it.
//!
//! The oracle traces the partner's residual on each carrier's own
//! ruling chart — azimuth periodic, the signed distance along the line
//! compactified as `L·tan ψ`, so a component reaching the window's edge
//! runs to infinity — counts its components by cell adjacency, and
//! classes each: touching the edge, unbounded; else essential iff it
//! crosses the seam an odd number of times. It shares nothing with the
//! arm but the carriers. `classify` must agree with 0 mismatches:
//!
//! - on the cone's chart, part for part in count and class
//!   (`essential_f`), and on the partner's chart the same
//!   (`essential_g`);
//! - `single` only on one traced component;
//! - every witness on both carriers, and on a traced component of its
//!   part's class — distinct components for distinct parts;
//! - the swapped order the same classification.
//!
//! Below the trace's resolution (`0.03` of the scale) the class is not
//! traced: an offset inside the band's Zero (a quarter of it) must
//! refuse R-tan; one between must answer — R-tan only within a few
//! escalation widths — with every witness on both carriers. A generic
//! pose (no degeneracy built in) is traced at every draw. Any other
//! refusal is counted and printed, never a mismatch.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

test_utils::gated_to![
    "crates/topo/src/boolean/section_cert.rs",
    "crates/topo/src/boolean/section_cert/ruling.rs",
    "crates/topo/src/boolean/circle_roots.rs",
    "crates/geom-brep/src/implicit.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/linalg/",
];

use super::cone_search::{Class, Frame, Traced, band, offset, random_cone, trace, unit_vec};
use super::*;
use core::f64::consts::{FRAC_PI_2, TAU};
use geom::Surface;
use geom_core::{Point3, Vec3};
use test_utils::fuzz::{self, Rng};

/// How close to `±π/2` the compactified chart runs: `L·tan ψ` reaches
/// `10⁴ L`.
const EDGE: f64 = 1e-4;

/// The section traced on the ruling chart `line(θ) = (B, v)`, the
/// distance along `v` compactified as `scale·tan ψ`.
fn ruled(
    line: &dyn Fn(f64) -> (Point3<f64>, Vec3<f64>),
    scale: f64,
    partner: &Surface<f64>,
) -> Traced {
    trace(
        &|u, psi| {
            let (b, v) = line(u);
            b + v * (scale * psi.tan())
        },
        &|q| geom_brep::implicit_residual(partner, q),
        (-FRAC_PI_2 + EDGE, FRAC_PI_2 - EDGE),
        720,
        721,
    )
}

/// A ruling chart: the line at each angle, as `(B, v)`.
type Lines = Box<dyn Fn(f64) -> (Point3<f64>, Vec3<f64>)>;

/// A partner's ruling chart and its carrier.
struct Partner {
    surface: Surface<f64>,
    line: Lines,
    /// The chart point `(θ, ψ)` of a point on the carrier.
    chart: Box<dyn Fn(Point3<f64>) -> (f64, f64)>,
}

fn cylinder(o: Point3<f64>, d: Vec3<f64>, r: f64, scale: f64) -> Partner {
    let (e1, e2) = d.orthonormal_basis();
    Partner {
        surface: Surface::Cylinder {
            origin: o,
            axis: d,
            radius: r,
            u_ref: e1,
        },
        line: Box::new(move |u| (o + (e1 * u.cos() + e2 * u.sin()) * r, d)),
        chart: Box::new(move |q| {
            let w = q - o;
            ((w.dot(e2)).atan2(w.dot(e1)), (w.dot(d) / scale).atan())
        }),
    }
}

fn cone(k: Frame) -> Partner {
    let surface = k.surface();
    let scale = k.scale;
    let k = std::rc::Rc::new(k);
    let (k1, k2) = (k.clone(), k);
    Partner {
        surface,
        line: Box::new(move |u| {
            let (s, c) = k1.alpha.sin_cos();
            (k1.apex, k1.a * c + k1.radial(u) * s)
        }),
        chart: Box::new(move |q| {
            let (u, t) = k2.chart(q);
            (u, (t / scale).atan())
        }),
    }
}

fn frame(apex: Point3<f64>, a: Vec3<f64>, alpha: f64, scale: f64) -> Frame {
    let (e1, e2) = a.orthonormal_basis();
    Frame {
        scale,
        apex,
        a,
        e1,
        e2,
        alpha,
    }
}

struct Pose {
    what: String,
    partner: Partner,
    /// The built degeneracy's margin in metres, `None` for a generic pose.
    margin: Option<f64>,
}

fn sign(rng: &mut Rng) -> f64 {
    if rng.unit() < 0.5 { 1.0 } else { -1.0 }
}

/// A unit direction at angle `beta` from `a`.
fn leaning(rng: &mut Rng, k: &Frame, beta: f64) -> Vec3<f64> {
    k.a * beta.cos() + k.radial(rng.range(0.0, TAU)) * beta.sin()
}

/// A point on the cone `t` along the generator at a random azimuth, and
/// the carrier's unit normal there.
fn on_cone(rng: &mut Rng, k: &Frame) -> (Point3<f64>, Vec3<f64>) {
    let t = k.scale * rng.range(0.3, 1.5) * sign(rng);
    let q = k.at(rng.range(0.0, TAU), t);
    let n = geom_brep::implicit_gradient(&k.surface(), q);
    (q, n / n.norm())
}

fn generic_cylinder(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let beta = rng.range(0.1, FRAC_PI_2);
    let d = leaning(rng, k, beta);
    if (d.dot(k.a).acos() - k.alpha).abs() < 0.05 {
        return None;
    }
    let o = k.apex + unit_vec(rng) * (sc * rng.range(0.0, 1.5));
    let r = sc * rng.range(0.1, 1.5);
    Some(Pose {
        what: format!("oblique cylinder o {o:?} d {d:?} r {r}"),
        partner: cylinder(o, d, r, sc),
        margin: None,
    })
}

fn cylinder_near_apex(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let beta = rng.range(0.1, FRAC_PI_2);
    let d = leaning(rng, k, beta);
    let o = k.apex + unit_vec(rng) * (sc * rng.range(0.2, 1.5));
    let dist = (k.apex - o).cross(d).norm();
    let shift = sc * offset(rng);
    let r = dist + shift;
    (r > 0.05 * sc).then(|| Pose {
        what: format!("cylinder near the apex o {o:?} d {d:?} r {r}"),
        partner: cylinder(o, d, r, sc),
        margin: Some(shift),
    })
}

fn cylinder_near_tangent(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let (q, n) = on_cone(rng, k);
    let d = n.cross(unit_vec(rng));
    if d.norm() < 0.2 {
        return None;
    }
    let d = d / d.norm();
    let r = sc * rng.range(0.1, 1.5);
    let shift = sc * offset(rng);
    let side = sign(rng);
    let o = q + n * (side * r + shift);
    Some(Pose {
        what: format!("cylinder tangent at {q:?} side {side} d {d:?} r {r}"),
        partner: cylinder(o, d, r, sc),
        margin: Some(shift),
    })
}

fn cylinder_near_aperture(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let lean = offset(rng) * 0.2;
    let d = leaning(rng, k, k.alpha + lean);
    let o = k.apex + unit_vec(rng) * (sc * rng.range(0.0, 1.5));
    let r = sc * rng.range(0.1, 1.5);
    let lever = 4.0 * reach_of(k).radius;
    Some(Pose {
        what: format!("cylinder near the aperture (lean {lean}) o {o:?} d {d:?} r {r}"),
        partner: cylinder(o, d, r, sc),
        margin: Some(lean * lever * k.alpha.sin()),
    })
}

fn generic_cone(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let a2 = unit_vec(rng);
    let apex = k.apex + unit_vec(rng) * (sc * rng.range(0.2, 2.0));
    let alpha = rng.range(0.25, 1.2);
    Some(Pose {
        what: format!("tilted cone apex {apex:?} a {a2:?} α {alpha}"),
        partner: cone(frame(apex, a2, alpha, sc)),
        margin: None,
    })
}

fn parallel_cone(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let alpha = rng.range(0.25, 1.2);
    if (alpha.tan() - k.alpha.tan()).abs() < 0.1 {
        return None;
    }
    let apex = k.apex
        + k.radial(rng.range(0.0, TAU)) * (sc * rng.range(0.1, 1.5))
        + k.a * (sc * rng.range(-1.5, 1.5));
    let a2 = k.a * sign(rng);
    Some(Pose {
        what: format!("parallel cone apex {apex:?} a {a2:?} α {alpha}"),
        partner: cone(frame(apex, a2, alpha, sc)),
        margin: None,
    })
}

fn cone_near_apex(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let (q, n) = on_cone(rng, k);
    let shift = sc * offset(rng);
    let apex = q + n * shift;
    let alpha = rng.range(0.25, 1.2);
    let a2 = unit_vec(rng);
    Some(Pose {
        what: format!("cone near the apex {apex:?} a {a2:?} α {alpha}"),
        partner: cone(frame(apex, a2, alpha, sc)),
        margin: Some(shift * 0.5),
    })
}

fn cone_near_tangent(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let (q, n) = on_cone(rng, k);
    let g = n.cross(unit_vec(rng));
    if g.norm() < 0.2 {
        return None;
    }
    let g = g / g.norm();
    let alpha = rng.range(0.25, 1.2);
    let a2 = g * alpha.cos() + n * (alpha.sin() * sign(rng));
    let shift = sc * offset(rng);
    let apex = q - g * (sc * rng.range(0.3, 2.0) * sign(rng)) + n * shift;
    Some(Pose {
        what: format!("cone tangent at {q:?}, apex {apex:?} a {a2:?} α {alpha}"),
        partner: cone(frame(apex, a2, alpha, sc)),
        margin: Some(shift * 0.5),
    })
}

fn cone_near_asymptote(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let alpha = rng.range(0.25, 1.2);
    let lean = offset(rng) * 0.2;
    let beta = if rng.unit() < 0.5 {
        k.alpha + alpha
    } else {
        (k.alpha - alpha).abs()
    } + lean;
    if !(0.05..core::f64::consts::PI - 0.05).contains(&beta) {
        return None;
    }
    let a2 = leaning(rng, k, beta);
    let apex = k.apex + unit_vec(rng) * (sc * rng.range(0.2, 2.0));
    let lever = 4.0 * reach_of(k).radius;
    Some(Pose {
        what: format!("cone near the asymptote (lean {lean}) apex {apex:?} a {a2:?} α {alpha}"),
        partner: cone(frame(apex, a2, alpha, sc)),
        margin: Some(lean * lever * 0.05),
    })
}

/// Whether the trace resolves the pose. Every asymptotic direction the
/// two carriers come near sharing is `0.05` rad clear of being shared,
/// so a bounded component stays well inside the window (near one, a
/// loop runs out to `~1/gap` scales, past the compactified chart's
/// resolution, and reads as reaching its edge); and each apex is `0.05`
/// of the scale off the other carrier, so a loop about it is not
/// narrower than the other chart's cells.
fn traceable(k: &Frame, partner: &Surface<f64>) -> bool {
    let clear = |surface: &Surface<f64>, q: Point3<f64>| {
        geom_brep::implicit_residual(surface, q).abs() > 0.05 * k.scale
    };
    if !clear(partner, k.apex) {
        return false;
    }
    match *partner {
        Surface::Cylinder { axis, .. } => (axis.dot(k.a).abs().acos() - k.alpha).abs() > 0.05,
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            if !clear(&k.surface(), apex) {
                return false;
            }
            let beta = axis.dot(k.a).clamp(-1.0, 1.0).acos();
            [beta, core::f64::consts::PI - beta].iter().all(|&b| {
                (b - (k.alpha + half_angle)).abs() > 0.05
                    && (b - (k.alpha - half_angle).abs()).abs() > 0.05
            })
        }
        _ => true,
    }
}

/// The reach every search pose is classified with.
fn reach_of(k: &Frame) -> Reach<f64> {
    Reach {
        centre: k.apex,
        radius: 5.0 * k.scale,
    }
}

fn class_on(c: &Component<f64>, essential: bool) -> Class {
    match (c.unbounded, essential) {
        (true, _) => Class::Unbounded,
        (false, true) => Class::Essential,
        (false, false) => Class::Null,
    }
}

/// What one pose came to.
enum Outcome {
    Answered,
    Refused(&'static str),
}

/// The mismatches of one pose, as sentences.
#[allow(clippy::too_many_lines)] // the checks, each labelled
fn check(k: &Frame, pose: &Pose) -> (Outcome, Vec<String>) {
    let cone = k.surface();
    let partner = &pose.partner.surface;
    let reach = reach_of(k);
    let b = band();
    let sec = classify(&cone, partner, reach, b);
    let back = classify(partner, &cone, reach, b);
    let m = pose.margin.map_or(f64::INFINITY, f64::abs);
    let mut bad = Vec::new();
    let (parts, single) = match &sec {
        Section::Components { parts, single } => (parts.clone(), *single),
        Section::Tangent(name) => {
            if !matches!(back, Section::Tangent(_)) {
                bad.push(format!("one order R-tan, the other {back:?}"));
            }
            return (Outcome::Refused(name), bad);
        }
        other => return (Outcome::Refused("other"), vec![format!("{other:?}")]),
    };
    if m <= 0.25 * b.zero() {
        bad.push(format!("margin {m} in the band, answered {sec:?}"));
    }
    match &back {
        Section::Components {
            parts: bp,
            single: sb,
        } if *sb == single
            && bp.len() == parts.len()
            && bp.iter().zip(&parts).all(|(x, y)| {
                (x.unbounded, x.essential_f, x.essential_g)
                    == (y.unbounded, y.essential_g, y.essential_f)
            }) => {}
        other => bad.push(format!("the swapped order disagrees: {other:?}")),
    }
    for c in &parts {
        let Some(w) = c.witness else {
            if !c.unbounded {
                bad.push("a bounded part without a witness".into());
            }
            continue;
        };
        for surf in [&cone, partner] {
            let r = geom_brep::implicit_residual(surf, w);
            if r.abs() > 1e-9 * k.scale {
                bad.push(format!("the witness {w:?} is {r} off {surf:?}"));
            }
        }
    }
    if m < 0.03 * k.scale || !traceable(k, partner) {
        return (Outcome::Answered, bad);
    }
    let (s, c) = k.alpha.sin_cos();
    let mine = ruled(&|u| (k.apex, k.a * c + k.radial(u) * s), k.scale, partner);
    let theirs = ruled(&*pose.partner.line, k.scale, &cone);
    for (side, traced, flag) in [("cone", &mine, true), ("partner", &theirs, false)] {
        let mut want = traced.classes.clone();
        want.sort();
        let mut got: Vec<Class> = parts
            .iter()
            .map(|p| class_on(p, if flag { p.essential_f } else { p.essential_g }))
            .collect();
        got.sort();
        if want != got {
            bad.push(format!(
                "on the {side}'s chart traced {want:?}, classified {got:?}"
            ));
        }
        let mut seen = Vec::new();
        for p in &parts {
            let Some(w) = p.witness else {
                continue;
            };
            let (u, psi) = if flag {
                let (u, t) = k.chart(w);
                (u, (t / k.scale).atan())
            } else {
                (pose.partner.chart)(w)
            };
            let want = class_on(p, if flag { p.essential_f } else { p.essential_g });
            // The traced chart's `v` is `ψ`; `near` reads it directly.
            let on: Vec<usize> = traced
                .near(u, psi)
                .into_iter()
                .filter(|&i| traced.classes[i] == want && !seen.contains(&i))
                .collect();
            match on.first() {
                Some(&i) => seen.push(i),
                None => bad.push(format!(
                    "on the {side}'s chart the witness {w:?} of a {want:?} part is on no such traced component"
                )),
            }
        }
    }
    if single && mine.classes.len() != 1 {
        bad.push(format!("single, traced {:?}", mine.classes));
    }
    (Outcome::Answered, bad)
}

type Poser = fn(&mut Rng, &Frame) -> Option<Pose>;

/// **Every general-pose cone row against the traced section**, 0
/// mismatches.
#[test]
fn the_general_pose_cone_rows_agree_with_the_traced_section() {
    let mut rng = fuzz::start("section_cert_cone_pair_search");
    let arms: [(&str, Poser); 9] = [
        ("oblique cylinder", generic_cylinder),
        ("cylinder near the apex", cylinder_near_apex),
        ("cylinder near a tangency", cylinder_near_tangent),
        ("cylinder near the aperture", cylinder_near_aperture),
        ("tilted cone", generic_cone),
        ("parallel cone", parallel_cone),
        ("cone near an apex", cone_near_apex),
        ("cone near a tangency", cone_near_tangent),
        ("cone near the asymptote", cone_near_asymptote),
    ];
    let mut failures = Vec::new();
    for (name, poser) in arms {
        let mut tally = std::collections::BTreeMap::<String, usize>::new();
        let mut done = 0;
        while done < fuzz::scaled(12) {
            let k = random_cone(&mut rng);
            let Some(pose) = poser(&mut rng, &k) else {
                continue;
            };
            done += 1;
            let (outcome, bad) = check(&k, &pose);
            let key = match outcome {
                Outcome::Answered => "answered".to_owned(),
                Outcome::Refused(row) => format!("R-tan {row}"),
            };
            *tally.entry(key).or_default() += 1;
            for b in bad {
                failures.push(format!(
                    "{name}: cone apex {:?} axis {:?} α {} scale {}; {}, margin {:?}: {b}",
                    k.apex, k.a, k.alpha, k.scale, pose.what, pose.margin
                ));
            }
        }
        println!("[cone pair search] {name}: {tally:?}");
    }
    assert!(
        failures.is_empty(),
        "{} mismatches ({}):\n{}",
        failures.len(),
        fuzz::replay(),
        failures.join("\n")
    );
}

fn json_point(p: Point3<f64>) -> String {
    format!("[{:e}, {:e}, {:e}]", p.x, p.y, p.z)
}

fn json_vec(v: Vec3<f64>) -> String {
    format!("[{:e}, {:e}, {:e}]", v.x, v.y, v.z)
}

fn json_surface(s: &Surface<f64>) -> String {
    match *s {
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => format!(
            r#"{{"kind": "cone", "apex": {}, "axis": {}, "half_angle": {half_angle:e}}}"#,
            json_point(apex),
            json_vec(axis)
        ),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => format!(
            r#"{{"kind": "cylinder", "origin": {}, "axis": {}, "radius": {radius:e}}}"#,
            json_point(origin),
            json_vec(axis)
        ),
        _ => unreachable!("the search's partners are cylinders and cones"),
    }
}

/// One pose as the oracle reads it: the family, the scale, the cone,
/// the partner and `classify`'s answer, as JSON.
pub(super) fn dump_line(
    family: &str,
    scale: f64,
    cone: &Surface<f64>,
    partner: &Surface<f64>,
    sec: &Section<f64>,
) -> String {
    let answer = match sec {
        Section::Components { parts, single } => {
            let parts: Vec<String> = parts
                .iter()
                .map(|p| {
                    format!(
                        r#"{{"unbounded": {}, "ef": {}, "eg": {}, "witness": {}}}"#,
                        p.unbounded,
                        p.essential_f,
                        p.essential_g,
                        p.witness.map_or_else(|| "null".to_owned(), json_point)
                    )
                })
                .collect();
            format!(r#"{{"parts": [{}], "single": {single}}}"#, parts.join(", "))
        }
        Section::Tangent(name) => format!("\"R-tan {name}\""),
        other => format!("\"{}\"", format!("{other:?}").replace('"', "")),
    };
    format!(
        r#"{{"family": "{family}", "scale": {scale:e}, "cone": {}, "partner": {}, "answer": {answer}}}"#,
        json_surface(cone),
        json_surface(partner)
    )
}

/// **The draws, for the high-precision oracle.** Prints one JSON line
/// per pose, tagged `CPDUMP`: the family, the scale, the cone, the
/// partner and `classify`'s answer.
/// `scripts/oracles/cone_pair_sections_mpmath.py` re-reads each
/// answered pose's section at 50 digits on both carriers' charts and
/// checks the counts, the classes, every witness and `single`. Run:
/// `cargo nextest run -p topo --lib --run-ignored only
/// dump_for_the_mpmath_oracle --no-capture | sed -n 's/^CPDUMP //p' >
/// /tmp/cp.jsonl`, then
/// `python3 scripts/oracles/cone_pair_sections_mpmath.py /tmp/cp.jsonl`.
#[test]
#[ignore = "an oracle dump; run command in the docs"]
fn dump_for_the_mpmath_oracle() {
    let mut rng = fuzz::start("section_cert_cone_pair_search::dump_for_the_mpmath_oracle");
    let arms: [(&str, Poser); 9] = [
        ("oblique cylinder", generic_cylinder),
        ("cylinder near the apex", cylinder_near_apex),
        ("cylinder near a tangency", cylinder_near_tangent),
        ("cylinder near the aperture", cylinder_near_aperture),
        ("tilted cone", generic_cone),
        ("parallel cone", parallel_cone),
        ("cone near an apex", cone_near_apex),
        ("cone near a tangency", cone_near_tangent),
        ("cone near the asymptote", cone_near_asymptote),
    ];
    for (name, poser) in arms {
        let mut done = 0;
        while done < fuzz::scaled(10) {
            let k = random_cone(&mut rng);
            let Some(pose) = poser(&mut rng, &k) else {
                continue;
            };
            done += 1;
            let sec = classify(&k.surface(), &pose.partner.surface, reach_of(&k), band());
            println!(
                "CPDUMP {}",
                dump_line(name, k.scale, &k.surface(), &pose.partner.surface, &sec)
            );
        }
    }
}
