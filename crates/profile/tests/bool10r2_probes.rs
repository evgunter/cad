//! BOOL-10 second-round review probes: the declared-split arc leg
//! (`arc_to(spec.split(n))`) attacked off the PR's own fixtures.
//!
//! What is exercised here that the unit's rows do not: odd and even
//! counts on non-axis chords; major arcs (bulge > 1); CW windings;
//! near-π sweeps; `Center` legs split about their authored centre in
//! both windings; `Via` legs authored backwards; the endpoint-free
//! `ArcLen` leg; a full-period and an over-full `Sweep`; a split
//! closer whose seam arrival is declared; a large count. Every station
//! is measured against the leg's own carrier and every reversible leg
//! is measured against its reversed twin bit for bit.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{p2, pinned};
use geom_core::{Point2, Tol};
use profile::{
    ArcLen, ArcSide, ArcSweep, Bulge, Center, ClosedLoop, Open, PathError, ProfileLoop, Start,
    Sweep, Via,
};

fn bits(v: f64) -> u64 {
    v.to_bits()
}

/// The carrier of the chord-and-bulge arc, by the crate's closed form
/// (the split's own derivation, re-stated here so the measurement does
/// not read the kernel's private helper).
fn carrier(a: Point2<f64>, b: Point2<f64>, bulge: f64) -> (Point2<f64>, f64) {
    let d = b - a;
    let l = d.norm_squared().sqrt();
    let mid = Point2::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5);
    let n_hat = geom_core::Vec2::new(-d.y, d.x) * (1.0 / l);
    let apothem = l * (1.0 - bulge * bulge) / (4.0 * bulge);
    let r = l * (1.0 + bulge * bulge) / (4.0 * bulge.abs());
    (mid + n_hat * apothem, r)
}

/// Largest radial departure of the loop's vertices from the circle
/// `(centre, r)`, relative to the geometry's coordinate scale (the
/// larger of the centre's magnitude and the radius) — the rounding
/// class the authored endpoints themselves carry.
fn worst_radial(lp: &ProfileLoop<f64>, centre: Point2<f64>, r: f64) -> f64 {
    let scale = centre.x.abs().max(centre.y.abs()).max(r);
    lp.vertices()
        .iter()
        .map(|v| ((v.pos() - centre).norm_squared().sqrt() - r).abs() / scale)
        .fold(0.0, f64::max)
}

fn bulge_leg(a: Point2<f64>, b: Point2<f64>, bulge: f64, n: usize) -> ClosedLoop<f64> {
    Open.at(a)
        .arc_to(Bulge { p: b, b: bulge }.split(n), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap()
}

/// **Reversal identity and on-carrier placement, attacked.** Chords
/// off every axis, bulges spanning a 0.2-rad sliver to a 350° major
/// arc in both windings, counts 2..16 including every odd one to 9.
/// Both claims of the placement contract are asserted: the reversed
/// leg's stations are the same bits, and every piece is tan(θ/4n)
/// exactly. The off-carrier residue is bounded by 8 ulps relative —
/// the leg's own rounding class — and its maximum is printed.
#[test]
fn stations_reverse_bit_identically_and_stay_on_the_carrier() {
    let chords = [
        (p2(0.3, 0.1), p2(1.7, 0.9)),
        (p2(-2.0, 3.5), p2(0.25, -1.75)),
        (p2(1e-3, 7.0), p2(1e-3 + 1e-3, 7.0 + 2e-4)),
        (p2(100.0, -50.0), p2(-30.0, 80.0)),
    ];
    let bulges = [
        0.05,
        0.37,
        -0.37,
        0.999_999_999,
        1.0,
        1.000_000_001,
        -1.0,
        2.5,
        -2.5,
        11.43,
        -11.43,
    ];
    let counts = [2usize, 3, 4, 5, 6, 7, 8, 9, 12, 16];
    let mut worst = 0.0f64;
    let mut worst_at = String::new();
    for (a, b) in chords {
        for bulge in bulges {
            for n in counts {
                let forward = pinned(bulge_leg(a, b, bulge, n));
                let backward = pinned(bulge_leg(b, a, -bulge, n));
                assert_eq!(forward.vertices().len(), n + 1);
                let piece = (4.0 * bulge.atan() / (4.0 * n as f64)).tan();
                for k in 0..n {
                    let f = forward.vertices()[k];
                    assert_eq!(bits(f.bulge()), bits(piece), "b={bulge} n={n} piece {k}");
                }
                for k in 1..n {
                    let f = forward.vertices()[k];
                    let r = backward.vertices()[n - k];
                    assert_eq!(
                        (bits(f.pos().x), bits(f.pos().y)),
                        (bits(r.pos().x), bits(r.pos().y)),
                        "a={a:?} b={b:?} bulge={bulge} n={n}: station {k} vs reversed {}",
                        n - k
                    );
                }
                let joints: Vec<usize> = (1..n).collect();
                assert_eq!(forward.tangent_joints(), &joints[..]);
                let (c, r) = carrier(a, b, bulge);
                let off = worst_radial(&forward, c, r);
                if off > worst {
                    worst = off;
                    worst_at = format!("a={a:?} b={b:?} bulge={bulge} n={n}");
                }
            }
        }
    }
    eprintln!("worst relative radial residue over the sweep: {worst:e} at {worst_at}");
    assert!(
        worst < 8.0 * f64::EPSILON,
        "a station left the carrier by {worst:e} (relative) at {worst_at}"
    );
}

/// **A `Center` leg splits about its AUTHORED centre, both windings,
/// and reverses bit for bit.** Forward: CCW from `a` to `b`; backward:
/// CW from `b` to `a` about the same centre. Odd and even counts.
#[test]
fn center_split_reverses_bit_identically_in_both_windings() {
    let c = p2(0.2, -0.3);
    let r = 1.3;
    let a = p2(c.x + r * 0.4f64.cos(), c.y + r * 0.4f64.sin());
    let b = p2(c.x + r * 2.9f64.cos(), c.y + r * 2.9f64.sin());
    for n in [2usize, 3, 4, 5, 7, 8] {
        let leg = |from: Point2<f64>, to: Point2<f64>, winding: ArcSweep| {
            pinned(
                Open.at(from)
                    .arc_to(Center { c, winding, p: to }.split(n), Tol::witness())
                    .unwrap()
                    .line_to(Start, Tol::witness())
                    .unwrap(),
            )
        };
        let forward = leg(a, b, ArcSweep::Ccw);
        let backward = leg(b, a, ArcSweep::Cw);
        for k in 1..n {
            let f = forward.vertices()[k];
            let g = backward.vertices()[n - k];
            assert_eq!(
                (bits(f.pos().x), bits(f.pos().y)),
                (bits(g.pos().x), bits(g.pos().y)),
                "n={n}: station {k}"
            );
            assert_eq!(bits(f.bulge()), bits(-g.bulge()), "n={n}: piece {k}");
        }
        // Stations sit on the AUTHORED circle, to the leg's rounding.
        let off = worst_radial(&forward, c, r);
        assert!(
            off < 8.0 * f64::EPSILON,
            "n={n}: off the authored circle by {off:e}"
        );
        // Every piece is tan(θ/4n) of the authored sweep.
        let theta = (2.9f64 - 0.4).rem_euclid(core::f64::consts::TAU);
        let piece = (theta / (4.0 * n as f64)).tan();
        let got = forward.vertices()[0].bulge();
        assert!(
            (got - piece).abs() <= 4.0 * f64::EPSILON * piece.abs(),
            "n={n}: piece bulge {got} vs tan(θ/4n) {piece}"
        );
    }
}

/// **A `Via` leg authored backwards.** The PATHS row states the
/// reversal identity for the split without qualifying the mode; a
/// `Via` leg's bulge is DERIVED from three points, so the question is
/// whether the derivation is an exact negation under reversal. This
/// row records the answer either way: bit-identical stations, or the
/// worst ulp gap.
#[test]
fn via_split_reversal_is_measured() {
    let (a, q, b) = (p2(0.3, 0.1), p2(1.1, 1.3), p2(1.7, 0.9));
    let leg = |from: Point2<f64>, to: Point2<f64>, n: usize| {
        pinned(
            Open.at(from)
                .arc_to(Via { q, p: to }.split(n), Tol::witness())
                .unwrap()
                .line_to(Start, Tol::witness())
                .unwrap(),
        )
    };
    let mut worst_ulps = 0u64;
    for n in [3usize, 4, 5] {
        let forward = leg(a, b, n);
        let backward = leg(b, a, n);
        let fb = forward.vertices()[0].bulge();
        let bb = backward.vertices()[0].bulge();
        eprintln!(
            "via n={n}: forward piece {:#018x} backward piece {:#018x} (negated {:#018x})",
            bits(fb),
            bits(bb),
            bits(-bb)
        );
        for k in 1..n {
            let f = forward.vertices()[k];
            let g = backward.vertices()[n - k];
            for (x, y) in [(f.pos().x, g.pos().x), (f.pos().y, g.pos().y)] {
                let gap = (bits(x) as i64 - bits(y) as i64).unsigned_abs();
                worst_ulps = worst_ulps.max(gap);
            }
        }
    }
    eprintln!("via reversal: worst station gap {worst_ulps} ulps");
    assert!(
        worst_ulps <= 4,
        "a Via leg reversed moved a station by {worst_ulps} ulps"
    );
}

/// **The endpoint-free `ArcLen` leg splits and lands where the unsplit
/// leg does**, its stations on the derived carrier.
#[test]
fn arc_len_split_lands_on_the_unsplit_end() {
    let spec = ArcLen {
        r: 0.7,
        side: ArcSide::Right,
        len: 1.1,
    };
    let leg = |n: Option<usize>| {
        let p = Open.at(p2(0.1, 0.2)).angle(0.3, Tol::witness()).unwrap();
        let p = match n {
            None => p.arc_to(spec, Tol::witness()).unwrap(),
            Some(n) => p.arc_to(spec.split(n), Tol::witness()).unwrap(),
        };
        pinned(p.line_to(Start, Tol::witness()).unwrap())
    };
    let plain = leg(None);
    for n in [2usize, 3, 5] {
        let split = leg(Some(n));
        let e = plain.vertices()[1];
        let s = split.vertices()[n];
        assert_eq!(
            (bits(e.pos().x), bits(e.pos().y)),
            (bits(s.pos().x), bits(s.pos().y)),
            "n={n}: the end vertex moved"
        );
        // The derived centre: right of the departure by r.
        let (sx, cx) = 0.3f64.sin_cos();
        let centre = p2(0.1 + sx * 0.7, 0.2 - cx * 0.7);
        let off = worst_radial(&split, centre, 0.7);
        assert!(
            off < 8.0 * f64::EPSILON,
            "n={n}: off the carrier by {off:e}"
        );
        let joints: Vec<usize> = (1..n).collect();
        assert_eq!(split.tangent_joints(), &joints[..]);
    }
}

/// **Full-period and over-full sweeps.** `Sweep` gates its angle
/// positive and nothing else. A split's middle station is placed by the
/// chord's perpendicular, which presumes |θ| ≤ 2π; this row records
/// what the plain leg and the split do at θ = 2π and θ = 3π. The
/// assertion is fail-loud only: no `Ok` loop may carry a non-finite
/// vertex.
#[test]
fn full_and_over_full_sweeps_are_recorded() {
    use core::f64::consts::{FRAC_PI_2, PI, TAU};
    let run = |angle: f64, n: Option<usize>| {
        let p = Open
            .at(p2(1.0, 0.0))
            .angle(FRAC_PI_2, Tol::witness())
            .unwrap();
        let spec = Sweep {
            r: 1.0,
            side: ArcSide::Left,
            angle,
        };
        let r = match n {
            None => p.arc_to(spec, Tol::witness()),
            Some(n) => p.arc_to(spec.split(n), Tol::witness()),
        };
        // A detour before closing, so the LEG's own vertices are seen
        // even where the seam junction would refuse.
        r.map(|p| {
            p.line_to(p2(3.0, 0.0), Tol::witness())
                .and_then(|p| p.line_to(Start, Tol::witness()))
                .map(|c| c.loop_)
        })
    };
    for (angle, name) in [(TAU, "2π"), (3.0 * PI, "3π"), (TAU - 1e-9, "2π − 1e-9")] {
        for n in [None, Some(2), Some(4)] {
            match run(angle, n) {
                Ok(Ok(lp)) => {
                    let vs: Vec<_> = lp
                        .vertices()
                        .iter()
                        .map(|v| (v.pos().x, v.pos().y, v.bulge()))
                        .collect();
                    eprintln!("sweep {name} split {n:?}: Ok {vs:?}");
                    for (x, y, b) in vs {
                        assert!(
                            x.is_finite() && y.is_finite() && b.is_finite(),
                            "sweep {name} split {n:?}: a non-finite vertex was emitted"
                        );
                    }
                }
                Ok(Err(e)) => eprintln!("sweep {name} split {n:?}: closer refused {e}"),
                Err(e) => eprintln!("sweep {name} split {n:?}: leg refused {e}"),
            }
        }
    }
}

/// **A split closer whose seam arrival is DECLARED.** Entry departs +x
/// from the origin; the closer is the CCW semicircle from (0, 1) about
/// (0, ½), arriving heading +x — a tangent seam, declared — split in
/// two. The station is `(−½, ½)` exactly and the joints are the
/// station and the seam.
#[test]
fn a_split_closer_with_a_declared_arrival() {
    let lp = pinned(
        Open.at(p2(0.0, 0.0))
            .line_to(p2(1.0, 0.0), Tol::witness())
            .unwrap()
            .line_to(p2(0.0, 1.0), Tol::witness())
            .unwrap()
            .arc_to(
                Bulge {
                    p: Start.arrives_tangent(),
                    b: 1.0,
                }
                .split(2),
                Tol::witness(),
            )
            .unwrap(),
    );
    assert_eq!(lp.vertices().len(), 4);
    let s = lp.vertices()[3];
    assert_eq!((bits(s.pos().x), bits(s.pos().y)), (bits(-0.5), bits(0.5)));
    let q = core::f64::consts::FRAC_PI_8.tan();
    assert_eq!(bits(lp.vertices()[2].bulge()), bits(q));
    assert_eq!(bits(s.bulge()), bits(q));
    let mut joints = lp.tangent_joints().to_vec();
    joints.sort_unstable();
    assert_eq!(joints, vec![0, 3]);
}

/// **A large count**: 1000 pieces of a semicircle; every station on
/// the carrier, every joint declared, the replay bit-identical, the
/// middle station the axis point exactly.
#[test]
fn a_thousand_piece_split_stays_on_the_carrier() {
    let lp = pinned(bulge_leg(p2(0.0, -0.5), p2(0.0, 0.5), 1.0, 1000));
    assert_eq!(lp.vertices().len(), 1001);
    assert_eq!(lp.tangent_joints().len(), 999);
    let off = worst_radial(&lp, p2(0.0, 0.0), 0.5);
    eprintln!("n=1000 worst relative radial residue {off:e}");
    assert!(off < 8.0 * f64::EPSILON);
    let v500 = lp.vertices()[500];
    assert_eq!(
        (bits(v500.pos().x), bits(v500.pos().y)),
        (bits(0.5), bits(0.0))
    );
}

/// **The count refuses typed on every split-able mode**, not only the
/// `Bulge` leg the unit's row uses: `Via`, `Center`, `Sweep`, `ArcLen`,
/// and a `Start` closer.
#[test]
fn every_split_mode_refuses_a_count_below_two() {
    let is_count = |r: Result<(), PathError<f64>>, n: usize, what: &str| match r {
        Err(PathError::ArcSplitCount { n: got }) => assert_eq!(got, n, "{what}"),
        other => panic!("{what} .split({n}) must refuse ArcSplitCount, got {other:?}"),
    };
    for n in [0usize, 1] {
        let pt = || Open.at(p2(0.0, 0.0));
        is_count(
            pt().arc_to(
                Via {
                    q: p2(0.5, 0.5),
                    p: p2(1.0, 0.0),
                }
                .split(n),
                Tol::witness(),
            )
            .map(|_| ()),
            n,
            "Via",
        );
        is_count(
            pt().arc_to(
                Center {
                    c: p2(0.5, 0.0),
                    winding: ArcSweep::Ccw,
                    p: p2(1.0, 0.0),
                }
                .split(n),
                Tol::witness(),
            )
            .map(|_| ()),
            n,
            "Center",
        );
        let directed = || pt().angle(0.0, Tol::witness()).unwrap();
        is_count(
            directed()
                .arc_to(
                    Sweep {
                        r: 1.0,
                        side: ArcSide::Left,
                        angle: 1.0,
                    }
                    .split(n),
                    Tol::witness(),
                )
                .map(|_| ()),
            n,
            "Sweep",
        );
        is_count(
            directed()
                .arc_to(
                    ArcLen {
                        r: 1.0,
                        side: ArcSide::Left,
                        len: 1.0,
                    }
                    .split(n),
                    Tol::witness(),
                )
                .map(|_| ()),
            n,
            "ArcLen",
        );
        let two = pt().line_to(p2(1.0, 0.0), Tol::witness()).unwrap();
        is_count(
            two.arc_to(Bulge { p: Start, b: 0.5 }.split(n), Tol::witness())
                .map(|_| ()),
            n,
            "Bulge closer",
        );
    }
}
