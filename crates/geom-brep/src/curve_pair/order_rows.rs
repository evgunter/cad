//! [`meet`] read in either order: `meet(a, b)` and `meet(b, a)` give one
//! answer with the roles swapped, and the two-conic pairs whose answer
//! the conic read across the other's plane decided
//! ([`super::pose`]'s docs, "Two conics").

#![allow(clippy::unwrap_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/curve_pair.rs",
    "crates/geom-brep/src/curve_pair/",
    "crates/geom/src/carrier_speed.rs",
];

use core::f64::consts::{PI, TAU};

use geom::Curve3;
use geom_core::{Band, Point3, Vec3};
use test_utils::fuzz;

use super::meet_rows::{band, circle, ellipse, line, shallow_circles, touching_circles, u_for};
use super::{Hit, MeetFault, MeetRefusal, Meetings, Pose, Span, meet, pose};

type Read = Result<Meetings<f64>, MeetRefusal<f64>>;

/// Two spans, each a carrier over its parameters.
type Pair = [(Curve3<f64>, (f64, f64)); 2];

fn read(a: (&Curve3<f64>, (f64, f64)), b: (&Curve3<f64>, (f64, f64)), band: Band) -> Read {
    meet(&Span::new(a.0, a.1), &Span::new(b.0, b.1), band)
}

/// Whether two hits on one span are one place: the same end, or
/// parameters within `slack`, modulo a turn on a closed carrier.
fn same_hit(x: &Hit<f64>, y: &Hit<f64>, closed: bool, slack: f64) -> bool {
    match (x, y) {
        (Hit::On { t: s, .. }, Hit::On { t, .. }) => {
            let d = s - t;
            let d = if closed {
                d.rem_euclid(TAU).min(TAU - d.rem_euclid(TAU))
            } else {
                d.abs()
            };
            d <= slack
        }
        (Hit::End(p), Hit::End(q)) => p == q,
        _ => false,
    }
}

/// **One answer read in two orders**, `a` and `b` the carriers of
/// `got`'s first and second spans. Both orders may refuse: two
/// refusals agree whatever they name, since neither is an answer
/// (D4). One order deciding where the other refuses is a disagreement,
/// as is any difference in the shared run or the meetings: each of
/// `got`'s meetings must have a partner in `back` within the band's
/// escalation, sitting at the same place on each span, its parameter
/// within a thousand escalations (a turn's worth on a closed carrier).
fn agree(
    label: &str,
    (a, b): (&Curve3<f64>, &Curve3<f64>),
    got: &Read,
    back: &Read,
    band: Band,
) -> Option<String> {
    let (got, back) = match (got, back) {
        (Err(_), Err(_)) => return None,
        (Ok(g), Ok(b)) => (g, b),
        _ => return Some(format!("{label}: one order decides\n  {got:?}\n  {back:?}")),
    };
    if got.shared != back.shared || got.points.len() != back.points.len() {
        return Some(format!("{label}: answers differ\n  {got:?}\n  {back:?}"));
    }
    let e = band.escalate();
    let slack = 1e3 * e;
    for p in &got.points {
        let partner = back.points.iter().any(|q| {
            (p.point - q.point).norm() <= e
                && same_hit(&p.on_a, &q.on_b, a.is_periodic(), slack)
                && same_hit(&p.on_b, &q.on_a, b.is_periodic(), slack)
        });
        if !partner {
            return Some(format!("{label}: {p:?} has no swapped partner in {back:?}"));
        }
    }
    None
}

/// Each pair read in both orders, the failures collected.
fn every(pairs: &[(String, Pair)], band: Band) -> Vec<String> {
    pairs
        .iter()
        .filter_map(|(label, [(a, sa), (b, sb)])| {
            agree(
                label,
                (a, b),
                &read((a, *sa), (b, *sb), band),
                &read((b, *sb), (a, *sa), band),
                band,
            )
        })
        .collect()
}

/// **`meet` reads one answer in either order** over the meeting rows'
/// fixtures: two circles on one sphere at every tilt, circles crossing
/// across nearly parallel planes and touching the other's plane, a
/// thin ellipse in a circle's plane, near-concentric circles and their
/// ellipse twins, and a line against each. Red where the two-conic arm
/// parametrizes whichever conic comes first.
#[test]
fn meet_reads_one_answer_in_either_order() {
    let mut pairs = Vec::new();
    let centre = Point3::new(3.7, -2.1, 0.4);
    let radius = 1.3;
    for theta in [1e-2_f64, 1e-3, 3e-4, 1e-4, 1e-5, 1e-7, 1e-11] {
        let h = 0.5;
        let (s, c) = theta.sin_cos();
        let foot = |n: Vec3<f64>| {
            let d = Vec3::new(0.0, h, 0.0).dot(n);
            (centre + n * d, (radius * radius - d * d).sqrt())
        };
        let (n1, n2) = (Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, c, s));
        let ((c1, r1), (c2, r2)) = (foot(n1), foot(n2));
        let span = (0.05, TAU - 0.05);
        pairs.push((
            format!("one sphere, tilt {theta:e}"),
            [
                (circle(c1.to_array(), n1.to_array(), r1), span),
                (circle(c2.to_array(), n2.to_array(), r2), span),
            ],
        ));
    }
    for tilt in [2e-8, 1e-7, 1e-5, 1e-3] {
        for h in [0.5e-9, 3e-8, 1e-3] {
            pairs.push((
                format!("shallow, tilt {tilt:e}, h {h:e}"),
                shallow_circles(tilt, h),
            ));
        }
        let [_, b] = shallow_circles(tilt, 0.5e-9);
        pairs.push((
            format!("line × shallow, tilt {tilt:e}"),
            [(line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]), (-1.0, 1.0)), b],
        ));
    }
    for tilt in [1.2e-8_f64, 1e-6, 1e-4] {
        for angle in [2.0_f64, 20.0, 90.0] {
            let along: f64 = 1e-3;
            let lift = tilt.sin() * (1.0 - along.cos());
            pairs.push((
                format!("touching, tilt {tilt:e}, crossing {angle}°"),
                touching_circles(tilt, 6.5e-10 - lift, along, angle.to_radians()),
            ));
        }
    }
    for (b_over_a, tilt) in [(1e-3, 5e-7), (1e-2, 5e-8), (0.5, 0.3)] {
        let (s, c) = f64::sin_cos(tilt);
        pairs.push((
            format!("thin ellipse, b/a {b_over_a:e}, tilt {tilt:e}"),
            [
                (
                    ellipse(
                        [0.0, 0.0, 0.0],
                        [0.0, -s, c],
                        [1.0, 0.0, 0.0],
                        1.0,
                        b_over_a,
                    ),
                    (-PI, PI),
                ),
                (
                    circle([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.5),
                    (0.1, TAU - 0.1),
                ),
            ],
        ));
    }
    let unit = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    for (gap, tilt) in [(2e-10, 1e-10), (7e-10, 5e-10), (3e-10, 6e-10)] {
        let (s, c) = f64::sin_cos(tilt);
        let r = 1.0 + gap;
        pairs.push((
            format!("near-concentric, gap {gap:e}, tilt {tilt:e}"),
            [
                (unit.clone(), (0.3, 2.9)),
                (circle([0.0, 0.0, 0.0], [0.0, -s, c], r), (0.5, 3.1)),
            ],
        ));
        pairs.push((
            format!("near-concentric ellipse, gap {gap:e}, tilt {tilt:e}"),
            [
                (unit.clone(), (0.3, 2.9)),
                (
                    ellipse([0.0, 0.0, 0.0], [0.0, -s, c], [1.0, 0.0, 0.0], r, r),
                    (0.5, 3.1),
                ),
            ],
        ));
    }
    let bad = every(&pairs, band());
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

fn sph(polar: f64, az: f64) -> Vec3<f64> {
    Vec3::new(polar.sin() * az.cos(), polar.sin() * az.sin(), polar.cos())
}

/// **A parallel tilted off the equator, clear of the equator's plane,
/// reads apart whatever its tilt reads** (the slim lune's second leg
/// against its equatorial arc). On a sphere of radius `8.7·10⁻⁴`, the
/// parallel about `p`, `off` from the pole, through a corner at polar
/// `0.75π`, lies about `6·10⁻⁴` below the equator's plane, a plain miss,
/// while its swing above that plane, `r·sin(off)`, is in the band's gap
/// at `off = 3·10⁻³` and `10⁻²`. Red where the swing routes before the
/// offset is read.
#[test]
fn a_parallel_clear_of_the_equators_plane_reads_apart() {
    let band = Band::new(1e-6, 1e-5).unwrap();
    let c = Point3::new(3.0, -2.0, 5.0);
    let r = 8.726e-4;
    let z = Vec3::new(0.0, 0.0, 1.0);
    let edge = Curve3::circle(c, z, r, Vec3::new(1.0, 0.0, 0.0));
    let edge_span = (1.0, 1.0 + 1f64.to_radians());
    let target = sph(0.75 * PI, 1.0);
    for off in [0.0, 1e-3, 3e-3, 1e-2] {
        let p = sph(off, 3.0);
        let (cos, sin) = (p.dot(target), p.cross(target).norm());
        let leg = Curve3::circle(c + p * (r * cos), p, r * sin, u_for(p));
        let label = format!("off {off:e}");
        for (got, order) in [
            (
                pose(
                    &Span::new(&leg, (0.0, TAU)),
                    &Span::new(&edge, edge_span),
                    band,
                ),
                "leg first",
            ),
            (
                pose(
                    &Span::new(&edge, edge_span),
                    &Span::new(&leg, (0.0, TAU)),
                    band,
                ),
                "arc first",
            ),
        ] {
            assert!(matches!(got, Ok(Pose::Apart)), "{label}, {order}: {got:?}");
        }
        let m = read((&leg, (0.0, TAU)), (&edge, edge_span), band).unwrap();
        assert!(m.points.is_empty(), "{label}: {m:?}");
    }
}

/// **A great circle through a tiny cap crosses its rim twice, read in
/// either order** (the tiny cap's ray against its rim). The unit sphere's
/// rim at polar `10⁻⁴`, and a great circle through `p`, `off` from the
/// pole, along a tangent at `az`: its height above the rim's plane peaks
/// at about `4·10⁻⁹`, in the band's gap, while the rim crosses the great
/// circle's plane square to it. Red where the great circle, the conic
/// that comes first, is the one read across the other's plane.
#[test]
fn a_great_circle_crosses_a_tiny_rim_in_either_order() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let o = Point3::origin();
    let z = Vec3::new(0.0, 0.0, 1.0);
    let polar = 1e-4f64;
    let rim = Curve3::circle(
        o + z * polar.cos(),
        z,
        polar.sin(),
        Vec3::new(1.0, 0.0, 0.0),
    );
    for off in [5e-5, 2e-5] {
        let p = sph(off, 0.3);
        for az in [0.0f64, 1.0, 2.0] {
            let e1 = u_for(p);
            let t = e1 * az.cos() + p.cross(e1) * az.sin();
            let ray = Curve3::circle(o, p.cross(t).normalize(), 1.0, p);
            let label = format!("off {off:e}, az {az}");
            let got = read((&ray, (0.0, TAU)), (&rim, (0.0, TAU)), band);
            let back = read((&rim, (0.0, TAU)), (&ray, (0.0, TAU)), band);
            for (m, order) in [(&got, "ray first"), (&back, "rim first")] {
                let m = m
                    .as_ref()
                    .unwrap_or_else(|e| panic!("{label}, {order}: {e:?}"));
                assert_eq!(m.points.len(), 2, "{label}, {order}: {m:?}");
                for q in &m.points {
                    let height = (q.point - o).norm() - 1.0;
                    let off_rim = (q.point.z - polar.cos()).abs();
                    assert!(
                        height.abs() <= 1e-9 && off_rim <= 1e-9,
                        "{label}, {order}: {q:?} lies off the sphere or the rim's plane"
                    );
                }
            }
            assert!(
                agree(&label, (&ray, &rim), &got, &back, band).is_none(),
                "{label}: orders differ"
            );
        }
    }
}

/// **A torus meridian meets a parallel near the tube's top circle once,
/// read in either order.** On the torus `R = 3`, `r = 1` about `ẑ`, the
/// meridian in `y = 0` and the parallel at `v = π/2 − δ`: the parallel's
/// plane is tangent to the meridian near its top, so the meridian's
/// height above it has a near-double root, while the parallel crosses
/// the meridian's plane square to it. Both orders read the parallel
/// across the meridian's plane and meet at `(R + r·cos v, 0, r·sin v)`.
#[test]
fn a_meridian_meets_a_parallel_by_the_tubes_top_once() {
    let (major, minor) = (3.0, 1.0);
    let meridian = circle([major, 0.0, 0.0], [0.0, 1.0, 0.0], minor);
    for delta in [1e-2, 1e-4, 1e-6, 0.0] {
        let v = PI / 2.0 - delta;
        let parallel = circle(
            [0.0, 0.0, minor * v.sin()],
            [0.0, 0.0, 1.0],
            major + minor * v.cos(),
        );
        let at = Point3::new(major + minor * v.cos(), 0.0, minor * v.sin());
        let label = format!("δ {delta:e}");
        let got = read((&meridian, (0.0, TAU)), (&parallel, (0.0, TAU)), band());
        let back = read((&parallel, (0.0, TAU)), (&meridian, (0.0, TAU)), band());
        for (m, order) in [(&got, "meridian first"), (&back, "parallel first")] {
            let m = m
                .as_ref()
                .unwrap_or_else(|e| panic!("{label}, {order}: {e:?}"));
            assert!(
                m.points.len() == 1 && (m.points[0].point - at).norm() <= 1e-9,
                "{label}, {order}: one meeting at {at:?}: {m:?}"
            );
        }
        assert!(
            agree(&label, (&meridian, &parallel), &got, &back, band()).is_none(),
            "{label}: orders differ"
        );
    }
}

/// **A conic whose swing is in the gap beside a level conic reads one
/// answer in either order** (the review of PR 6492). A circle of radius
/// 0.4 in `z = 0`, and one of radius 0.02 tilted `7·10⁻⁹`, `6·10⁻¹⁰`
/// above it, `dx` along: the small circle's swing above the large one's
/// plane is `1.4·10⁻¹⁰`, zero; the large one's above the small one's is
/// `2.8·10⁻⁹`, in the gap. At `dx` 0.1 and 0.3 they miss by centimetres;
/// at 0.39 their least distance is `4.8·10⁻¹⁰`. Red where the first
/// conic's gap swing escalates before the second's zero swing is read.
#[test]
fn a_gap_swing_beside_a_level_conic_reads_one_answer_in_either_order() {
    for dx in [0.1, 0.3, 0.39] {
        let a = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.4);
        let b = circle([dx, 0.0, 6e-10], [0.0, 7e-9f64.sin(), 7e-9f64.cos()], 0.02);
        let s = (0.0, TAU);
        let got = read((&a, s), (&b, s), band());
        let back = read((&b, s), (&a, s), band());
        assert!(
            got.is_ok(),
            "dx {dx}: a level conic beside a gap swing decides: {got:?}"
        );
        assert!(
            agree(&format!("dx {dx}"), (&a, &b), &got, &back, band()).is_none(),
            "{got:?}\n{back:?}"
        );
    }
}

/// **A tied choice of the conic read across records no escalation**
/// (the review of PR 6492). Two equal great circles crossing
/// symmetrically, at `Interval`: the two `(A² − c₀²)/r²` tie, so their
/// difference straddles zero. The choice is between two sound readings,
/// taken off the log (ledger row F33); logged, it put
/// `curve_pair_pose_across` in the escalation record on every run.
#[test]
fn a_tied_across_choice_records_no_escalation() {
    use geom_core::{Interval, k_stats::Bracket};
    let i = |x: f64| Interval::from_bounds(x - 1e-15, x + 1e-15);
    let (s, c) = (0.25f64.sin(), 0.25f64.cos());
    let mk = |y: f64| {
        let n = Vec3::new(0.0, y, c);
        let u = u_for(n);
        Curve3::circle(
            Point3::new(i(0.3), i(-0.2), i(0.1)),
            Vec3::new(i(n.x), i(n.y), i(n.z)),
            i(1.0),
            Vec3::new(i(u.x), i(u.y), i(u.z)),
        )
    };
    let (a, b) = (mk(s), mk(-s));
    let br = Bracket::open();
    let got = meet(
        &Span::new(&a, (i(0.1), i(6.0))),
        &Span::new(&b, (i(0.1), i(6.0))),
        band(),
    );
    let rec = br.finish();
    assert!(
        rec.escalations
            .iter()
            .all(|e| e.predicate() != "curve_pair_pose_across"),
        "the across choice recorded an escalation: {got:?}"
    );
}

/// A near-parallel, near-coplanar or near-tangent conic pair (the review
/// of PR 6492): a circle, and a circle or an ellipse whose plane is
/// tilted from 0 to `10⁻²` off the first's, set a zero, an escalation,
/// its swing or its swing and an escalation off the first's plane.
fn near_pair(rng: &mut fuzz::Rng, k: usize) -> (Curve3<f64>, Curve3<f64>, String) {
    let bd = band();
    let n = Vec3::new(
        rng.range(-1.0, 1.0),
        rng.range(-1.0, 1.0),
        rng.range(-1.0, 1.0),
    )
    .normalize();
    let ca = Point3::new(
        rng.range(-5.0, 5.0),
        rng.range(-5.0, 5.0),
        rng.range(-5.0, 5.0),
    );
    let ra = rng.log_uniform(1e-4, 3.0);
    let tilt = if k.is_multiple_of(3) {
        0.0
    } else {
        rng.log_uniform(1e-13, 1e-2)
    };
    let az = rng.range(0.0, TAU);
    let nb = {
        let (u, w) = (u_for(n), n.cross(u_for(n)));
        (n * tilt.cos() + (u * az.cos() + w * az.sin()) * tilt.sin()).normalize()
    };
    let scale = [
        bd.zero(),
        bd.escalate(),
        ra * tilt.sin(),
        ra * tilt.sin() + bd.escalate(),
    ][k % 4];
    let side = if rng.unit() < 0.5 { -1.0 } else { 1.0 };
    let h = scale * rng.range(0.5, 2.0) * side;
    let inplane =
        u_for(n) * rng.range(-2.0, 2.0) * ra + n.cross(u_for(n)) * rng.range(-2.0, 2.0) * ra;
    let cb = ca + n * h + inplane;
    let rb = rng.log_uniform(1e-4, 3.0);
    let a = circle(ca.to_array(), n.to_array(), ra);
    let b = if k % 5 == 1 {
        let minor = rb * rng.range(0.05, 0.95);
        ellipse(
            cb.to_array(),
            nb.to_array(),
            u_for(nb).to_array(),
            rb,
            minor,
        )
    } else {
        circle(cb.to_array(), nb.to_array(), rb)
    };
    (
        a,
        b,
        format!("draw {k}: ra {ra:e} rb {rb:e} tilt {tilt:e} h {h:e}"),
    )
}

/// **`meet` reads one answer in either order, over random near-parallel
/// conic pairs** (the review of PR 6492's fuzz): every draw read
/// `meet(a, b)` and `meet(b, a)` agrees ([`agree`]: both refuse, or one
/// answer with the roles swapped, parameters per side). A counterexample
/// search: the seed varies, `CAD_FUZZ_SEED` replays it, and the count
/// scales with `CAD_FUZZ_EFFORT`. It found 26 asymmetric draws of 600 on
/// the parent of PR 6492 and 3 on its first head, the swing read in
/// order.
#[test]
fn meet_reads_one_answer_in_either_order_over_random_near_parallel_pairs() {
    let mut rng = fuzz::start("curve_pair order symmetry");
    let bd = band();
    let (sa, sb) = ((0.2, 5.9), (0.4, 6.1));
    let mut bad = Vec::new();
    for k in 0..fuzz::scaled(600) {
        let (a, b, label) = near_pair(&mut rng, k);
        let got = read((&a, sa), (&b, sb), bd);
        let back = read((&b, sb), (&a, sa), bd);
        if let Some(why) = agree(&label, (&a, &b), &got, &back, bd) {
            bad.push(why);
        }
    }
    assert!(
        bad.is_empty(),
        "{} asymmetric draws ({}):\n{}",
        bad.len(),
        fuzz::replay(),
        bad.iter().take(6).cloned().collect::<Vec<_>>().join("\n")
    );
}

/// **A level conic's offset at the screen's flip is no contradiction**
/// (the delta review of PR 6492). A circle 250 m out and a small circle
/// tilted `2·10⁻⁸` beside it, its centre a few ulps either side of the
/// height where the parallel screen's margin flips: the screen and the
/// level conic's offset read one margin, so neither can read clear where
/// the other did not. Red where the two were spelled apart and their
/// rounding disagreement was raised as `MeetFault::Contradiction`.
#[test]
fn a_level_conics_offset_at_the_screens_flip_is_no_contradiction() {
    let t = 2e-8f64;
    let a = circle([-250.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.4);
    let h0 = 1.0400888888962435e-8f64;
    for d in -2i64..=2 {
        let h = f64::from_bits(h0.to_bits().wrapping_add_signed(d));
        let b = circle([-249.6, 0.0, h], [0.0, t.sin(), t.cos()], 0.02);
        let s = (0.0, TAU);
        for got in [
            read((&a, s), (&b, s), band()),
            read((&b, s), (&a, s), band()),
        ] {
            assert!(
                !matches!(&got, Err(e) if matches!(e.fault, MeetFault::Contradiction { .. })),
                "h {h:e}: {got:?}"
            );
        }
    }
}
