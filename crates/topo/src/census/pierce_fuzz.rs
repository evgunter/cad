//! **Pass 4's edge–face pierce is read where the plane meets the edge**:
//! an edge through a prism's tilted side face, near the face's top
//! boundary, the face's stored origin moved along its plane metres
//! from the edge, read by [`pair_edge_face`] at `f64` and at
//! [`Interval`], at ε = 1e-6, 1e-9 and 1e-12 (K = 10).
//!
//! The edge grazes the plane: its ends stand off it by a few `Kε` to a
//! tenth of its length, so the plane's residuals, read off an origin
//! metres away, place the crossing to within their rounding over the
//! lever. Each pose is measured exactly (the crossing of the stored
//! plane with the stored ends, [`exact_depth`]) and read only where the
//! crossing stands at least `2Kε` inside or outside the face's top
//! edge. A pierce read where it is not, or none read and nothing
//! escalated where it is, is wrong; an escalation never is.
//!
//! Beside it: the closed form's own readings, each residual against
//! its charge and the placement's enclosure against the exact crossing;
//! a tilted non-convex cap's notches, spikes, slots and reflex vertices,
//! where the placement's span can outreach the feature ([`tilted`]);
//! the pinned rows ([`rows`]); and printing probes, ignored ([`probe`]).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Gated to the readings it checks: the pierce lane and the containment
// door it reads, the band and the rounding charge they decide with.
test_utils::gated_to![
    "crates/topo/src/census.rs",
    "crates/topo/src/boolean/contain.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/k_stats.rs",
    "crates/geom-core/src/running.rs",
];

use super::segment_fuzz::{big, big_dot, ratio};
use super::*;
use geom_core::{Interval, Tol};
use num_bigint::BigInt;
use test_utils::fuzz::Rng;

const K: f64 = 10.0;
const EPS: [f64; 3] = [1e-6, 1e-9, 1e-12];

/// Poses drawn per ε at effort 1.
const COUNT: usize = 60;

/// The prism's profile; side face 0 runs from its first corner to its
/// second, `z` over `[0, 1]`.
const PROFILE: [(f64, f64); 3] = [(0.0, 0.0), (1.0, 0.37), (0.2, 1.1)];

/// One edge against side face 0, its stored origin moved along the
/// plane by `far` (across, up).
#[derive(Clone, Copy, Debug)]
struct Pose {
    far: (f64, f64),
    /// How far below the face's top edge the edge is aimed at the
    /// plane (negative above it).
    depth: f64,
    /// How far each end stands off the plane, opposite sides.
    off: f64,
    /// Where along the face's width it is aimed, a fraction.
    across: f64,
}

/// What a reading found: a pierce, nothing, or a refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Found {
    Pierce,
    Nothing,
    Refused,
}

/// What one pair's findings say: a refusal where any refused, nothing
/// where there are none, else a pierce.
fn found(errors: &[ValidationError]) -> Found {
    if errors.iter().any(|e| {
        matches!(
            e,
            ValidationError::CensusEscalated { .. } | ValidationError::CensusUnsupported { .. }
        )
    }) {
        Found::Refused
    } else if errors.is_empty() {
        Found::Nothing
    } else {
        Found::Pierce
    }
}

/// `face`'s plane in `body`, its stored origin moved to `o`.
fn move_origin<T: Real>(body: &mut Body<T>, face: FaceKey, o: Point3<T>) {
    let s = body.get_face(face).unwrap().surface;
    match body.surfaces.get_mut(s).unwrap() {
        geom::Surface::Plane { origin, .. } => *origin = o,
        _ => panic!("a planar face"),
    }
}

/// An edge from `ends[0]` to `ends[1]` against `face` of `body`, as
/// pass 4 reads the pair.
struct Pair<'b, T: Decide> {
    body: &'b Body<T>,
    geo: Geo<T>,
    face: FaceKey,
    e: EdgeGeo<T>,
}

impl<'b, T: Decide> Pair<'b, T> {
    fn new(body: &'b Body<T>, face: FaceKey, [p0, p1]: [Point3<T>; 2]) -> Self {
        Self {
            body,
            geo: snapshot(body),
            face,
            e: segment((p0, p1)),
        }
    }

    fn f(&self) -> &FaceGeo<T> {
        self.geo.faces.iter().find(|f| f.key == self.face).unwrap()
    }

    /// What [`pair_edge_face`] pushes for the pair.
    fn read(&self, band: Band) -> Vec<ValidationError> {
        let declared = Declared::index(&self.geo, &ContactRecords::default());
        let mut errors = Vec::new();
        let (e, f) = (&self.e, self.f());
        pair_edge_face(self.body, &self.geo, &declared, band, e, f, &mut errors);
        errors
    }

    /// Where [`pierce_span`] places the crossing.
    fn span(&self, band: Band) -> Option<PierceSpan<T>> {
        let f = self.f();
        let r = [self.e.p0, self.e.p1].map(|p| (p - f.origin).dot(f.normal));
        pierce_span(&self.e, f, r, band, &mut Vec::new())
    }
}

/// The residual `(p − o)·n` of `p` off the plane through `o` normal to
/// `n`, exactly, in units of `2⁻²¹⁴⁸`.
fn exact_residual(p: Point3<f64>, o: Point3<f64>, n: Vec3<f64>) -> BigInt {
    let n = [n.x, n.y, n.z].map(big);
    big_dot(&[p.x, p.y, p.z].map(big), &n) - big_dot(&[o.x, o.y, o.z].map(big), &n)
}

/// Where the plane through `o` normal to `n` meets the line through
/// `p0`, `p1`: `p₀ + (p₁ − p₀)·r₀/(r₀ − r₁)`, each coordinate to a few
/// ulps; `None` for a line in a parallel plane.
fn exact_crossing(o: Point3<f64>, n: Vec3<f64>, [p0, p1]: [Point3<f64>; 2]) -> Option<Point3<f64>> {
    let (r0, r1) = (exact_residual(p0, o, n), exact_residual(p1, o, n));
    let lever = &r0 - &r1;
    if lever == BigInt::from(0) {
        return None;
    }
    let den = &lever << 1074_usize;
    let at = |a: f64, b: f64| {
        let num = big(a) * &lever + (big(b) - big(a)) * &r0;
        let (num, den) = if den < BigInt::from(0) {
            (-num, -&den)
        } else {
            (num, den.clone())
        };
        if num < BigInt::from(0) {
            -ratio(&-num, &den)
        } else {
            ratio(&num, &den)
        }
    };
    Some(Point3::new(at(p0.x, p1.x), at(p0.y, p1.y), at(p0.z, p1.z)))
}

/// Side face 0's stored origin, moved, and its normal.
fn plane(pose: &Pose) -> (Point3<f64>, Vec3<f64>) {
    let prism = crate::test_support_fixtures::prism_z::<f64>(&PROFILE, 0.0, 1.0, Tol::witness());
    let s = prism.body.get_face(prism.side_faces[0]).unwrap().surface;
    let geom::Surface::Plane { origin, normal, .. } = *prism.body.surfaces.get(s).unwrap() else {
        panic!("a prism's side is a plane")
    };
    let along = normal.cross(Vec3::new(0.0, 0.0, 1.0)).normalize();
    (
        origin + along * pose.far.0 + Vec3::new(0.0, 0.0, pose.far.1),
        normal,
    )
}

/// The edge's ends: up the face through its aim, `off` either side of
/// the plane.
fn ends(pose: &Pose, normal: Vec3<f64>) -> [Point3<f64>; 2] {
    let [(x0, y0), (x1, y1)] = [PROFILE[0], PROFILE[1]];
    let aim = Point3::new(
        x0 + (x1 - x0) * pose.across,
        y0 + (y1 - y0) * pose.across,
        1.0 - pose.depth,
    );
    let half = Vec3::new(0.0, 0.0, 0.5);
    [
        aim - half + normal * pose.off,
        aim + half - normal * pose.off,
    ]
}

/// The pair read at scalar `T`.
fn read<T: Decide + crate::props::AtRestPolicy>(pose: &Pose, band: Band) -> Found {
    let mut prism = crate::test_support_fixtures::prism_z::<T>(&PROFILE, 0.0, 1.0, Tol::witness());
    let face = prism.side_faces[0];
    let (o, n) = plane(pose);
    let lift = |p: Point3<f64>| p.map(T::from_f64);
    move_origin(&mut prism.body, face, lift(o));
    let errors = Pair::new(&prism.body, face, ends(pose, n).map(lift)).read(band);
    let got = found(&errors);
    assert!(
        got != Found::Pierce
            || errors.iter().all(|e| matches!(
                e,
                ValidationError::UnrecordedContact {
                    contact: CensusContact::EdgeFacePierce { .. },
                    ..
                }
            )),
        "{pose:?}: {errors:?}"
    );
    got
}

/// How far below the face's top edge `z = 1` the stored plane meets
/// the stored edge, in metres, measured exactly.
fn exact_depth(pose: &Pose) -> f64 {
    let (o, n) = plane(pose);
    depth_below_top(o, n, ends(pose, n)).expect("an edge across the plane")
}

/// How far below `z = 1` the plane through `o` normal to `n` meets the
/// line through `p0`, `p1` ([`exact_crossing`]); `None` for a line in a
/// parallel plane.
fn depth_below_top(o: Point3<f64>, n: Vec3<f64>, ends: [Point3<f64>; 2]) -> Option<f64> {
    exact_crossing(o, n, ends).map(|x| 1.0 - x.z)
}

/// The pose read at both scalars: `f64`'s reading, and what was wrong,
/// a pierce read where the crossing stands outside the face or nothing
/// read where it stands inside. A pose within `2Kε` of the edge is not
/// read.
fn wrong(pose: &Pose, band: Band) -> Option<(Found, Option<String>)> {
    let depth = exact_depth(pose);
    let margin = 2.0 * band.escalate();
    let truth = if depth >= margin {
        Found::Pierce
    } else if depth <= -margin {
        Found::Nothing
    } else {
        return None;
    };
    let (f, iv) = (read::<f64>(pose, band), read::<Interval>(pose, band));
    let bad = |got: Found| got != Found::Refused && got != truth;
    Some((
        f,
        (bad(f) || bad(iv)).then(|| {
            format!(
                "{pose:?}: the crossing {depth:e} below the top edge, f64 {f:?}, Interval {iv:?}"
            )
        }),
    ))
}

/// **An edge grazing a face whose stored origin is metres away is
/// pierced where it pierces**: every pose drawn, at each ε, read right
/// or refused at both scalars.
#[test]
fn a_grazing_edge_is_read_pierced_where_it_pierces() {
    let label = "census_grazing_pierce";
    let mut r = test_utils::fuzz::start(label);
    let mut bad = Vec::new();
    for eps in EPS {
        let band = Band::new(eps, K * eps).unwrap();
        let (mut read, mut held) = (0usize, 0usize);
        for _ in 0..test_utils::fuzz::scaled(COUNT) {
            let pose = draw(&mut r, eps);
            let Some((f, said)) = wrong(&pose, band) else {
                continue;
            };
            read += 1;
            held += usize::from(f == Found::Refused);
            bad.extend(said.map(|said| format!("ε {eps:e} {said}")));
        }
        println!("[fuzz] {label} at ε {eps:e}: {read} poses read, {held} refused at f64");
    }
    assert!(
        bad.is_empty(),
        "{label}: {} readings wrong; {}:\n{}",
        bad.len(),
        test_utils::fuzz::replay(),
        bad[..bad.len().min(8)].join("\n")
    );
}

/// A pose: the origin up to 30 m off, the crossing within a centimetre
/// of the top edge either side, the ends a few `Kε` to a tenth off the
/// plane.
fn draw(r: &mut Rng, eps: f64) -> Pose {
    let sign = |r: &mut Rng| if r.below(2) == 0 { 1.0 } else { -1.0 };
    Pose {
        far: (r.range(-30.0, 30.0), r.range(-30.0, 30.0)),
        depth: sign(r) * r.log_uniform(3.0 * K * eps, 1e-2),
        off: r.log_uniform(1.5 * K * eps, 0.1),
        across: r.range(0.2, 0.8),
    }
}

/// **A grazing edge through a face whose stored origin is metres away
/// refuses near the face's boundary, and is read away from it**: the
/// origin moved 3 m across the plane and 3 m up it, each end `1.5e-11`
/// off the plane, at ε = 1e-12. Two of these poses read wrong at `f64`
/// with the crossing placed off the residuals uncharged, its point
/// moved ~1e-5 by their rounding over a lever of `3e-11`: one crossing
/// 1.4e-6 inside the top edge read no pierce, one 6.3e-8 outside it
/// read one. Charged, both refuse; 0.3 m in, the edge pierces, and
/// 0.3 m out, it does not.
#[test]
fn a_grazing_edge_refuses_near_a_far_origined_faces_boundary_and_reads_away_from_it() {
    let band = Band::new(1e-12, 1e-11).unwrap();
    let pose = |depth, across| Pose {
        far: (3.0, 3.0),
        depth,
        off: 1.5e-11,
        across,
    };
    let (missed, false_pierce) = (pose(3e-6, 0.5), pose(1e-6, 0.3));
    let (inside, outside) = (exact_depth(&missed), exact_depth(&false_pierce));
    assert!(
        (1e-6..2e-6).contains(&inside) && (-1e-7..-1e-8).contains(&outside),
        "the poses stand where they are named: {inside:e} inside, {outside:e} outside"
    );
    for (what, p, want) in [
        ("1.4e-6 inside", missed, Found::Refused),
        ("6.3e-8 outside", false_pierce, Found::Refused),
        ("0.3 inside", pose(0.3, 0.5), Found::Pierce),
        ("0.3 outside", pose(-0.3, 0.5), Found::Nothing),
    ] {
        assert_eq!(read::<f64>(&p, band), want, "{what}");
    }
}

#[cfg(test)]
#[path = "pierce_probe.rs"]
mod probe;

#[cfg(test)]
#[path = "pierce_rows.rs"]
mod rows;

#[cfg(test)]
#[path = "pierce_tilted.rs"]
mod tilted;

/// **Each residual's rounding stays inside half its charge, and the
/// placement's enclosure holds the exact crossing**: poses as the
/// grazing fuzzer draws them, the origin up to a kilometre off, each
/// residual, the lever `|r₀ − r₁|` and the fraction `r₀/(r₀ − r₁)`
/// measured exactly against what [`pierce_span`] reads at `f64`.
#[test]
fn each_residual_stays_inside_its_charge_and_the_span_holds_the_crossing() {
    let label = "census_pierce_span_encloses";
    let mut r = test_utils::fuzz::start(label);
    let mut bad = Vec::new();
    let mut placed = 0usize;
    for eps in EPS {
        let band = Band::new(eps, K * eps).unwrap();
        for _ in 0..test_utils::fuzz::scaled(4 * COUNT) {
            let mut pose = draw(&mut r, eps);
            let scale = r.log_uniform(1.0, 1e3);
            pose.far = (pose.far.0 * scale / 30.0, pose.far.1 * scale / 30.0);
            let (o, n) = plane(&pose);
            let [p0, p1] = ends(&pose, n);
            let f = FaceGeo {
                key: FaceKey::default(),
                origin: o,
                normal: n,
                boundary: BTreeSet::new(),
                loops: Vec::new(),
                polygon: false,
                walked: true,
            };
            let e = segment((p0, p1));
            let read = [e.p0, e.p1].map(|p| (p - o).dot(n));
            let exact = [p0, p1].map(|p| exact_residual(p, o, n));
            for i in 0..2 {
                let charge = geom_core::rounding_charge(([p0, p1][i] - o).norm());
                let off = (big(read[i]) << 1074_usize) - &exact[i];
                let room = big(charge / 2.0) << 1074_usize;
                if off.magnitude() > room.magnitude() {
                    bad.push(format!(
                        "ε {eps:e} {pose:?}: residual {i} off past half its charge"
                    ));
                }
            }
            let mut errors = Vec::new();
            let Some(PierceSpan {
                t: [lo, hi],
                lever: reach,
            }) = pierce_span(&e, &f, read, band, &mut errors)
            else {
                continue;
            };
            placed += 1;
            let lever = &exact[0] - &exact[1];
            let (num, lever) = if lever < BigInt::from(0) {
                (-&exact[0] << 1074_usize, -lever)
            } else {
                (exact[0].clone() << 1074_usize, lever)
            };
            if (big(reach) << 1074_usize) < lever {
                bad.push(format!(
                    "ε {eps:e} {pose:?}: the lever's bound {reach:e} under the lever"
                ));
            }
            if !(big(lo) * &lever <= num && num <= big(hi) * &lever) {
                bad.push(format!(
                    "ε {eps:e} {pose:?}: [{lo:e}, {hi:e}] misses the crossing"
                ));
            }
        }
    }
    println!("[fuzz] {label}: {placed} spans placed");
    assert!(
        bad.is_empty(),
        "{label}: {} readings outside their charge; {}:\n{}",
        bad.len(),
        test_utils::fuzz::replay(),
        bad[..bad.len().min(8)].join("\n")
    );
}

/// **A residual rounded 2.7u of its lever stays inside half its
/// charge**: the worst of 300 000 random residuals `(p − o)·n`, `o` up
/// to a kilometre off, measured exactly. Its rounding is `2.67u·|p − o|`
/// against [`residual_charge`]'s `16u`, so the charge has sixfold
/// headroom here and the reading has under half of it.
#[test]
fn the_worst_residual_drawn_stays_inside_half_its_charge() {
    let p = Point3::new(
        -2.367_246_549_859_963,
        -0.825_544_484_523_304_9,
        2.735_809_625_235_145,
    );
    let o = Point3::new(
        -611.180_671_475_044_7,
        76.180_933_621_736_81,
        -136.522_140_914_879_97,
    );
    let n = Vec3::new(
        -0.988_106_083_684_830_1,
        0.035_422_402_105_880_754,
        0.149_638_299_957_186_22,
    );
    let f = FaceGeo {
        key: FaceKey::default(),
        origin: o,
        normal: n,
        boundary: BTreeSet::new(),
        loops: Vec::new(),
        polygon: false,
        walked: true,
    };
    let exact = exact_residual(p, o, n);
    let off = (big((p - o).dot(n)) << 1074_usize) - exact;
    let lever = (p - o).norm() * f64::EPSILON / 2.0;
    let err = ratio(&off, &(BigInt::from(1) << 2148_usize)).abs();
    assert!(
        err > 2.6 * lever,
        "the pose rounds {:.2}u, not its 2.67u",
        err / lever
    );
    assert!(
        err <= residual_charge(&f, p) / 2.0,
        "{err:e} past half the charge {:e}",
        residual_charge(&f, p)
    );
}
