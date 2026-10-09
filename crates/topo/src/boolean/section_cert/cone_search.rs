//! **The cone rows against a traced section** (a counterexample
//! search, shape 1): random cone poses at scales from 1 m to 1 km
//! against every partner the cone arms classify, each pose built at a
//! signed offset from one classification margin's zero, the offset
//! log-uniform from `1e-12` of the scale up to half of it. The oracle
//! for an offset the trace can resolve traces the zero set of the partner's residual on the double
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
//!
//! Below the trace's resolution (`0.03` of the scale) the class is not
//! traced: an offset inside the band's Zero (a quarter of it) must
//! refuse R-tan (the plane's aperture answers its Zero class, the
//! parabola's); one between must answer — R-tan only within a few
//! escalation widths — with every witness on both carriers. The class
//! at small offsets is the closed form's, in the near-axis search
//! below: a ball centred a hair off a tilted axis, at scales to 1 km,
//! against the extreme generators' distances.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

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
                let (x, y) = (
                    find(&mut parent, cell(i, j)),
                    find(&mut parent, cell(i + 1, j)),
                );
                parent[x] = y;
            }
            // The upper neighbour shares row j + 1.
            if j + 1 < nv && s(i, j + 1) != s(i + 1, j + 1) {
                let (x, y) = (
                    find(&mut parent, cell(i, j)),
                    find(&mut parent, cell(i, j + 1)),
                );
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
                if let Some(c) = self.comp[jj as usize * self.nu + ii]
                    && !out.contains(&c)
                {
                    out.push(c);
                }
            }
        }
        out
    }
}

/// A cone's frame, and its chart on the double carrier: `(u, t)` is
/// the point `t` along the generator at azimuth `u`.
struct Frame {
    /// The pose's length scale.
    scale: f64,
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
        let r = across(d, self.a) / (t * s);
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
    let scale = rng.range(0.0, 1e3f64.ln()).exp();
    Frame {
        scale,
        apex: Point3::new(
            rng.range(-scale, scale),
            rng.range(-scale, scale),
            rng.range(-scale, scale),
        ),
        a,
        e1,
        e2,
        alpha: rng.range(0.25, 1.2),
    }
}

/// An offset from a margin's zero, relative, either sign: half the
/// draws log-uniform in `[0.03, 0.5]`, which the trace resolves, half
/// in `[1e-12, 0.5]`, into the band and through it.
fn offset(rng: &mut Rng) -> f64 {
    let floor = if rng.unit() < 0.5 { 0.03f64 } else { 1e-12 };
    let m = (rng.range(floor.ln(), 0.5f64.ln())).exp();
    if rng.unit() < 0.5 { m } else { -m }
}

/// `v`'s part square to the unit `a`, by the cross product (the oracle's
/// own spelling, independent of the arm's).
fn across(v: Vec3<f64>, a: Vec3<f64>) -> Vec3<f64> {
    a.cross(v.cross(a))
}

/// The reach every search pose is classified with.
fn reach_of(k: &Frame) -> Reach<f64> {
    Reach {
        centre: k.apex,
        radius: 5.0 * k.scale,
    }
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
    /// The value, in metres as the arm decides it, of the margin the
    /// pose was built near; `None` for a pose built near none.
    margin: Option<f64>,
}

fn plane_pose(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let beta = (PI / 2.0 - k.alpha + offset(rng)).clamp(1e-3, PI / 2.0);
    let phi = rng.range(0.0, TAU);
    let mut n = k.a * beta.cos() + k.radial(phi) * beta.sin();
    if rng.unit() < 0.5 {
        n = -n;
    }
    let m = k.scale * rng.range(0.1, 1.5) * if rng.unit() < 0.5 { 1.0 } else { -1.0 };
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
    let lever = (p0 - k.apex).dot(n).abs() + 2.0 * reach_of(k).radius;
    let margin = (n.dot(k.a).abs() - s) * lever;
    // Near the parabola the section runs far past any trace window; the
    // check below the trace's resolution reads no window.
    (window < 30.0 * k.scale || margin.abs() < 0.03 * k.scale).then(|| Pose {
        what: format!("plane p0 {p0:?} n {n:?} (β {beta})"),
        partner: Surface::Plane {
            origin: p0,
            normal: n,
            u_ref: n.orthonormal_basis().0,
        },
        window,
        count: Count::Exact,
        margin: Some(margin),
    })
}

fn sphere_pose(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let centre = k.apex
        + k.a * (sc * rng.range(-2.0, 2.0))
        + k.radial(rng.range(0.0, TAU)) * (sc * rng.range(0.0, 2.0));
    let delta = k.apex - centre;
    let off = across(delta, k.a);
    let r = if off.norm() > 0.0 {
        off / off.norm()
    } else {
        k.e1
    };
    let (s, c) = k.alpha.sin_cos();
    let line = |w: Vec3<f64>| delta.cross(w).norm();
    let near = [delta.norm(), line(k.a * c + r * s), line(k.a * c - r * s)][rng.below(3)];
    let shift = sc * offset(rng);
    let rho = near + shift;
    (rho > 0.05 * sc).then(|| Pose {
        what: format!("sphere {centre:?} ρ {rho}"),
        partner: Surface::Sphere {
            center: centre,
            radius: rho,
            axis: k.a,
            u_ref: k.e1,
        },
        window: delta.norm() + rho + 0.1 * sc,
        count: Count::Exact,
        margin: Some(shift),
    })
}

fn coaxial_cylinder_pose(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let rc = k.scale * rng.range(0.1, 1.5);
    Some(Pose {
        what: format!("coaxial cylinder r {rc}"),
        partner: Surface::Cylinder {
            origin: k.apex + k.a * (k.scale * rng.range(-2.0, 2.0)),
            axis: if rng.unit() < 0.5 { k.a } else { -k.a },
            radius: rc,
            u_ref: k.e1,
        },
        window: 1.4 * rc / k.alpha.sin(),
        count: Count::Parallels(&[2]),
        margin: None,
    })
}

fn coaxial_cone_pose(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let near_apex = rng.unit() < 0.3;
    let d = k.scale
        * if near_apex {
            offset(rng)
        } else {
            rng.range(0.2, 1.5) * if rng.unit() < 0.5 { 1.0 } else { -1.0 }
        };
    let alpha2 = rng.range(0.25, 1.2);
    let (t1, t2) = (k.alpha.tan(), alpha2.tan());
    if (t1 - t2).abs() < 0.15 {
        return None;
    }
    let far = [d * t2 / (t1 + t2), d * t2 / (t2 - t1)]
        .iter()
        .fold(0.0f64, |w, h| w.max(h.abs() / k.alpha.cos()));
    (far < 20.0 * k.scale).then(|| Pose {
        what: format!("coaxial cone d {d} α₂ {alpha2}"),
        partner: Surface::Cone {
            apex: k.apex + k.a * d,
            axis: if rng.unit() < 0.5 { k.a } else { -k.a },
            half_angle: alpha2,
            u_ref: k.e1,
        },
        window: 1.3 * far,
        count: Count::Parallels(&[2]),
        margin: near_apex.then_some(d),
    })
}

fn coaxial_torus_pose(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let (s, c) = k.alpha.sin_cos();
    let sc = k.scale;
    let z = sc * rng.range(-2.0, 2.0);
    let r = sc * rng.range(0.1, 0.5);
    let (big, margin) = if rng.unit() < 0.5 {
        (rng.range(r + 0.1 * sc, 2.5 * sc), None)
    } else {
        let side = if rng.unit() < 0.5 { 1.0 } else { -1.0 };
        let shift = sc * offset(rng);
        ((side * z * s + r + shift) / c, Some(shift))
    };
    (big > r + 0.05 * sc).then(|| Pose {
        what: format!("coaxial torus z {z} R {big} r {r}"),
        partner: Surface::Torus {
            center: k.apex + k.a * z,
            axis: k.a,
            major_radius: big,
            minor_radius: r,
            u_ref: k.e1,
        },
        window: z.abs() + big + r + 0.2 * sc,
        count: Count::Parallels(&[0, 2, 4]),
        margin,
    })
}

fn parallel_cylinder_pose(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let e = sc * rng.range(0.2, 1.5);
    let shift = sc * offset(rng);
    let rc = e + shift;
    (rc > 0.05 * sc).then(|| Pose {
        what: format!("parallel cylinder e {e} r {rc}"),
        partner: Surface::Cylinder {
            origin: k.apex + k.e1 * e + k.a * (sc * rng.range(-2.0, 2.0)),
            axis: k.a,
            radius: rc,
            u_ref: k.e1,
        },
        window: 1.3 * (e + rc) / k.alpha.sin(),
        count: Count::Exact,
        margin: Some(shift),
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
    let reach = reach_of(k);
    let b = band();
    let sec = classify(&cone, &pose.partner, reach, b);
    let back = classify(&pose.partner, &cone, reach, b);
    let m = pose.margin.map_or(f64::INFINITY, f64::abs);
    // In the band's Zero: R-tan is the answer, except at the plane's
    // aperture, where Zero is the parabola's class — one part, essential,
    // no witness, uncounted.
    if m <= 0.25 * b.zero() {
        let parabola = matches!(pose.partner, Surface::Plane { .. })
            && matches!(&sec, Section::Components { parts, single: false }
                if parts.len() == 1
                    && parts[0].essential_f
                    && !parts[0].unbounded
                    && parts[0].witness.is_none());
        return match &sec {
            Section::Tangent(_) if !matches!(pose.partner, Surface::Plane { .. }) => Vec::new(),
            _ if parabola => Vec::new(),
            other => vec![format!("margin {m} in the band, answered {other:?}")],
        };
    }
    let (parts, single) = match &sec {
        Section::Components { parts, single } => (parts.clone(), *single),
        Section::Tangent(_) if m < 4.0 * b.escalate() => return Vec::new(),
        other => return vec![format!("margin {m}: not classified: {other:?}")],
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
    for c in &parts {
        let Some(w) = c.witness else {
            continue;
        };
        for surf in [&cone, &pose.partner] {
            let r = geom_brep::implicit_residual(surf, w);
            if r.abs() > 1e-9 * k.scale {
                bad.push(format!("the witness {w:?} is {r} off {surf:?}"));
            }
        }
    }
    // Below the trace's resolution the class is the near-axis search's.
    if m < 0.03 * k.scale {
        return bad;
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
        && !matches!(pose.count, Count::Parallels(_))
    {
        let (e1, e2) = axis.orthonormal_basis();
        let z0 = (k.apex - origin).dot(axis);
        let on_cyl = trace(
            &|u, z| origin + (e1 * u.cos() + e2 * u.sin()) * radius + axis * z,
            &|q| geom_brep::implicit_residual(&cone, q),
            (z0 - pose.window, z0 + pose.window),
            400,
            401,
        );
        if on_cyl.classes.len() != parts.len()
            || on_cyl.classes.iter().any(|c| *c != Class::Essential)
            || parts.iter().any(|c| !c.essential_g)
        {
            bad.push(format!(
                "on the cylinder traced {:?}, classified {parts:?}",
                on_cyl.classes
            ));
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
        while done < fuzz::scaled(16) {
            let k = random_cone(&mut rng);
            let Some(pose) = poser(&mut rng, &k) else {
                continue;
            };
            done += 1;
            let bad = check(&k, &pose);
            match classify(&k.surface(), &pose.partner, reach_of(&k), band()) {
                Section::Components { parts, .. } => {
                    let mut cs: Vec<Class> = parts.iter().map(class_of).collect();
                    cs.sort();
                    *tally.entry(format!("{cs:?}")).or_default() += 1;
                }
                other => *tally.entry(format!("{other:?}")).or_default() += 1,
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

/// **Near-axis balls against the closed form** (shape 1). A cone on a
/// random axis at scale `L ∈ [1, 1e3]` m, a ball centred `h = L` up the
/// axis and `e = f·L` off it, `f ∈ [1e-10, 1e-2]` log-uniform: the
/// extreme generators stand `|e·cos α ∓ h·sin α|` from the centre, and a
/// radius between them, beyond them or short of them is one null loop,
/// two essential curves or nothing on the opening nappe. Every margin
/// is half the generators' spread, `e·cos α`; where that is inside a
/// few escalation widths (plus the rounding of an `L`-sized frame), R-tan
/// is a correct answer, and anything else must be the closed form's
/// class with every witness within `1e-11·L` of both carriers.
#[test]
fn near_axis_balls_agree_with_the_closed_form() {
    let mut rng = fuzz::start("section_cert_cone_near_axis");
    let b = band();
    let mut failures = Vec::new();
    let (mut answered, mut tangent) = (0usize, 0usize);
    for _ in 0..fuzz::scaled(2000) {
        let scale = rng.range(0.0, 1e3f64.ln()).exp();
        let frac = rng.range(1e-10f64.ln(), 1e-2f64.ln()).exp();
        let a = unit_vec(&mut rng);
        let (b1, b2) = a.orthonormal_basis();
        let phi = rng.range(0.0, TAU);
        let u = b1 * phi.cos() + b2 * phi.sin();
        let alpha = rng.range(0.25, 1.2);
        let apex = Point3::new(
            rng.range(-scale, scale),
            rng.range(-scale, scale),
            rng.range(-scale, scale),
        );
        let (h, e) = (scale, frac * scale);
        let centre = apex + a * h + u * e;
        let (s, c) = alpha.sin_cos();
        let (near, far) = ((h * s - e * c).abs(), (h * s + e * c).abs());
        let gap = 0.5 * (far - near);
        let (rho, want): (f64, &[Class]) = match rng.below(3) {
            0 => (near + gap, &[Class::Null]),
            1 => (far + gap, &[Class::Essential, Class::Essential]),
            _ => (near - gap, &[]),
        };
        let cone = Surface::Cone {
            apex,
            axis: a,
            half_angle: alpha,
            u_ref: b1,
        };
        let ball = Surface::Sphere {
            center: centre,
            radius: rho,
            axis: a,
            u_ref: b1,
        };
        let what = format!("L {scale}, e/L {frac}, α {alpha}, ρ {rho}, want {want:?}");
        let reach = Reach {
            centre: apex,
            radius: 5.0 * scale,
        };
        match classify(&cone, &ball, reach, b) {
            Section::Tangent(_) => {
                tangent += 1;
                if gap > 8.0 * b.escalate() + 64.0 * f64::EPSILON * scale {
                    failures.push(format!("{what}: R-tan at a margin of {gap}"));
                }
            }
            Section::Components { parts, single } => {
                answered += 1;
                let mut got: Vec<Class> = parts.iter().map(class_of).collect();
                got.sort();
                if got != want || single != (want.len() == 1) {
                    failures.push(format!("{what}: classified {got:?}, single {single}"));
                }
                for p in &parts {
                    let w = p.witness.expect("every sphere part carries a witness");
                    for surf in [&cone, &ball] {
                        let r = geom_brep::implicit_residual(surf, w);
                        if r.abs() > 1e-11 * scale {
                            failures.push(format!("{what}: the witness {w:?} is {r} off"));
                        }
                    }
                }
            }
            other => failures.push(format!("{what}: {other:?}")),
        }
    }
    println!("[cone near-axis] {answered} answered, {tangent} R-tan");
    assert!(
        failures.is_empty(),
        "{} mismatches ({}):\n{}",
        failures.len(),
        fuzz::replay(),
        failures.join("\n")
    );
}
