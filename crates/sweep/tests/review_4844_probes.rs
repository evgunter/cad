//! Review probes for PR 4844 (the turn's two readings, overrun step 1),
//! dual-review lane. Each row states the claim it tries to falsify.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::Band;
use sweep::blend::build::chamfer_edges;
use sweep::test_support::{assert_naming_totality, at_rest, turn_readings};
use topo::{Body, EdgeKey, validate_geometric};

use crate::band_planar_cut_off::{D, edge, tol, volume};
use crate::band_turn_overrun_probe::{Cone, V, Verb, cross, measure, mul, norm, sub};
use crate::band_turn_overrun_verdict::{Outcome, corner, expected, kind, outcome};
use crate::common::operands::{leaning_turn, parallelepiped};

/// The probe's closed forms at `size`: the gap (positive where `e₁`'s
/// foot is further) and each band's chord were it the further.
fn closed_at(c: &Cone, verb: Verb, size: f64) -> (f64, [f64; 2]) {
    let t = measure(c, verb, size, 2);
    let chord = |k: usize| norm(sub(t.q[1 - k], mul(c.l, t.feet[k])));
    (t.feet[0] - t.feet[1], [chord(0), chord(1)])
}

/// **Claim 3, closed forms.** At a box corner (`φ₁ = φ₂ = 90°`, any
/// dihedral `γ` at `L`) the chamfer's gap AND its chord are quadratic
/// in the skew, chord/gap → `√(1 + 1/sin²γ)` (√2 at 90°), for every
/// skew direction that moves the feet apart; the supplementary
/// direction keeps both exactly zero. The fillet's gap and chord are
/// linear and equal to leading order. Analytic: `gap = d(1/sin φ₁ − 1/sin φ₂) ≈
/// d(δ₁² − δ₂²)/2`, and the chord is the overrun triangle's side
/// opposite the angle `φⱼ` at the near foot, `chord = gap·sin φⱼ /
/// sin θ`, `θ` the angle between band `k`'s plane's trace on `Fⱼ` and
/// `eⱼ`, `tan θ = sin γ` at a box corner.
#[test]
fn review_4844_box_corner_chamfer_chord_is_quadratic_like_its_gap() {
    for gamma in [60.0f64, 90.0, 120.0] {
        let ratio = (1.0 + 1.0 / gamma.to_radians().sin().powi(2)).sqrt();
        for dir in [(1.0, 0.0), (0.0, 1.0), (1.0, 0.5), (1.0, -0.3)] {
            let at = |delta: f64, verb: Verb| {
                let c = Cone::angles(
                    [PI / 2.0 + dir.0 * delta, PI / 2.0 + dir.1 * delta],
                    gamma.to_radians(),
                );
                let (gap, chords) = closed_at(&c, verb, D);
                (gap, chords[usize::from(gap < 0.0)])
            };
            let what = format!("γ {gamma}°, skew direction {dir:?}");
            let (g3, c3) = at(1e-3, Verb::Chamfer);
            let (g4, c4) = at(1e-4, Verb::Chamfer);
            let want = D * (dir.0 * dir.0 - dir.1 * dir.1) / 2.0;
            assert!(
                (g4 / 1e-8 - want).abs() < 1e-3 * want.abs(),
                "{what}: chamfer gap/δ² {} against d(δ₁²−δ₂²)/2 {want}",
                g4 / 1e-8
            );
            assert!(
                (g3 / g4 - 100.0).abs() < 1.0 && (c3 / c4 - 100.0).abs() < 1.0,
                "{what}: chamfer gap and chord both quadratic: gap ×{}, chord ×{} per decade",
                g3 / g4,
                c3 / c4
            );
            assert!(
                (c4 / g4.abs() - ratio).abs() < 1e-3,
                "{what}: chamfer chord/gap {} against √(1+1/sin²γ) {ratio}",
                c4 / g4.abs()
            );
            let (f3, fc3) = at(1e-3, Verb::Fillet);
            let (f4, fc4) = at(1e-4, Verb::Fillet);
            assert!(
                (f3 / f4 - 10.0).abs() < 0.1 && (fc3 / f3.abs() - 1.0).abs() < 1e-3,
                "{what}: fillet gap linear and equal to its chord: ×{}, chord {fc3:e} gap {f3:e} {fc4:e}",
                f3 / f4
            );
        }
        // The supplementary direction: both chamfer readings vanish.
        let c = Cone::angles([PI / 2.0 + 1e-3, PI / 2.0 - 1e-3], gamma.to_radians());
        let (gap, chords) = closed_at(&c, Verb::Chamfer, D);
        assert!(
            gap.abs() < 1e-15 && chords[0].max(chords[1]) < 1e-15,
            "γ {gamma}°: the supplementary skew keeps the chamfer feet one point: {gap:e} {chords:?}"
        );
    }
}

/// **Claim 3, on the battery's own reader** (`leaning_turn`, face angle
/// `90° + s` against `90°`, `γ = 90°`): the gap is `d(sec s − 1)` and
/// the far chord `√2` times it, both quadratic; the fillet's are
/// linear.
#[test]
fn review_4844_the_leaning_box_corner_reads_chord_root_two_times_gap() {
    for s in [1e-2f64, 3e-3, 1e-3] {
        let (body, edges) = leaning_turn(s.tan());
        let (gap, chords) = turn_readings(&body, edges, D, kind(Verb::Chamfer), tol());
        let want = D * (1.0 / s.cos() - 1.0);
        let far = chords[usize::from(gap < 0.0)];
        assert!(
            (gap.abs() - want).abs() < 1e-6 * want,
            "lean {s}: chamfer gap {gap:e} against d(sec s − 1) {want:e}"
        );
        assert!(
            (far / gap.abs() - 2f64.sqrt()).abs() < 1e-3,
            "lean {s}: chord/gap {} against √2",
            far / gap.abs()
        );
        let (fgap, fchords) = turn_readings(&body, edges, D, kind(Verb::Fillet), tol());
        let ffar = fchords[usize::from(fgap < 0.0)];
        assert!(
            (fgap.abs() / s - D).abs() < 0.02 * D && (ffar / fgap.abs() - 1.0).abs() < 1e-3,
            "lean {s}: fillet gap {fgap:e} ≈ d·s, chord {ffar:e} equal to it"
        );
    }
}

/// What a lone chamfer of `edge` removes: cut off at both ends, the
/// kernel's own single-edge build.
fn removed_alone(body: &Body<f64>, e: EdgeKey) -> f64 {
    let out = chamfer_edges(&at_rest(body, tol()), &[e], D, tol())
        .map_err(|r| r.error)
        .unwrap_or_else(|err| panic!("a lone chamfer builds, got {err}"));
    volume(body) - volume(&out.body)
}

/// The two chamfer prisms' overlap at a corner whose feet coincide: the
/// pyramid over the rhombus the two strips share on `S` (sides
/// `d / sin θ_S`, area `d² / sin θ_S`), apex the common foot `d / sin φ`
/// down `L`, its height that times `L`'s rise off `S`.
fn overlap(c: &Cone) -> f64 {
    (D * D / c.theta_s().sin()) * (D / c.phi(0).sin()) * c.h() / 3.0
}

fn built(body: &Body<f64>, edges: [EdgeKey; 2], what: &str) -> f64 {
    let out = chamfer_edges(&at_rest(body, tol()), &edges, D, tol())
        .map_err(|r| r.error)
        .unwrap_or_else(|err| panic!("{what}: the chamfer mitres, got {err}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
    let c = topo::readback::euler_counts(&out.body);
    assert_eq!((c.s, c.genus()), (1, Ok(0)), "{what}: one genus-0 shell");
    assert_naming_totality(body, &out, &edges, what);
    assert!(
        out.coincidences
            .iter()
            .any(|r| r.relation == topo::Relation::CommonFoot),
        "{what}: the CommonFoot row"
    );
    volume(body) - volume(&out.body)
}

/// **Claim 1: the supplementary chamfer's body**, against a closed form
/// derived here: the sheared box's two corners, `s = 0.3`. Each band's
/// prism removes `½d² sin β · ℓ` (both dihedrals at the top have
/// `sin β = 1/k`, `k = √(1+s²)`; the end faces at each band's ends are
/// parallel, so the centroid length is the edge length, `2` and `1.5`),
/// less the corner pyramid `d³/(3k)`: `ΔV = (1.75 d² − d³/3)/k` at
/// both corners.
#[test]
fn review_4844_the_sheared_box_supplementary_chamfer_at_its_closed_form() {
    let s = 0.3;
    let body = parallelepiped(s);
    let k = (1.0 + s * s).sqrt();
    let want = (1.75 * D * D - D * D * D / 3.0) / k;
    let (v, w) = ([s, s, 1.0], [2.0 + s, s, 1.0]);
    for (what, edges) in [
        (
            "isosceles",
            [
                edge(&body, v, [2.0 + s, s, 1.0]),
                edge(&body, v, [s, 1.5 + s, 1.0]),
            ],
        ),
        (
            "supplementary",
            [
                edge(&body, w, [s, s, 1.0]),
                edge(&body, w, [2.0 + s, 1.5 + s, 1.0]),
            ],
        ),
    ] {
        let dv = built(&body, edges, what);
        assert!(
            (dv - want).abs() < 1e-12,
            "{what}: ΔV {dv} against (1.75d² − d³/3)/k {want}"
        );
    }
}

/// **Claim 1 over supplementary corners `(φ, 180° − φ)`** at three
/// dihedrals: every chamfer builds tier 3, total naming, its row, and
/// removes the two lone bands less the corner pyramid; every fillet
/// overruns (a fillet's feet stand apart off the isosceles line).
#[test]
fn review_4844_supplementary_chamfers_build_at_the_lone_bands_less_the_pyramid() {
    for gamma in [60.0f64, 90.0, 120.0] {
        for phi in [40.0f64, 65.0, 80.0, 100.0, 115.0, 140.0] {
            let c = Cone::angles(
                [phi.to_radians(), (180.0 - phi).to_radians()],
                gamma.to_radians(),
            );
            let (body, edges) = corner(&c);
            let what = format!("({phi}°, {}°, γ {gamma}°)", 180.0 - phi);
            let dv = built(&body, edges, &what);
            let want =
                removed_alone(&body, edges[0]) + removed_alone(&body, edges[1]) - overlap(&c);
            assert!(
                (dv - want).abs() < 1e-11,
                "{what}: ΔV {dv} against the lone bands less the pyramid {want} (pyramid {})",
                overlap(&c)
            );
            let f = outcome(&body, edges, Verb::Fillet);
            assert!(
                matches!(f, Outcome::Overrun(_)),
                "{what}: the fillet overruns, got {f:?}"
            );
        }
    }
}

/// A seeded splitmix64, its seed logged and overridable for replay
/// (`REVIEW_4844_SEED`).
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        ((z ^ (z >> 31)) >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next()
    }
}

/// `v` rotated by the unit quaternion `q`.
fn rotate(q: [f64; 4], v: V) -> V {
    let (w, u) = (q[0], [q[1], q[2], q[3]]);
    let t = mul(cross(u, v), 2.0);
    let a = [v[0] + w * t[0], v[1] + w * t[1], v[2] + w * t[2]];
    let b = cross(u, t);
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// **Claim 2 over random corners in random orientations and sizes**,
/// both verbs, a third of them pulled near a coincidence (isosceles or
/// supplementary, exact or skewed by `1e-11…1e-2` rad): the readings equal the
/// probe's closed forms, and wherever the closed forms lie clear of the
/// band the verdict is theirs — never a mitre where the feet stand
/// apart, never an overrun naming the near band.
#[test]
fn review_4844_random_corners_decide_as_their_closed_forms() {
    let seed = std::env::var("REVIEW_4844_SEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64
        });
    eprintln!("review_4844 seed {seed}");
    let mut rng = Rng(seed);
    let band = Band::linear(tol()).expect("the run's band");
    let mut tally = [0usize; 5];
    for i in 0..36 {
        let gamma = rng.range(40.0, 140.0).to_radians();
        let p1 = rng.range(30.0, 150.0);
        let p2 = match i % 3 {
            0 => rng.range(30.0, 150.0),
            near => {
                let skew = if rng.next() < 0.3 {
                    0.0
                } else {
                    10f64.powf(rng.range(-11.0, -2.0)).to_degrees()
                        * if rng.next() < 0.5 { -1.0 } else { 1.0 }
                };
                (if near == 1 { p1 } else { 180.0 - p1 }) + skew
            }
        };
        let base = Cone::angles([p1.to_radians(), p2.to_radians()], gamma);
        let q = {
            let a = [
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            ];
            let n = (a.iter().map(|x| x * x).sum::<f64>()).sqrt();
            a.map(|x| x / n)
        };
        let c = Cone::of(
            rotate(q, base.l),
            rotate(q, base.e[0]),
            rotate(q, base.e[1]),
        );
        let size = D * 10f64.powf(rng.range(-0.7, 0.5));
        // The fixture's own Boolean may escalate on a random pose; that
        // corner is skipped, loudly.
        let Ok((body, edges)) = std::panic::catch_unwind(|| corner(&c)) else {
            eprintln!("review_4844 #{i}: the fixture's Boolean refused, skipped");
            tally[3] += 1;
            continue;
        };
        for verb in [Verb::Chamfer, Verb::Fillet] {
            let what = format!(
                "seed {seed} #{i}: ({p1:.9}°, {p2:.9}°, γ {:.3}°) size {size:.4} {verb:?}",
                gamma.to_degrees()
            );
            let (gap, chords) = closed_at(&c, verb, size);
            let (g, ch) = turn_readings(&body, edges, size, kind(verb), tol());
            for (got, want, name) in [
                (g, gap, "gap"),
                (ch[0], chords[0], "chord 1"),
                (ch[1], chords[1], "chord 2"),
            ] {
                assert!(
                    (got - want).abs() < 1e-9 * size,
                    "{what}: the {name} reads {got:e}, the closed form {want:e}"
                );
            }
            let got = outcome_at(&body, edges, verb, size);
            match expected(gap, chords, band) {
                // The clearance screen refuses some acute or large-size
                // corners before the turn is read: conservative, never a
                // build, and not this unit's.
                Some(_) if matches!(&got, Outcome::Refused(t) if t.contains("clearance screen")) => {
                    tally[4] += 1;
                }
                Some(want) => {
                    assert_eq!(got, want, "{what}: gap {gap:e}, chords {chords:?}");
                    tally[usize::from(want != Outcome::Mitre)] += 1;
                }
                None => {
                    tally[2] += 1;
                    assert!(
                        !matches!(got, Outcome::Refused(_)),
                        "{what}: a turn verdict in the margins, got {got:?}"
                    );
                    if gap.abs() > band.escalate() {
                        assert_ne!(
                            got,
                            Outcome::Mitre,
                            "{what}: feet definitely apart never mitre"
                        );
                    }
                }
            }
        }
    }
    eprintln!("review_4844 mitre/overrun/margins/fixture-skipped/clearance {tally:?}");
    assert!(
        tally[0] >= 3 && tally[0] + tally[1] >= 40,
        "enough corners decided: {tally:?}"
    );
}

/// `band_turn_overrun_verdict::outcome` at `size`.
fn outcome_at(body: &Body<f64>, edges: [EdgeKey; 2], verb: Verb, size: f64) -> Outcome {
    use sweep::blend::BlendError;
    use sweep::blend::battery::{BlendRequest, TURN_OVERRUN, run_battery_for};
    let band = Band::linear(tol()).expect("the run's band");
    match run_battery_for(
        &BlendRequest {
            body,
            edges: edges.to_vec(),
            size,
        },
        band,
        kind(verb),
    ) {
        Ok(_) => Outcome::Mitre,
        Err(BlendError::UnsupportedRunOut {
            at: topo::EntityId::Edge(e),
            detail,
        }) if detail == TURN_OVERRUN => Outcome::Overrun(
            edges
                .iter()
                .position(|x| *x == e)
                .expect("a requested edge"),
        ),
        Err(BlendError::Escalated { decision, .. }) => Outcome::Escalated(decision),
        Err(e) => Outcome::Refused(e.to_string()),
    }
}

/// **Claim 4, the recourse's middle leg.** An escalating turn
/// (`FILLET3_TURN_RECOURSE`) offers "move them clearly apart (a larger
/// setback, or a more asymmetric corner)". Followed — the leaning box
/// corner's chamfer escalates at a lean of `0.015°`; a lean of `5°`
/// moves the feet clearly apart — the turn refuses `TURN_OVERRUN`
/// rather than building, since the overrun is not built in this step.
#[test]
fn review_4844_the_turn_recourses_middle_leg_leads_to_a_refusal() {
    use sweep::blend::{BlendDecision, FILLET3_TURN_RECOURSE};
    let (body, edges) = leaning_turn(0.015f64.to_radians().tan());
    let got = outcome(&body, edges, Verb::Chamfer);
    assert!(
        matches!(
            got,
            Outcome::Escalated(
                BlendDecision::TurnFeetGap
                    | BlendDecision::TurnOverrunChord
                    | BlendDecision::TurnFeet
            )
        ),
        "the lean escalates on a turn reading, got {got:?}"
    );
    assert!(FILLET3_TURN_RECOURSE.contains("move them clearly apart"));
    let (body, edges) = leaning_turn(5f64.to_radians().tan());
    let followed = outcome(&body, edges, Verb::Chamfer);
    assert_eq!(
        followed,
        Outcome::Overrun(0),
        "the followed leg refuses as the overrun, it does not build"
    );
}
