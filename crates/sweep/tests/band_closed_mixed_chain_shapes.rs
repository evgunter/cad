//! **Rounded outlines of every shape fillet whole**, each against
//! Pappus's closed form, read off the outline's own pieces, and a
//! membership grid read off its signed distance: rotated rectangles,
//! rounded polygons, a rotated slot, arcs flanking a short flat, a
//! four-arc barrel, a hole beside an arc, a crease. The harness is the
//! one review lane band-chain-r2 built for PR 6520.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::too_many_lines, clippy::many_single_char_names)]

use core::f64::consts::PI;

use geom_core::{Band, Bounds, CertifiedBounds, Decide, Interval, Point2, Point3, Tol};
use profile::{SketchPlane, test_support::bulge_loop};
use sweep::blend::{BlendError, fillet_edges};
use sweep::test_support::{at_rest, extruded};
use topo::{Body, EdgeKey, FinishedBody as _, LeafBody as _, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

type P = (f64, f64);

fn sub(a: P, b: P) -> P {
    (a.0 - b.0, a.1 - b.1)
}
fn add(a: P, b: P) -> P {
    (a.0 + b.0, a.1 + b.1)
}
fn mul(a: P, k: f64) -> P {
    (a.0 * k, a.1 * k)
}
fn dot(a: P, b: P) -> f64 {
    a.0 * b.0 + a.1 * b.1
}
fn cross(a: P, b: P) -> f64 {
    a.0 * b.1 - a.1 * b.0
}
fn norm(a: P) -> f64 {
    dot(a, a).sqrt()
}
fn unit(a: P) -> P {
    mul(a, 1.0 / norm(a))
}
fn left(a: P) -> P {
    (-a.1, a.0)
}

#[derive(Clone, Debug)]
enum Piece {
    Seg(P, P),
    Arc {
        c: P,
        r: f64,
        a0: P,
        a1: P,
        turn: f64,
    },
}

/// A closed outline as a bulge chain, its pieces, and a dense polygon.
#[derive(Clone, Debug)]
struct Outline {
    chain: Vec<(P, f64)>,
    pieces: Vec<Piece>,
    dense: Vec<P>,
}

fn outline_from(chain: Vec<(P, f64)>) -> Outline {
    let n = chain.len();
    let mut pieces = Vec::new();
    let mut dense = Vec::new();
    for i in 0..n {
        let (p0, b) = chain[i];
        let p1 = chain[(i + 1) % n].0;
        if b == 0.0 {
            pieces.push(Piece::Seg(p0, p1));
            dense.push(p0);
        } else {
            let turn = 4.0 * b.atan();
            let ch = norm(sub(p1, p0));
            let dir = unit(sub(p1, p0));
            let mid = mul(add(p0, p1), 0.5);
            let c = add(mid, mul(left(dir), 0.5 * ch * (1.0 - b * b) / (2.0 * b)));
            let r = ch * (1.0 + b * b) / (4.0 * b.abs());
            pieces.push(Piece::Arc {
                c,
                r,
                a0: p0,
                a1: p1,
                turn,
            });
            let u0 = sub(p0, c);
            let k = 96;
            for j in 0..k {
                let a = turn * f64::from(j) / f64::from(k);
                let (s, co) = a.sin_cos();
                dense.push(add(c, (u0.0 * co - u0.1 * s, u0.0 * s + u0.1 * co)));
            }
        }
    }
    Outline {
        chain,
        pieces,
        dense,
    }
}

/// A CCW polygon with per-corner radius (0 = sharp), and an optional
/// per-corner sweep deficit (a non-tangent arc between the same feet).
fn rounded(poly: &[P], radii: &[f64], deficit: &[f64]) -> Outline {
    let n = poly.len();
    let mut chain = Vec::new();
    for i in 0..n {
        let v = poly[i];
        let din = unit(sub(v, poly[(i + n - 1) % n]));
        let dout = unit(sub(poly[(i + 1) % n], v));
        let turn = cross(din, dout).atan2(dot(din, dout));
        let r = radii[i];
        if r == 0.0 {
            chain.push((v, 0.0));
            continue;
        }
        let l = r * (turn.abs() / 2.0).tan();
        let tin = sub(v, mul(din, l));
        let tout = add(v, mul(dout, l));
        let sweep = turn - deficit[i] * turn.signum();
        chain.push((tin, (sweep / 4.0).tan()));
        chain.push((tout, 0.0));
    }
    outline_from(chain)
}

/// The obround: centres `(±a, 0)`, radius `rr`.
fn slot(a: f64, rr: f64) -> Outline {
    outline_from(vec![
        ((-a, -rr), 0.0),
        ((a, -rr), 1.0),
        ((a, rr), 0.0),
        ((-a, rr), 1.0),
    ])
}

fn regular(n: usize, circ: f64, rr: f64) -> Outline {
    let poly: Vec<P> = (0..n)
        .map(|i| {
            let a = 2.0 * PI * (i as f64) / (n as f64) - PI / 2.0;
            (circ * a.cos(), circ * a.sin())
        })
        .collect();
    rounded(&poly, &vec![rr; n], &vec![0.0; n])
}

fn rect(hx: f64, hy: f64, radii: [f64; 4]) -> Outline {
    rounded(
        &[(-hx, -hy), (hx, -hy), (hx, hy), (-hx, hy)],
        &radii,
        &[0.0; 4],
    )
}

impl Outline {
    fn inside(&self, q: P) -> bool {
        let mut wn = false;
        let n = self.dense.len();
        for i in 0..n {
            let (a, b) = (self.dense[i], self.dense[(i + 1) % n]);
            if (a.1 > q.1) != (b.1 > q.1) && q.0 < a.0 + (q.1 - a.1) / (b.1 - a.1) * (b.0 - a.0) {
                wn = !wn;
            }
        }
        wn
    }
    /// Signed distance, positive inside.
    fn sd(&self, q: P) -> f64 {
        let mut d = f64::INFINITY;
        for p in &self.pieces {
            let e = match *p {
                Piece::Seg(a, b) => {
                    let ab = sub(b, a);
                    let t = (dot(sub(q, a), ab) / dot(ab, ab)).clamp(0.0, 1.0);
                    norm(sub(q, add(a, mul(ab, t))))
                }
                Piece::Arc { c, r, a0, a1, turn } => {
                    let u = sub(a0, c);
                    let w = sub(q, c);
                    let mut t = cross(u, w).atan2(dot(u, w));
                    if turn > 0.0 && t < 0.0 {
                        t += 2.0 * PI;
                    }
                    if turn < 0.0 && t > 0.0 {
                        t -= 2.0 * PI;
                    }
                    if t.abs() <= turn.abs() {
                        (norm(w) - r).abs()
                    } else {
                        norm(sub(q, a0)).min(norm(sub(q, a1)))
                    }
                }
            };
            d = d.min(e);
        }
        if self.inside(q) { d } else { -d }
    }
    fn bbox(&self) -> (P, P) {
        let mut lo = (f64::INFINITY, f64::INFINITY);
        let mut hi = (f64::NEG_INFINITY, f64::NEG_INFINITY);
        for p in &self.dense {
            lo = (lo.0.min(p.0), lo.1.min(p.1));
            hi = (hi.0.max(p.0), hi.1.max(p.1));
        }
        (lo, hi)
    }
    /// Pappus: the section removed/added along this outline, where the
    /// ball lies on the `ball_inside` side of the wall.
    fn pappus(&self, r: f64, ball_inside: bool) -> f64 {
        let a = r * r * (1.0 - PI / 4.0);
        let d = r * (10.0 - 3.0 * PI) / (3.0 * (4.0 - PI));
        let mut v = 0.0;
        for p in &self.pieces {
            match *p {
                Piece::Seg(x, y) => v += a * norm(sub(y, x)),
                Piece::Arc { r: rr, turn, .. } => {
                    let centre_inside = turn > 0.0;
                    let rc = if centre_inside == ball_inside {
                        rr - d
                    } else {
                        rr + d
                    };
                    v += a * turn.abs() * rc;
                }
            }
        }
        v
    }
}

fn prism<T: Decide + topo::AtRestPolicy>(o: &Outline, z0: f64, h: f64) -> Body<T> {
    let plane = SketchPlane::xy_at(Point3::new(T::zero(), T::zero(), T::from_f64(z0)));
    let p = |q: P| Point2::new(T::from_f64(q.0), T::from_f64(q.1));
    extruded(
        plane,
        vec![bulge_loop(
            o.chain
                .iter()
                .map(|&(q, b)| (p(q), T::from_f64(b)))
                .collect(),
        )],
        T::from_f64(h),
        tol(),
    )
}

fn every_edge<T: Decide>(body: &Body<T>) -> Vec<EdgeKey> {
    let mut all = topo::query::all_edges(body);
    all.sort_unstable();
    all
}

fn mid<T: Bounds>(x: T) -> f64 {
    0.5 * (x.lo() + x.hi())
}

fn ends<T: Decide + Bounds>(body: &Body<T>, e: EdgeKey) -> (Point3<f64>, Point3<f64>) {
    let at = |v| {
        let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        Point3::new(mid(p.x), mid(p.y), mid(p.z))
    };
    let (s, t) = body.edge_vertices(e).unwrap();
    (at(s), at(t))
}

/// The edges whose ends both lie at height `z` and satisfy `keep`.
fn rim_where<T: Decide + Bounds>(
    body: &Body<T>,
    z: f64,
    keep: impl Fn(Point3<f64>) -> bool,
) -> Vec<EdgeKey> {
    every_edge(body)
        .into_iter()
        .filter(|&e| {
            let (a, b) = ends(body, e);
            (a.z - z).abs() < 1e-9 && (b.z - z).abs() < 1e-9 && keep(a) && keep(b)
        })
        .collect()
}

fn volume<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy>(body: &Body<T>) -> (f64, f64) {
    let p = topo::mass_properties(
        &topo::test_support::finished("the measured body", Body::clone(body), tol()),
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .expect("certified props");
    let (lo, hi) = (p.enclosure.volume.lo(), p.enclosure.volume.hi());
    (0.5 * (lo + hi), 0.5 * (hi - lo) + p.enclosure.volume_pad)
}

fn counts<T: Decide>(body: &Body<T>) -> (usize, usize, usize) {
    (
        body.faces().count(),
        body.edges().count(),
        body.vertices().count(),
    )
}

/// A pseudo-random sequence in [0, 1).
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

/// The rolling-ball round's membership correction at a rim: `s` the
/// distance into the side the ball is on... `material` the reading of the
/// unrounded body, returns the reading with the round applied, or `None`
/// within `eps` of any boundary of the truth.
///
/// Convex: material is s > 0, t > 0 near the rim; the round takes away
/// where s < r, t < r and outside the ball's circle.
/// Concave: void is s > 0, t > 0; the round fills where s < r, t < r and
/// outside the ball's circle.
fn round_reading(s: f64, t: f64, r: f64) -> Option<bool> {
    // true: inside the ball-excluded corner (the corner the round flips)
    if s < r && t < r && s > 0.0 && t > 0.0 {
        let m = ((r - s).powi(2) + (r - t).powi(2)).sqrt() - r;
        if m.abs() < 1e-5 {
            return None;
        }
        Some(m > 0.0)
    } else {
        Some(false)
    }
}

/// Checks `body` against `truth` on `n` points in the box, drawn from
/// one fixed sequence so a row reads the same points at every ε and on
/// every run. A point the classifier escalates (one within its band of a
/// face) is no answer either way: it is counted, never read as wrong,
/// and the row fails if more than one point in fifty escalates.
fn membership(
    what: &str,
    body: &Body<f64>,
    lo: Point3<f64>,
    hi: Point3<f64>,
    n: usize,
    truth: impl Fn(Point3<f64>) -> Option<bool>,
) {
    let band = Band::linear(tol()).expect("the band");
    let mut rng = Lcg(0x5eed_1234);
    let (mut read, mut escalated, mut wrong) = (0, 0, Vec::new());
    for _ in 0..n {
        let p = Point3::new(
            lo.x + (hi.x - lo.x) * rng.next(),
            lo.y + (hi.y - lo.y) * rng.next(),
            lo.z + (hi.z - lo.z) * rng.next(),
        );
        let Some(inside) = truth(p) else { continue };
        read += 1;
        let got = topo::point_in_solid(body, p, band, tol());
        if matches!(got, Err(topo::PointInSolidError::Escalated { .. })) {
            escalated += 1;
            continue;
        }
        if !matches!(
            (&got, inside),
            (Ok(topo::SolidContainment::In), true) | (Ok(topo::SolidContainment::Out), false)
        ) {
            wrong.push((p, inside, got));
        }
    }
    assert!(
        escalated * 50 <= read,
        "{what}: {escalated} of {read} points escalate"
    );
    assert!(
        read > n / 2 && wrong.is_empty(),
        "{what}: of {read} points {} wrong: {:?}",
        wrong.len(),
        &wrong[..wrong.len().min(6)]
    );
}

/// The outline turned by `a` about the origin.
fn rotated(o: &Outline, a: f64) -> Outline {
    let (s, c) = a.sin_cos();
    outline_from(
        o.chain
            .iter()
            .map(|&(p, b)| ((c * p.0 - s * p.1, s * p.0 + c * p.1), b))
            .collect(),
    )
}

/// Tangent arcs only: `(centre, radius, start angle, sweep)`, CCW.
fn arcs(list: &[(P, f64, f64, f64)]) -> Outline {
    outline_from(
        list.iter()
            .map(|&(c, r, a0, sw)| ((c.0 + r * a0.cos(), c.1 + r * a0.sin()), (sw / 4.0).tan()))
            .collect(),
    )
}

/// **The shared row's touch reading at `Interval` at the tightest ε**:
/// at ε = 1e-12 every mixed rim's section refuses at the certified
/// scalar, its contact class `shared_row_touch` escalating
/// (`row_contact`'s touch bound, levered at the edge's extent, encloses
/// about `[0, 4e-12]`, in that row's band), while it builds at every
/// other ε, and at `f64` everywhere
/// (`work/band/every-mixed-rims-section-escalates-its-touch-reading-at-interval-at-eps-1e-12.md`).
/// The rows pin it both ways: at `Interval` and ε = 1e-12 a mixed rim
/// refuses with exactly that escalation ([`assert_touch_escalation`]),
/// and anywhere else it builds.
pub(crate) fn tightest_interval<T>() -> bool {
    core::any::type_name::<T>().contains("Interval") && tol().eps().to_bits() == 1e-12_f64.to_bits()
}

/// The refusal [`tightest_interval`] pins: the description pass's
/// certificate escalating the section's `shared_row_touch`.
pub(crate) fn assert_touch_escalation(what: &str, e: &BlendError) {
    let text = format!("{e:?}");
    assert!(
        text.contains("surgery contact edge")
            && text.contains("Escalated { check: Transversality")
            && text.contains("predicate: Some(\"shared_row_touch\")"),
        "{what}: at Interval at ε 1e-12 the section's shared row escalates its touch, got {e:?}"
    );
}

/// A plate of outline `o`, `z ∈ [0, h]`, every edge filleted at `r`, at
/// any scalar: tier 3, one band face per link of each rim, Euler, and
/// the volume removed at Pappus's closed form within a tight enclosure.
fn plate_row<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy>(
    what: &str,
    o: &Outline,
    h: f64,
    r: f64,
) -> Option<Body<T>> {
    let body = prism::<T>(o, 0.0, h);
    let out = match fillet_edges(
        &at_rest(&body, tol()),
        &every_edge(&body),
        T::from_f64(r),
        tol(),
    ) {
        Ok(out) => {
            assert!(
                !tightest_interval::<T>(),
                "{what}: built where its touch escalates"
            );
            out
        }
        Err(e) if tightest_interval::<T>() => {
            assert_touch_escalation(what, &e.error);
            return None;
        }
        Err(e) => panic!("{what}: fillets whole, got {:?}", e.error),
    };
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
    let (f0, _, _) = counts(&body);
    let (f1, e1, v1) = counts(&out.body);
    assert_eq!(
        v1 as i64 - e1 as i64 + f1 as i64,
        2,
        "{what}: V − E + F = 2"
    );
    let links = o.pieces.len();
    assert_eq!(
        f1,
        f0 + 2 * links,
        "{what}: one band face per link of each rim"
    );
    let truth = 2.0 * o.pappus(r, true);
    let (va, wa) = volume(&body);
    let (vb, wb) = volume(&out.body);
    let removed = va - vb;
    assert!(
        (removed - truth).abs() <= wa + wb + 1e-12,
        "{what}: ΔV {removed} is Pappus's {truth} (±{})",
        wa + wb
    );
    assert!(
        wa + wb < 1e-7,
        "{what}: the enclosure is a claim: ±{}",
        wa + wb
    );
    Some(Body::clone(&out.body))
}

/// [`plate_row`] at `f64`, with membership near the rims.
fn plate_check(what: &str, o: &Outline, h: f64, r: f64) {
    let out = plate_row::<f64>(what, o, h, r).expect("f64 never skips");
    let (lo, hi) = o.bbox();
    membership(
        what,
        &out,
        Point3::new(lo.0 - 0.02, lo.1 - 0.02, -0.02),
        Point3::new(hi.0 + 0.02, hi.1 + 0.02, h + 0.02),
        1500,
        |p| {
            let s = o.sd((p.x, p.y));
            if s.abs() < 1e-6 || p.z.abs() < 1e-6 || (p.z - h).abs() < 1e-6 {
                return None;
            }
            if !(s > 0.0 && p.z > 0.0 && p.z < h) {
                return Some(false);
            }
            let top = round_reading(s, h - p.z, r)?;
            let bot = round_reading(s, p.z, r)?;
            Some(!(top || bot))
        },
    );
}

/// Both scalars.
fn plate_both(what: &str, o: &Outline, h: f64, r: f64) {
    plate_check(what, o, h, r);
    plate_row::<Interval>(&format!("{what} at Interval"), o, h, r);
}

/// **Off the axes**: the rounded rectangle turned in its plane, and the
/// slot turned.
#[test]
fn turned_rounded_outlines_fillet_whole() {
    let rect = rect(1.0, 0.6, [0.3; 4]);
    for deg in [7.0_f64, 30.0, 45.0, 90.0] {
        plate_both(
            &format!("rectangle at {deg}°"),
            &rotated(&rect, deg.to_radians()),
            0.5,
            0.1,
        );
    }
    plate_both("slot at 30°", &rotated(&slot(0.7, 0.3), PI / 6.0), 0.5, 0.1);
}

/// **Rounded polygons**, whose sides run off the axes.
#[test]
fn rounded_polygons_fillet_whole() {
    plate_both("triangle", &regular(3, 1.0, 0.3), 0.5, 0.1);
    plate_both("pentagon", &regular(5, 1.0, 0.3), 0.5, 0.1);
    plate_both("hexagon", &regular(6, 1.0, 0.25), 0.5, 0.08);
}

/// **Two arcs flanking a short flat are one band**, not a clearance
/// pair: a `2 × 1.2` and a `2 × 1` rectangle of corner radius `0.45`,
/// whose short flats run `0.3` and `0.1`, at `r = 0.2`; and the rounded
/// octagon, whose sides are joined by arcs of an eighth of a turn.
#[test]
fn arcs_flanking_a_short_flat_fillet_whole() {
    plate_both("2 × 1.2, R 0.45", &rect(1.0, 0.6, [0.45; 4]), 0.5, 0.2);
    plate_both("2 × 1, R 0.45", &rect(1.0, 0.5, [0.45; 4]), 0.5, 0.2);
    plate_both("octagon", &regular(8, 1.0, 0.2), 0.5, 0.09);
}

/// **Torus into torus**: a four-arc barrel, no lines, every junction
/// between two arcs on different cylinders.
#[test]
fn a_four_arc_barrel_fillets_whole() {
    let (a, c, r1) = (0.6_f64, 0.4_f64, 0.3_f64);
    let r2 = a.hypot(c) + r1;
    let t0 = c.atan2(a);
    let o = arcs(&[
        ((a, 0.0), r1, -t0, 2.0 * t0),
        ((0.0, -c), r2, t0, PI - 2.0 * t0),
        ((-a, 0.0), r1, PI - t0, 2.0 * t0),
        ((0.0, c), r2, PI + t0, PI - 2.0 * t0),
    ]);
    plate_both("barrel", &o, 0.5, 0.05);
}

/// A plate with a hole of radius `hole_r` at `hole_at` in its top.
fn holed_plate(hole_at: P, hole_r: f64) -> (Outline, Body<f64>) {
    let o = rect(1.0, 0.6, [0.3; 4]);
    let (x, y) = hole_at;
    let rev = vec![((x - hole_r, y), -1.0), ((x + hole_r, y), -1.0)];
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, 0.0));
    let lp =
        |c: &[(P, f64)]| bulge_loop(c.iter().map(|&(q, b)| (Point2::new(q.0, q.1), b)).collect());
    let body = extruded(plane, vec![lp(&o.chain), lp(&rev)], 0.5, tol());
    (o, body)
}

/// **A hole beside the outer rim** builds where it clears every link's
/// strip within that link's window — beside a straight link, and on the
/// far side of a corner arc's circle, `0.6` from its window — and refuses
/// where it reaches a strip.
#[test]
fn a_hole_beside_the_rim_builds_where_it_clears_the_strips() {
    let r = 0.05;
    for (at, hr, fits) in [
        ((0.0, -0.6 + 0.08 + 0.05), 0.05, true),
        ((0.0, -0.6 + 0.03 + 0.05), 0.05, false),
        ((0.43, 0.3), 0.02, true),
    ] {
        let (o, body) = holed_plate(at, hr);
        let rim = rim_where(&body, 0.5, |p| o.sd((p.x, p.y)).abs() < 1e-9);
        assert_eq!(rim.len(), 8, "the outer rim");
        let got = fillet_edges(&at_rest(&body, tol()), &rim, r, tol());
        match got {
            Ok(out) => {
                assert!(fits, "hole at {at:?}: reaches a strip and still built");
                validate_geometric(&out.body, tol())
                    .unwrap_or_else(|e| panic!("hole at {at:?}: tier 3, got {e:?}"));
                let (va, wa) = volume(&body);
                let (vb, wb) = volume(&out.body);
                let truth = o.pappus(r, true);
                assert!(
                    ((va - vb) - truth).abs() <= wa + wb + 1e-12,
                    "hole at {at:?}: ΔV {} is Pappus's {truth}",
                    va - vb
                );
            }
            Err(e) => assert!(
                !fits
                    && matches!(
                        e.error,
                        BlendError::RingClearance { .. }
                            | BlendError::FaceClearanceUncertified { .. }
                    ),
                "hole at {at:?}: got {:?}",
                e.error
            ),
        }
    }
}

/// **A crease at one corner**: the arc between the same feet sweeps `δ`
/// short of the turn. A definite crease refuses at predicate 4; one well
/// below the run's band (a hundredth of its ε, at every eps row) is the
/// input's tolerance, and builds.
#[test]
fn a_creased_corner_refuses_where_definite_and_builds_below_the_band() {
    for delta in [0.2, tol().eps() * 1e-2, tol().eps() * 1e-4] {
        let o = rounded(
            &[(-1.0, -0.6), (1.0, -0.6), (1.0, 0.6), (-1.0, 0.6)],
            &[0.3; 4],
            &[0.0, delta, 0.0, 0.0],
        );
        let body = prism::<f64>(&o, 0.0, 0.5);
        let rim = rim_where(&body, 0.5, |_| true);
        let got = fillet_edges(&at_rest(&body, tol()), &rim, 0.1, tol());
        if delta > 1e-3 {
            assert!(
                matches!(&got, Err(e) if matches!(e.error, BlendError::ChainNotG1 { .. })),
                "δ = {delta}: a definite crease refuses at predicate 4, got {:?}",
                got.as_ref().map(|_| ()).map_err(|e| &e.error)
            );
        } else {
            let out = got.unwrap_or_else(|e| panic!("δ = {delta}: builds, got {:?}", e.error));
            validate_geometric(&out.body, tol())
                .unwrap_or_else(|e| panic!("δ = {delta}: tier 3, got {e:?}"));
        }
    }
}

/// **The rounded rectangle drawn the way a user draws it**: through the
/// path lattice, each corner a tangent arc and each side a tangent line,
/// closing on the start's tangent, about `(x0, y0)`. The lattice puts
/// some vertices a few ulps off the bulge table's.
fn lattice_rect(x0: f64, y0: f64) -> profile::ConstructedLoop<f64> {
    use profile::{Open, Start};
    let (hx, hy, r) = (1.0, 0.6, 0.3);
    let p = |x: f64, y: f64| Point2::new(x0 + x, y0 + y);
    Open.at(p(-(hx - r), -hy))
        .line_to(p(hx - r, -hy), tol())
        .unwrap()
        .tangent()
        .tangent_arc_to(p(hx, -(hy - r)), tol())
        .unwrap()
        .tangent()
        .line(2.0 * (hy - r), tol())
        .unwrap()
        .tangent()
        .tangent_arc_to(p(hx - r, hy), tol())
        .unwrap()
        .tangent()
        .line(2.0 * (hx - r), tol())
        .unwrap()
        .tangent()
        .tangent_arc_to(p(-hx, hy - r), tol())
        .unwrap()
        .tangent()
        .line(2.0 * (hy - r), tol())
        .unwrap()
        .tangent()
        .tangent_arc_to(Start.arrives_tangent(), tol())
        .unwrap()
        .into()
}

/// The lattice outline about `(x0, y0)` on the plane `z = z0`, validated
/// at `f64` as a user's sketch is and lifted onto the scalar, extruded
/// up `h`.
fn lattice_slab<T: Decide + topo::AtRestPolicy>(x0: f64, y0: f64, z0: f64, h: f64) -> Body<T> {
    let pf = profile::ConstructedProfile::new(
        SketchPlane::xy_at(Point3::new(0.0, 0.0, z0)),
        vec![lattice_rect(x0, y0)],
    )
    .validate(tol())
    .expect("the lattice outline validates")
    .lift_onto(SketchPlane::xy_at(Point3::new(
        T::zero(),
        T::zero(),
        T::from_f64(z0),
    )))
    .expect("the lattice outline lifts onto the scalar");
    sweep::extrude(
        &pf,
        sweep::Extrusion::Distance {
            depth: T::from_f64(h),
            side: sweep::ExtrudeSide::Along,
        },
        tol(),
    )
    .expect("the lattice outline extrudes")
    .body
    .into_body()
}

/// **A lattice-drawn rounded plate fillets whole**, at the pin's radius
/// and near `R/2`, at both scalars, removing Pappus's volume.
#[test]
fn a_lattice_drawn_rounded_plate_fillets_whole() {
    fn row<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy>(what: &str, r: f64) {
        let body = lattice_slab::<T>(0.0, 0.0, 0.0, 0.5);
        let out = match fillet_edges(
            &at_rest(&body, tol()),
            &every_edge(&body),
            T::from_f64(r),
            tol(),
        ) {
            Ok(out) => {
                assert!(
                    !tightest_interval::<T>(),
                    "{what}: built where its touch escalates"
                );
                out
            }
            Err(e) if tightest_interval::<T>() => {
                assert_touch_escalation(what, &e.error);
                return;
            }
            Err(e) => panic!("{what}: fillets whole, got {:?}", e.error),
        };
        validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
        let truth = 2.0 * rect(1.0, 0.6, [0.3; 4]).pappus(r, true);
        let ((va, wa), (vb, wb)) = (volume(&body), volume(&out.body));
        assert!(
            wa + wb < 1e-7,
            "{what}: the enclosure is a claim: ±{}",
            wa + wb
        );
        assert!(
            ((va - vb) - truth).abs() <= wa + wb + 1e-12,
            "{what}: ΔV {} is Pappus's {truth}",
            va - vb
        );
    }
    for r in [0.05, 0.14] {
        row::<f64>(&format!("lattice plate, r = {r}"), r);
        row::<Interval>(&format!("lattice plate at Interval, r = {r}"), r);
    }
}

/// **A lattice-drawn pocket's mouth off the origin**: a `3 × 2 × 1`
/// block less the lattice rectangle about `(1.5, 1.0)` from `z = 0.5`;
/// its mouth removes `A·(P + 2πd)`.
#[test]
fn a_lattice_drawn_pockets_mouth_off_the_origin_fillets_whole() {
    fn row<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy>(what: &str, r: f64) {
        use topo::boolean::{BooleanInputs, BooleanOp, SweepStrategy, boolean_op_with};
        let block = sweep::test_support::block::<T>(3.0, 2.0, 1.0, tol());
        let cut = lattice_slab::<T>(1.5, 1.0, 0.5, 1.0);
        let pocket = boolean_op_with(
            BooleanOp::Subtract,
            &sweep::test_support::finished("the block", block, tol()),
            &sweep::test_support::finished("the lattice prism", cut, tol()),
            &BooleanInputs::default(),
            SweepStrategy::Realized,
            tol(),
        )
        .unwrap_or_else(|e| panic!("{what}: the pocket builds: {e:?}"))
        .body()
        .expect("the pocket is a body")
        .body
        .clone()
        .into_body();
        let mouth = rim_where(&pocket, 1.0, |p| {
            (p.x - 1.5).abs() <= 1.0 + 1e-9 && (p.y - 1.0).abs() <= 0.6 + 1e-9
        });
        assert_eq!(mouth.len(), 8, "{what}: the mouth");
        let out = match fillet_edges(&at_rest(&pocket, tol()), &mouth, T::from_f64(r), tol()) {
            Ok(out) => {
                assert!(
                    !tightest_interval::<T>(),
                    "{what}: built where its touch escalates"
                );
                out
            }
            Err(e) if tightest_interval::<T>() => {
                assert_touch_escalation(what, &e.error);
                return;
            }
            Err(e) => panic!("{what}: fillets whole, got {:?}", e.error),
        };
        validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
        let truth = rect(1.0, 0.6, [0.3; 4]).pappus(r, false);
        let ((va, wa), (vb, wb)) = (volume(&pocket), volume(&out.body));
        assert!(
            wa + wb < 1e-7,
            "{what}: the enclosure is a claim: ±{}",
            wa + wb
        );
        assert!(
            ((va - vb) - truth).abs() <= wa + wb + 1e-12,
            "{what}: ΔV {} is Pappus's {truth}",
            va - vb
        );
    }
    for r in [0.05, 0.14] {
        row::<f64>(&format!("lattice mouth, r = {r}"), r);
        row::<Interval>(&format!("lattice mouth at Interval, r = {r}"), r);
    }
}

/// The plate of outline `o`, every edge filleted at `r`, at `f64`:
/// `Ok` with the tier-3 check, Pappus and membership of [`plate_check`],
/// or the refusal.
fn plate_try(what: &str, o: &Outline, r: f64) -> Result<(), BlendError> {
    let body = prism::<f64>(o, 0.0, 0.5);
    match fillet_edges(&at_rest(&body, tol()), &every_edge(&body), r, tol()) {
        Ok(_) => {
            plate_check(what, o, 0.5, r);
            Ok(())
        }
        Err(e) => Err(e.error),
    }
}

/// **R1's adversarial outlines** (PR 6520's review): an L whose reflex
/// corner is a small concave arc, a dog-bone whose neck the two rims'
/// bands overlap past a radius, and a pinched outline whose two waists
/// do. Each builds exact where its bands fit and refuses typed where
/// they overlap.
#[test]
fn adversarial_outlines_build_where_their_bands_fit_and_refuse_where_they_overlap() {
    let l = |rr: [f64; 6]| {
        rounded(
            &[
                (0.0, 0.0),
                (2.0, 0.0),
                (2.0, 1.0),
                (1.0, 1.0),
                (1.0, 2.0),
                (0.0, 2.0),
            ],
            &rr,
            &[0.0; 6],
        )
    };
    let dog = rounded(
        &[
            (-2.0, -1.0),
            (-1.0, -1.0),
            (-1.0, -0.1),
            (1.0, -0.1),
            (1.0, -1.0),
            (2.0, -1.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 0.1),
            (-1.0, 0.1),
            (-1.0, 1.0),
            (-2.0, 1.0),
        ],
        &[
            0.3, 0.3, 0.05, 0.05, 0.3, 0.3, 0.3, 0.3, 0.05, 0.05, 0.3, 0.3,
        ],
        &[0.0; 12],
    );
    let pinched = rounded(
        &[
            (-1.0, -0.6),
            (-0.3, -0.6),
            (0.0, -0.1),
            (0.3, -0.6),
            (1.0, -0.6),
            (1.0, 0.6),
            (0.3, 0.6),
            (0.0, 0.1),
            (-0.3, 0.6),
            (-1.0, 0.6),
        ],
        &[0.35, 0.35, 0.05, 0.35, 0.35, 0.35, 0.35, 0.05, 0.35, 0.35],
        &[0.0; 10],
    );
    // `None` builds; `Some(m)` refuses at the support screen with margin
    // `m`: the dog-bone's neck is 0.2 across against two setbacks of
    // 0.12, an overlap of exactly 0.04; the pinched waists read theirs
    // at the screen's closest samples.
    let rows: Vec<(&str, Outline, f64, Option<f64>)> = vec![
        (
            "L, corners 0.45, reflex 0.1, r 0.2",
            l([0.45, 0.45, 0.45, 0.1, 0.45, 0.45]),
            0.2,
            None,
        ),
        (
            "L, corners 0.3, reflex 0.1, r 0.12",
            l([0.3, 0.3, 0.3, 0.1, 0.3, 0.3]),
            0.12,
            None,
        ),
        ("dog-bone, r 0.08", dog.clone(), 0.08, None),
        ("dog-bone, r 0.12", dog, 0.12, Some(-0.04)),
        ("pinched, r 0.13", pinched.clone(), 0.13, None),
        (
            "pinched, r 0.16",
            pinched,
            0.16,
            Some(-0.025_634_936_838_49),
        ),
    ];
    for (what, o, r, overlap) in rows {
        match (plate_try(what, &o, r), overlap) {
            (Ok(()), None) => {}
            (Err(BlendError::FaceClearanceUncertified { margin, .. }), Some(m)) => {
                let read = margin
                    .reading
                    .diagnostic_f64_for_error_text()
                    .value()
                    .expect("a definite reading");
                assert!(
                    (read - m).abs() < 1e-9,
                    "{what}: refuses at {read}, not {m}"
                );
            }
            (got, want) => panic!("{what}: got {got:?}, want the overlap {want:?}"),
        }
    }
}

/// **Holes at a corner arc's wedge**: the top-right arc (centre
/// `(0.7, 0.3)`, `R = 0.3`) at `r = 0.05` takes the annulus `0.25…0.3`
/// within its quarter turn off the cap. A radius-0.02 hole inside the
/// wedge, and just outside it on either side, each half a hundredth
/// clear of the strips and half a hundredth into them: clear builds,
/// overlapping refuses.
#[test]
fn holes_at_an_arcs_wedge_build_clear_and_refuse_overlapping() {
    let (r, hr) = (0.05, 0.02);
    let o = rect(1.0, 0.6, [0.3; 4]);
    for (deg, rad) in [
        (45.0_f64, 0.25 - hr - 0.005),
        (45.0, 0.25 - hr + 0.005),
        (-5.0, 0.225),
        (-5.0, 0.235),
        (95.0, 0.225),
        (95.0, 0.235),
        (180.0, 0.275),
    ] {
        let at = (
            0.7 + rad * deg.to_radians().cos(),
            0.3 + rad * deg.to_radians().sin(),
        );
        let clear = o.sd(at) - hr - r;
        let what = format!("hole at {deg}°, {rad:.3} out (clear {clear:.4})");
        let (_, body) = holed_plate(at, hr);
        let rim = rim_where(&body, 0.5, |p| o.sd((p.x, p.y)).abs() < 1e-9);
        let got = fillet_edges(&at_rest(&body, tol()), &rim, r, tol());
        println!(
            "{what}: {:?}",
            got.as_ref().map(|_| ()).map_err(|e| &e.error)
        );
        match got {
            Ok(out) => {
                assert!(clear > 0.0, "{what}: overlaps a strip and built");
                validate_geometric(&out.body, tol())
                    .unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
                let (va, wa) = volume(&body);
                let (vb, wb) = volume(&out.body);
                let truth = o.pappus(r, true);
                assert!(
                    ((va - vb) - truth).abs() <= wa + wb + 1e-12,
                    "{what}: ΔV {} is Pappus's {truth}",
                    va - vb
                );
            }
            Err(e) => {
                assert!(clear < 0.0, "{what}: clear and refused, got {:?}", e.error);
                let BlendError::FaceClearanceUncertified { margin, .. } = e.error else {
                    panic!("{what}: refuses at the support screen, got {:?}", e.error);
                };
                // Inside the wedge the screen reads the exact overlap; at
                // its edges its closest samples read less of it.
                let read = margin
                    .reading
                    .diagnostic_f64_for_error_text()
                    .value()
                    .expect("a definite reading");
                assert!(
                    read < 0.0 && read >= clear - 1e-9,
                    "{what}: refuses at {read}"
                );
                if (deg - 45.0).abs() < 1e-12 {
                    assert!(
                        (read - clear).abs() < 1e-9,
                        "{what}: at the exact overlap {clear}"
                    );
                }
            }
        }
    }
}
