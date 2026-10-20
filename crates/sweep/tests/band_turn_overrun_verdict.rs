//! **A turn's verdict** (`work/band/a-non-isosceles-turn-overruns-past-the-mitre.md`,
//! step 1): two requested edges at a trivalent plane–plane vertex whose
//! third edge `L` is unrequested read two closed-form lengths, both
//! signed by which band's foot on `L` is further — the gap between the
//! feet (`fillet3_turn_feet_gap`) and the overrun's chord from the
//! mitre's end to the far foot (`fillet3_turn_overrun_chord`). Both Zero
//! build the mitre on `L`; both definite are the overrun, which refuses
//! naming the far band; anything else escalates.
//!
//! The readings are held to the probe's closed forms
//! (`band_turn_overrun_probe`), which share no code with the battery
//! past the cone.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Bounds, Decide, Interval, Point2, Point3, Vec3};
use sweep::blend::battery::{BlendRequest, TurnRunOut, run_battery_for};
use sweep::blend::{BlendDecision, BlendError, BlendKind};
use sweep::test_support::{
    prism_on, realized, sketch_from_axes, turn_readings, turn_readings_interval,
};
use topo::boolean::BooleanOp;
use topo::{Body, EdgeKey};

use crate::band_planar_cut_off::{D, edge, tol, volume};
use crate::band_turn_overrun_probe::{
    Cone, V, Verb, across, add, cross, dot, measure, mul, norm, sub, unit,
};
use crate::common::operands::{
    edge_between, leaning_turn, leaning_turn_at, parallelepiped, parallelepiped_at,
};

fn point(v: V) -> Point3<f64> {
    Point3::new(v[0], v[1], v[2])
}

fn vector(v: V) -> Vec3<f64> {
    Vec3::new(v[0], v[1], v[2])
}

/// **The cone as a body**: a prism along `L` over the wedge between
/// `L`'s two faces, cut by the shared face's plane through the vertex
/// (the origin), and the two requested edges there, `e₁` then `e₂`.
/// Its faces stand `R` from the vertex, wide of every band at `D`.
pub(crate) fn corner(c: &Cone) -> (Body<f64>, [EdgeKey; 2]) {
    const R: f64 = 10.0;
    let l = c.l;
    let ray = [across(c.e[0], l), across(c.e[1], l)];
    // The wedge's first ray is the one the other turns from
    // counter-clockwise about `L`, so the outline winds positively.
    let first = usize::from(dot(cross(ray[0], ray[1]), l) < 0.0);
    let u = ray[first];
    let v = cross(l, u);
    let other = ray[1 - first];
    let g = dot(other, v).atan2(dot(other, u));
    let (a, b, m) = (
        (1.0, 0.0),
        (g.cos(), g.sin()),
        ((g / 2.0).cos(), (g / 2.0).sin()),
    );
    let outline = [
        (0.0, 0.0),
        a,
        (a.0 + m.0, a.1 + m.1),
        (b.0 + m.0, b.1 + m.1),
        b,
    ];
    // A prism on the plane through `o` spanned by `u` and `v`, along
    // `u × v`.
    let prism = |o: V, u: V, v: V, outline: &[(f64, f64)], h: f64| {
        prism_on(
            sketch_from_axes(point(o), vector(u), vector(v), tol()),
            outline
                .iter()
                .map(|&(x, y)| (Point2::new(x * R, y * R), 0.0))
                .collect(),
            h * R,
            tol(),
        )
    };
    let wedge = prism(mul(l, -R), u, v, &outline, 2.0);
    // The material side of the shared face: a block on its plane.
    let (su, sv) = (c.e[0], cross(c.n_s, c.e[0]));
    let shared = prism(
        mul(add(su, sv), -4.0 * R),
        su,
        sv,
        &[(0.0, 0.0), (8.0, 0.0), (8.0, 8.0), (0.0, 8.0)],
        8.0,
    );
    let body = realized(BooleanOp::Intersect, &wedge, &shared, tol());
    let along = |d: V| {
        topo::query::all_edges(&body)
            .into_iter()
            .find(|&e| {
                let he = body.get_edge(e).unwrap().he_plus;
                let p = |x| {
                    let q = body.get_point(body.get_vertex(x).unwrap().point).unwrap();
                    [q.x, q.y, q.z]
                };
                let (s, t) = (
                    p(body.get_half_edge(he).unwrap().start),
                    p(body.half_edge_end(he).unwrap()),
                );
                let from = |o: V, f: V| norm(o) < 1e-12 && dot(f, d) > norm(f) * (1.0 - 1e-9);
                from(s, t) || from(t, s)
            })
            .unwrap_or_else(|| panic!("an edge out of the vertex along {d:?}"))
    };
    let edges = [along(c.e[0]), along(c.e[1])];
    (body, edges)
}

/// The closed forms on `c`: the probe's gap (positive where `e₁`'s foot
/// is further) and each band's chord were it the further. A chamfer's
/// chord is the gap times `sin φⱼ / sin θ`, the triangle on the near
/// face `Fⱼ` whose angle at the near foot is `φⱼ` and at the mitre's
/// end `θ`, between `eⱼ` and band `k`'s plane's trace on `Fⱼ` — that
/// plane's normal `n_S + n_k`, the bisector of its two supports'. A
/// fillet's is the probe's mitre end against the far foot.
fn closed(c: &Cone, verb: Verb) -> (f64, [f64; 2]) {
    let t = measure(c, verb, D, 2);
    let gap = t.feet[0] - t.feet[1];
    let chord = |k: usize| {
        let j = 1 - k;
        match verb {
            Verb::Chamfer => {
                let trace = unit(cross(add(c.n_s, c.n[k]), c.n[j]));
                gap.abs() * norm(cross(c.e[j], c.l)) / norm(cross(trace, c.e[j]))
            }
            Verb::Fillet => norm(sub(t.q[j], mul(c.l, t.feet[k]))),
        }
    };
    (gap, [chord(0), chord(1)])
}

/// The corner of `body` the two requested edges turn at, read off its
/// points alone, `e₁` then `e₂` in `edges`' order and `L` its third
/// edge — so the closed forms on it share nothing with the battery.
pub(crate) fn cone_of(body: &Body<f64>, edges: [EdgeKey; 2]) -> Cone {
    let ends = |e: EdgeKey| {
        let he = body.get_edge(e).expect("an edge").he_plus;
        [
            body.get_half_edge(he).expect("a half-edge").start,
            body.half_edge_end(he).expect("an end"),
        ]
    };
    let at = |v| {
        let p = body
            .get_point(body.get_vertex(v).expect("a vertex").point)
            .expect("a point");
        [p.x, p.y, p.z]
    };
    let vertex = ends(edges[0])
        .into_iter()
        .find(|v| ends(edges[1]).contains(v))
        .expect("the two edges share a vertex");
    let out = |e: EdgeKey| {
        let [a, b] = ends(e);
        sub(at(if a == vertex { b } else { a }), at(vertex))
    };
    let thirds: Vec<EdgeKey> = topo::query::all_edges(body)
        .into_iter()
        .filter(|e| !edges.contains(e) && ends(*e).contains(&vertex))
        .collect();
    let [third] = thirds[..] else {
        panic!("a trivalent corner: {} other edges", thirds.len())
    };
    Cone::of(out(third), out(edges[0]), out(edges[1]))
}

/// The band whose foot on `L` the closed forms put further.
pub(crate) fn far_band(body: &Body<f64>, edges: [EdgeKey; 2], verb: Verb) -> usize {
    usize::from(closed(&cone_of(body, edges), verb).0 < 0.0)
}

fn kind(verb: Verb) -> BlendKind {
    match verb {
        Verb::Chamfer => BlendKind::Chamfer,
        Verb::Fillet => BlendKind::Fillet,
    }
}

/// What the battery decided at the turn.
#[derive(Clone, Debug, PartialEq)]
enum Outcome {
    /// The mitre, its coincidence recorded.
    Mitre,
    /// The overrun, its far band `edges[k]`.
    Overrun(usize),
    Escalated(BlendDecision),
    Refused(String),
}

fn outcome<T: Decide + Bounds>(body: &Body<T>, edges: [EdgeKey; 2], verb: Verb) -> Outcome {
    let band = Band::linear(tol()).expect("the run's band");
    let request = BlendRequest {
        body,
        edges: edges.to_vec(),
        size: T::from_f64(D),
    };
    match run_battery_for(&request, band, kind(verb)) {
        Ok(verdict) => {
            let [turn] = &verdict.turns[..] else {
                panic!("{verb:?}: one turn, got {}", verdict.turns.len())
            };
            match &turn.run_out {
                TurnRunOut::Mitre(row) => {
                    assert_eq!(row.relation, topo::Relation::CommonFoot);
                    assert_eq!(row.site, topo::DecisionSite::BatteryTurn);
                    Outcome::Mitre
                }
                TurnRunOut::Overrun(o) => Outcome::Overrun(
                    edges
                        .iter()
                        .position(|x| *x == turn.requested[o.far])
                        .expect("the overrun's far band is a requested edge"),
                ),
            }
        }
        Err(BlendError::Escalated { decision, .. }) => Outcome::Escalated(decision),
        Err(e) => Outcome::Refused(e.to_string()),
    }
}

/// The verdict the closed forms call for, where they lie clear of the
/// band's edges; `None` in the margins around them, where rounding may
/// take either side.
fn expected(gap: f64, chords: [f64; 2], band: Band) -> Option<Outcome> {
    let (zero, definite) = (band.zero() / 2.0, band.escalate() * 2.0);
    let far = usize::from(gap < 0.0);
    if gap.abs() < zero && chords[0].max(chords[1]) < zero {
        Some(Outcome::Mitre)
    } else if gap.abs() > definite && chords[far] > definite {
        Some(Outcome::Overrun(far))
    } else {
        None
    }
}

/// **The readings are the probe's closed forms, and the verdict follows
/// them** over a sweep of face-angle pairs at three dihedrals at `L`,
/// both verbs: the gap and both chords agree with the probe's to
/// rounding, and every corner whose closed forms lie clear of the band
/// decides as they call for — the mitre on both the isosceles line and,
/// for a chamfer only, the supplementary one; the overrun everywhere
/// else, naming the band whose foot is further.
#[test]
fn the_verdict_reads_the_probes_closed_forms_over_the_face_angle_sweep() {
    let band = Band::linear(tol()).expect("the run's band");
    let mut decided = [[0usize; 2]; 2];
    for gamma in [60.0f64, 90.0, 120.0] {
        for p1 in (30..=150).step_by(20) {
            for p2 in (30..=150).step_by(20) {
                let c = Cone::angles(
                    [f64::from(p1).to_radians(), f64::from(p2).to_radians()],
                    gamma.to_radians(),
                );
                let (body, edges) = corner(&c);
                for verb in [Verb::Chamfer, Verb::Fillet] {
                    let what = format!("({p1}°, {p2}°, γ {gamma}°) {verb:?}");
                    let (gap, chords) = closed(&c, verb);
                    let (g, ch) = turn_readings(&body, edges, D, kind(verb), tol());
                    for (got, want, name) in [
                        (g, gap, "gap"),
                        (ch[0], chords[0], "chord 1"),
                        (ch[1], chords[1], "chord 2"),
                    ] {
                        assert!(
                            (got - want).abs() < 1e-9 * D,
                            "{what}: the {name} reads {got:e}, the closed form {want:e}"
                        );
                    }
                    let Some(want) = expected(gap, chords, band) else {
                        continue;
                    };
                    let got = outcome(&body, edges, verb);
                    assert_eq!(got, want, "{what}: gap {gap:e}, chords {chords:?}");
                    decided[usize::from(verb == Verb::Fillet)]
                        [usize::from(want != Outcome::Mitre)] += 1;
                }
            }
        }
    }
    // Per verb: the isosceles diagonal (7 per γ), and for the chamfer
    // the supplementary one too (6 more off the diagonal per γ).
    assert_eq!(decided[0][0], 3 * 13, "chamfer mitres: {decided:?}");
    assert_eq!(decided[1][0], 3 * 7, "fillet mitres: {decided:?}");
    // Every other corner overruns: none lies in the band's margins.
    assert_eq!(
        decided[0][1],
        3 * 49 - 3 * 13,
        "chamfer overruns: {decided:?}"
    );
    assert_eq!(
        decided[1][1],
        3 * 49 - 3 * 7,
        "fillet overruns: {decided:?}"
    );
}

/// How a run of corners decided, by its skew from the coincidence, in
/// degrees: the largest skew that mitred, the escalated span, and the
/// smallest that overran.
#[derive(Debug, Default)]
struct Window {
    mitre: Option<f64>,
    escalated: Option<(f64, f64)>,
    overrun: Option<f64>,
}

/// The run's verdicts at increasing `skews`, each the outcome of `at`;
/// panics unless they run mitre, then escalate (on one of the turn's
/// own decisions, or where `section` says the run's overrun is a
/// fillet's cut off near square, on its section's kind too), then
/// overrun, never back.
fn window(what: &str, skews: &[f64], section: bool, at: impl Fn(f64) -> Outcome) -> Window {
    let mut w = Window::default();
    let mut stage = 0;
    for &skew in skews {
        let (now, o) = match at(skew) {
            Outcome::Mitre => {
                w.mitre = Some(skew);
                (0, "mitre")
            }
            Outcome::Escalated(d)
                if matches!(
                    d,
                    BlendDecision::TurnFeetGap
                        | BlendDecision::TurnOverrunChord
                        | BlendDecision::TurnFeet
                ) || (section && matches!(d, BlendDecision::CapTransverse)) =>
            {
                let (lo, _) = w.escalated.unwrap_or((skew, skew));
                w.escalated = Some((lo, skew));
                (1, "escalated")
            }
            Outcome::Overrun(_) => {
                w.overrun.get_or_insert(skew);
                (2, "overrun")
            }
            other => panic!("{what} at {skew:e}°: a turn verdict, got {other:?}"),
        };
        assert!(
            now >= stage,
            "{what} at {skew:e}°: {o} after a later verdict: {w:?}"
        );
        stage = now;
    }
    w
}

/// **The escalate band around a coincidence**, both verbs, at the box
/// corner (`leaning_turn`, its face angle `90° + skew` against `90°`)
/// and at a `60°` corner (`(60°, 60° + skew)`, `γ = 90°`): over four
/// decades of skew the verdict runs mitre, escalate, overrun, never
/// back. The band is narrow: the readings move linearly in the skew,
/// so they leave it within a decade, about `1e-6°…1e-5°` at `D = 0.1`
/// and the default band. The box corner's chamfer is the exception:
/// both its readings move quadratically there, the gap as `d·s²/2` in
/// the lean `s` and the chord `√2` times it, so it mitres until the
/// chord leaves the zero band (`≈0.0068°` at the default band) and
/// overruns once the gap is definite (`≈0.0256°`); its row brackets
/// both edges, computed from the run's band. The box corner's fillet
/// leaves the turn's band within the decade on either side: where it
/// leans the wall (a positive lean) its overrun is the far band's
/// cut-off at that wall, tilted off square by the skew, the exact
/// ellipse however near-circular; leaning the other way, the far band
/// is cut off by the front face, square to its axis, in a circle.
#[test]
fn the_verdict_escalates_only_in_a_narrow_band_around_a_coincidence() {
    // Where the readings are linear in the skew, the skews scale with
    // the run's band as its zero over the default run's.
    let z = Band::linear(tol()).expect("the run's band").zero() / 1e-9;
    let skews = |from: f64| -> Vec<f64> {
        (0..=24)
            .map(|i| from * 10f64.powf(f64::from(i) / 6.0))
            .collect()
    };
    let linear = skews(1e-6 * z);
    let at_box = |verb: Verb| {
        move |skew: f64| {
            let (body, edges) = leaning_turn(skew.to_radians().tan());
            outcome(&body, edges, verb)
        }
    };
    let at_sixty = |verb: Verb| {
        move |skew: f64| {
            let c = Cone::angles(
                [60f64.to_radians(), (60.0 + skew).to_radians()],
                90f64.to_radians(),
            );
            let (body, edges) = corner(&c);
            outcome(&body, edges, verb)
        }
    };
    let inside = |w: &Window, (lo, hi): (f64, f64), what: &str| {
        let (a, b) = w
            .escalated
            .unwrap_or_else(|| panic!("{what}: some skew escalates: {w:?}"));
        assert!(
            lo <= a && b <= hi && w.overrun.is_some(),
            "{what}: escalates within {lo:e}°…{hi:e}°, then overruns: {w:?}"
        );
    };
    // The box chamfer's edges: the mitre holds while the chord
    // `√2·d·s²/2` is Zero, and the overrun starts where the gap `d·s²/2`
    // is definite; sampled at 24 per decade, each edge within a step.
    let band = Band::linear(tol()).expect("the run's band");
    let (mitre_end, overrun_from) = (
        (2f64.sqrt() * band.zero() / D).sqrt().to_degrees(),
        (2.0 * band.escalate() / D).sqrt().to_degrees(),
    );
    let fine: Vec<f64> = (0..=48)
        .map(|i| mitre_end / 10.0 * 10f64.powf(f64::from(i) / 24.0))
        .collect();
    let chamfer_box = window("box chamfer", &fine, false, at_box(Verb::Chamfer));
    let step = 10f64.powf(1.0 / 24.0) * 1.01;
    assert!(
        chamfer_box
            .mitre
            .is_some_and(|m| mitre_end / step <= m && m <= mitre_end * 1.01)
            && chamfer_box
                .overrun
                .is_some_and(|o| overrun_from / 1.01 <= o && o <= overrun_from * step)
            && chamfer_box.escalated.is_some(),
        "the box chamfer mitres to {mitre_end:e}° and overruns from {overrun_from:e}°: \
         {chamfer_box:?}"
    );
    let fillet_box = window("box fillet", &linear, true, at_box(Verb::Fillet));
    inside(&fillet_box, (1e-6 * z, 1e-5 * z), "box fillet");
    let square = window("box fillet leaning square", &linear, false, |skew| {
        let (body, edges) = leaning_turn(-skew.to_radians().tan());
        outcome(&body, edges, Verb::Fillet)
    });
    inside(&square, (1e-6 * z, 1e-5 * z), "box fillet leaning square");
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let w = window(&format!("60° {verb:?}"), &linear, false, at_sixty(verb));
        inside(&w, (1e-6 * z, 1e-5 * z), &format!("60° {verb:?}"));
    }
}

/// **The needle never mitres on a definite chord.** Two requested edges
/// turning `180° − ε` on the shared face, the third edge rising off it: as the turn
/// flattens, a chamfer's gap closes as `ε²` and its chord only as `ε`,
/// so a gap read Zero can stand beside an overrun chord well outside
/// the band — a mitre built there would miss its trimlines by that
/// chord. The verdict escalates the straddle (`fillet3_turn_feet`)
/// instead: at the default band, overrun at `1°` and `0.1°`, the gap in
/// band at `0.03°`, and the gap Zero under a definite chord at `0.01°`
/// (the angles scale with the square root of the run's band, as the
/// gap does). The fillet's gap closes as its chord does, so it mitres
/// only where both read Zero (at `0.01°` at the default band); what the
/// fillet's surgery does there is
/// `band_turn_overrun_fillet::the_needle_builds_then_refuses_at_the_turn_as_it_flattens`.
#[test]
fn a_flattening_needle_escalates_its_chamfer_and_mitres_its_fillet_only_at_zero() {
    let band = Band::linear(tol()).expect("the run's band");
    let mut fillet_mitres = Vec::new();
    let r = (band.zero() / 1e-9).sqrt();
    let flat = |eps: f64| {
        let e = eps.to_radians();
        corner(&Cone::of(
            [0.0, 0.5, 1.0],
            [1.0, 0.0, 0.0],
            [-e.cos(), e.sin(), 0.0],
        ))
    };
    let chamfer = [
        (1.0, None),
        (0.1, None),
        (0.03, Some(BlendDecision::TurnFeetGap)),
        (0.01, Some(BlendDecision::TurnFeet)),
    ];
    for (eps, escalates) in chamfer {
        let eps = eps * r;
        let (body, edges) = flat(eps);
        let got = outcome(&body, edges, Verb::Chamfer);
        match escalates {
            Some(decision) => assert_eq!(got, Outcome::Escalated(decision), "chamfer at {eps}°"),
            None => assert!(
                matches!(got, Outcome::Overrun(_)),
                "chamfer at {eps}°: {got:?}"
            ),
        }
        let (gap, chords) = turn_readings(&body, edges, D, BlendKind::Chamfer, tol());
        if escalates == Some(BlendDecision::TurnFeet) {
            assert!(
                gap.abs() < band.zero() && chords[0].min(chords[1]) > band.escalate(),
                "the needle: gap {gap:e} Zero, chords {chords:?} outside the band"
            );
        }
        let mitres = outcome(&body, edges, Verb::Fillet) == Outcome::Mitre;
        let (gap, chords) = turn_readings(&body, edges, D, BlendKind::Fillet, tol());
        assert_eq!(
            mitres,
            gap.abs() < band.zero() && chords[0].max(chords[1]) < band.zero(),
            "fillet at {eps}°: mitres exactly where its gap {gap:e} and chords {chords:?} \
             are Zero"
        );
        fillet_mitres.push(mitres);
    }
    if tol().eps() == geom_core::tolerance::DEFAULT_EPS {
        assert_eq!(
            fillet_mitres,
            [false, false, false, true],
            "the fillet mitres at the flattest turn alone at the default band"
        );
    }
}

/// A witness: its name, its body, its two requested edges, and the
/// table's gap over the size for each verb.
type Witness = (&'static str, Body<f64>, [EdgeKey; 2], [f64; 2]);

/// **The witnesses' readings** against the probe's table
/// (`d = r = 0.1`): the bracket's two section-face corners (its closed
/// form, `(45°, 90°)` and `(135°, 90°)` at `γ = 90°`: the leaning prism
/// at a lean of `∓1`), the sheared
/// box's supplementary corner and the leaning prism at `±0.5`. Each
/// overrun names the band the table puts further.
#[test]
fn the_witnesses_read_the_probes_table() {
    let s = 0.3;
    let sheared = parallelepiped(s);
    let w = [2.0 + s, s, 1.0];
    let rows: Vec<Witness> = vec![
        {
            let (b, e) = leaning_turn(-1.0);
            ("bracket y = 0", b, e, [0.414, 1.000])
        },
        {
            let (b, e) = leaning_turn(1.0);
            ("bracket y = 1", b, e, [0.414, 1.000])
        },
        {
            let e = [
                edge(&sheared, w, [s, s, 1.0]),
                edge(&sheared, w, [2.0 + s, 1.5 + s, 1.0]),
            ];
            (
                "sheared box, supplementary",
                sheared.clone(),
                e,
                [0.0, 0.624],
            )
        },
        {
            let (b, e) = leaning_turn(0.5);
            ("leaning prism 0.5", b, e, [0.118, 0.500])
        },
        {
            let (b, e) = leaning_turn(-0.5);
            ("leaning prism −0.5", b, e, [0.118, 0.500])
        },
    ];
    for (what, body, edges, want) in rows {
        for (verb, want) in [Verb::Chamfer, Verb::Fillet].into_iter().zip(want) {
            let (gap, chords) = turn_readings(&body, edges, D, kind(verb), tol());
            let (closed_gap, _) = closed(&cone_of(&body, edges), verb);
            assert!(
                (closed_gap.abs() / D - want).abs() < 5e-4 && (gap - closed_gap).abs() < 1e-9 * D,
                "{what} ({verb:?}): gap {gap:e}, the closed form {closed_gap:e}, the table's {want}·size"
            );
            let got = outcome(&body, edges, verb);
            if want == 0.0 {
                assert_eq!(got, Outcome::Mitre, "{what} ({verb:?})");
                assert!(
                    chords[0].max(chords[1]) < 1e-12,
                    "{what}: chords {chords:?}"
                );
            } else {
                let far = far_band(&body, edges, verb);
                assert_eq!(got, Outcome::Overrun(far), "{what} ({verb:?})");
            }
        }
    }
}

/// A replayed corner: its name, then its body and requested edges at
/// `f64` and at `Interval`.
type Replay = (
    String,
    Body<f64>,
    [EdgeKey; 2],
    Body<Interval>,
    [EdgeKey; 2],
);

/// **The `Interval` replay brackets `f64` on both sides**: the box
/// corner at leans that mitre, escalate and overrun for the chamfer
/// (`skew` `1e-3°`, `0.015°`, `atan 0.5`) and at `−0.5`, and the sheared
/// box's supplementary corner, both verbs — every reading's enclosure
/// holds the `f64` reading and is narrow (rounding's width, far inside
/// the zero band), and the verdict at `Interval` is the `f64` one: the
/// supplementary chamfer mitres there too.
#[test]
fn the_interval_readings_bracket_the_f64_readings() {
    use crate::common::interval::iv;
    let mut cases: Vec<Replay> = [
        1e-3f64.to_radians().tan(),
        0.015f64.to_radians().tan(),
        0.5,
        -0.5,
    ]
    .into_iter()
    .map(|s| {
        let (body, edges) = leaning_turn(s);
        let (ibody, iedges) = leaning_turn_at::<Interval>(s);
        (format!("lean {s:e}"), body, edges, ibody, iedges)
    })
    .collect();
    let s = 0.3;
    let (body, ibody) = (parallelepiped(s), parallelepiped_at::<Interval>(s));
    let (w, ends) = ([2.0 + s, s, 1.0], [[s, s, 1.0], [2.0 + s, 1.5 + s, 1.0]]);
    let (edges, iedges) = (
        ends.map(|e| edge_between(&body, w, e)),
        ends.map(|e| edge_between(&ibody, w, e)),
    );
    cases.push((
        "sheared box, supplementary".into(),
        body,
        edges,
        ibody,
        iedges,
    ));
    for (what, body, edges, ibody, iedges) in &cases {
        for verb in [Verb::Chamfer, Verb::Fillet] {
            let what = format!("{what} ({verb:?})");
            let (g, ch) = turn_readings(body, *edges, D, kind(verb), tol());
            let (ig, ich) = turn_readings_interval(ibody, *iedges, iv(D), kind(verb), tol());
            for (x, ix, name) in [
                (g, ig, "gap"),
                (ch[0], ich[0], "chord 1"),
                (ch[1], ich[1], "chord 2"),
            ] {
                assert!(
                    ix.lo() <= x && x <= ix.hi() && ix.hi() - ix.lo() < 1e-12,
                    "{what}: the {name}'s enclosure {ix:?} brackets {x:e}"
                );
            }
            let f = outcome(body, *edges, verb);
            if let Outcome::Overrun(far) = f {
                assert_eq!(far, far_band(body, *edges, verb), "{what}: the far band");
            }
            assert_eq!(
                outcome(ibody, *iedges, verb),
                f,
                "{what}: the verdict at Interval"
            );
        }
    }
    let (_, body, edges, ..) = &cases[4];
    assert_eq!(
        outcome(body, *edges, Verb::Chamfer),
        Outcome::Mitre,
        "the supplementary chamfer mitres"
    );
}

/// **The supplementary corner, concave**: the sheared box cut up into a
/// block from below, its top corner `(2 + s, s, 1)` now the pocket's
/// ceiling corner. Its readings are the convex corner's — they read the
/// cone alone — and the chamfer mitres there too, tier-3 valid with its
/// row recorded; the fillet overruns.
#[test]
fn the_concave_supplementary_corner_reads_and_decides_as_the_convex_one() {
    let s = 0.3;
    let (w, ends) = ([2.0 + s, s, 1.0], [[s, s, 1.0], [2.0 + s, 1.5 + s, 1.0]]);
    let convex = parallelepiped(s);
    let block =
        crate::common::cavity::brick(Point3::new(-1.0, -1.0, 0.5), Point3::new(4.0, 3.0, 2.0));
    let concave = realized(BooleanOp::Subtract, &block, &convex, tol());
    let vex = ends.map(|e| edge_between(&convex, w, e));
    let cav = ends.map(|e| edge_between(&concave, w, e));
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let (g, ch) = turn_readings(&convex, vex, D, kind(verb), tol());
        let (cg, cch) = turn_readings(&concave, cav, D, kind(verb), tol());
        assert!(
            (g - cg).abs() < 1e-12
                && (ch[0] - cch[0]).abs() < 1e-12
                && (ch[1] - cch[1]).abs() < 1e-12,
            "{verb:?}: convex gap {g:e} chords {ch:?}, concave gap {cg:e} chords {cch:?}"
        );
        let want = match verb {
            Verb::Chamfer => Outcome::Mitre,
            Verb::Fillet => Outcome::Overrun(far_band(&concave, cav, verb)),
        };
        assert_eq!(outcome(&concave, cav, verb), want, "{verb:?}, concave");
    }
    let out = sweep::blend::build::chamfer_edges(
        &sweep::test_support::at_rest(&concave, tol()),
        &cav,
        D,
        tol(),
    )
    .unwrap_or_else(|r| panic!("the concave supplementary chamfer builds, got {}", r.error));
    topo::validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("the concave supplementary chamfer is tier-3 valid: {e:?}"));
    assert!(
        out.coincidences
            .iter()
            .any(|r| r.relation == topo::Relation::CommonFoot),
        "the concave supplementary chamfer records its common foot"
    );
    // The two band prisms `½d² sin β · len` (`sin β = 1/k`, lengths 2
    // and 1.5) less their overlap, the pyramid on the `d × d` square
    // whose apex is the common foot, `d/k` off the ceiling.
    let k = (1.0 + s * s).sqrt();
    let want = (0.5 * D * D * 3.5 - D * D * D / 3.0) / k;
    let added = volume(&out.body) - volume(&concave);
    assert!(
        (added - want).abs() < 1e-10,
        "the concave supplementary chamfer adds {added}, the closed form {want}"
    );
}
