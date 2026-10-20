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
//! Beside it: the same edge across a thin notch or strip of a
//! non-convex cap face, where the placement's span can outreach the
//! feature; and the closed form's own readings, each residual against
//! its charge and the placement's enclosure against the exact crossing.
//! The two review probes ([`probe`], [`probe_notch`]) print only.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Gated to the readings it checks: the pierce lane and the containment
// door it reads, the band and the rounding charge they decide with.
test_utils::gated_to![
    "crates/topo/src/census.rs",
    "crates/topo/src/boolean/contain.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/k_stats.rs",
    "crates/geom-brep/src/implicit.rs",
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
    let s = prism.body.get_face(face).unwrap().surface;
    let (o, n) = plane(pose);
    let lift = |p: Point3<f64>| p.map(T::from_f64);
    match prism.body.surfaces.get_mut(s).unwrap() {
        geom::Surface::Plane { origin, .. } => *origin = lift(o),
        _ => panic!("a prism's side is a plane"),
    }
    let geo = snapshot(&prism.body);
    let f = geo.faces.iter().find(|f| f.key == face).unwrap();
    let [p0, p1] = ends(pose, n);
    let e = segment((lift(p0), lift(p1)));
    let declared = Declared::index(&geo, &ContactRecords::default());
    let mut errors = Vec::new();
    pair_edge_face(&prism.body, &geo, &declared, band, None, &e, f, &mut errors);
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
        assert!(
            errors.iter().all(|e| matches!(
                e,
                ValidationError::UnrecordedContact {
                    contact: CensusContact::EdgeFacePierce { .. },
                    ..
                }
            )),
            "{pose:?}: {errors:?}"
        );
        Found::Pierce
    }
}

/// How far below the face's top edge `z = 1` the stored plane meets
/// the stored edge, in metres, measured exactly: `1 − z₀ − (z₁ − z₀)·
/// r₀/(r₀ − r₁)`, every `f64` an integer count of `2⁻¹⁰⁷⁴`.
fn exact_depth(pose: &Pose) -> f64 {
    let (o, n) = plane(pose);
    let [p0, p1] = ends(pose, n);
    let n = [n.x, n.y, n.z].map(big);
    let r = |p: Point3<f64>| {
        big_dot(&[p.x, p.y, p.z].map(big), &n) - big_dot(&[o.x, o.y, o.z].map(big), &n)
    };
    let (r0, r1) = (r(p0), r(p1));
    let lever = &r0 - &r1;
    let num = (big(1.0) - big(p0.z)) * &lever - (big(p1.z) - big(p0.z)) * &r0;
    let den = lever << 1074_usize;
    if den < BigInt::from(0) {
        ratio(&-num, &-den)
    } else {
        ratio(&num, &den)
    }
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
#[path = "pierce_probe_notch.rs"]
mod probe_notch;

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
            };
            let e = segment((p0, p1));
            let read = [e.p0, e.p1].map(|p| (p - o).dot(n));
            let bn = [n.x, n.y, n.z].map(big);
            let exact = [p0, p1].map(|p| {
                big_dot(&[p.x, p.y, p.z].map(big), &bn) - big_dot(&[o.x, o.y, o.z].map(big), &bn)
            });
            for i in 0..2 {
                let charge = geom_brep::rounding_charge(([p0, p1][i] - o).norm());
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

/// A cap face with a thin feature at `x = 0.5`, its plane `z = 1`, its
/// stored origin moved to `(far, 1)`.
#[derive(Clone, Copy, Debug)]
enum Thin {
    /// The unit square less a V notch from the bottom edge, half as wide
    /// as `w` at `y = 0`, closing at `(0.5, 0.5)`.
    Notch(f64),
    /// The strip `|x − 0.5| < h`, `y` over `[0, 1]`.
    Strip(f64),
}

impl Thin {
    fn profile(self) -> Vec<(f64, f64)> {
        match self {
            Thin::Notch(w) => vec![
                (0.0, 0.0),
                (0.5 - w, 0.0),
                (0.5, 0.5),
                (0.5 + w, 0.0),
                (1.0, 0.0),
                (1.0, 1.0),
                (0.0, 1.0),
            ],
            Thin::Strip(h) => vec![
                (0.5 - h, 0.0),
                (0.5 + h, 0.0),
                (0.5 + h, 1.0),
                (0.5 - h, 1.0),
            ],
        }
    }

    /// How far inside the face the point `(x, y)` of its plane stands
    /// across `x` (negative outside), to a few ulps: the horizontal gap
    /// to the feature's sides, no more than the distance to them.
    fn inside(self, x: f64, y: f64) -> f64 {
        match self {
            Thin::Notch(w) => {
                let half = if y < 0.5 { w * (1.0 - y / 0.5) } else { 0.0 };
                ((x - 0.5).abs() - half).min(x).min(1.0 - x)
            }
            Thin::Strip(h) => h - (x - 0.5).abs(),
        }
    }
}

/// A grazing edge across the cap along `x` at height `y`, its ends at
/// `x₀`, `x₁` and `±off` off the plane.
#[derive(Clone, Copy, Debug)]
struct Across {
    thin: Thin,
    far: (f64, f64),
    y: f64,
    x: (f64, f64),
    off: f64,
}

impl Across {
    fn ends(self) -> [Point3<f64>; 2] {
        [
            Point3::new(self.x.0, self.y, 1.0 + self.off),
            Point3::new(self.x.1, self.y, 1.0 - self.off),
        ]
    }

    /// Where the edge meets `z = 1`, exactly to a few ulps: the plane's
    /// normal is `+z`, so each residual is `z − 1` exactly.
    fn crossing(self) -> f64 {
        let [p0, p1] = self.ends();
        let r = |p: Point3<f64>| big(p.z) - big(1.0);
        let (r0, r1) = (r(p0), r(p1));
        let num = big(p0.x) * (&r0 - &r1) + (big(p1.x) - big(p0.x)) * &r0;
        let den = (&r0 - &r1) << 1074_usize;
        if den < BigInt::from(0) {
            ratio(&-num, &-den)
        } else {
            ratio(&num, &den)
        }
    }

    fn read<T: Decide + crate::props::AtRestPolicy>(self, band: Band) -> Found {
        let mut prism = crate::test_support_fixtures::prism_z::<T>(
            &self.thin.profile(),
            0.0,
            1.0,
            Tol::witness(),
        );
        let face = prism.top_face;
        let s = prism.body.get_face(face).unwrap().surface;
        match prism.body.surfaces.get_mut(s).unwrap() {
            geom::Surface::Plane { origin, .. } => {
                *origin = Point3::new(self.far.0, self.far.1, 1.0).map(T::from_f64);
            }
            _ => panic!("a prism's cap is a plane"),
        }
        let geo = snapshot(&prism.body);
        let f = geo.faces.iter().find(|f| f.key == face).unwrap();
        let [p0, p1] = self.ends();
        let e = segment((p0.map(T::from_f64), p1.map(T::from_f64)));
        let declared = Declared::index(&geo, &ContactRecords::default());
        let mut errors = Vec::new();
        pair_edge_face(&prism.body, &geo, &declared, band, None, &e, f, &mut errors);
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
}

/// **A grazing edge across a thin feature of a non-convex face is
/// pierced where it pierces**: a cap with a V notch or a strip as thin
/// as a micrometre (the fixture's shortest edge) to a centimetre, the
/// crossing within a few widths of it, the origin up to 30 m off, the
/// ends a few `Kε` to `1e4·Kε` off the plane, where the span can
/// outreach the feature.
/// Each crossing is measured exactly and read only `2Kε` clear of the
/// feature's sides; a pierce read where it is not, or none where it is,
/// at either scalar, is wrong.
#[test]
fn a_grazing_edge_across_a_thin_feature_is_read_pierced_where_it_pierces() {
    let label = "census_thin_pierce";
    let mut r = test_utils::fuzz::start(label);
    let mut bad = Vec::new();
    for eps in EPS {
        let band = Band::new(eps, K * eps).unwrap();
        let (mut read, mut held) = (0usize, 0usize);
        for _ in 0..test_utils::fuzz::scaled(COUNT / 2) {
            let width = r.log_uniform(1e-6, 1e-2);
            let thin = if r.below(2) == 0 {
                Thin::Notch(2.0 * width)
            } else {
                Thin::Strip(width)
            };
            let aim = 0.5 + r.range(-3.0, 3.0) * width;
            let half = r.range(0.05, 0.3);
            let pose = Across {
                thin,
                far: (r.range(-30.0, 30.0), r.range(-30.0, 30.0)),
                y: r.range(0.1, 0.45),
                x: (aim - half, aim + half),
                off: r.log_uniform(1.5 * K * eps, 1e4 * K * eps),
            };
            let inside = thin.inside(pose.crossing(), pose.y);
            let margin = 2.0 * band.escalate();
            let truth = if inside >= margin {
                Found::Pierce
            } else if inside <= -margin {
                Found::Nothing
            } else {
                continue;
            };
            let (f, iv) = (pose.read::<f64>(band), pose.read::<Interval>(band));
            read += 1;
            held += usize::from(f == Found::Refused);
            let wrong = |got: Found| got != Found::Refused && got != truth;
            if wrong(f) || wrong(iv) {
                bad.push(format!(
                    "ε {eps:e} {pose:?}: {inside:e} inside, f64 {f:?}, Interval {iv:?}"
                ));
            }
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
    };
    let bn = [n.x, n.y, n.z].map(big);
    let exact = big_dot(&[p.x, p.y, p.z].map(big), &bn) - big_dot(&[o.x, o.y, o.z].map(big), &bn);
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
