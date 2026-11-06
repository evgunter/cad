//! [`Curve3::foot`] and [`Curve3::on_carrier`] on every kind: a point a
//! stated number of zero bands off the carrier, read at its foot.

use geom_core::{Band, Point3, Vec3};

use super::{CarrierEscalation, CarrierRefusal, OnCarrier, PieceWalkRefusal};
use crate::Curve3;

const ZERO: f64 = 1e-9;

fn band() -> Band {
    Band::new(ZERO, 10.0 * ZERO).unwrap()
}

fn circle() -> Curve3<f64> {
    Curve3::circle(
        Point3::new(0.0, 0.0, 1.0),
        Vec3::unit_z(),
        2.0,
        Vec3::unit_x(),
    )
}

fn ellipse(a: f64, b: f64) -> Curve3<f64> {
    Curve3::Ellipse(crate::Ellipse::new(
        crate::Frame::new(Point3::new(0.2, -0.1, 0.4), Vec3::unit_z(), Vec3::unit_x()),
        a,
        b,
    ))
}

fn spiric() -> Curve3<f64> {
    Curve3::Spiric {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
        major_radius: 3.0,
        minor_radius: 1.0,
        offset: 0.5,
    }
}

fn spline() -> Curve3<f64> {
    Curve3::Nurbs(std::sync::Arc::new(
        crate::NurbsCurve3::new(
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2)
                .unwrap(),
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(1.0, 0.5, 0.0),
                Point3::new(2.0, -0.5, 0.0),
                Point3::new(3.0, 0.0, 0.0),
            ],
            vec![1.0, 2.0, 1.0, 1.0],
        )
        .unwrap(),
    ))
}

/// The curve's in-plane-or-any normal at `t`.
fn normal(c: &Curve3<f64>, t: f64) -> Vec3<f64> {
    let d = c.ders1(t).1;
    let n = Vec3::unit_z().cross(d);
    if n.norm() > 1e-6 {
        n.normalize()
    } else {
        Vec3::unit_x().cross(d).normalize()
    }
}

fn name<T>(r: Result<OnCarrier<T>, CarrierRefusal>) -> &'static str {
    match r {
        Ok(OnCarrier::On { .. }) => "on",
        Ok(OnCarrier::Off) => "off",
        Err(CarrierRefusal::Escalated(_)) => "escalated",
        Err(CarrierRefusal::Unwalked(PieceWalkRefusal::Exhausted)) => "exhausted",
        Err(CarrierRefusal::Unwalked(PieceWalkRefusal::Unbounded)) => "unbounded",
    }
}

/// **Every kind reads a point by its distance**: half a zero band off
/// the carrier is on it, thirty bands off is off it, read at the foot
/// found from a seed a whole radian away. A thin ellipse (`a/b = 1000`)
/// is the case the unit-coordinate angle alone gets wrong.
#[test]
fn a_point_is_on_or_off_every_carrier_by_its_distance() {
    for (c, t) in [
        (circle(), 0.7),
        (ellipse(2.0, 1.0), 0.7),
        (ellipse(1e3, 1.0), 0.9),
        (ellipse(1.0, 1e-2), 2.1),
        (spiric(), 0.8),
        (spline(), 0.3),
        (spline(), 0.8),
    ] {
        let n = normal(&c, t);
        for (bands, want) in [(0.5, "on"), (30.0, "off")] {
            let q = c.eval(t) + n * (bands * ZERO);
            let got = name(c.on_carrier(q, t + 1.0, band()));
            assert_eq!(got, want, "{:?} at {t}, {bands} bands", c.kind());
        }
    }
    let line = Curve3::line(Point3::new(1.0, 2.0, 3.0), Vec3::new(0.6, 0.0, 0.8));
    let q = line.eval(4.0) + Vec3::unit_y() * (0.5 * ZERO);
    assert_eq!(name(line.on_carrier(q, 0.0, band())), "on");
}

/// **Off needs the whole carrier clear.** An ellipse bending tighter
/// than the band (`b²/a = ε/100`) and a point `20ε` beyond its major
/// vertex: its gap at the vertex is `20ε`, past the band, while the
/// ellipse's lower bound there reads within the zero band, so the two
/// straddle the band and escalate typed, never `Off`.
#[test]
fn a_gap_past_the_band_with_the_bound_inside_it_straddles() {
    let b = (ZERO / 100.0).sqrt();
    let c = ellipse(1.0, b);
    let q = c.eval(0.0) + Vec3::unit_x() * (20.0 * ZERO);
    assert!(matches!(
        c.on_carrier(q, 0.0, band()),
        Err(CarrierRefusal::Escalated(CarrierEscalation::Straddle(_)))
    ));
}

#[test]
fn a_point_in_the_bands_gap_escalates() {
    for (c, t) in [(circle(), 0.7), (ellipse(3.0, 1.0), 1.2), (spiric(), 0.8)] {
        let q = c.eval(t) + normal(&c, t) * (5.0 * ZERO);
        assert_eq!(
            name(c.on_carrier(q, t, band())),
            "escalated",
            "{:?}",
            c.kind()
        );
    }
}

/// **The foot's gap is never worse than its seed's**, and on a thin
/// ellipse it is the point's distance where the unit-coordinate angle
/// reads it `a/b` times farther.
#[test]
fn a_thin_ellipses_foot_reads_the_distance_not_its_unit_image() {
    let c = ellipse(1e3, 1.0);
    let t = 0.9;
    let q = c.eval(t) + normal(&c, t) * ZERO;
    let foot = c.foot(q, 0.0);
    assert!(foot.gap() < 1.5 * ZERO, "{}", foot.gap());
    assert!((foot.seed() - t).abs() < 1e-6, "{}", foot.seed());
    assert!((foot.point() - q).norm() == 0.0);
    assert!(
        foot.nearest().is_none(),
        "an ellipse's foot is a stationary point"
    );
}

/// **A closed spline's foot crosses its seam**: a point on the loop just
/// short of the domain's end, seeded at its start (the same point), and
/// one just past the start seeded at the end. The seed lies nearer than
/// any span sample, and its descent is held at the end it stands on, so
/// the foot is found from the other end, on the carrier.
#[test]
fn a_closed_splines_foot_crosses_its_seam() {
    let c = Curve3::Nurbs(std::sync::Arc::new(
        crate::NurbsCurve3::new(
            geom_core::spline::KnotVector::clamped(
                vec![0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0],
                2,
            )
            .unwrap(),
            vec![
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
                Point3::new(-1.0, 1.0, 0.0),
                Point3::new(-1.0, -1.0, 0.0),
                Point3::new(1.0, -1.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
            ],
            vec![1.0; 6],
        )
        .unwrap(),
    ));
    for (t, seed) in [(0.99, 0.0), (0.01, 1.0)] {
        let q = c.eval(t);
        let foot = c.foot(q, seed);
        assert!(
            foot.gap() < ZERO,
            "t {t} from {seed}: foot at {} gap {:e}",
            foot.seed(),
            foot.gap()
        );
        assert_eq!(
            name(c.on_carrier(q, seed, band())),
            "on",
            "t {t} from {seed}"
        );
    }
}

/// **A closed spline's foot is found from any seed**: two hundred closed
/// quadratic and cubic loops on uneven knots, twenty points each on the
/// loop within 0.03 of its seam, seeded at either end or anywhere: every
/// foot lies on the carrier.
#[test]
fn a_closed_splines_foot_is_found_from_any_seed() {
    let mut s: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut rnd = || {
        s = s
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((s >> 11) as f64) / ((1u64 << 53) as f64)
    };
    let mut miss = 0;
    for case in 0..200 {
        let deg = 2 + case % 2;
        let n = 5 + (rnd() * 6.0) as usize;
        let mut pts: Vec<Point3<f64>> = (0..n)
            .map(|i| {
                let a = std::f64::consts::TAU * i as f64 / n as f64;
                let r = 0.5 + rnd();
                Point3::new(r * a.cos(), r * a.sin(), 0.3 * (rnd() - 0.5))
            })
            .collect();
        pts.push(pts[0]);
        let m = pts.len();
        let w: Vec<f64> = (0..m - deg - 1).map(|_| 0.2 + rnd()).collect();
        let tot = w.iter().sum::<f64>() + 0.2 + rnd();
        let mut ks = vec![0.0; deg + 1];
        let mut c = 0.0;
        for a in &w {
            c += a / tot;
            ks.push(c);
        }
        ks.extend(vec![1.0; deg + 1]);
        let kv = geom_core::spline::KnotVector::clamped(ks, deg).unwrap();
        let curve = Curve3::Nurbs(std::sync::Arc::new(
            crate::NurbsCurve3::new(kv, pts, vec![1.0; m]).unwrap(),
        ));
        for _ in 0..20 {
            let t = if rnd() < 0.5 {
                rnd() * 0.03
            } else {
                1.0 - rnd() * 0.03
            };
            let seed = [0.0, 1.0, rnd()][(rnd() * 3.0) as usize];
            // A NaN gap is a miss too.
            if curve.foot(curve.eval(t), seed).gap().partial_cmp(&ZERO)
                != Some(core::cmp::Ordering::Less)
            {
                miss += 1;
            }
        }
    }
    assert_eq!(
        miss, 0,
        "{miss} points on a closed spline missed by their foot"
    );
}

/// A cubic hairpin with branches `h` apart, and the point on its near
/// branch at `t = 0.08`.
fn hairpin(h: f64) -> (Curve3<f64>, Point3<f64>) {
    let c = Curve3::Nurbs(std::sync::Arc::new(
        crate::NurbsCurve3::new(
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3)
                .unwrap(),
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(1.4, 0.0, 0.0),
                Point3::new(1.4, h, 0.0),
                Point3::new(0.0, h, 0.0),
            ],
            vec![1.0; 4],
        )
        .unwrap(),
    ));
    let p = c.eval(0.08);
    (c, p)
}

/// The point `p` widened to an `Interval` box of half-width `r`.
fn boxed(p: Point3<f64>, r: f64) -> Point3<geom_core::Interval> {
    let w = |x: f64| geom_core::Interval::from_bounds(x - r, x + r);
    Point3::new(w(p.x), w(p.y), w(p.z))
}

/// **A tie across a hairpin's branches is the certificate's to read**.
/// The hairpin's branches lie 0.2 apart. On an `Interval` box about a
/// point on the near branch, seeded on the far one, a wide box ties
/// every polish candidate and the polish keeps the seed's branch. That
/// pick is a heuristic's and stays out of the log (ledger row F32).
/// What a reader can hold is the certificate, and across the box ladder
/// it never says `On` at the far branch:
///
/// - ±0.05 and ±1e-3: the spline's piece walk runs past its budget
///   (`Unwalked(Exhausted)`), a refusal no narrower box in that range
///   answers (`a-splines-on-carrier-exhausts-its-piece-budget-at-interval`);
/// - ±1e-6: an escalation in the band (`Escalated(InBand)`), the
///   driver's cue to refine;
/// - ±1e-11: `On` at the near branch, `t ≈ 0.08`.
#[test]
fn a_tie_across_a_hairpins_branches_is_the_certificates_to_read() {
    use geom_core::{Bounds, Interval, Real, k_stats::detached};
    let (c, p) = hairpin(0.2);
    let ci: Curve3<Interval> = c.map_scalar(Interval::from_f64);
    let seed = Interval::from_f64(0.92);
    let (foot, rec) = detached(|| ci.foot(boxed(p, 0.05), seed));
    assert!(
        foot.seed().lo() > 0.5,
        "the tie keeps the seed's branch: {:?}",
        foot.seed()
    );
    assert!(
        !rec.recorded()
            .escalations
            .iter()
            .any(|e| e.predicate() == "curve_foot_polish"),
        "the polish's tie-break is a heuristic's and stays out of the log"
    );
    for r in [0.05, 1e-3] {
        assert_eq!(
            name(ci.on_carrier(boxed(p, r), seed, band())),
            "exhausted",
            "±{r}: the piece walk runs past its budget"
        );
    }
    assert!(
        matches!(
            ci.on_carrier(boxed(p, 1e-6), seed, band()),
            Err(CarrierRefusal::Escalated(CarrierEscalation::InBand(_)))
        ),
        "±1e-6: the certificate escalates in the band"
    );
    let narrow = ci.on_carrier(boxed(p, 1e-11), seed, band());
    assert!(
        matches!(narrow, Ok(OnCarrier::On { t, .. }) if t.lo() > 0.08 - 1e-6 && t.hi() < 0.08 + 1e-6),
        "±1e-11: on at the near branch, not the seed's: {narrow:?}"
    );
}

/// **Below the band's zero, `On` certifies the point and not the
/// branch**. A hairpin whose branches lie 3e-10 apart, inside the 1e-9
/// zero, reads `On` at the seed's branch at `Interval` (`t = 0.92`, the
/// reversed tangent) with nothing logged, and `On` at the near branch at
/// `f64`. Both are true of the point: it lies within the band of the
/// carrier at either parameter. A reader of the branch (a tangent)
/// rests on the carrier's branches being decided apart, which this
/// carrier's are not (`a-carriers-branches-within-the-band-of-each-other`).
#[test]
fn a_hairpin_closer_than_the_band_certifies_on_at_the_seeds_branch_at_interval() {
    use geom_core::{Bounds, Interval, Real};
    let (c, p) = hairpin(3e-10);
    let ci: Curve3<Interval> = c.map_scalar(Interval::from_f64);
    let on = ci.on_carrier(boxed(p, 1e-10), Interval::from_f64(0.92), band());
    assert!(
        matches!(on, Ok(OnCarrier::On { t, .. }) if t.lo() > 0.5),
        "On's t is the seed's branch below the band: {on:?}"
    );
    assert!(
        matches!(c.on_carrier(p, 0.92, band()), Ok(OnCarrier::On { t, .. }) if t < 0.5),
        "at f64 the polish finds the near branch"
    );
}

/// **A spline's foot finds the point's own branch from any seed**: on
/// a cubic spline of five knot spans, a point on the carrier at
/// `t = 0.35` lies 0.05 from a stationary point near `t = 0.03` whose
/// basin holds the least-gap sample. Polishing from that sample alone
/// lands there; the search also polishes from each local minimum of the
/// sampled gaps and the few least gaps, and reads `On` at `t = 0.35`
/// from every seed.
#[test]
fn a_splines_foot_finds_the_points_own_branch_from_any_seed() {
    let c = Curve3::Nurbs(std::sync::Arc::new(
        crate::NurbsCurve3::new(
            geom_core::spline::KnotVector::clamped(
                vec![0.0, 0.0, 0.0, 0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 1.0, 1.0, 1.0],
                3,
            )
            .unwrap(),
            vec![
                Point3::new(0.25350192097619306, 1.366246580479202, 0.10734265927471985),
                Point3::new(
                    0.8036949878629633,
                    0.013067913669976727,
                    0.03308834410972377,
                ),
                Point3::new(1.480946409416911, 0.4537707418975514, 0.1764560519141859),
                Point3::new(0.03646373591417751, 1.0290841026582689, 0.07594285140734353),
                Point3::new(0.4630589620214056, 1.7067471422313527, 0.031091972942131577),
                Point3::new(1.1544836707277542, 0.48528073793130444, 0.01100075298321821),
                Point3::new(0.7893432608276867, 1.5915056214557455, 0.025122531729525922),
                Point3::new(1.2935210084059152, 1.3895124153536436, 0.08367157011772855),
            ],
            vec![1.0; 8],
        )
        .unwrap(),
    ));
    let p = c.eval(0.35);
    for seed in [0.0, 0.3, 0.5, 1.0] {
        let read = c.foot(p, seed).on_carrier(band());
        assert!(
            matches!(read, Ok(OnCarrier::On { t, .. }) if (t - 0.35).abs() < 1e-6),
            "seed {seed}: on at the point's own branch: {read:?}"
        );
    }
}

/// The reads of a foot's seed in `code` (comment- and string-blanked
/// text): each `.seed()` call, and each path from `Foot` to `::seed` in
/// any spelling (`Foot::seed`, `Foot::<f64>::seed`, the qualified
/// `<geom::Foot<'_, f64>>::seed`, a method reference `.map(Foot::seed)`),
/// the path ending at the first `(`, `)`, `;`, `{` or `}`.
fn seed_reads(code: &str) -> usize {
    let ident = |c: char| c.is_alphanumeric() || c == '_';
    let calls = code.matches(".seed()").count();
    let paths = code
        .match_indices("Foot")
        .filter(|&(at, _)| !code[..at].ends_with(ident))
        .filter(|&(at, _)| {
            let rest = &code[at + 4..];
            let end = rest.find(['(', ')', ';', '{', '}']).unwrap_or(rest.len());
            rest[..end].trim_end().ends_with("::seed")
                || rest[..end]
                    .find("::seed")
                    .is_some_and(|k| !rest[k + 6..].starts_with(ident))
        })
        .count();
    calls + paths
}

/// **`seed_reads` sees every spelling**: a row a bug in the count could
/// break, on text, not on the tree.
#[test]
fn every_spelling_of_a_seed_read_is_counted() {
    for (code, n) in [
        ("f.seed()", 1),
        ("geom::Foot::seed(f)", 1),
        ("geom::Foot::<f64>::seed(f)", 1),
        ("<geom::Foot<'_, f64>>::seed(f)", 1),
        ("feet.map(Foot::seed)", 1),
        ("fn _v(f: Foot<'_, f64>) -> f64 { f.seed() }", 1),
        ("Foot::seed_of(f); BigFoot::seed(f); Foot::nearest(f)", 0),
        ("cell.seed(); a.seed()", 2),
    ] {
        assert_eq!(seed_reads(code), n, "{code}");
    }
}

/// **A foot's seed is read only where it seeds a search or a bound**
/// (module docs, "A foot has no bare parameter"). [`super::Foot::seed`]
/// returns the same field as [`OnCarrier::On`]'s `t`, so the type alone
/// cannot keep a verdict reader off it: every file under `crates/*/src`
/// that reads it ([`seed_reads`]) is listed here with the number of reads
/// and what they seed. A new read goes red here, in a listed file or not,
/// and owes either a certificate (`On`'s `t`, [`super::Foot::nearest`])
/// or its count raised with its reason.
///
/// Blind spots: `.seed()` is a method name, not a type, so another
/// type's `seed()` is counted too and listed as such; a `Foot` renamed on
/// import (`use geom::Foot as F`) is not seen; and `tests/`, `benches`
/// and `demos` are not scanned (only `crates/*/src`, where the
/// `#[cfg(test)]` rows are).
#[test]
fn a_foots_seed_is_read_only_where_it_seeds_a_search_or_a_bound() {
    const READERS: &[(&str, usize, &str)] = &[
        (
            "geom/src/curves/on_carrier/rows.rs",
            6,
            "the foot's own rows",
        ),
        (
            "geom-brep/src/curve_pair/walk.rs",
            3,
            "the walk's feet seed `read_both`, which reads the span's certificate",
        ),
        (
            "geom-brep/src/curve_pair/pose.rs",
            1,
            "a candidate's parameters seed the pose's reading of the pair",
        ),
        (
            "geom-brep/src/curve_pair/pose/window.rs",
            1,
            "the bound along `b` reads the speed at the seed, anchored on the gap",
        ),
        (
            "geom-brep/src/curve_pair/span.rs",
            1,
            "`Span::place` reads the ends' sides at the seed; `Span::read` takes \
             `On`'s `t` for `Reading::On`. `place` is itself a reading, which \
             `curved_edges::passes` reads directly \
             (`curved-edges-passes-hand-composes-on-carrier-and-span-place`)",
        ),
        (
            "topo/src/boolean/solid_contain/curved_edges.rs",
            1,
            "the ray's foot iteration seeds `on_carrier` and `Span::place` at its point \
             (`curved-edges-passes-hand-composes-on-carrier-and-span-place`)",
        ),
        (
            "geom-brep/src/ssi/exhaust.rs",
            2,
            "a cell's own `seed()`, not a foot's",
        ),
    ];
    let crates = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut found: Vec<(String, usize)> = test_utils::source::rust_sources(&crates)
        .into_iter()
        .filter_map(|f| {
            let p = f
                .strip_prefix(&crates)
                .ok()?
                .to_string_lossy()
                .replace('\\', "/");
            if !p.contains("/src/") {
                return None;
            }
            let code = test_utils::source::code_only(&std::fs::read_to_string(&f).unwrap());
            let n = seed_reads(&code);
            (n > 0).then_some((p, n))
        })
        .collect();
    found.sort();
    let mut want: Vec<(String, usize)> =
        READERS.iter().map(|&(p, n, _)| (p.to_owned(), n)).collect();
    want.sort();
    assert_eq!(
        found, want,
        "a foot's seed read outside its list, or a listed count moved"
    );
}

#[test]
#[ignore = "measurement: cargo nextest run -p geom --cargo-profile release --run-ignored only -E 'test(measure_spline_foot_misses)' --no-capture"]
fn measure_spline_foot_misses() {
    let mut rng = test_utils::fuzz::Rng::from_seed(0x5eed_f007);
    let (mut misses, mut total) = (0usize, 0usize);
    let mut curves_missing = 0usize;
    let mut spent = std::time::Duration::ZERO;
    for _ in 0..300 {
        let ctrl: Vec<Point3<f64>> = (0..8)
            .map(|_| {
                Point3::new(
                    rng.range(0.0, 1.5),
                    rng.range(0.0, 1.8),
                    rng.range(0.0, 0.2),
                )
            })
            .collect();
        let c = Curve3::Nurbs(std::sync::Arc::new(
            crate::NurbsCurve3::new(
                geom_core::spline::KnotVector::clamped(
                    vec![0.0, 0.0, 0.0, 0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 1.0, 1.0, 1.0],
                    3,
                )
                .unwrap(),
                ctrl,
                vec![1.0; 8],
            )
            .unwrap(),
        ));
        let before = misses;
        for k in 1..100 {
            let p = c.eval(f64::from(k) / 100.0);
            for s in 0..21 {
                total += 1;
                let clock = std::time::Instant::now();
                let gap = c.foot(p, f64::from(s) / 20.0).gap();
                spent += clock.elapsed();
                if gap > 1e-6 {
                    misses += 1;
                }
            }
        }
        curves_missing += usize::from(misses > before);
    }
    eprintln!(
        "MEASURE misses {misses} of {total}; curves with a miss {curves_missing} of 300; {:.2} us a foot",
        spent.as_secs_f64() * 1e6 / total as f64
    );
}
