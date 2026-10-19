//! **The link test reads two polygon cones as their materials do**:
//! random pairs of prisms, each over a profile star-shaped from one
//! corner, that corner at the origin, read by [`rest::judge`] over the
//! two solids' links there and against an exact oracle.
//!
//! A profile's corner at the origin spans 20° to 340°, so the cones are
//! convex, reflex (a saddle at the prism's corner), or anything between;
//! half the pairs are turned at random, half by quarter turns over
//! profiles whose rays lie on multiples of 45°, so coplanar faces,
//! facing either way, and rays lying in faces are common.
//!
//! The oracle is exact and shares nothing with the reader: each
//! prism's cone at the corner is a union of convex cones, one per gap
//! between neighbouring profile rays (under 150°), each three
//! half-spaces in the prism's own frame. Two convex cones' interiors
//! meet iff their intersection's extreme rays (the feasible cross
//! products of two of the six planes) span it and their mean lies
//! strictly inside all six, by a margin read as the least normalized
//! constraint value there. **What is wrong**: a rest where some pair
//! overlaps by more than `1e-6` (a depth of a micrometre per metre, far
//! past the band), and a crossing where no pair overlaps at all. A
//! refusal is never wrong here; each kind is counted and logged.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Gated to the readings it checks and the band they decide with.
test_utils::gated_to![
    "crates/topo/src/census.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
];

use super::*;
use test_utils::fuzz::Rng;

type V = [f64; 3];
type Rot = [V; 3];

/// One prism: its profile (the corner at the origin first, then the
/// rest counterclockwise), height, and frame (rows: where local `x`,
/// `y`, `z` go).
struct Prism {
    profile: Vec<(f64, f64)>,
    h: f64,
    rot: Rot,
}

impl Prism {
    fn draw(r: &mut Rng, snapped: bool) -> Self {
        let sweep = if snapped {
            45f64.to_radians() * (1 + r.below(7)) as f64
        } else {
            r.range(20f64.to_radians(), 340f64.to_radians())
        };
        // Rays at the two bounds and a few between, no gap past 150°.
        let gaps = 1 + (sweep / 150f64.to_radians()).floor() as usize + r.below(3);
        let start = if snapped {
            45f64.to_radians() * r.below(8) as f64
        } else {
            r.range(0.0, std::f64::consts::TAU)
        };
        let mut profile = vec![(0.0, 0.0)];
        for k in 0..=gaps {
            let a = start + sweep * k as f64 / gaps as f64;
            let rad = r.range(0.4, 1.0);
            profile.push((rad * a.cos(), rad * a.sin()));
        }
        let rot = if snapped {
            quarter_turns(r)
        } else {
            random_rotation(r)
        };
        Self {
            profile,
            h: r.range(0.3, 1.0),
            rot,
        }
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
        let rays: Vec<(f64, f64)> = self.profile[1..]
            .iter()
            .map(|&(x, y)| (x / x.hypot(y), y / x.hypot(y)))
            .collect();
        let world = |h: V| {
            let [a, b, c] = self.rot;
            [0, 1, 2].map(|i| a[i] * h[0] + b[i] * h[1] + c[i] * h[2])
        };
        rays.windows(2)
            .map(|w| {
                let ((ax, ay), (bx, by)) = (w[0], w[1]);
                [
                    world([0.0, 0.0, 1.0]),
                    world([-ay, ax, 0.0]),
                    world([by, -bx, 0.0]),
                ]
            })
            .collect()
    }
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

/// The two prisms as two solids of one body, and the link of each at
/// its corner on the origin.
fn links(a: &Prism, b: &Prism, band: Band) -> (Body<f64>, LinkPair<f64>) {
    use crate::test_support_fixtures::{FaceGeometry, describe_as_intersections, prism_ops};
    let build = |p: &Prism| {
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
    };
    let mut body = build(a);
    crate::instance::graft_disjoint(&mut body, &build(b)).unwrap();
    let geo = snapshot(&body);
    let at_origin: Vec<VertexKey> = geo
        .verts
        .iter()
        .filter(|(_, q)| q.x == 0.0 && q.y == 0.0 && q.z == 0.0)
        .map(|&(v, _)| v)
        .collect();
    assert_eq!(at_origin.len(), 2, "one corner of each prism on the origin");
    let link = |v: VertexKey| {
        let keys = BTreeSet::from([geo.vpoint[&v]]);
        let solid = census_entity_solid(&body, EntityId::Vertex(v)).unwrap();
        Link::at_point(
            &body,
            &geo,
            &Ties::default(),
            solid,
            &keys,
            geo.vmap[&v],
            band,
        )
    };
    let pair = (link(at_origin[0]), link(at_origin[1]));
    (body, pair)
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
        let (a, b) = (Prism::draw(&mut r, snapped), Prism::draw(&mut r, snapped));
        let (_, pair) = links(&a, &b, band);
        let verdict = match pair {
            (Ok(x), Ok(y)) => rest::judge(&x, &y, band),
            (Err(v), _) | (_, Err(v)) => v,
        };
        let depth = a
            .pieces()
            .iter()
            .flat_map(|p| b.pieces().into_iter().map(move |q| overlap(p, &q)))
            .fold(0.0, f64::max);
        let (clear, any) = (depth > 1e-6, depth > 0.0);
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
                "trial {trial} ({}): {kind}, oracle depth {depth:e}; a {:?} h {} rot {:?}; b {:?} h {} rot {:?}",
                if snapped { "snapped" } else { "turned" },
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

/// **R2 review probe (PR 5331): the whole link of a pinch.** Solid A is
/// two prisms whose corners share the origin's key and whose cones are
/// apart (oracle depth 0), made one solid; solid B one prism on the
/// same key. `rest::judge` over A's whole link (both cones, gathered by
/// `Link::at_point` from the key alone) against B's, checked against
/// the same exact oracle over A's two cones.
#[test]
fn r2_a_pinch_link_agrees_with_the_prisms_own_inequalities() {
    use crate::test_support_fixtures::{FaceGeometry, describe_as_intersections, prism_ops};
    let label = "census_link_pinch_fuzz";
    let mut r = test_utils::fuzz::start(label);
    let band = Band::linear(Tol::witness()).unwrap();
    let build = |p: &Prism| {
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
    };
    let depth = |xs: &[&Prism], y: &Prism| {
        xs.iter()
            .flat_map(|x| x.pieces())
            .flat_map(|p| y.pieces().into_iter().map(move |q| overlap(&p, &q)))
            .fold(0.0, f64::max)
    };
    let mut bad = Vec::new();
    let mut tally: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for trial in 0..test_utils::fuzz::scaled(300) {
        let snapped = trial % 2 == 1;
        let (a1, a2) = loop {
            let (x, y) = (Prism::draw(&mut r, snapped), Prism::draw(&mut r, snapped));
            if depth(&[&x], &y) == 0.0 {
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
        let origin: Vec<VertexKey> = body
            .vertices
            .keys()
            .filter(|&v| {
                let q = body.points[body.vertices[v].point];
                q.x == 0.0 && q.y == 0.0 && q.z == 0.0
            })
            .collect();
        assert_eq!(origin.len(), 3);
        let key = body.vertices[origin[0]].point;
        for &v in &origin[1..] {
            let old = body.vertices[v].point;
            body.vertices[v].point = key;
            body.points.remove(old);
        }
        let geo = snapshot(&body);
        let keys = BTreeSet::from([key]);
        let solids: BTreeSet<_> = origin
            .iter()
            .map(|&v| census_entity_solid(&body, EntityId::Vertex(v)).unwrap())
            .collect();
        assert_eq!(solids.len(), 2);
        let sb = *solids.iter().find(|&&s| s != sa).unwrap();
        let o = Point3::new(0.0, 0.0, 0.0);
        let link = |s| Link::at_point(&body, &geo, &Ties::default(), s, &keys, o, band);
        let verdict = match (link(sa), link(sb)) {
            (Ok(x), Ok(y)) => rest::judge(&x, &y, band),
            (Err(v), _) | (_, Err(v)) => v,
        };
        let d = depth(&[&a1, &a2], &b);
        let (clear, any) = (d > 1e-6, d > 0.0);
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

/// The verdict of `rest::judge` over two prisms' links at the origin,
/// built and read at `T`.
fn r2_verdict_at<T: Decide + Bounds + crate::props::AtRestPolicy>(
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
    let zero = |t: T| t.lo() == 0.0 && t.hi() == 0.0;
    let at_origin: Vec<VertexKey> = geo
        .verts
        .iter()
        .filter(|(_, q)| zero(q.x) && zero(q.y) && zero(q.z))
        .map(|&(v, _)| v)
        .collect();
    assert_eq!(at_origin.len(), 2);
    let link = |v: VertexKey| {
        let keys = BTreeSet::from([geo.vpoint[&v]]);
        let solid = census_entity_solid(&body, EntityId::Vertex(v)).unwrap();
        Link::at_point(
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
        (Err(v), _) | (_, Err(v)) => v,
    }
}

/// **R2 review probe (PR 5331): the link test at `Interval`.** The same
/// draws as the shipped falsifier, read at `Interval`, against the same
/// exact oracle, and against the `f64` reading (a decided class the two
/// scalars disagree on is wrong).
#[test]
fn r2_the_link_test_at_interval_agrees_with_the_oracle() {
    use geom_core::Interval;
    let label = "census_link_interval_fuzz";
    let mut r = test_utils::fuzz::start(label);
    let band = Band::linear(Tol::witness()).unwrap();
    let mut bad = Vec::new();
    let mut tally: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for trial in 0..test_utils::fuzz::scaled(100) {
        let snapped = trial % 2 == 1;
        let (a, b) = (Prism::draw(&mut r, snapped), Prism::draw(&mut r, snapped));
        let iv = r2_verdict_at::<Interval>(&a, &b, band);
        let fl = r2_verdict_at::<f64>(&a, &b, band);
        let depth = a
            .pieces()
            .iter()
            .flat_map(|p| b.pieces().into_iter().map(move |q| overlap(p, &q)))
            .fold(0.0, f64::max);
        let class = |v: &TouchVerdict| match v {
            TouchVerdict::Rest(_) => "rest".to_owned(),
            TouchVerdict::Crossing => "crossing".to_owned(),
            other => format!("{other:?}"),
        };
        let (ci, cf) = (class(&iv), class(&fl));
        *tally.entry(format!("iv {ci} / f64 {cf}")).or_default() += 1;
        let decided = |c: &str| c == "rest" || c == "crossing";
        let wrong_iv = (ci == "rest" && depth > 1e-6) || (ci == "crossing" && depth == 0.0);
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
