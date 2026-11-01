//! **The link test reads two polygon cones as their materials do**:
//! random pairs of prisms, each over a profile star-shaped from one
//! corner, that corner at the origin, read by [`rest::judge`] over the
//! two solids' links there and against an exact oracle.
//!
//! A profile's corner at the origin spans 20° to 340°, so the cones are
//! convex, reflex (a saddle at the prism's corner), or anything between.
//! Four families, a quarter each:
//! - **turned** at random;
//! - **snapped**: quarter turns over profiles whose rays lie on
//!   multiples of 45°, so coplanar faces, facing either way, and rays
//!   lying in faces are common;
//! - **bowtie**: one prism over a profile that passes through the origin
//!   twice, at two vertices on one key — two cones whose bottom face is
//!   one face held at two corners — against a snapped or turned prism;
//! - **near**: snapped poses each turned off by a few band widths, so
//!   faces lie within or just past the band of coplanar and readings
//!   take their in-band paths.
//!
//! The oracle is exact and shares nothing with the reader: each
//! prism's cone at the corner is a union of convex cones, one per gap
//! between neighbouring profile rays (under 150°), each three
//! half-spaces in the prism's own frame. Two convex cones' interiors
//! meet iff their intersection's extreme rays (the feasible cross
//! products of two of the six planes) span it and their mean lies
//! strictly inside all six, by a margin read as the least normalized
//! constraint value there; whether they meet at all is read exactly,
//! the same construction over the normals as exact integers
//! ([`overlaps_exactly`]), so an overlap a hair wide counts. **What is
//! wrong**: a rest where some pair overlaps by more than `1e-6` (a depth
//! of a micrometre per metre, far past the band), and a crossing where
//! no pair overlaps at all. A refusal is never wrong here; each kind is
//! counted and logged.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Gated to the readings it checks and the band they decide with.
test_utils::gated_to![
    "crates/topo/src/census.rs",
    "crates/topo/src/link.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
];

use super::*;
use crate::link::LINK_PLANE;
use crate::link::metric::{self, Distance};
use test_utils::fuzz::Rng;

type V = [f64; 3];
type Rot = [V; 3];

/// One prism: its profile (counterclockwise, through the origin once,
/// or twice for a bowtie), the profile's fans about the origin (each a
/// run of rays, its material's cone the wedges between neighbours),
/// height, and frame (rows: where local `x`, `y`, `z` go).
struct Prism {
    profile: Vec<(f64, f64)>,
    fans: Vec<Vec<(f64, f64)>>,
    h: f64,
    rot: Rot,
}

/// Rays from the origin over `sweep` from `start`, at least `min_gaps`
/// gaps none past 150°, each at a random reach.
fn fan(r: &mut Rng, start: f64, sweep: f64, min_gaps: usize) -> Vec<(f64, f64)> {
    let gaps = min_gaps.max(1 + (sweep / 150f64.to_radians()).floor() as usize) + r.below(3);
    (0..=gaps)
        .map(|k| {
            let a = start + sweep * k as f64 / gaps as f64;
            let rad = r.range(0.4, 1.0);
            (rad * a.cos(), rad * a.sin())
        })
        .collect()
}

impl Prism {
    fn draw(r: &mut Rng, snapped: bool) -> Self {
        let sweep = if snapped {
            45f64.to_radians() * (1 + r.below(7)) as f64
        } else {
            r.range(20f64.to_radians(), 340f64.to_radians())
        };
        let start = if snapped {
            45f64.to_radians() * r.below(8) as f64
        } else {
            r.range(0.0, std::f64::consts::TAU)
        };
        let rays = fan(r, start, sweep, 1);
        let rot = if snapped {
            quarter_turns(r)
        } else {
            random_rotation(r)
        };
        let mut profile = vec![(0.0, 0.0)];
        profile.extend(&rays);
        Self {
            profile,
            fans: vec![rays],
            h: r.range(0.3, 1.0),
            rot,
        }
    }

    /// A bowtie: two fans in disjoint angular ranges, the profile running
    /// out through one and back to the origin, then out through the
    /// other: one loop through the origin twice.
    fn bowtie(r: &mut Rng, snapped: bool) -> Self {
        let step = 45f64.to_radians();
        let (start, a, gap, b) = if snapped {
            let a = step * (1 + r.below(2)) as f64;
            let gap = step * (1 + r.below(2)) as f64;
            let b = step * (1 + r.below(2)) as f64;
            (step * r.below(8) as f64, a, gap, b)
        } else {
            let a = r.range(20f64.to_radians(), 120f64.to_radians());
            let gap = r.range(15f64.to_radians(), 100f64.to_radians());
            let b = r.range(20f64.to_radians(), 120f64.to_radians());
            (r.range(0.0, std::f64::consts::TAU), a, gap, b)
        };
        let fa = fan(r, start, a, 1);
        let fb = fan(r, start + a + gap, b, 1);
        let mut profile = vec![(0.0, 0.0)];
        profile.extend(&fa);
        profile.push((0.0, 0.0));
        profile.extend(&fb);
        let rot = if snapped {
            quarter_turns(r)
        } else {
            random_rotation(r)
        };
        Self {
            profile,
            fans: vec![fa, fb],
            h: r.range(0.3, 1.0),
            rot,
        }
    }

    /// The same prism turned by `angle` about a random axis.
    fn nudged(mut self, r: &mut Rng, angle: f64) -> Self {
        let k = {
            let v = [0; 3].map(|_| r.range(-1.0, 1.0));
            let n = dot(v, v).sqrt();
            v.map(|x| x / n)
        };
        let (c, sn) = (angle.cos(), angle.sin());
        let turn = |v: V| {
            let kv = cross(k, v);
            let kd = dot(k, v);
            [0, 1, 2].map(|i| v[i] * c + kv[i] * sn + k[i] * kd * (1.0 - c))
        };
        self.rot = self.rot.map(turn);
        self
    }

    fn map(&self, x: f64, y: f64, z: f64) -> Point3<f64> {
        let [a, b, c] = self.rot;
        Point3::new(
            a[0] * x + b[0] * y + c[0] * z,
            a[1] * x + b[1] * y + c[1] * z,
            a[2] * x + b[2] * y + c[2] * z,
        )
    }

    /// The prism's cone at the corner as convex cones, each the three
    /// inward normals (in world coordinates) of `z ≥ 0` and the two
    /// rays bounding one gap of the profile.
    fn pieces(&self) -> Vec<[V; 3]> {
        let world = |h: V| {
            let [a, b, c] = self.rot;
            [0, 1, 2].map(|i| a[i] * h[0] + b[i] * h[1] + c[i] * h[2])
        };
        let mut out = Vec::new();
        for fan in &self.fans {
            let rays: Vec<(f64, f64)> = fan
                .iter()
                .map(|&(x, y)| (x / x.hypot(y), y / x.hypot(y)))
                .collect();
            out.extend(rays.windows(2).map(|w| {
                let ((ax, ay), (bx, by)) = (w[0], w[1]);
                [
                    world([0.0, 0.0, 1.0]),
                    world([-ay, ax, 0.0]),
                    world([by, -bx, 0.0]),
                ]
            }));
        }
        out
    }
}

/// Whether two convex cones' interiors meet, exactly: their normals
/// (each an `f64` triple) read as exact integers, the intersection's
/// extreme rays the feasible cross products of two of the six, and the
/// interior met iff their sum lies strictly inside all six — no
/// threshold, so an overlap a hair wide counts.
fn overlaps_exactly(p: &[V; 3], q: &[V; 3]) -> bool {
    use super::segment_fuzz::{big, big_cross, big_dot};
    use num_bigint::BigInt;
    let zero = BigInt::from(0);
    let h: Vec<[BigInt; 3]> = p.iter().chain(q).map(|v| v.map(big)).collect();
    let mut sum = [zero.clone(), zero.clone(), zero.clone()];
    let mut any = false;
    for i in 0..6 {
        for j in i + 1..6 {
            let c = big_cross(&h[i], &h[j]);
            if c.iter().all(|x| *x == zero) {
                continue;
            }
            for r in [c.clone(), c.clone().map(|x| -x)] {
                if h.iter().all(|g| big_dot(g, &r) >= zero) {
                    any = true;
                    for k in 0..3 {
                        sum[k] += &r[k];
                    }
                }
            }
        }
    }
    any && h.iter().all(|g| big_dot(g, &sum) > zero)
}

/// The deepest overlap of any convex piece of one cone with any of the
/// other's ([`overlap`]).
fn depth(xs: &[&Prism], y: &Prism) -> f64 {
    xs.iter()
        .flat_map(|x| x.pieces())
        .flat_map(|p| y.pieces().into_iter().map(move |q| overlap(&p, &q)))
        .fold(0.0, f64::max)
}

/// Whether any convex piece of one cone meets any of the other's,
/// exactly ([`overlaps_exactly`]).
fn meet(xs: &[&Prism], y: &Prism) -> bool {
    xs.iter()
        .flat_map(|x| x.pieces())
        .any(|p| y.pieces().iter().any(|q| overlaps_exactly(&p, q)))
}

fn dot(a: V, b: V) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: V, b: V) -> V {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// How deep two convex cones' interiors overlap: the least normalized
/// constraint value at the mean of their intersection's extreme rays,
/// or zero where the intersection spans no volume.
fn overlap(p: &[V; 3], q: &[V; 3]) -> f64 {
    let h: Vec<V> = p.iter().chain(q).copied().collect();
    let mut rays: Vec<V> = Vec::new();
    for i in 0..6 {
        for j in i + 1..6 {
            let c = cross(h[i], h[j]);
            let n = dot(c, c).sqrt();
            if n < 1e-14 {
                continue;
            }
            for s in [1.0, -1.0] {
                let r = c.map(|x| s * x / n);
                if h.iter().all(|&g| dot(g, r) >= -1e-12) {
                    rays.push(r);
                }
            }
        }
    }
    let m = rays.iter().fold([0.0; 3], |acc, r| {
        [acc[0] + r[0], acc[1] + r[1], acc[2] + r[2]]
    });
    let spans = rays.iter().any(|&a| {
        rays.iter()
            .any(|&b| rays.iter().any(|&c| dot(cross(a, b), c).abs() > 1e-9))
    });
    if !spans {
        return 0.0;
    }
    h.iter()
        .map(|&g| dot(g, m) / (dot(g, g) * dot(m, m)).sqrt())
        .fold(f64::INFINITY, f64::min)
        .max(0.0)
}

fn quarter_turns(r: &mut Rng) -> Rot {
    let axes: Rot = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let mut rows = [0usize, 1, 2];
    for i in (1..3).rev() {
        rows.swap(i, r.below(i + 1));
    }
    let mut out = rows.map(|i| axes[i]);
    for row in &mut out[..2] {
        if r.below(2) == 1 {
            *row = row.map(|x| -x);
        }
    }
    // Right-handed: the third row the cross of the first two.
    let [a, b, _] = out;
    out[2] = [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ];
    out
}

fn random_rotation(r: &mut Rng) -> Rot {
    let q = loop {
        let q = [0; 4].map(|_| r.range(-1.0, 1.0));
        let n = q.iter().map(|x| x * x).sum::<f64>().sqrt();
        if n > 0.1 && n < 1.0 {
            break q.map(|x| x / n);
        }
    };
    let [w, x, y, z] = q;
    [
        [
            1.0 - 2.0 * (y * y + z * z),
            2.0 * (x * y - w * z),
            2.0 * (x * z + w * y),
        ],
        [
            2.0 * (x * y + w * z),
            1.0 - 2.0 * (x * x + z * z),
            2.0 * (y * z - w * x),
        ],
        [
            2.0 * (x * z - w * y),
            2.0 * (y * z + w * x),
            1.0 - 2.0 * (x * x + y * y),
        ],
    ]
}

/// A prism as a body of its own.
fn build(p: &Prism) -> Body<f64> {
    use crate::test_support_fixtures::{FaceGeometry, describe_as_intersections, prism_ops};
    let mut body = Body::<f64>::new();
    prism_ops(
        &mut body,
        &p.profile,
        (0.0, p.h),
        |x, y, z| p.map(x, y, z),
        FaceGeometry::Certified,
        Tol::witness(),
    );
    describe_as_intersections(&mut body, Tol::witness());
    body
}

/// `body`'s vertices on the origin moved onto one key, as an op that
/// decided them one point leaves them; that key.
fn one_key(body: &mut Body<f64>) -> crate::geometry::PointKey {
    let origin: Vec<VertexKey> = body
        .vertices
        .keys()
        .filter(|&v| {
            let q = body.points[body.vertices[v].point];
            q.x == 0.0 && q.y == 0.0 && q.z == 0.0
        })
        .collect();
    let key = body.vertices[origin[0]].point;
    for &v in &origin[1..] {
        let old = body.vertices[v].point;
        body.vertices[v].point = key;
        if old != key {
            body.points.remove(old);
        }
    }
    key
}

/// The verdict on the two solids of `body` at the origin's key, over
/// their whole links there.
fn verdict_at_origin(body: &mut Body<f64>, band: Band) -> TouchVerdict {
    let key = one_key(body);
    let geo = snapshot(body);
    let keys = BTreeSet::from([key]);
    let solids: BTreeSet<_> = body
        .vertices
        .keys()
        .filter(|&v| body.vertices[v].point == key)
        .map(|v| census_entity_solid(body, EntityId::Vertex(v)).unwrap())
        .collect();
    assert_eq!(solids.len(), 2, "two solids at the origin");
    let o = Point3::new(0.0, 0.0, 0.0);
    let link = |s| Link::touch_at_point(body, &geo, &Ties::default(), s, &keys, o, band);
    let mut it = solids.iter().copied();
    let (sa, sb) = (it.next().unwrap(), it.next().unwrap());
    match (link(sa), link(sb)) {
        (Ok(x), Ok(y)) => rest::judge(&x, &y, band),
        (Err(v), _) | (_, Err(v)) => v.into(),
    }
}

/// **A pinched face read at both its corners.** A bowtie prism's caps
/// pass the origin twice, through two vertices on its one key: its lobes
/// over `0°–45°` and `135°–180°`. Each corner's sector ends where the
/// loop next returns to the point, on the other lobe's edge, not at its
/// own pass's incoming edge. A wedge prism in the gap between the lobes
/// reads a decided rest, one reaching into the first lobe a crossing,
/// as the oracle says. Red where a pass ends at its own incoming ray:
/// the sectors then cross the gap, and the link refuses unanalysed
/// either way. The bowtie family of the falsifier below draws these
/// shapes at random; this is the floor that its decided verdicts stand on.
#[test]
fn a_bowtie_reads_a_rest_in_its_gap_and_a_crossing_in_a_lobe() {
    let band = Band::linear(Tol::witness()).unwrap();
    let id: Rot = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let bowtie = Prism {
        profile: vec![
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (0.0, 0.0),
            (-1.0, 1.0),
            (-1.0, 0.0),
        ],
        fans: vec![vec![(1.0, 0.0), (1.0, 1.0)], vec![(-1.0, 1.0), (-1.0, 0.0)]],
        h: 0.5,
        rot: id,
    };
    let wedge = |profile: Vec<(f64, f64)>| Prism {
        fans: vec![profile[1..].to_vec()],
        profile,
        h: 0.5,
        rot: id,
    };
    let verdict = |b: &Prism| {
        let mut body = build(&bowtie);
        crate::instance::graft_disjoint(&mut body, &build(b)).unwrap();
        verdict_at_origin(&mut body, band)
    };
    let gap = wedge(vec![(0.0, 0.0), (1.0, 2.0), (-1.0, 2.0)]);
    let lobe = wedge(vec![(0.0, 0.0), (1.0, 0.5), (0.0, 1.0)]);
    assert!(!meet(&[&bowtie], &gap) && meet(&[&bowtie], &lobe));
    assert!(
        matches!(verdict(&gap), TouchVerdict::Rest(_)),
        "{:?}",
        verdict(&gap)
    );
    assert_eq!(verdict(&lobe), TouchVerdict::Crossing);
}

/// **A link pair's rest is never an overlap, nor its crossing a pair
/// apart.**
#[test]
fn the_link_test_agrees_with_the_prisms_own_inequalities() {
    let label = "census_link_fuzz";
    let mut r = test_utils::fuzz::start(label);
    let band = Band::linear(Tol::witness()).unwrap();
    let mut bad = Vec::new();
    let mut tally: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for trial in 0..test_utils::fuzz::scaled(300) {
        let snapped = trial % 2 == 1;
        let (family, a, b) = match trial % 4 {
            0 | 1 => (
                if snapped { "snapped" } else { "turned" },
                Prism::draw(&mut r, snapped),
                Prism::draw(&mut r, snapped),
            ),
            2 => {
                let snapped = r.below(2) == 0;
                (
                    "bowtie",
                    Prism::bowtie(&mut r, snapped),
                    Prism::draw(&mut r, snapped),
                )
            }
            _ => {
                // A few band widths off a snapped pose, at the profile's
                // metre scale.
                let nudge = |p: Prism, r: &mut Rng| {
                    let angle = band.zero() * 10f64.powf(r.range(-0.5, 1.5));
                    p.nudged(r, angle)
                };
                let (a, b) = (Prism::draw(&mut r, true), Prism::draw(&mut r, true));
                let a = nudge(a, &mut r);
                ("near", a, nudge(b, &mut r))
            }
        };
        let mut body = build(&a);
        crate::instance::graft_disjoint(&mut body, &build(&b)).unwrap();
        let verdict = verdict_at_origin(&mut body, band);
        let depth = depth(&[&a], &b);
        let (clear, any) = (depth > 1e-6, meet(&[&a], &b));
        let kind = match &verdict {
            TouchVerdict::Rest(_) => "rest",
            TouchVerdict::Crossing => "crossing",
            other => {
                *tally.entry(format!("{family}: {other:?}")).or_default() += 1;
                continue;
            }
        };
        *tally
            .entry(format!("{family}: {kind}, oracle overlap {clear}/{any}"))
            .or_default() += 1;
        if (kind == "rest" && clear) || (kind == "crossing" && !any) {
            bad.push(format!(
                "trial {trial} ({family}): {kind}, oracle depth {depth:e}; a {:?} h {} rot {:?}; b {:?} h {} rot {:?}",
                a.profile,
                a.h,
                a.rot,
                b.profile,
                b.h,
                b.rot
            ));
        }
    }
    println!("[fuzz] {label}: {tally:?}");
    assert!(
        bad.is_empty(),
        "{label}: {} wrong; {}:\n{}",
        bad.len(),
        test_utils::fuzz::replay(),
        bad[..bad.len().min(6)].join("\n")
    );
}

/// **The whole link of a pinch.** Solid A is two prisms whose corners
/// share the origin's key and whose cones are apart (oracle depth 0),
/// made one solid; solid B one prism on the same key. `rest::judge` over
/// A's whole link (both cones, gathered by `Link::at_point` from the key
/// alone) against B's, checked against the same exact oracle over A's
/// two cones.
#[test]
fn a_pinch_link_agrees_with_the_prisms_own_inequalities() {
    let label = "census_link_pinch_fuzz";
    let mut r = test_utils::fuzz::start(label);
    let band = Band::linear(Tol::witness()).unwrap();
    let mut bad = Vec::new();
    let mut tally: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for trial in 0..test_utils::fuzz::scaled(300) {
        let snapped = trial % 2 == 1;
        let (a1, a2) = loop {
            let (x, y) = (Prism::draw(&mut r, snapped), Prism::draw(&mut r, snapped));
            if !meet(&[&x], &y) {
                break (x, y);
            }
        };
        let b = Prism::draw(&mut r, snapped);
        let mut body = build(&a1);
        let sa = body.solids().next().map(|(k, _)| k).unwrap();
        let s2 = crate::instance::graft_disjoint(&mut body, &build(&a2)).unwrap();
        crate::instance::graft_disjoint(&mut body, &build(&b)).unwrap();
        for sh in body.solids.remove(s2).unwrap().shells {
            body.shells[sh].solid = sa;
            body.solids[sa].shells.push(sh);
        }
        let verdict = verdict_at_origin(&mut body, band);
        let d = depth(&[&a1, &a2], &b);
        let (clear, any) = (d > 1e-6, meet(&[&a1, &a2], &b));
        let kind = match &verdict {
            TouchVerdict::Rest(_) => "rest",
            TouchVerdict::Crossing => "crossing",
            other => {
                *tally.entry(format!("{other:?}")).or_default() += 1;
                continue;
            }
        };
        *tally
            .entry(format!("{kind}, oracle overlap {clear}/{any}"))
            .or_default() += 1;
        if (kind == "rest" && clear) || (kind == "crossing" && !any) {
            bad.push(format!(
                "trial {trial} ({snapped}): {kind}, depth {d:e}; a1 {:?} h {} rot {:?}; a2 {:?} h {} rot {:?}; b {:?} h {} rot {:?}",
                a1.profile, a1.h, a1.rot, a2.profile, a2.h, a2.rot, b.profile, b.h, b.rot
            ));
        }
    }
    println!("[fuzz] {label}: {tally:?}");
    assert!(
        bad.is_empty(),
        "{label}: {} wrong; {}:\n{}",
        bad.len(),
        test_utils::fuzz::replay(),
        bad[..bad.len().min(6)].join("\n")
    );
}

fn verdict_at<T: Decide + crate::props::AtRestPolicy>(
    a: &Prism,
    b: &Prism,
    band: Band,
) -> TouchVerdict {
    use crate::test_support_fixtures::{FaceGeometry, describe_as_intersections, prism_ops};
    let build = |p: &Prism| {
        let mut body = Body::<T>::new();
        prism_ops(
            &mut body,
            &p.profile,
            (0.0, p.h),
            |x, y, z| {
                let q = p.map(x, y, z);
                Point3::new(T::from_f64(q.x), T::from_f64(q.y), T::from_f64(q.z))
            },
            FaceGeometry::Certified,
            Tol::witness(),
        );
        describe_as_intersections(&mut body, Tol::witness());
        body
    };
    let mut body = build(a);
    crate::instance::graft_disjoint(&mut body, &build(b)).unwrap();
    let geo = snapshot(&body);
    // At the origin: decided on each axis's plane through it.
    let (zero, one) = (T::from_f64(0.0), T::from_f64(1.0));
    let o = Point3::new(zero, zero, zero);
    let axes = [
        Vec3::new(one, zero, zero),
        Vec3::new(zero, one, zero),
        Vec3::new(zero, zero, one),
    ];
    let at_origin: Vec<VertexKey> = geo
        .verts
        .iter()
        .filter(|&&(_, q)| {
            axes.iter()
                .all(|&e| metric::sign(LINK_PLANE, Distance::of(q, o, e), band) == Some(Sign::Zero))
        })
        .map(|&(v, _)| v)
        .collect();
    assert_eq!(at_origin.len(), 2);
    let link = |v: VertexKey| {
        let keys = BTreeSet::from([geo.vpoint[&v]]);
        let solid = census_entity_solid(&body, EntityId::Vertex(v)).unwrap();
        Link::touch_at_point(
            &body,
            &geo,
            &Ties::default(),
            solid,
            &keys,
            geo.vmap[&v],
            band,
        )
    };
    match (link(at_origin[0]), link(at_origin[1])) {
        (Ok(x), Ok(y)) => rest::judge(&x, &y, band),
        (Err(v), _) | (_, Err(v)) => v.into(),
    }
}

/// **The link test at `Interval`.** Prisms drawn as the falsifier's
/// turned and snapped families, read at `Interval`, against the same
/// exact oracle, and against the `f64` reading (a decided class the two
/// scalars disagree on is wrong).
#[test]
fn the_link_test_at_interval_agrees_with_the_oracle() {
    use geom_core::Interval;
    let label = "census_link_interval_fuzz";
    let mut r = test_utils::fuzz::start(label);
    let band = Band::linear(Tol::witness()).unwrap();
    let mut bad = Vec::new();
    let mut tally: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for trial in 0..test_utils::fuzz::scaled(100) {
        let snapped = trial % 2 == 1;
        let (a, b) = (Prism::draw(&mut r, snapped), Prism::draw(&mut r, snapped));
        let iv = verdict_at::<Interval>(&a, &b, band);
        let fl = verdict_at::<f64>(&a, &b, band);
        let depth = depth(&[&a], &b);
        let class = |v: &TouchVerdict| match v {
            TouchVerdict::Rest(_) => "rest".to_owned(),
            TouchVerdict::Crossing => "crossing".to_owned(),
            other => format!("{other:?}"),
        };
        let (ci, cf) = (class(&iv), class(&fl));
        *tally.entry(format!("iv {ci} / f64 {cf}")).or_default() += 1;
        let decided = |c: &str| c == "rest" || c == "crossing";
        let wrong_iv = (ci == "rest" && depth > 1e-6) || (ci == "crossing" && !meet(&[&a], &b));
        if wrong_iv || (decided(&ci) && decided(&cf) && ci != cf) {
            bad.push(format!(
                "trial {trial}: iv {ci}, f64 {cf}, depth {depth:e}; a {:?} h {} rot {:?}; b {:?} h {} rot {:?}",
                a.profile, a.h, a.rot, b.profile, b.h, b.rot
            ));
        }
    }
    println!("[fuzz] {label}: {tally:?}");
    assert!(
        bad.is_empty(),
        "{label}: {} wrong; {}:\n{}",
        bad.len(),
        test_utils::fuzz::replay(),
        bad[..bad.len().min(6)].join("\n")
    );
}
