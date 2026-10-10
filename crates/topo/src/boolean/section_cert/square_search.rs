//! **The square-wall arm against a traced section** (a counterexample
//! search, shape 1): a torus at scales from `1e-2` to `1e2` m with
//! `r/R ∈ [0.05, 0.95]`, against a cylinder whose axis stands square to
//! the torus axis (or within the band of square), radius from `0.05 r`
//! to past the tube and past the whole torus, its trace in the meridian
//! plane built at a signed offset from one classification margin's zero
//! (the offset log-uniform from `1e-12` of the scale up to half of it),
//! or anywhere.
//!
//! The oracle traces the zero set of the TORUS's residual on the
//! cylinder's own chart — azimuth `θ` periodic, `t` along the stored
//! axis over a window holding the whole torus — and cuts it into
//! components by cell adjacency. It never reads the meridian plane the
//! arm works in. Each component is classed twice from the cells it
//! crosses: essential on the wall iff it crosses the seam `θ = 0` an odd
//! number of times, and essential on the torus iff the torus azimuth or
//! tube angle of the cells it runs through wraps an odd number of times
//! (a simple closed curve on a torus is null or of class `(p, q)` with
//! `gcd(p, q) = 1`, so one of the two is odd). `classify` must agree with
//! 0 mismatches where the trace resolves the pose: part for part in count
//! and both classes, `single` only on one traced component, every
//! witness on both carriers and on a traced component of its part's
//! classes, distinct components for distinct parts, and the swapped order
//! the same with its flags exchanged.
//!
//! Where it does not — an offset under `0.03` of the smaller of the two
//! radii, or two trace resolutions disagreeing — the classes are not
//! read: an offset inside a quarter of the band's Zero must refuse R-tan,
//! and one between must answer with every witness on both carriers, or
//! refuse R-tan within a few escalation widths.
//!
//! `dump_for_the_mpmath_oracle` writes the fixed rows' poses and a
//! sample of this search's, with the arm's answers, for
//! `scripts/oracles/torus_square_wall_mpmath.py`, which re-solves each at
//! 40 digits by an independent route (each ruling's quartic).

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
use core::f64::consts::TAU;
use geom::Surface;
use geom_core::{Band, Point3, Tol, Vec3};
use test_utils::fuzz::{self, Rng};

pub(super) fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A torus's frame.
#[derive(Clone, Copy, Debug)]
pub(super) struct Ring {
    pub c: Point3<f64>,
    pub a: Vec3<f64>,
    pub b1: Vec3<f64>,
    pub big_r: f64,
    pub r: f64,
}

impl Ring {
    pub(super) fn surface(&self) -> Surface<f64> {
        Surface::Torus {
            center: self.c,
            axis: self.a,
            major_radius: self.big_r,
            minor_radius: self.r,
            u_ref: self.b1,
        }
    }

    /// The azimuth and the tube angle of a point off the core circle.
    fn angles(&self, q: Point3<f64>) -> (f64, f64) {
        let w = q - self.c;
        let b2 = self.a.cross(self.b1);
        let z = w.dot(self.a);
        let rho = self.a.cross(w.cross(self.a)).norm();
        (w.dot(b2).atan2(w.dot(self.b1)), z.atan2(rho - self.big_r))
    }
}

/// A cylinder's chart: `θ` about the axis from `e1`, `t` along it from
/// `o`.
#[derive(Clone, Copy, Debug)]
pub(super) struct Wall {
    pub o: Point3<f64>,
    pub d: Vec3<f64>,
    pub e1: Vec3<f64>,
    pub rc: f64,
}

impl Wall {
    fn e2(&self) -> Vec3<f64> {
        self.d.cross(self.e1)
    }

    fn at(&self, th: f64, t: f64) -> Point3<f64> {
        self.o + (self.e1 * th.cos() + self.e2() * th.sin()) * self.rc + self.d * t
    }

    fn chart(&self, q: Point3<f64>) -> (f64, f64) {
        let w = q - self.o;
        (w.dot(self.e2()).atan2(w.dot(self.e1)), w.dot(self.d))
    }

    pub(super) fn surface(&self) -> Surface<f64> {
        Surface::Cylinder {
            origin: self.o,
            axis: self.d,
            radius: self.rc,
            u_ref: self.e1,
        }
    }
}

/// A component's classes: essential on the torus, essential on the wall.
pub(super) type Class = (bool, bool);

pub(super) fn class_of(c: &Component<f64>) -> Class {
    (c.essential_f, c.essential_g)
}

/// The torus's zero set on the wall's chart, cut into cells.
pub(super) struct Traced {
    nth: usize,
    nt: usize,
    t0: f64,
    t1: f64,
    comp: Vec<Option<usize>>,
    pub classes: Vec<Class>,
}

fn find(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

/// Traces the torus's residual over `θ ∈ [0, 2π)` (periodic) and
/// `t ∈ [t0, t1]`; `None` when a component reaches the window's edge.
pub(super) fn trace(
    ring: &Ring,
    wall: &Wall,
    (t0, t1): (f64, f64),
    nth: usize,
    nt: usize,
) -> Option<Traced> {
    let torus = ring.surface();
    // The torus lies in the slab `|height| ≤ r`, and a ruling's height
    // drifts by `|d·a|` per metre along it: a ruling clear of the slab
    // over the window is outside the torus, and is not evaluated.
    let drift = wall.d.dot(ring.a).abs() * t0.abs().max(t1.abs());
    let clear: Vec<bool> = (0..nth)
        .map(|i| {
            let h = (wall.at(TAU * i as f64 / nth as f64, 0.0) - ring.c).dot(ring.a);
            h.abs() > 1.05 * ring.r + drift
        })
        .collect();
    let node = |i: usize, j: usize| {
        let (th, t) = (
            TAU * i as f64 / nth as f64,
            t0 + (t1 - t0) * j as f64 / nt as f64,
        );
        clear[i] || geom_brep::implicit_residual(&torus, wall.at(th, t)) >= 0.0
    };
    let pos: Vec<bool> = (0..=nt)
        .flat_map(|j| (0..nth).map(move |i| (i, j)))
        .map(|(i, j)| node(i, j))
        .collect();
    let s = |i: usize, j: usize| pos[j * nth + i % nth];
    let cell = |i: usize, j: usize| j * nth + i % nth;
    let n = nth * nt;
    let mut parent: Vec<usize> = (0..n).collect();
    let mut active = vec![false; n];
    // Each shared cell edge the zero set crosses: the two cells, and
    // whether the edge is the seam.
    let mut edges = Vec::new();
    for j in 0..nt {
        for i in 0..nth {
            let (a, b, c, d) = (s(i, j), s(i + 1, j), s(i, j + 1), s(i + 1, j + 1));
            active[cell(i, j)] = !(a == b && b == c && c == d);
            if b != d {
                edges.push((cell(i, j), cell(i + 1, j), i + 1 == nth));
            }
            if j + 1 < nt && c != d {
                edges.push((cell(i, j), cell(i, j + 1), false));
            }
            if (j == 0 && a != b) || (j + 1 == nt && c != d) {
                return None;
            }
        }
    }
    for &(x, y, _) in &edges {
        let (x, y) = (find(&mut parent, x), find(&mut parent, y));
        parent[x] = y;
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
    let centre = |k: usize| {
        let (i, j) = (k % nth, k / nth);
        ring.angles(wall.at(
            TAU * (i as f64 + 0.5) / nth as f64,
            t0 + (t1 - t0) * (j as f64 + 0.5) / nt as f64,
        ))
    };
    let mut odd = vec![[false; 3]; index.len()];
    for &(x, y, seam) in &edges {
        let Some(k) = comp[x] else { continue };
        let ((ux, vx), (uy, vy)) = (centre(x), centre(y));
        let flips = [
            (ux - uy).abs() > core::f64::consts::PI,
            (vx - vy).abs() > core::f64::consts::PI,
            seam,
        ];
        for (o, f) in odd[k].iter_mut().zip(flips) {
            *o ^= f;
        }
    }
    let classes = odd.iter().map(|&[u, v, seam]| (u || v, seam)).collect();
    Some(Traced {
        nth,
        nt,
        t0,
        t1,
        comp,
        classes,
    })
}

impl Traced {
    /// The components crossing the cells about chart point `(θ, t)`.
    fn near(&self, th: f64, t: f64) -> Vec<usize> {
        let i = (th.rem_euclid(TAU) / TAU * self.nth as f64) as isize;
        let j = ((t - self.t0) / (self.t1 - self.t0) * self.nt as f64) as isize;
        let mut out = Vec::new();
        for dj in -2..=2 {
            for di in -2..=2 {
                let jj = j + dj;
                if jj < 0 || jj >= self.nt as isize {
                    continue;
                }
                let ii = (i + di).rem_euclid(self.nth as isize) as usize;
                if let Some(c) = self.comp[jj as usize * self.nth + ii]
                    && !out.contains(&c)
                {
                    out.push(c);
                }
            }
        }
        out
    }

    fn sorted(&self) -> Vec<Class> {
        let mut v = self.classes.clone();
        v.sort_unstable();
        v
    }
}

/// The window along the wall's axis holding every point of the torus:
/// `t` of the torus centre, `±1.05 (R + r)`.
pub(super) fn window(ring: &Ring, wall: &Wall) -> (f64, f64) {
    let mid = (ring.c - wall.o).dot(wall.d);
    let w = 1.05 * (ring.big_r + ring.r);
    (mid - w, mid + w)
}

/// Cells about the wall: `K`'s features (the strip, the tube circles)
/// stand `min(r, rc)` apart, so at least forty cells to that length
/// around the wall.
pub(super) fn resolution(ring: &Ring, wall: &Wall) -> usize {
    let n = 40.0 * TAU * wall.rc / ring.r.min(wall.rc);
    (n.ceil() as usize).clamp(480, 8000)
}

/// How far the pose stands from the nearest configuration where the
/// trace `K` of the wall in the meridian plane changes how it meets the
/// torus's: `K` tangent to a strip line `z = ±r`, tangent to a tube
/// circle, or through a tube circle's top or bottom point. Read in the
/// oracle's own frame from the stored carriers.
pub(super) fn degeneracy(ring: &Ring, wall: &Wall) -> f64 {
    let d = ring.a.cross(wall.d.cross(ring.a)).normalize();
    let across = ring.a.cross(d);
    let w = wall.o - ring.c;
    let (e, z0, rc, big_r, r) = (w.dot(across), w.dot(ring.a), wall.rc, ring.big_r, ring.r);
    let mut m = f64::INFINITY;
    for edge in [r, -r] {
        m = m.min((z0 + rc - edge).abs()).min((z0 - rc - edge).abs());
    }
    for sigma in [1.0, -1.0] {
        let dist = (e - sigma * big_r).hypot(z0);
        m = m
            .min((dist - (r + rc)).abs())
            .min((dist - (r - rc).abs()).abs());
        for edge in [r, -r] {
            m = m.min(((e - sigma * big_r).hypot(z0 - edge) - rc).abs());
        }
    }
    m
}

/// The mismatches of `sec` against the traced section, `resolved` saying
/// whether the classes are read; the carrier checks hold either way.
pub(super) fn mismatches(
    ring: &Ring,
    wall: &Wall,
    sec: &Section<f64>,
    resolved: Option<&Traced>,
) -> Vec<String> {
    let mut bad = Vec::new();
    let Section::Components { parts, single } = sec else {
        return vec![format!("not classified: {sec:?}")];
    };
    let scale = ring.big_r + ring.r;
    for c in parts {
        let Some(w) = c.witness else {
            bad.push(format!("a part with no witness: {c:?}"));
            continue;
        };
        for surf in [&ring.surface(), &wall.surface()] {
            let r = geom_brep::implicit_residual(surf, w);
            if r.abs() > 1e-10 * scale {
                bad.push(format!("the witness {w:?} is {r} off {surf:?}"));
            }
        }
    }
    let Some(traced) = resolved else {
        return bad;
    };
    let mut got: Vec<Class> = parts.iter().map(class_of).collect();
    got.sort_unstable();
    if got != traced.sorted() {
        bad.push(format!("traced {:?}, classified {got:?}", traced.sorted()));
    }
    if *single != (traced.classes.len() == 1) {
        bad.push(format!("single {single}, traced {:?}", traced.classes));
    }
    let mut seen = Vec::new();
    for c in parts {
        let Some(w) = c.witness else { continue };
        let (th, t) = wall.chart(w);
        let on: Vec<usize> = traced
            .near(th, t)
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
    bad
}

/// The swapped order agrees, flags exchanged.
pub(super) fn swapped_agrees(sec: &Section<f64>, back: &Section<f64>) -> bool {
    match (sec, back) {
        (
            Section::Components { parts, single },
            Section::Components {
                parts: b,
                single: sb,
            },
        ) => {
            single == sb
                && parts.len() == b.len()
                && parts.iter().zip(b).all(|(x, y)| {
                    (x.unbounded, x.essential_f, x.essential_g)
                        == (y.unbounded, y.essential_g, y.essential_f)
                        && x.witness.map(|p| [p.x, p.y, p.z]) == y.witness.map(|p| [p.x, p.y, p.z])
                })
        }
        (Section::Tangent(x), Section::Tangent(y)) => x == y,
        (Section::Intractable, Section::Intractable) => true,
        _ => false,
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

/// An offset from a margin's zero, relative, either sign: half the
/// draws log-uniform in `[0.03, 0.5]`, half in `[1e-12, 0.5]`, into the
/// band and through it.
fn offset(rng: &mut Rng) -> f64 {
    let floor = if rng.unit() < 0.5 { 0.03f64 } else { 1e-12 };
    let m = (rng.range(floor.ln(), 0.5f64.ln())).exp();
    if rng.unit() < 0.5 { m } else { -m }
}

/// One search pose: the torus, the wall, the trace `(e, z0, rc)` of its
/// axis in the meridian plane, and the margin it was built near (in
/// metres, as the arm decides it), `None` for none.
pub(super) struct Pose {
    pub what: String,
    pub ring: Ring,
    pub wall: Wall,
    pub margin: Option<f64>,
}

/// The margin families a pose is built near.
const FAMILIES: [&str; 6] = [
    "anywhere",
    "a strip edge",
    "a tube circle, outside",
    "a tube circle, nested",
    "a strip crossing at a tube circle's top",
    "two tube circles at once",
];

/// A random pose of family `fam`.
pub(super) fn pose(rng: &mut Rng, fam: usize) -> Pose {
    let big_r = rng.range(1e-2f64.ln(), 1e2f64.ln()).exp();
    let r = big_r * rng.range(0.05, 0.95);
    // Up to and past the tube, and now and then past the whole torus.
    let rc = if rng.unit() < 0.8 {
        r * rng.range(0.05f64.ln(), 3.0f64.ln()).exp()
    } else {
        (big_r + r) * rng.range(0.5, 2.5)
    };
    let sigma = if rng.unit() < 0.5 { 1.0 } else { -1.0 };
    let phi = rng.range(0.0, TAU);
    let dlen = offset(rng) * r.min(rc);
    let (e, z0, margin) = match fam {
        1 => {
            let flip = if rng.unit() < 0.5 { 1.0 } else { -1.0 };
            let e = rng.range(-1.3, 1.3) * (big_r + r);
            (e, flip * (r - rc + dlen), Some(dlen))
        }
        2 => {
            let k = r + rc - dlen;
            (sigma * big_r + k * phi.cos(), k * phi.sin(), Some(dlen))
        }
        3 => {
            let k = (r - rc).abs() + dlen.abs();
            (
                sigma * big_r + k * phi.cos(),
                k * phi.sin(),
                Some(dlen.abs()),
            )
        }
        4 => {
            // A crossing of `z = ±r` at `|y| = R − δ`, the circle through
            // it with its centre off the vertical there (where it would
            // touch the edge).
            let flip = if rng.unit() < 0.5 { 1.0 } else { -1.0 };
            let chi = rng.range(-1.37, 1.37) + if rng.unit() < 0.5 { 0.0 } else { TAU / 2.0 };
            let (yy, zz) = (sigma * (big_r - dlen), flip * r);
            (yy + rc * chi.cos(), zz + rc * chi.sin(), Some(dlen))
        }
        5 => {
            // About the torus axis's own trace, touching both tube
            // circles at once: their near points at `rc = R − r`, their
            // far points at `rc = R + r`.
            let (e, z0) = (0.0, 0.0);
            // The near points only where that radius is one (`R − r`
            // can be shorter than the offset).
            let near = big_r - r + dlen;
            let rc_both = if rng.unit() < 0.5 && near > 0.0 {
                near
            } else {
                big_r + r - dlen
            };
            return square_pose_at(rng, fam, (big_r, r), (e, z0, rc_both), Some(dlen));
        }
        _ => (
            rng.range(-1.3, 1.3) * (big_r + r),
            if rc > 2.0 * r && rng.unit() < 0.3 {
                sigma * (rc - r * rng.range(-1.5, 1.5))
            } else {
                rng.range(-1.5, 1.5) * r
            },
            None,
        ),
    };
    square_pose_at(rng, fam, (big_r, r), (e, z0, rc), margin)
}

/// A pose of family `fam` with the trace `(e, z0, rc)`, on a random
/// torus frame: the wall's axis square to it, turned by an in-band tilt
/// now and then, and stored from anywhere on it.
fn square_pose_at(
    rng: &mut Rng,
    fam: usize,
    (big_r, r): (f64, f64),
    (e, z0, rc): (f64, f64, f64),
    margin: Option<f64>,
) -> Pose {
    let a = unit_vec(rng);
    let (b1, b2) = a.orthonormal_basis();
    let c = Point3::new(
        rng.range(-big_r, big_r),
        rng.range(-big_r, big_r),
        rng.range(-big_r, big_r),
    );
    let psi = rng.range(0.0, TAU);
    let d0 = b1 * psi.cos() + b2 * psi.sin();
    let across = a.cross(d0);
    let tilt = if rng.unit() < 0.3 {
        rng.range(1e-18f64.ln(), 1e-13f64.ln()).exp()
    } else {
        0.0
    };
    let d = (d0 + a * tilt).normalize();
    let foot = c + across * e + a * z0;
    let o = foot + d * rng.range(-3.0, 3.0) * big_r;
    let e1 = d.orthonormal_basis().0;
    Pose {
        what: format!(
            "{}: R {big_r}, r {r}, (e, z0, rc) ({e}, {z0}, {rc}), tilt {tilt}",
            FAMILIES[fam]
        ),
        ring: Ring { c, a, b1, big_r, r },
        wall: Wall { o, d, e1, rc },
        margin,
    }
}

/// The reach every search pose is classified with: a ball about the
/// torus holding all of it.
pub(super) fn reach_of(ring: &Ring) -> Reach<f64> {
    Reach {
        centre: ring.c,
        radius: 1.5 * (ring.big_r + ring.r),
    }
}

/// The mismatches of one pose, and how it was read.
fn check(p: &Pose) -> (Vec<String>, &'static str) {
    let b = band();
    let (torus, wall) = (p.ring.surface(), p.wall.surface());
    let reach = reach_of(&p.ring);
    let sec = classify(&torus, &wall, reach, b);
    let back = classify(&wall, &torus, reach, b);
    let mut bad = Vec::new();
    if !swapped_agrees(&sec, &back) {
        bad.push(format!(
            "the swapped order disagrees: {sec:?} against {back:?}"
        ));
    }
    let m = p.margin.map_or(f64::INFINITY, f64::abs);
    if m <= 0.25 * b.zero() {
        if !matches!(sec, Section::Tangent(_)) {
            bad.push(format!("margin {m} in the band, answered {sec:?}"));
        }
        return (bad, "in the band");
    }
    if let Section::Tangent(_) = sec {
        if m > 16.0 * b.escalate() {
            bad.push(format!("R-tan at a margin of {m}: {sec:?}"));
        }
        return (bad, "R-tan");
    }
    let win = window(&p.ring, &p.wall);
    let nth = resolution(&p.ring, &p.wall);
    let fine = trace(&p.ring, &p.wall, win, nth, 801);
    let coarse = trace(&p.ring, &p.wall, win, nth / 2, 401);
    let clear = degeneracy(&p.ring, &p.wall) >= 0.03 * p.ring.r.min(p.wall.rc);
    let resolved = match (&fine, &coarse) {
        (Some(f), Some(c)) if clear && f.sorted() == c.sorted() => Some(f),
        (None, _) => {
            bad.push("the trace's window misses a component".to_string());
            None
        }
        _ => None,
    };
    bad.extend(mismatches(&p.ring, &p.wall, &sec, resolved));
    (
        bad,
        if resolved.is_some() {
            "traced"
        } else {
            "carriers only"
        },
    )
}

/// **The square-wall arm against the traced section**, 0 mismatches.
#[test]
fn the_square_wall_arm_agrees_with_the_traced_section() {
    let mut rng = fuzz::start("section_cert_square_search");
    let mut failures = Vec::new();
    for (fam, name) in FAMILIES.iter().enumerate() {
        let mut tally = std::collections::BTreeMap::<String, usize>::new();
        for _ in 0..fuzz::scaled(40) {
            let p = pose(&mut rng, fam);
            let (bad, read) = check(&p);
            let sec = classify(
                &p.ring.surface(),
                &p.wall.surface(),
                reach_of(&p.ring),
                band(),
            );
            let key = match &sec {
                Section::Components { parts, .. } => {
                    let mut cs: Vec<Class> = parts.iter().map(class_of).collect();
                    cs.sort_unstable();
                    format!("{read} {cs:?}")
                }
                other => format!("{read} {other:?}"),
            };
            *tally.entry(key).or_default() += 1;
            for b in bad {
                failures.push(format!("{}: {b}", p.what));
            }
        }
        println!("[square search] {name}: {tally:?}");
    }
    assert!(
        failures.is_empty(),
        "{} mismatches ({}):\n{}",
        failures.len(),
        fuzz::replay(),
        failures.join("\n")
    );
}

fn json_vec(v: [f64; 3]) -> String {
    format!("[{:e}, {:e}, {:e}]", v[0], v[1], v[2])
}

/// One pose and the arm's answer, as `scripts/oracles/torus_square_wall_mpmath.py`
/// reads it.
pub(super) fn dump_line(what: &str, ring: &Ring, wall: &Wall) -> String {
    let sec = classify(&ring.surface(), &wall.surface(), reach_of(ring), band());
    let xyz = |p: Vec3<f64>| json_vec([p.x, p.y, p.z]);
    let answer = match &sec {
        Section::Components { parts, single } => {
            let parts: Vec<String> = parts
                .iter()
                .map(|c| {
                    let w = c.witness.expect("every square-wall part carries a witness");
                    format!(
                        "{{\"torus\": {}, \"wall\": {}, \"witness\": {}}}",
                        c.essential_f,
                        c.essential_g,
                        json_vec([w.x, w.y, w.z])
                    )
                })
                .collect();
            format!(
                "{{\"kind\": \"components\", \"single\": {single}, \"parts\": [{}]}}",
                parts.join(", ")
            )
        }
        other => format!(
            "{{\"kind\": \"{}\"}}",
            format!("{other:?}").replace('"', "'")
        ),
    };
    format!(
        "{{\"what\": \"{what}\", \"torus\": {{\"c\": {}, \"a\": {}, \"b1\": {}, \"R\": {:e}, \"r\": {:e}}}, \
         \"wall\": {{\"o\": {}, \"d\": {}, \"e1\": {}, \"rc\": {:e}}}, \"answer\": {answer}}}",
        xyz(ring.c - Point3::origin()),
        xyz(ring.a),
        xyz(ring.b1),
        ring.big_r,
        ring.r,
        xyz(wall.o - Point3::origin()),
        xyz(wall.d),
        xyz(wall.e1),
        wall.rc,
    )
}

/// **Not a gate: writes poses for the mpmath oracle.** Every search
/// family's poses, as `SQDUMP` lines. Run with `--run-ignored only
/// --no-capture`, keep the `SQDUMP` lines, and pass the file to
/// `python3 scripts/oracles/torus_square_wall_mpmath.py`.
#[test]
#[ignore = "writes the mpmath oracle's input; run by hand"]
fn dump_for_the_mpmath_oracle() {
    let mut rng = fuzz::start("section_cert_square_dump");
    for (fam, name) in FAMILIES.iter().enumerate() {
        for i in 0..fuzz::scaled(25) {
            let p = pose(&mut rng, fam);
            println!(
                "SQDUMP {}",
                dump_line(&format!("{name} {i}"), &p.ring, &p.wall)
            );
        }
    }
}
