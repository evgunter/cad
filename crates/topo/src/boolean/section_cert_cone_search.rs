//! **The cone rows against a traced section** (a counterexample
//! search, shape 1): random cone poses against every partner the cone
//! arms classify, biased toward each classification margin's zero. The
//! oracle traces the zero set of the partner's residual on the double
//! cone's chart — azimuth `u` periodic, signed slant `t` over a window
//! holding every bounded component — counts its components by cell
//! adjacency, and classes each: touching the window's edge, unbounded;
//! else essential iff it crosses the seam `u = 0` an odd number of
//! times. `classify` must agree with 0 mismatches:
//!
//! - where it counts the components (plane, sphere, parallel-axis
//!   cylinder), part for part in count and class; where it lists
//!   parallels nominally (the coaxial rows), every traced component is
//!   essential and the count is the geometry's;
//! - `single` only on one traced component;
//! - every witness on both carriers, and on a traced component of its
//!   part's class — distinct components for distinct parts;
//! - against a parallel-axis cylinder, every component traced on the
//!   cylinder's chart essential there too.

#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

// Gated to the code it tests, as every fuzzer is: the classifier, the
// residuals the witnesses are checked against, and the band and
// geometry types a decision reads.
test_utils::gated_to![
    "crates/topo/src/boolean/section_cert.rs",
    "crates/geom-brep/src/implicit.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/linalg/",
];

use super::*;
use core::f64::consts::{PI, TAU};
use geom::Surface;
use geom_core::{Band, Point3, Tol, Vec3};
use test_utils::fuzz::{self, Rng};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Class {
    Unbounded,
    Essential,
    Null,
}

/// The zero set of a residual on a periodic chart, cut into cells.
struct Traced {
    nu: usize,
    nv: usize,
    v0: f64,
    v1: f64,
    /// Each cell's component, if the zero set crosses it.
    comp: Vec<Option<usize>>,
    classes: Vec<Class>,
}

fn find(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

/// Traces `f ∘ chart` over `u ∈ [0, 2π)` (periodic) and `v ∈ [v0, v1]`.
fn trace(
    chart: &dyn Fn(f64, f64) -> Point3<f64>,
    f: &dyn Fn(Point3<f64>) -> f64,
    (v0, v1): (f64, f64),
    nu: usize,
    nv: usize,
) -> Traced {
    let pos: Vec<bool> = (0..=nv)
        .flat_map(|j| {
            let v = v0 + (v1 - v0) * j as f64 / nv as f64;
            (0..nu).map(move |i| (i, v))
        })
        .map(|(i, v)| f(chart(TAU * i as f64 / nu as f64, v)) >= 0.0)
        .collect();
    let s = |i: usize, j: usize| pos[j * nu + i % nu];
    let cell = |i: usize, j: usize| j * nu + i % nu;
    let n = nu * nv;
    let mut parent: Vec<usize> = (0..n).collect();
    let mut active = vec![false; n];
    for j in 0..nv {
        for i in 0..nu {
            let (a, b, c, d) = (s(i, j), s(i + 1, j), s(i, j + 1), s(i + 1, j + 1));
            active[cell(i, j)] = !(a == b && b == c && c == d);
            // The right neighbour shares column i + 1.
            if s(i + 1, j) != s(i + 1, j + 1) {
                let (x, y) = (find(&mut parent, cell(i, j)), find(&mut parent, cell(i + 1, j)));
                parent[x] = y;
            }
            // The upper neighbour shares row j + 1.
            if j + 1 < nv && s(i, j + 1) != s(i + 1, j + 1) {
                let (x, y) = (find(&mut parent, cell(i, j)), find(&mut parent, cell(i, j + 1)));
                parent[x] = y;
            }
        }
    }
    let mut index = std::collections::BTreeMap::new();
    let mut comp = vec![None; n];
    for k in 0..n {
        if active[k] {
            let root = find(&mut parent, k);
            let next = index.len();
            comp[k] = Some(*index.entry(root).or_insert(next));
        }
    }
    let mut unbounded = vec![false; index.len()];
    let mut seam = vec![0usize; index.len()];
    for i in 0..nu {
        if s(i, 0) != s(i + 1, 0) {
            unbounded[comp[cell(i, 0)].unwrap()] = true;
        }
        if s(i, nv) != s(i + 1, nv) {
            unbounded[comp[cell(i, nv - 1)].unwrap()] = true;
        }
    }
    for j in 0..nv {
        if s(0, j) != s(0, j + 1) {
            seam[comp[cell(nu - 1, j)].unwrap()] += 1;
        }
    }
    let classes = (0..index.len())
        .map(|c| match (unbounded[c], seam[c] % 2) {
            (true, _) => Class::Unbounded,
            (false, 1) => Class::Essential,
            (false, _) => Class::Null,
        })
        .collect();
    Traced {
        nu,
        nv,
        v0,
        v1,
        comp,
        classes,
    }
}

impl Traced {
    /// The components crossing the cells about chart point `(u, v)`.
    fn near(&self, u: f64, v: f64) -> Vec<usize> {
        let i = (u.rem_euclid(TAU) / TAU * self.nu as f64) as isize;
        let j = ((v - self.v0) / (self.v1 - self.v0) * self.nv as f64) as isize;
        let mut out = Vec::new();
        for dj in -1..=1 {
            for di in -1..=1 {
                let jj = j + dj;
                if jj < 0 || jj >= self.nv as isize {
                    continue;
                }
                let ii = (i + di).rem_euclid(self.nu as isize) as usize;
                if let Some(c) = self.comp[jj as usize * self.nu + ii] {
                    if !out.contains(&c) {
                        out.push(c);
                    }
                }
            }
        }
        out
    }
}

/// A cone's frame, and its chart on the double carrier: `(u, t)` is
/// the point `t` along the generator at azimuth `u`.
struct Frame {
    apex: Point3<f64>,
    a: Vec3<f64>,
    e1: Vec3<f64>,
    e2: Vec3<f64>,
    alpha: f64,
}

impl Frame {
    fn radial(&self, u: f64) -> Vec3<f64> {
        self.e1 * u.cos() + self.e2 * u.sin()
    }
    fn at(&self, u: f64, t: f64) -> Point3<f64> {
        let (s, c) = self.alpha.sin_cos();
        self.apex + (self.a * c + self.radial(u) * s) * t
    }
    /// The chart point of a point on the carrier.
    fn chart(&self, q: Point3<f64>) -> (f64, f64) {
        let (s, c) = self.alpha.sin_cos();
        let d = q - self.apex;
        let t = d.dot(self.a) / c;
        let r = (d - self.a * d.dot(self.a)) / (t * s);
        (r.dot(self.e2).atan2(r.dot(self.e1)), t)
    }
    fn surface(&self) -> Surface<f64> {
        Surface::Cone {
            apex: self.apex,
            axis: self.a,
            half_angle: self.alpha,
            u_ref: self.e1,
        }
    }
}

fn unit_vec(rng: &mut Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let n = v.norm();
        if n > 0.2 && n < 1.0 {
            return v / n;
        }
    }
}

fn random_cone(rng: &mut Rng) -> Frame {
    let a = unit_vec(rng);
    let (e1, e2) = a.orthonormal_basis();
    Frame {
        apex: Point3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        ),
        a,
        e1,
        e2,
        alpha: rng.range(0.25, 1.2),
    }
}

/// An offset from a margin's zero: magnitude log-uniform in
/// `[0.03, 0.5]`, either sign.
fn offset(rng: &mut Rng) -> f64 {
    let m = (rng.range(0.03f64.ln(), 0.5f64.ln())).exp();
    if rng.unit() < 0.5 { m } else { -m }
}

/// What the arm promises of the traced count.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Count {
    /// Part for part.
    Exact,
    /// Nominal parallels: every traced component essential, count in
    /// the set.
    Parallels(&'static [usize]),
}

struct Pose {
    what: String,
    partner: Surface<f64>,
    window: f64,
    count: Count,
}

fn plane_pose(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let beta = (PI / 2.0 - k.alpha + offset(rng)).clamp(0.02, PI / 2.0);
    let phi = rng.range(0.0, TAU);
    let mut n = k.a * beta.cos() + k.radial(phi) * beta.sin();
    if rng.unit() < 0.5 {
        n = -n;
    }
    let m = rng.range(0.1, 1.5) * if rng.unit() < 0.5 { 1.0 } else { -1.0 };
    let p0 = k.apex - n * m;
    // Every generator's crossing with the plane: one sign throughout is
    // the ellipse, bounded by the farthest; else the branches' vertices.
    let (s, c) = k.alpha.sin_cos();
    let lam: Vec<f64> = (0..3600)
        .map(|i| {
            let g = k.a * c + k.radial(TAU * f64::from(i) / 3600.0) * s;
            (p0 - k.apex).dot(n) / g.dot(n)
        })
        .collect();
    let window = if lam.iter().all(|l| *l > 0.0) || lam.iter().all(|l| *l < 0.0) {
        1.3 * lam.iter().fold(0.0f64, |w, l| w.max(l.abs()))
    } else {
        let nearest = |pos: bool| {
            lam.iter()
                .filter(|l| (**l > 0.0) == pos)
                .fold(f64::INFINITY, |w, l| w.min(l.abs()))
        };
        3.0 * nearest(true).max(nearest(false))
    };
    (window < 30.0).then(|| Pose {
        what: format!("plane p0 {p0:?} n {n:?} (β {beta})"),
        partner: Surface::Plane {
            origin: p0,
            normal: n,
            u_ref: n.orthonormal_basis().0,
        },
        window,
        count: Count::Exact,
    })
}

fn sphere_pose(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let centre = k.apex + k.a * rng.range(-2.0, 2.0) + k.radial(rng.range(0.0, TAU)) * rng.range(0.0, 2.0);
    let delta = k.apex - centre;
    let across = delta - k.a * delta.dot(k.a);
    let r = if across.norm() > 1e-9 {
        across / across.norm()
    } else {
        k.e1
    };
    let (s, c) = k.alpha.sin_cos();
    let line = |w: Vec3<f64>| (delta - w * delta.dot(w)).norm();
    let near = [
        delta.norm(),
        line(k.a * c + r * s),
        line(k.a * c - r * s),
    ][rng.below(3)];
    let rho = near + offset(rng);
    (rho > 0.05).then(|| Pose {
        what: format!("sphere {centre:?} ρ {rho}"),
        partner: Surface::Sphere {
            center: centre,
            radius: rho,
            axis: k.a,
            u_ref: k.e1,
        },
        window: delta.norm() + rho + 0.1,
        count: Count::Exact,
    })
}

fn coaxial_cylinder_pose(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let rc = rng.range(0.1, 1.5);
    Some(Pose {
        what: format!("coaxial cylinder r {rc}"),
        partner: Surface::Cylinder {
            origin: k.apex + k.a * rng.range(-2.0, 2.0),
            axis: if rng.unit() < 0.5 { k.a } else { -k.a },
            radius: rc,
            u_ref: k.e1,
        },
        window: 1.4 * rc / k.alpha.sin(),
        count: Count::Parallels(&[2]),
    })
}

fn coaxial_cone_pose(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let d = rng.range(0.2, 1.5) * if rng.unit() < 0.5 { 1.0 } else { -1.0 };
    let alpha2 = rng.range(0.25, 1.2);
    let (t1, t2) = (k.alpha.tan(), alpha2.tan());
    if (t1 - t2).abs() < 0.15 {
        return None;
    }
    let far = [d * t2 / (t1 + t2), d * t2 / (t2 - t1)]
        .iter()
        .fold(0.0f64, |w, h| w.max(h.abs() / k.alpha.cos()));
    (far < 20.0).then(|| Pose {
        what: format!("coaxial cone d {d} α₂ {alpha2}"),
        partner: Surface::Cone {
            apex: k.apex + k.a * d,
            axis: if rng.unit() < 0.5 { k.a } else { -k.a },
            half_angle: alpha2,
            u_ref: k.e1,
        },
        window: 1.3 * far,
        count: Count::Parallels(&[2]),
    })
}

fn coaxial_torus_pose(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let (s, c) = k.alpha.sin_cos();
    let z = rng.range(-2.0, 2.0);
    let r = rng.range(0.1, 0.5);
    let big = if rng.unit() < 0.5 {
        rng.range(r + 0.1, 2.5)
    } else {
        let side = if rng.unit() < 0.5 { 1.0 } else { -1.0 };
        (side * z * s + r + offset(rng)) / c
    };
    (big > r + 0.05).then(|| Pose {
        what: format!("coaxial torus z {z} R {big} r {r}"),
        partner: Surface::Torus {
            center: k.apex + k.a * z,
            axis: k.a,
            major_radius: big,
            minor_radius: r,
            u_ref: k.e1,
        },
        window: z.abs() + big + r + 0.2,
        count: Count::Parallels(&[0, 2, 4]),
    })
}

fn parallel_cylinder_pose(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let e = rng.range(0.2, 1.5);
    let rc = e + offset(rng);
    (rc > 0.05).then(|| Pose {
        what: format!("parallel cylinder e {e} r {rc}"),
        partner: Surface::Cylinder {
            origin: k.apex + k.e1 * e + k.a * rng.range(-2.0, 2.0),
            axis: k.a,
            radius: rc,
            u_ref: k.e1,
        },
        window: 1.3 * (e + rc) / k.alpha.sin(),
        count: Count::Exact,
    })
}

fn class_of(c: &Component<f64>) -> Class {
    if c.unbounded {
        Class::Unbounded
    } else if c.essential_f {
        Class::Essential
    } else {
        Class::Null
    }
}

/// The mismatches of one pose, as sentences.
fn check(k: &Frame, pose: &Pose) -> Vec<String> {
    let cone = k.surface();
    let reach = Reach {
        centre: k.apex,
        radius: 5.0,
    };
    let sec = classify(&cone, &pose.partner, reach, band());
    let back = classify(&pose.partner, &cone, reach, band());
    let (parts, single) = match &sec {
        Section::Components { parts, single } => (parts.clone(), *single),
        other => return vec![format!("not classified: {other:?}")],
    };
    let mut bad = Vec::new();
    match &back {
        Section::Components {
            parts: b,
            single: sb,
        } if *sb == single
            && b.len() == parts.len()
            && b.iter().zip(&parts).all(|(x, y)| {
                (x.unbounded, x.essential_f, x.essential_g)
                    == (y.unbounded, y.essential_g, y.essential_f)
            }) => {}
        other => bad.push(format!("the swapped order disagrees: {other:?}")),
    }
    let partner = pose.partner.clone();
    let traced = trace(
        &|u, t| k.at(u, t),
        &|q| geom_brep::implicit_residual(&partner, q),
        (-pose.window, pose.window),
        400,
        401,
    );
    let mut want: Vec<Class> = traced.classes.clone();
    want.sort();
    let mut got: Vec<Class> = parts.iter().map(class_of).collect();
    got.sort();
    match pose.count {
        Count::Exact => {
            if want != got {
                bad.push(format!("traced {want:?}, classified {got:?}"));
            }
        }
        Count::Parallels(counts) => {
            if !counts.contains(&want.len()) || want.iter().any(|c| *c != Class::Essential) {
                bad.push(format!("traced {want:?}, not parallels"));
            }
            if want.is_empty() != parts.is_empty()
                || parts.iter().any(|c| !(c.essential_f && c.essential_g))
            {
                bad.push(format!("traced {want:?}, classified {parts:?}"));
            }
        }
    }
    if single && want.len() != 1 {
        bad.push(format!("single, traced {want:?}"));
    }
    let mut seen = Vec::new();
    for c in &parts {
        let Some(w) = c.witness else {
            continue;
        };
        for surf in [&cone, &pose.partner] {
            let r = geom_brep::implicit_residual(surf, w);
            if r.abs() > 1e-9 {
                bad.push(format!("the witness {w:?} is {r} off {surf:?}"));
            }
        }
        let (u, t) = k.chart(w);
        let on: Vec<usize> = traced
            .near(u, t)
            .into_iter()
            .filter(|&i| traced.classes[i] == class_of(c) && !seen.contains(&i))
            .collect();
        match on.first() {
            Some(&i) => seen.push(i),
            None => bad.push(format!(
                "the witness {w:?} of a {:?} part is on no such traced component",
                class_of(c)
            )),
        }
    }
    if let Surface::Cylinder {
        origin,
        axis,
        radius,
        ..
    } = pose.partner
    {
        if !matches!(pose.count, Count::Parallels(_)) {
            let (e1, e2) = axis.orthonormal_basis();
            let z0 = (k.apex - origin).dot(axis);
            let on_cyl = trace(
                &|u, z| origin + (e1 * u.cos() + e2 * u.sin()) * radius + axis * z,
                &|q| geom_brep::implicit_residual(&cone, q),
                (z0 - pose.window, z0 + pose.window),
                400,
                401,
            );
            if on_cyl.classes.iter().any(|c| *c != Class::Essential)
                || parts.iter().any(|c| !c.essential_g)
            {
                bad.push(format!(
                    "on the cylinder traced {:?}, classified {parts:?}",
                    on_cyl.classes
                ));
            }
        }
    }
    bad
}

type Poser = fn(&mut Rng, &Frame) -> Option<Pose>;

/// **Every cone arm against the traced section**, 0 mismatches.
#[test]
fn the_cone_arms_agree_with_the_traced_section() {
    let mut rng = fuzz::start("section_cert_cone_search");
    let arms: [(&str, Poser); 6] = [
        ("plane", plane_pose),
        ("sphere", sphere_pose),
        ("coaxial cylinder", coaxial_cylinder_pose),
        ("coaxial cone", coaxial_cone_pose),
        ("coaxial torus", coaxial_torus_pose),
        ("parallel cylinder", parallel_cylinder_pose),
    ];
    let mut failures = Vec::new();
    for (name, poser) in arms {
        let mut tally = std::collections::BTreeMap::<String, usize>::new();
        let mut done = 0;
        while done < fuzz::scaled(8) {
            let k = random_cone(&mut rng);
            let Some(pose) = poser(&mut rng, &k) else {
                continue;
            };
            done += 1;
            let bad = check(&k, &pose);
            if let Section::Components { parts, .. } =
                classify(&k.surface(), &pose.partner, Reach { centre: k.apex, radius: 5.0 }, band())
            {
                let mut cs: Vec<Class> = parts.iter().map(class_of).collect();
                cs.sort();
                *tally.entry(format!("{cs:?}")).or_default() += 1;
            }
            for b in bad {
                failures.push(format!(
                    "{name}: cone apex {:?} axis {:?} α {}; {}: {b}",
                    k.apex, k.a, k.alpha, pose.what
                ));
            }
        }
        println!("[cone search] {name}: {tally:?}");
    }
    assert!(
        failures.is_empty(),
        "{} mismatches ({}):\n{}",
        failures.len(),
        fuzz::replay(),
        failures.join("\n")
    );
}
