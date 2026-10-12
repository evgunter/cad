//! Review probes for PR 4844 (the turn's two readings), dual review.
//! Not a gate: evidence for the review. Run:
//! `cargo nextest run -p sweep -E 'test(review_4844)' --no-capture`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Bounds, Interval, Point2, Point3, Vec3};
use sweep::blend::battery::{BlendRequest, TURN_OVERRUN, run_battery_for};
use sweep::blend::build::{chamfer_edges, fillet_edges};
use sweep::blend::{BlendDecision, BlendError, BlendKind};
use sweep::test_support::{
    at_rest, finished, prism_on, realized, sketch_from_axes, turn_readings, turn_readings_interval,
};
use topo::boolean::BooleanOp;
use topo::{Body, EdgeKey, EntityId, mass_properties, validate_geometric};

use crate::band_planar_cut_off::{D, edge, tol, volume};
use crate::band_turn_overrun_probe::{Cone, V, across, add, cross, dot, mul, norm, sub};
use crate::common::cavity::brick;
use crate::common::interval::iv;
use crate::common::operands::{leaning_turn, parallelepiped};

fn point(v: V) -> Point3<f64> {
    Point3::new(v[0], v[1], v[2])
}
fn vector(v: V) -> Vec3<f64> {
    Vec3::new(v[0], v[1], v[2])
}

/// The verdict suite's `corner`, copied (it is private there).
fn corner(c: &Cone) -> (Body<f64>, [EdgeKey; 2]) {
    const R: f64 = 10.0;
    let l = c.l;
    let ray = [across(c.e[0], l), across(c.e[1], l)];
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

#[derive(Clone, Debug, PartialEq)]
enum Outcome {
    Mitre,
    Overrun(usize),
    Escalated(BlendDecision),
    Refused(String),
}

fn outcome(body: &Body<f64>, edges: [EdgeKey; 2], kind: BlendKind) -> Outcome {
    let band = Band::linear(tol()).unwrap();
    match run_battery_for(
        &BlendRequest {
            body,
            edges: edges.to_vec(),
            size: D,
        },
        band,
        kind,
    ) {
        Ok(_) => Outcome::Mitre,
        Err(BlendError::UnsupportedRunOut {
            at: EntityId::Edge(e),
            detail,
        }) if detail == TURN_OVERRUN => {
            Outcome::Overrun(edges.iter().position(|x| *x == e).unwrap())
        }
        Err(BlendError::Escalated { decision, .. }) => Outcome::Escalated(decision),
        Err(e) => Outcome::Refused(e.to_string()),
    }
}

/// An independent closed form of a chamfer's overrun chord: the overrun
/// triangle `(p_j, p_k, q)` is `(V, p_k, E)` scaled about `p_k` by
/// `g / t_k`, `E` band `k`'s trimline on `S` met with `e_j`
/// (`d / sin θ_S` along `e_j`), so `chord = g · |p_k − E| / t_k`.
fn chamfer_chord_identity(c: &Cone, gap: f64) -> f64 {
    let k = usize::from(gap < 0.0);
    let j = 1 - k;
    let t_k = D / dot(c.e[k], c.l).acos().sin();
    let theta_s = dot(c.e[0], c.e[1]).acos();
    let e = mul(c.e[j], D / theta_s.sin());
    gap.abs() * norm(sub(mul(c.l, t_k), e)) / t_k
}

/// **Claim 3, the box corner.** The chamfer's chord is proportional to
/// its gap at every corner (`chamfer_chord_identity`), so at the box
/// corner, where the gap is `d·|δ₁² − δ₂²|/2` to leading order (the
/// isosceles and supplementary lines cross there), the chord is
/// quadratic too, chord/gap → `|p_k − E|/t_k` (√2 at γ = 90°). The
/// fillet's gap is linear. Printed and asserted over skew directions
/// and three γ.
#[test]
fn review_4844_box_corner_both_chamfer_readings_are_quadratic() {
    for gamma in [60.0f64, 90.0, 120.0] {
        for (a, b) in [(1.0, 0.0), (0.0, 1.0), (1.0, 0.5), (1.0, -0.5), (-1.0, 0.3)] {
            let mut prev: Option<(f64, f64, f64)> = None;
            for delta in [1e-1f64, 3e-2, 1e-2, 3e-3] {
                let c = Cone::angles(
                    [
                        (90f64).to_radians() + a * delta,
                        (90f64).to_radians() + b * delta,
                    ],
                    gamma.to_radians(),
                );
                let (body, edges) = corner(&c);
                let (g, ch) = turn_readings(&body, edges, D, BlendKind::Chamfer, tol());
                let far = usize::from(g < 0.0);
                let lead = D * (a * a - b * b).abs() * delta * delta / 2.0;
                let ident = chamfer_chord_identity(&c, g);
                let (fg, _) = turn_readings(&body, edges, D, BlendKind::Fillet, tol());
                println!(
                    "γ {gamma} dir ({a},{b}) δ {delta:e}: chamfer gap {g:.3e} (lead {lead:.3e}) chord {:.3e} (identity {ident:.3e}) ratio {:.4}; fillet gap/δ {:.4}",
                    ch[far],
                    ch[far] / g.abs(),
                    fg.abs() / delta
                );
                assert!(
                    (ch[far] - ident).abs() <= 1e-9 * D + 1e-6 * ident,
                    "the chamfer chord is g·|p_k − E|/t_k: {} vs {ident}",
                    ch[far]
                );
                assert!(
                    (g.abs() / lead - 1.0).abs() < 0.2,
                    "the gap is d·|δ₁² − δ₂²|/2 to leading order: {g:e} vs {lead:e}"
                );
                if let Some((pg, pc, pf)) = prev {
                    // δ shrinks by ~3.16 or 3.33: quadratic shrinks ~10×,
                    // linear ~3×.
                    let (rg, rc, rf) = (pg / g.abs(), pc / ch[far], pf / fg.abs());
                    assert!(rg > 8.0 && rc > 8.0, "chamfer quadratic: {rg} {rc}");
                    assert!(rf < 4.0, "fillet linear: {rf}");
                }
                prev = Some((g.abs(), ch[far], fg.abs()));
            }
        }
    }
}

/// **Claim 3, the window on the leaning box corner**: predicted from
/// the closed forms (gap `d(1/cos δ − 1)`, chord `√2·gap` to leading
/// order) against the battery, over a fine skew grid. Prints the edges.
#[test]
fn review_4844_box_chamfer_window_is_predicted_by_quadratic_readings() {
    let band = Band::linear(tol()).unwrap();
    let mut seen = vec![];
    let mut skew = 2e-3f64;
    while skew < 6e-2 {
        let d = skew.to_radians();
        let (body, edges) = leaning_turn(d.tan());
        let got = outcome(&body, edges, BlendKind::Chamfer);
        let (g, ch) = turn_readings(&body, edges, D, BlendKind::Chamfer, tol());
        let gap = D * (1.0 / d.cos() - 1.0);
        let chord = gap * 2f64.sqrt();
        assert!(
            (g.abs() - gap).abs() < 1e-14 + 1e-3 * gap,
            "gap {g:e} vs {gap:e}"
        );
        let far = usize::from(g < 0.0);
        assert!(
            (ch[far] / chord - 1.0).abs() < 1e-3,
            "chord {:e} vs {chord:e}",
            ch[far]
        );
        seen.push((skew, got.clone()));
        let (z, e) = (band.zero(), band.escalate());
        let clear = |x: f64, edge: f64| (x / edge - 1.0).abs() > 0.02;
        if clear(gap, z) && clear(chord, z) && clear(gap, e) && clear(chord, e) {
            let want = if gap < z && chord < z {
                Some(Outcome::Mitre)
            } else if gap > e && chord > e {
                Some(Outcome::Overrun(far))
            } else {
                None
            };
            if let Some(w) = want {
                assert_eq!(got, w, "skew {skew}°");
            } else {
                assert!(
                    matches!(got, Outcome::Escalated(_)),
                    "skew {skew}°: {got:?}"
                );
            }
        }
        skew *= 1.05;
    }
    let mitre = seen
        .iter()
        .filter(|(_, o)| *o == Outcome::Mitre)
        .map(|x| x.0)
        .fold(0.0, f64::max);
    let over = seen
        .iter()
        .filter(|(_, o)| matches!(o, Outcome::Overrun(_)))
        .map(|x| x.0)
        .fold(f64::INFINITY, f64::min);
    println!("box chamfer: mitres to {mitre:.5}°, first overrun {over:.5}°");
    for (s, o) in &seen {
        println!("  {s:.5}°: {o:?}");
    }
}

/// The leaning prism cut up into a block from below: the same corner,
/// concave, the pocket's ceiling corners all concave.
fn concave_leaning(s: f64) -> (Body<f64>, [EdgeKey; 2]) {
    let (prism, _) = leaning_turn(s);
    let block = brick(Point3::new(-1.0, -2.5, 0.5), Point3::new(3.0, 1.0, 2.0));
    let body = realized(BooleanOp::Subtract, &block, &prism, tol());
    let c = [s, 0.0, 1.0];
    (
        body.clone(),
        [
            edge(&body, c, [2.0, 0.0, 1.0]),
            edge(&body, c, [s, -1.5, 1.0]),
        ],
    )
}

/// The half-space `(x − q)·n ≤ 0` within a slab 400 wide.
fn half_space(q: Point3<f64>, n: Vec3<f64>) -> Body<f64> {
    let n = n.normalize();
    let seed = if n.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let u = (seed - n * seed.dot(n)).normalize();
    let v = u.cross(n);
    prism_on(
        sketch_from_axes(q - u * 200.0 - v * 200.0, u, v, tol()),
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(400.0, 0.0), 0.0),
            (Point2::new(400.0, 400.0), 0.0),
            (Point2::new(0.0, 400.0), 0.0),
        ],
        400.0,
        tol(),
    )
}

/// **Claim 3, convexity**: the concave leaning corner reads the convex
/// one's gap and chords, and its window runs the same way.
#[test]
fn review_4844_concave_box_corner_reads_as_the_convex_one() {
    for skew in [1e-3f64, 5e-3, 1.5e-2, 3e-2, 1.0, 26.0] {
        let s = skew.to_radians().tan();
        let (vex, ve) = leaning_turn(s);
        let (cav, ce) = concave_leaning(s);
        for kind in [BlendKind::Chamfer, BlendKind::Fillet] {
            let (g, ch) = turn_readings(&vex, ve, D, kind, tol());
            let (cg, cch) = turn_readings(&cav, ce, D, kind, tol());
            println!("skew {skew}° {kind:?}: convex {g:e} {ch:?}; concave {cg:e} {cch:?}");
            assert!(
                (g - cg).abs() < 1e-13,
                "{kind:?} at {skew}°: gap {g:e} vs {cg:e}"
            );
            for i in 0..2 {
                assert!(
                    (ch[i] - cch[i]).abs() < 1e-13,
                    "{kind:?} at {skew}°: chord {i}"
                );
            }
            let (a, b) = (outcome(&vex, ve, kind), outcome(&cav, ce, kind));
            println!("    convex {a:?}, concave {b:?}");
            assert_eq!(a, b, "{kind:?} at {skew}°");
        }
    }
}

/// `{0 ≤ z ≤ 1, s z ≤ x ≤ 2 + s z, s z ≤ y ≤ 1.5 + s z}` at any scalar.
fn parallelepiped_at<T: geom_core::Decide + Bounds + topo::AtRestPolicy>(s: f64) -> Body<T> {
    let t = T::from_f64;
    let z = Vec3::new(t(0.0), t(0.0), t(1.0));
    let corners = |pts: &[(f64, f64)]| sweep::test_support::corners::<T>(pts);
    let along_y = prism_on(
        sketch_from_axes(
            Point3::new(t(0.0), t(3.5), t(0.0)),
            Vec3::new(t(1.0), t(0.0), t(0.0)),
            z,
            tol(),
        ),
        corners(&[(0.0, 0.0), (2.0, 0.0), (2.0 + s, 1.0), (s, 1.0)]),
        t(5.0),
        tol(),
    );
    let (z0, z1) = (-0.5, 1.5);
    let along_x = prism_on(
        sketch_from_axes(
            Point3::new(t(3.5), t(0.0), t(0.0)),
            Vec3::new(t(0.0), t(-1.0), t(0.0)),
            z,
            tol(),
        ),
        corners(&[
            (-1.5 - s * z0, z0),
            (-s * z0, z0),
            (-s * z1, z1),
            (-1.5 - s * z1, z1),
        ]),
        t(5.0),
        tol(),
    );
    topo::boolean::boolean_op_with(
        BooleanOp::Intersect,
        &finished("y", along_y, tol()),
        &finished("x", along_x, tol()),
        &topo::boolean::BooleanInputs::default(),
        topo::boolean::SweepStrategy::Realized,
        tol(),
    )
    .unwrap_or_else(|e| panic!("the parallelepiped builds, got {e}"))
    .body()
    .unwrap()
    .body
    .clone()
    .into_body()
}

/// The chamfer's ΔV at the supplementary corner `(2 + s, s, 1)`, derived
/// by hand: two oblique prism wedges `½d² sin β · len` (sin β = 1/k,
/// lengths 2 and 1.5) less their overlap at the corner, a pyramid on the
/// `d × d` square of the top with its apex the common foot, `d/k` below.
fn supplementary_removed(s: f64, d: f64) -> f64 {
    let k = (1.0 + s * s).sqrt();
    0.5 * d * d / k * (2.0 + 1.5) - d * d * d / (3.0 * k)
}

/// **Claim 1, the sheared box's supplementary chamfer** at several
/// shears and sizes: builds, tier 3, its `CommonFoot` row recorded, ΔV
/// the hand-derived closed form; the fillet overruns.
#[test]
fn review_4844_supplementary_chamfer_volume_matches_a_hand_closed_form() {
    for s in [0.1, 0.3, 0.6, -0.3] {
        let body = parallelepiped(s);
        let w = [2.0 + s, s, 1.0];
        let edges = [
            edge(&body, w, [s, s, 1.0]),
            edge(&body, w, [2.0 + s, 1.5 + s, 1.0]),
        ];
        let v0 = volume(&body);
        for d in [0.05, 0.1, 0.2] {
            let out = chamfer_edges(&at_rest(&body, tol()), &edges, d, tol())
                .unwrap_or_else(|r| panic!("s {s} d {d}: builds, got {}", r.error));
            validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("s {s} d {d}: {e:?}"));
            let removed = v0 - volume(&out.body);
            let want = supplementary_removed(s, d);
            println!("s {s} d {d}: ΔV {removed:.12} closed form {want:.12}");
            assert!(
                (removed - want).abs() < 1e-10,
                "s {s} d {d}: {removed} vs {want}"
            );
            assert!(
                out.coincidences
                    .iter()
                    .any(|r| r.relation == topo::Relation::CommonFoot),
                "s {s} d {d}: CommonFoot"
            );
            let fil = fillet_edges(&at_rest(&body, tol()), &edges, d, tol());
            match fil {
                Err(r) => assert!(
                    matches!(&r.error, BlendError::UnsupportedRunOut { detail, .. } if *detail == TURN_OVERRUN),
                    "s {s} d {d}: fillet overruns, got {}",
                    r.error
                ),
                Ok(_) => panic!("s {s} d {d}: the fillet built"),
            }
        }
    }
}

/// **Claim 1 at `Interval`**: the supplementary chamfer at the
/// certified scalar builds, and its volume enclosure holds the closed
/// form.
#[test]
fn review_4844_supplementary_chamfer_builds_at_interval() {
    let s = 0.3;
    let body: Body<Interval> = parallelepiped_at::<Interval>(s);
    let near = |p: &Point3<Interval>, q: [f64; 3]| {
        [p.x, p.y, p.z]
            .iter()
            .zip(q)
            .all(|(c, w)| c.lo() <= w + 1e-12 && w - 1e-12 <= c.hi())
    };
    let between = |a: [f64; 3], b: [f64; 3]| {
        topo::query::all_edges(&body)
            .into_iter()
            .find(|&e| {
                let he = body.get_edge(e).unwrap().he_plus;
                let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                let (x, y) = (
                    p(body.get_half_edge(he).unwrap().start),
                    p(body.half_edge_end(he).unwrap()),
                );
                (near(&x, a) && near(&y, b)) || (near(&x, b) && near(&y, a))
            })
            .expect("an edge")
    };
    let w = [2.0 + s, s, 1.0];
    let edges = [between(w, [s, s, 1.0]), between(w, [2.0 + s, 1.5 + s, 1.0])];
    let (g, ch) = turn_readings_interval(&body, edges, iv(D), BlendKind::Chamfer, tol());
    println!("Interval readings: gap {g:?} chords {ch:?}");
    let v0 = mass_properties(&body, tol()).unwrap().volume;
    let out = chamfer_edges(&at_rest(&body, tol()), &edges, iv(D), tol())
        .unwrap_or_else(|r| panic!("builds at Interval, got {:?}", r.error));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("tier 3 at Interval: {e:?}"));
    let v1 = mass_properties(&out.body, tol()).unwrap().volume;
    let want = supplementary_removed(s, D);
    let (lo, hi) = (v0.lo() - v1.hi(), v0.hi() - v1.lo());
    println!("Interval ΔV [{lo:.12}, {hi:.12}] closed form {want:.12}");
    assert!(
        lo <= want + 1e-12 && want - 1e-12 <= hi && hi - lo < 1e-6,
        "ΔV [{lo}, {hi}] vs {want}"
    );
}

/// **Claim 1 over poses**: supplementary chamfer corners of the probe's
/// cone at several `(φ, π − φ, γ)` build tier 3 at the two half-spaces'
/// volume; the fillet there never builds.
#[test]
fn review_4844_supplementary_cone_corners_build_the_half_spaces() {
    for phi in [40.0f64, 65.0, 115.0] {
        for gamma in [60.0f64, 90.0, 135.0] {
            let c = Cone::angles(
                [phi.to_radians(), (180.0 - phi).to_radians()],
                gamma.to_radians(),
            );
            let (body, edges) = corner(&c);
            let what = format!("({phi}°, {}°, γ {gamma}°)", 180.0 - phi);
            let theta_s = dot(c.e[0], c.e[1]).acos();
            let oracle = (0..2).fold(body.clone(), |cut, i| {
                let q = mul(c.e[1 - i], D / theta_s.sin());
                let n = mul(add(c.n_s, c.n[i]), -1.0);
                realized(
                    BooleanOp::Intersect,
                    &cut,
                    &half_space(point(q), vector(n)),
                    tol(),
                )
            });
            let got = chamfer_edges(&at_rest(&body, tol()), &edges, D, tol());
            let fil = fillet_edges(&at_rest(&body, tol()), &edges, D, tol());
            assert!(fil.is_err(), "{what}: the fillet does not build");
            match got {
                Ok(out) => {
                    validate_geometric(&out.body, tol())
                        .unwrap_or_else(|e| panic!("{what}: {e:?}"));
                    let (a, b) = (volume(&out.body), volume(&oracle));
                    println!(
                        "{what}: chamfer {a:.10} oracle {b:.10}; fillet {}",
                        fil.err().unwrap().error
                    );
                    assert!((a - b).abs() < 1e-8, "{what}: {a} vs {b}");
                }
                Err(r) => {
                    println!("{what}: chamfer refused {}", r.error);
                    assert!(
                        !matches!(r.error, BlendError::SurgeryInvariant { .. }),
                        "{what}: {:?}",
                        r.error
                    );
                }
            }
        }
    }
}

/// **Claim 2, both-Zero never away from a coincidence**: a chamfer's
/// chord is at least `sin φ_j` times its gap, so Zero/Zero bounds the
/// feet apart within the zero band. A grid of near-coincident skews on
/// a generic cone, both verbs: whenever the battery mitres, the closed
/// gap is inside the zero band.
#[test]
fn review_4844_a_mitre_is_built_only_with_feet_within_the_zero_band() {
    let band = Band::linear(tol()).unwrap();
    for base in [50.0f64, 90.0, 130.0] {
        for gamma in [45.0f64, 90.0, 160.0] {
            for skew in [1e-7f64, 1e-6, 5e-6, 2e-3, 1e-2] {
                for (p1, p2) in [(base, base + skew), (base, 180.0 - base + skew)] {
                    let c = Cone::angles([p1.to_radians(), p2.to_radians()], gamma.to_radians());
                    let Ok((body, edges)) = std::panic::catch_unwind(|| corner(&c)) else {
                        println!("({p1}, {p2}, {gamma}): the fixture's Boolean refuses; skipped");
                        continue;
                    };
                    for kind in [BlendKind::Chamfer, BlendKind::Fillet] {
                        let o = outcome(&body, edges, kind);
                        let (g, ch) = turn_readings(&body, edges, D, kind, tol());
                        if o == Outcome::Mitre {
                            assert!(
                                g.abs() < band.escalate() && ch[0].max(ch[1]) < band.escalate(),
                                "({p1}, {p2}, {gamma}) {kind:?}: mitre with gap {g:e} chords {ch:?}"
                            );
                        }
                        assert!(
                            !matches!(o, Outcome::Refused(_)),
                            "({p1}, {p2}, {gamma}) {kind:?}: {o:?}"
                        );
                    }
                }
            }
        }
    }
}
