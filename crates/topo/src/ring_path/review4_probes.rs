//! Review-4 probes (scratch, not for merge). Each arm with an in-span
//! reading, attacked at a vertex under carrier jitter over ε × scale ×
//! offset × jitter, on cones from near-cylinder to near-flat and on
//! sections from moderate to near-parabolic. Prints wrong/decided per
//! cell; `R4_MUT` toggles the mutants in `ring_path.rs`.

use super::*;
use geom_core::{Band, Point3, Tol, Vec3};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(
                self.range(-1., 1.),
                self.range(-1., 1.),
                self.range(-1., 1.),
            );
            let n = v.norm();
            if n > 0.2 && n < 1.0 {
                return v / n;
            }
        }
    }
    fn frame(&mut self) -> [Vec3<f64>; 3] {
        let z = self.unit();
        let x = self.unit().cross(z).normalize();
        [x, z.cross(x), z]
    }
}

fn nudges() -> Vec<f64> {
    let mut out = vec![0.0];
    for k in -16..=-1 {
        for m in [1.0, 3.0] {
            out.extend([m * 10f64.powi(k), -m * 10f64.powi(k)]);
        }
    }
    out
}

const EPSS: [f64; 3] = [1e-6, 1e-9, 1e-12];
const SCALES: [f64; 3] = [1e-3, 1.0, 1e3];
const OFFS: [f64; 2] = [0.0, 1e3];

fn jitters(eps: f64, s: f64, off: f64) -> [f64; 3] {
    [0.0, 4.0 * f64::EPSILON * (off + s), 0.5 * eps]
}

#[derive(Default, Debug)]
struct Tally {
    unresolvable: usize,
    wrong: usize,
    decided: usize,
    first: Option<String>,
}

fn resolvable(eps: f64, size: f64) -> bool {
    eps >= 64.0 * f64::EPSILON * size
}

impl Tally {
    fn read(&mut self, got: Result<Option<bool>, Indeterminate>, truth: bool, tag: impl Fn() -> String) {
        match got {
            Ok(Some(p)) if p == truth => self.decided += 1,
            Ok(Some(_)) => {
                self.wrong += 1;
                self.first.get_or_insert_with(tag);
            }
            _ => {}
        }
    }
}

/// Parallel × ruling: a ruling split at a vertex, the parallel through
/// (a nudge off) it. Truth odd.
fn parallel_ruling(band: Band, alpha: f64, s: f64, off: f64, jit: f64, rng: &mut Rng, t: &mut Tally) {
    let q = rng.frame();
    let w = |v: Vec3<f64>| q[0] * v.x + q[1] * v.y + q[2] * v.z;
    let apex = Point3::origin() + rng.unit() * off;
    let z = w(Vec3::unit_z());
    let e = w(Vec3::new(alpha.sin(), 0.0, alpha.cos()));
    let l = s / alpha.sin();
    if !resolvable(band.zero(), off + 2.0 * l) {
        t.unresolvable += 1;
        return;
    }
    for jdir in [rng.unit(), z, -z] {
        let arcs = [
            LoopArc::Line {
                origin: apex,
                dir: e,
                span: (l - s, l),
            },
            LoopArc::Line {
                origin: apex + jdir * jit,
                dir: (e + jdir.cross(e) * (jit / l)).normalize(),
                span: (l, l + s),
            },
        ];
        for nudge in nudges() {
            let ln = l + nudge * s;
            let piece = CircleArc {
                centre: apex + z * (ln * alpha.cos()),
                axis: z,
                radius: ln * alpha.sin(),
                u_ref: w(Vec3::new((-0.3f64).cos(), (-0.3f64).sin(), 0.0)),
                span: (0.0, 0.6),
            };
            let path = Path {
                pieces: vec![Piece::Arc(piece)],
                arrival: piece.tangent(0.6),
            };
            t.read(path_parity(&path, &arcs, None, band), true, || {
                format!("α {alpha:e} s {s:e} off {off:e} jit {jit:e} nudge {nudge:e}")
            });
        }
    }
}

/// The section of the cone (apex, z, α) by the plane through the axis
/// point at height `h0` with normal at `theta` from the axis.
fn section(apex: Point3<f64>, q: [Vec3<f64>; 3], alpha: f64, h0: f64, theta: f64, band: Band) -> Option<geom::Curve3<f64>> {
    let w = |v: Vec3<f64>| q[0] * v.x + q[1] * v.y + q[2] * v.z;
    let cone = geom::Surface::Cone {
        apex,
        axis: w(Vec3::unit_z()),
        half_angle: alpha,
        u_ref: w(Vec3::unit_x()),
    };
    let n = w(Vec3::new(theta.sin(), 0.0, theta.cos()));
    let o = apex + w(Vec3::unit_z()) * h0;
    let plane = geom::Surface::Plane {
        origin: o,
        normal: n,
        u_ref: n.orthonormal_basis().0,
    };
    let extent = 8.0 * h0 / (0.5 * core::f64::consts::PI - alpha - theta).sin().max(1e-300);
    match geom_brep::plane_cone_section(&plane, &cone, extent.min(1e12), band) {
        Ok(geom_brep::PlaneConeSection::TiltedEllipse(c)) => Some(c),
        _ => None,
    }
}

fn conic_parts(c: &geom::Curve3<f64>) -> (Point3<f64>, Vec3<f64>, Vec3<f64>, f64, f64) {
    match *c {
        geom::Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => (center, axis, u_ref, major, minor),
        _ => unreachable!(),
    }
}

fn split(c: &geom::Curve3<f64>, v: f64, jdir: Vec3<f64>, jit: f64, s: f64) -> [LoopArc<f64>; 2] {
    let (centre, axis, u_ref, a, b) = conic_parts(c);
    let pi = core::f64::consts::PI;
    [
        LoopArc::Conic {
            centre,
            axis,
            u_ref,
            a,
            b,
            span: (v - pi, v),
        },
        LoopArc::Conic {
            centre: centre + jdir * jit,
            axis: (axis + jdir.cross(axis) * (jit / s)).normalize(),
            u_ref,
            a,
            b,
            span: (v, v + pi),
        },
    ]
}

/// Segment × section: a ruling segment through (a nudge off) a vertex of
/// a whole split ellipse, near-parabolic by `eta`. Truth odd.
fn segment_section(band: Band, sec_band: Band, alpha: f64, eta: f64, s: f64, off: f64, jit: f64, rng: &mut Rng, t: &mut Tally, skipped: &mut usize) {
    let q = rng.frame();
    let apex = Point3::origin() + rng.unit() * off;
    let theta = 0.5 * core::f64::consts::PI - alpha - eta;
    let Some(c) = section(apex, q, alpha, s, theta, sec_band) else {
        *skipped += 1;
        return;
    };
    // The vertex farthest from the apex (the shallowest ruling) and two
    // others.
    let (_, _, _, ma, _) = conic_parts(&c);
    let size = off + (c.eval(0.0) - apex).norm() + 2.0 * ma;
    if !resolvable(band.zero(), size) {
        t.unresolvable += 1;
        return;
    }
    let far = (0..4096)
        .map(|i| i as f64 * core::f64::consts::TAU / 4096.0)
        .max_by(|x, y| {
            (c.eval(*x) - apex)
                .norm()
                .partial_cmp(&(c.eval(*y) - apex).norm())
                .unwrap()
        })
        .unwrap();
    for tv in [far, far + 0.3, far + 2.0] {
        for jdir in [rng.unit(), q[2]] {
            let arcs = split(&c, tv, jdir, jit, ma);
            for nudge in nudges() {
                let x = c.eval(tv + nudge);
                let e = (x - apex).normalize();
                let slant = (x - apex).norm();
                let (from, to) = (apex + e * (slant * 0.7), apex + e * (slant * 1.3));
                let path = Path {
                    pieces: vec![Piece::Segment {
                        from,
                        to,
                        ruling: e,
                        lever: slant * 0.7,
                    }],
                    arrival: e,
                };
                t.read(path_parity(&path, &arcs, None, band), true, || {
                    format!("α {alpha:e} η {eta:e} s {s:e} off {off:e} jit {jit:e} tv-far {} nudge {nudge:e}", tv - far)
                });
            }
        }
    }
}

/// Parallel × section on a cone: a parallel grazing a whole split
/// tilted ellipse near its top, one root at (a nudge off) the vertex.
/// Truth even.
fn parallel_section_graze(band: Band, sec_band: Band, alpha: f64, theta: f64, s: f64, off: f64, jit: f64, rng: &mut Rng, t: &mut Tally, skipped: &mut usize) {
    let q = rng.frame();
    let w = |v: Vec3<f64>| q[0] * v.x + q[1] * v.y + q[2] * v.z;
    let apex = Point3::origin() + rng.unit() * off;
    let z = w(Vec3::unit_z());
    let Some(c) = section(apex, q, alpha, s, theta, sec_band) else {
        *skipped += 1;
        return;
    };
    let (centre, axis, u_ref, a, b) = conic_parts(&c);
    if !resolvable(band.zero(), off + (centre - apex).norm() + 2.0 * a) {
        t.unresolvable += 1;
        return;
    }
    let v_ref = axis.cross(u_ref);
    let zc = z.dot(centre - apex);
    let (cu, cv) = (z.dot(u_ref) * a, z.dot(v_ref) * b);
    let amp = (cu * cu + cv * cv).sqrt();
    let top = cv.atan2(cu);
    let zmax = zc + amp;
    let top_pt = c.eval(top);
    let radial = {
        let r = (top_pt - apex) - z * z.dot(top_pt - apex);
        r / r.norm()
    };
    let jdir = rng.unit();
    for gk in [1.01, 1.5, 3.0, 10.0, 100.0, 1e4, 1e6] {
        for top_side in [1.0, -1.0] {
            // below the top (top_side 1) or above the bottom (-1)
            let dh = gk * band.escalate();
            let h = if top_side > 0.0 { zmax - dh } else { zc - amp + dh };
            let half = ((h - zc) / amp).clamp(-1.0, 1.0).acos();
            if half > 0.5 {
                continue;
            }
            let centre_ang = if top_side > 0.0 { top } else { top + core::f64::consts::PI };
            let root = if top_side > 0.0 { top + half } else { top - half };
            let _ = centre_ang;
            let rad_dir = if top_side > 0.0 {
                radial
            } else {
                let p = c.eval(top + core::f64::consts::PI);
                let r = (p - apex) - z * z.dot(p - apex);
                r / r.norm()
            };
            let u0 = (rad_dir * 1.0f64.cos() - z.cross(rad_dir) * 1.0f64.sin()).normalize();
            let piece = CircleArc {
                centre: apex + z * h,
                axis: z,
                radius: h * alpha.tan(),
                u_ref: u0,
                span: (0.0, 2.0),
            };
            let path = Path {
                pieces: vec![Piece::Arc(piece)],
                arrival: piece.tangent(2.0),
            };
            let other = if top_side > 0.0 { top - half } else { top + half };
            let inside = |p: Point3<f64>| {
                let t = piece.param(p);
                t > 0.05 && t < 1.95
            };
            if !(inside(c.eval(root)) && inside(c.eval(other))) {
                t.unresolvable += 1;
                continue;
            }
            for nudge in nudges() {
                let arcs = split(&c, root + nudge * half, jdir, jit, a);
                t.read(path_parity(&path, &arcs, None, band), false, || {
                    format!("α {alpha:e} θ {theta:e} s {s:e} off {off:e} jit {jit:e} gk {gk} side {top_side} nudge {nudge:e}")
                });
            }
        }
    }
}

fn run<F: FnMut(Band, f64, f64, f64, &mut Rng, &mut Tally)>(name: &str, mut f: F) -> usize {
    let mut rng = Rng(0x9e3779b97f4a7c15);
    let mut total_wrong = 0;
    for eps in EPSS {
        let band = Band::linear_at(Tol::witness(), eps).unwrap();
        let mut d = 0;
        let mut unres = 0;
        for s in SCALES {
            for off in OFFS {
                for jit in jitters(eps, s, off) {
                    let mut t = Tally::default();
                    f(band, s, off, jit, &mut rng, &mut t);
                    d += t.decided;
                    unres += t.unresolvable;
                    total_wrong += t.wrong;
                    if t.wrong > 0 {
                        println!("{name} WRONG ε {eps:e} s {s:e} off {off:e} jit {jit:e}: {} wrong / {} right; first {:?}", t.wrong, t.decided, t.first);
                    }
                }
            }
        }
        println!("{name} ε {eps:e}: decided {d}, unresolvable/skipped cases {unres}");
    }
    println!("{name} TOTAL WRONG {total_wrong}");
    total_wrong
}

#[test]
fn r4_parallel_ruling() {
    let mut w = 0;
    for alpha_deg in [1e-4, 0.1, 30.0, 80.0, 89.0, 89.9, 89.999, 89.99999] {
        let alpha = (alpha_deg as f64).to_radians();
        w += run(&format!("PR α°{alpha_deg}"), |band, s, off, jit, rng, t| {
            parallel_ruling(band, alpha, s, off, jit, rng, t)
        });
    }
    println!("PR ALL WRONG {w}");
}

#[test]
fn r4_segment_section() {
    let mut w = 0;
    let sec_band = Band::linear_at(Tol::witness(), 1e-12).unwrap();
    for alpha_deg in [0.5, 30.0, 85.0] {
        for eta in [0.5, 1e-2, 1e-4, 1e-6] {
            let alpha = (alpha_deg as f64).to_radians();
            let mut skipped = 0;
            w += run(&format!("SS α°{alpha_deg} η{eta:e}"), |band, s, off, jit, rng, t| {
                segment_section(band, sec_band, alpha, eta, s, off, jit, rng, t, &mut skipped)
            });
            println!("SS α°{alpha_deg} η{eta:e} skipped(section refused) {skipped}");
        }
    }
    println!("SS ALL WRONG {w}");
}

#[test]
fn r4_parallel_section_graze() {
    let mut w = 0;
    let sec_band = Band::linear_at(Tol::witness(), 1e-12).unwrap();
    for alpha_deg in [5.0, 30.0, 80.0] {
        for theta_frac in [0.2, 0.6, 0.95] {
            let alpha = (alpha_deg as f64).to_radians();
            let theta = theta_frac * (0.5 * core::f64::consts::PI - alpha);
            let mut skipped = 0;
            w += run(&format!("PS α°{alpha_deg} θf{theta_frac}"), |band, s, off, jit, rng, t| {
                parallel_section_graze(band, sec_band, alpha, theta, s, off, jit, rng, t, &mut skipped)
            });
            println!("PS α°{alpha_deg} θf{theta_frac} skipped {skipped}");
        }
    }
    println!("PS ALL WRONG {w}");
}

/// A section crossing a real cone path at its turn (the seam between
/// the ruling segment and the parallel arc), with the ellipse's minor
/// semi-axis far above the parallel's radius: the piece-side in-span
/// reading near the arc's start against the segment's side reading.
/// Truth: whether `from` and `to` lie on opposite sides of the plane.
fn at_the_turn(band: Band, s: f64, off: f64, jit: f64, tilt_frac: f64, rng: &mut Rng, t: &mut Tally) {
    let q = rng.frame();
    let w = |v: Vec3<f64>| q[0] * v.x + q[1] * v.y + q[2] * v.z;
    let apex = Point3::origin() + rng.unit() * off;
    let alpha = core::f64::consts::FRAC_PI_6;
    let z = w(Vec3::unit_z());
    let cone = geom::Surface::Cone {
        apex,
        axis: z,
        half_angle: alpha,
        u_ref: w(Vec3::unit_x()),
    };
    let quadric = Quadric::of(&cone).unwrap();
    let on = |h: f64, az: f64| apex + w(Vec3::new(h * alpha.tan() * az.cos(), h * alpha.tan() * az.sin(), h));
    for (hf, ht) in [(2.0 * s, 0.01 * s), (0.01 * s, 2.0 * s), (2.0 * s, 0.2 * s)] {
        let lift = |p: Point3<f64>, k: f64| {
            let r = (p - apex) - z * z.dot(p - apex);
            p + r / r.norm() * (k * band.escalate())
        };
        let lk = std::env::var("R4_LIFT").ok().and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
        let (from, to) = (lift(on(hf, 0.0), lk), lift(on(ht, 0.8), -lk));
        // The turns of both paths.
        let turns = [on(ht, 0.0), on(hf, 0.8)];
        let size = off + 3.0 * s;
        if !resolvable(band.zero(), size) {
            t.unresolvable += 1;
            return;
        }
        let Ok(paths) = quadric.paths((from, to), band) else { continue };
        for (k, turn) in turns.into_iter().enumerate() {
            let Some(path) = paths.get(k) else { continue };
            let theta = tilt_frac * (0.5 * core::f64::consts::PI - alpha);
            // Tilt across the turn's azimuth so the parallel crosses
            // the section transversally there.
            let az = if k == 0 { 0.0 } else { 0.8 };
            let side = w(Vec3::new(-(az as f64).sin(), az.cos(), 0.0));
            let n = (z * theta.cos() + side * theta.sin()).normalize();
            for nudge in nudges() {
                let o = turn + n * (nudge * s);
                let plane = geom::Surface::Plane {
                    origin: o,
                    normal: n,
                    u_ref: n.orthonormal_basis().0,
                };
                let Ok(geom_brep::PlaneConeSection::TiltedEllipse(c)) =
                    geom_brep::plane_cone_section(&plane, &cone, 1e3 * s, band)
                else {
                    continue;
                };
                let (_, _, _, ma, _) = conic_parts(&c);
                if !resolvable(band.zero(), off + 3.0 * ma + 3.0 * s) {
                    t.unresolvable += 1;
                    continue;
                }
                let (df, dt) = (n.dot(from - o), n.dot(to - o));
                if df.abs() < 1e3 * band.escalate() || dt.abs() < 1e3 * band.escalate() {
                    continue;
                }
                let truth = (df > 0.0) != (dt > 0.0);
                let arcs = split(&c, 0.7, rng.unit(), jit, ma);
                t.read(path_parity(path, &arcs, None, band), truth, || {
                    format!("s {s:e} off {off:e} jit {jit:e} hf/ht {hf}/{ht} path {k} tilt {tilt_frac} nudge {nudge:e}")
                });
            }
        }
    }
}

#[test]
fn r4_at_the_turn() {
    let mut w = 0;
    for tilt in [0.3, 0.8, 0.99] {
        w += run(&format!("TURN tilt {tilt}"), |band, s, off, jit, rng, t| {
            at_the_turn(band, s, off, jit, tilt, rng, t)
        });
    }
    println!("TURN ALL WRONG {w}");
}
