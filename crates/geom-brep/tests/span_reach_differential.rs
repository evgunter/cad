//! **The span-reach differential**: main's whole-turn conic reach
//! ([`Reach::lever_from`]) against the span's ([`Reach::span_reach_from`])
//! where a face's measure reads it, on faces bounded by rim arcs.
//!
//! Each face is measured as `topo`'s `splitting::rules::face_reach_from`
//! and `face_axial_range` measure it: its corners, one [`Reach::Span`]
//! per boundary edge. Two families:
//!
//! - **cylinder patches** `[u0, u1] × [v0, v1]` (two rim arcs, two
//!   rulings) cut by a plane through a corner, read by the actual
//!   [`plane_cylinder_section`] on [`Reach::Face`] at a corner (the
//!   chord_join reading) and at the corners' centre (the germ frame's).
//!   The truth is the exact offset `(P − hinge)·(n − ŵ)` of the real
//!   plane from the rulings' plane over a grid of the patch: Zero within
//!   one zero band, definite past the escalation band. A served zero
//!   lane is read on: rulings against the in-section clearance over the
//!   patch's stations, a tangent ruling against its farthest point off
//!   either surface, an empty section against the clearance.
//! - **plane sectors** (an annular sector: two arcs, two radial
//!   segments) under the split lane's coplanarity row, the face's
//!   normal against a plane tilted about a line through a corner,
//!   levered at the face extent from that corner. The truth is the
//!   sector's farthest point's distance off the tilted plane.
//!
//! Not a gate: it asserts nothing. Run it with
//! `cargo nextest run -p geom-brep -E 'test(span_reach_differential)' --run-ignored only --no-capture --release`.

#![allow(clippy::panic, clippy::too_many_lines, clippy::cast_precision_loss)]

use crate::shared::tol::band;
use geom::{Curve3, Surface};
use geom_brep::intersect::{PlaneCylinderSection, plane_cylinder_section};
use geom_brep::{Reach, SectionError, WallBend};
use geom_core::k_stats::decide;
use geom_core::{Margin, Point3, Sign, Vec3};
use std::collections::BTreeMap;
use test_utils::fuzz;

struct Rng(u64);
impl Rng {
    fn u(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn r(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.u()
    }
    fn log(&mut self, a: f64, b: f64) -> f64 {
        (a.ln() + (b.ln() - a.ln()) * self.u()).exp()
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.r(-1., 1.), self.r(-1., 1.), self.r(-1., 1.));
            let n = v.norm();
            if n > 0.2 && n < 1.0 {
                return v / n;
            }
        }
    }
}

fn perp(a: Vec3<f64>, rng: &mut Rng) -> Vec3<f64> {
    loop {
        let v = rng.unit();
        let w = v - a * v.dot(a);
        if w.norm() > 0.3 {
            return w.normalize();
        }
    }
}

/// A face as the topo measures read it: its corners and its edges.
struct Face {
    corners: Vec<Point3<f64>>,
    edges: Vec<Reach<f64>>,
    grid: Vec<Point3<f64>>,
}

impl Face {
    /// `face_reach_from(at)`: main's (`whole`) or the span's.
    fn across(&self, at: Point3<f64>, whole: bool) -> f64 {
        let e = self.edges.iter().fold(0.0_f64, |m, s| {
            m.max(if whole {
                s.lever_from(at)
            } else {
                s.span_reach_from(at)
            })
        });
        self.corners.iter().fold(e, |m, p| m.max((*p - at).norm()))
    }
    /// `face_axial_range(at, axis)`.
    fn axial(&self, at: Point3<f64>, axis: Vec3<f64>) -> (f64, f64) {
        let (lo, hi) = self.edges.iter().fold((0.0_f64, 0.0_f64), |(lo, hi), s| {
            let (l, h) = s.range_along(at, axis);
            (lo.max(-l), hi.max(h))
        });
        self.corners.iter().fold((lo, hi), |(lo, hi), p| {
            let x = (*p - at).dot(axis);
            (lo.max(-x), hi.max(x))
        })
    }
    fn sampled_reach(&self, at: Point3<f64>) -> f64 {
        self.grid
            .iter()
            .fold(0.0_f64, |m, p| m.max((*p - at).norm()))
    }
}

/// Four ulps of the coordinates at `p` and of a distance `d` read from
/// it: the rounding two spellings of one reach can differ by.
fn ulps(p: Point3<f64>, d: f64) -> f64 {
    4.0 * f64::EPSILON * ((p - Point3::origin()).norm() + d).max(1.0)
}

fn line(p: Point3<f64>, q: Point3<f64>) -> Reach<f64> {
    Reach::Span {
        carrier: Curve3::Line {
            origin: p,
            dir: q - p,
        },
        t0: 0.0,
        t1: 1.0,
    }
}

/// The cylinder patch `[u0, u0 + du] × [v0, v0 + h]` of the wall about
/// `(o, a)` of radius `r`, seam `x`.
fn patch(
    o: Point3<f64>,
    a: Vec3<f64>,
    x: Vec3<f64>,
    r: f64,
    (u0, du): (f64, f64),
    (v0, h): (f64, f64),
) -> Face {
    let y = a.cross(x);
    let at = |u: f64, v: f64| o + (x * u.cos() + y * u.sin()) * r + a * v;
    let rim = |v: f64| Reach::Span {
        carrier: Curve3::Circle {
            center: o + a * v,
            axis: a,
            radius: r,
            u_ref: x,
        },
        t0: u0,
        t1: u0 + du,
    };
    let c = [
        at(u0, v0),
        at(u0 + du, v0),
        at(u0 + du, v0 + h),
        at(u0, v0 + h),
    ];
    let n = 48;
    let grid = (0..=n)
        .flat_map(|i| {
            (0..=n).map(move |j| (u0 + du * i as f64 / n as f64, v0 + h * j as f64 / n as f64))
        })
        .map(|(u, v)| at(u, v))
        .collect();
    Face {
        corners: c.to_vec(),
        edges: vec![rim(v0), rim(v0 + h), line(c[1], c[2]), line(c[3], c[0])],
        grid,
    }
}

/// The annular sector `ρ ∈ [r1, r2]`, `t ∈ [t0, t0 + dt]` of the plane
/// about `c` spanned by `(x, y)`.
fn sector(
    c: Point3<f64>,
    (x, y): (Vec3<f64>, Vec3<f64>),
    (r1, r2): (f64, f64),
    (t0, dt): (f64, f64),
) -> Face {
    let at = |rho: f64, t: f64| c + (x * t.cos() + y * t.sin()) * rho;
    let arc = |rho: f64| Reach::Span {
        carrier: Curve3::Circle {
            center: c,
            axis: x.cross(y),
            radius: rho,
            u_ref: x,
        },
        t0,
        t1: t0 + dt,
    };
    let k = [at(r1, t0), at(r2, t0), at(r2, t0 + dt), at(r1, t0 + dt)];
    let n = 48;
    let grid = (0..=n)
        .flat_map(|i| {
            (0..=n).map(move |j| {
                (
                    r1 + (r2 - r1) * i as f64 / n as f64,
                    t0 + dt * j as f64 / n as f64,
                )
            })
        })
        .map(|(rho, t)| at(rho, t))
        .collect();
    Face {
        corners: k.to_vec(),
        edges: vec![arc(r1), arc(r2), line(k[0], k[1]), line(k[2], k[3])],
        grid,
    }
}

fn verdict(got: &Result<PlaneCylinderSection<f64>, SectionError>) -> &'static str {
    match got {
        Ok(PlaneCylinderSection::TiltedEllipse(_) | PlaneCylinderSection::Rim(_)) => "conic",
        Ok(PlaneCylinderSection::ParallelLines { .. }) => "lines",
        Ok(PlaneCylinderSection::TangentLine(_)) => "tangent",
        Ok(PlaneCylinderSection::Empty) => "empty",
        Err(SectionError::Escalated(_)) => "esc",
        Err(_) => "err",
    }
}

#[derive(Default)]
struct Tally {
    buckets: BTreeMap<String, usize>,
    against: [usize; 2],
    short_lever: usize,
    longer_than_main: usize,
    served_where_main_escalated: Vec<String>,
    good_esc: [usize; 2],
}

#[test]
#[ignore = "a differential that asserts nothing (module docs)"]
fn span_reach_differential() {
    let b = band();
    let z = b.zero();
    let kk = b.escalate() / z;
    let mut rng = Rng(0x5ba7);
    let mut tallies: BTreeMap<&'static str, Tally> = BTreeMap::new();
    let n = 10_000;
    for i in 0..n {
        let s = [1e-3, 1.0, 1e3][i % 3];
        let o = Point3::origin() + rng.unit() * 1e3;
        let a = rng.unit();
        let x = perp(a, &mut rng);
        let du = rng.log(1e-6, 3.0);
        let h = s * rng.log(1e-6, 10.0);
        let u0 = rng.r(-3.0, 3.0);
        let face = patch(o, a, x, s, (u0, du), (rng.r(-1.0, 1.0) * s, h));
        let cyl = Surface::Cylinder {
            origin: o,
            axis: a,
            radius: s,
            u_ref: x,
        };
        let centre = face.corners.iter().fold(Point3::origin(), |m, p| {
            m + (*p - Point3::origin()) / face.corners.len() as f64
        });
        let corner = face.corners[(rng.u() * 4.0) as usize % 4];
        let other = face.corners[(rng.u() * 4.0) as usize % 4];
        for (caller, at, through) in [("chord_join", corner, corner), ("germ", centre, other)] {
            let (below, above) = face.axial(at, a);
            let axial = face
                .grid
                .iter()
                .fold(0.0_f64, |m, p| m.max(((*p - at).dot(a)).abs()))
                .max(1e-300);
            // The plane through `through`, tilted off a radial (or off the
            // radial's normal, through the axis) by `β`.
            let radial = {
                let w = through - o;
                (w - a * w.dot(a)).normalize()
            };
            let w = if rng.u() < 0.3 {
                a.cross(radial) * if rng.u() < 0.5 { 1.0 } else { -1.0 }
            } else {
                perp(a, &mut rng)
            };
            let sin_beta = if rng.u() < 0.15 {
                rng.r(0.0, core::f64::consts::FRAC_PI_2).sin()
            } else {
                (rng.log(0.05, 3.0 * kk) * z / axial).min(1.0)
            } * if rng.u() < 0.5 { 1.0 } else { -1.0 };
            let nrm = w * (1.0 - sin_beta * sin_beta).max(0.0).sqrt() + a * sin_beta;
            let plane = Surface::Plane {
                origin: through,
                normal: nrm,
                u_ref: perp(nrm, &mut rng),
            };
            // The classifier's hinge and the truth there.
            let foot = o + a * (at - o).dot(a);
            let gap = (foot - through).dot(nrm);
            let hinge = foot - nrm * gap;
            let c = a.dot(nrm);
            let off = nrm - a * c;
            let cos = (1.0 - c * c).max(0.0).sqrt();
            // `(P − hinge)·(n − ŵ) = c·x_a − (1 − cos)·x_ŵ`; where the
            // normal names no direction across the wall, the sup over ŵ.
            let across_of = |p: Point3<f64>| {
                let x = p - hinge;
                if off.norm() > 1e-12 {
                    (x.dot(off.normalize())).abs()
                } else {
                    (x - a * x.dot(a)).norm()
                }
            };
            let offset = face
                .grid
                .iter()
                .map(|p| (c * (*p - hinge).dot(a)).abs() + (1.0 - cos) * across_of(*p))
                .fold(0.0_f64, f64::max);
            let signed = face
                .grid
                .iter()
                .map(|p| {
                    let x = *p - hinge;
                    if off.norm() > 1e-12 {
                        (c * x.dot(a) - (1.0 - cos) * x.dot(off.normalize())).abs()
                    } else {
                        (c * x.dot(a)).abs() + (1.0 - cos) * across_of(*p)
                    }
                })
                .fold(0.0_f64, f64::max);
            let offset = offset.min(signed);
            let truth = if offset <= z {
                "Zero"
            } else if offset >= b.escalate() {
                "definite"
            } else {
                "band"
            };
            let reading = |whole: bool| {
                let reach = Reach::Face {
                    at,
                    below,
                    above,
                    across: face.across(at, whole),
                };
                (plane_cylinder_section(&plane, &cyl, &reach, b), reach)
            };
            let (main, _) = reading(true);
            let (head, head_reach) = reading(false);
            let (vm, vh) = (verdict(&main), verdict(&head));
            // Both callers serve only what both measures serve (`topo`'s
            // `chord_join::agreed_section`).
            let vh = match (vm, vh) {
                (m, h) if m == h => h,
                ("esc", _) | (_, "esc") => "esc",
                _ => "disagree",
            };
            let t = tallies.entry(caller).or_default();
            *t.buckets
                .entry(format!("{truth:<8} {vm:<7} -> {vh}"))
                .or_default() += 1;
            // The lane's served object against the truth.
            let stations = face.grid.iter().map(|p| (*p - foot).dot(a));
            let clear = stations
                .map(|st| (gap + st * c).abs() / cos.max(1e-300) - s)
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), x| {
                    (lo.min(x), hi.max(x))
                });
            let against = |v: &str| match v {
                "conic" => truth != "definite",
                "lines" => truth != "Zero" || clear.1 >= 0.0,
                "empty" => truth != "Zero" || clear.0 <= 0.0,
                "tangent" => {
                    let st = face
                        .grid
                        .iter()
                        .map(|p| ((*p - hinge).dot(a) * c).abs())
                        .fold(0.0_f64, f64::max);
                    truth != "Zero" || st.max((gap.abs() * cos - s).abs()) > z
                }
                _ => false,
            };
            for (k, v) in [vm, vh].into_iter().enumerate() {
                if against(v) {
                    t.against[k] += 1;
                }
                if (v == "esc" || v == "disagree") && truth != "band" {
                    t.good_esc[k] += 1;
                }
            }
            if vm == "esc" && vh != "esc" && vh != "disagree" {
                t.served_where_main_escalated.push(format!(
                    "{i} r={s:e} du={du:.3e} h={h:.3e} truth={truth} offset={:.3}z -> {vh}{}",
                    offset / z,
                    if against(vh) { " AGAINST" } else { "" }
                ));
            }
            let across_head = face.across(at, false);
            if across_head < face.sampled_reach(at) - ulps(at, face.sampled_reach(at)) {
                println!(
                    "SHORT {caller} {i} r={s:e} du={du:e} h={h:e} head={across_head:e} sampled={:e} short_by={:e} ulps={:e} |at|={:e}",
                    face.sampled_reach(at),
                    face.sampled_reach(at) - across_head,
                    ulps(at, face.sampled_reach(at)),
                    (at - Point3::origin()).norm()
                );
                t.short_lever += 1;
            }
            if across_head > face.across(at, true) {
                t.longer_than_main += 1;
            }
            // Head's in-band conics: would the exact reach across the wall
            // from the hinge still serve? Then the sum of the two terms'
            // maxima serves it, not this reach.
            if truth == "band" && vh == "conic" {
                let hl = head_reach.hinge_lever(foot, -(gap * c));
                let exact = face
                    .grid
                    .iter()
                    .map(|p| across_of(*p))
                    .fold(0.0_f64, f64::max);
                let ideal = c.abs() * hl + exact * (1.0 - cos);
                let k = if ideal >= b.escalate() {
                    "sum of maxima"
                } else {
                    "this reach"
                };
                *t.buckets.entry(format!("  band conic by {k}")).or_default() += 1;
            }
        }
        for (family, case) in [
            ("split_cyl", split_cyl_case(&mut rng, b, s)),
            ("split", split_plane_case(&mut rng, b, s)),
        ] {
            let t = tallies.entry(family).or_default();
            *t.buckets
                .entry(format!(
                    "{:<11} {:<11} -> {}",
                    case.truth, case.main, case.head
                ))
                .or_default() += 1;
            for (k, v) in [case.main, case.head].into_iter().enumerate() {
                if case.against(v) {
                    t.against[k] += 1;
                }
                if v == "esc" && case.truth != "band" {
                    t.good_esc[k] += 1;
                }
            }
            if case.main != case.head && case.head != "esc" {
                t.served_where_main_escalated.push(format!(
                    "{i} r={s:e} truth={} main={} -> {}{}",
                    case.truth,
                    case.main,
                    case.head,
                    if case.against(case.head) {
                        " AGAINST"
                    } else {
                        ""
                    }
                ));
            }
            t.short_lever += usize::from(case.short);
            t.longer_than_main += usize::from(case.longer);
        }
    }
    for (caller, t) in &tallies {
        println!("== {caller}");
        for (k, v) in &t.buckets {
            println!("  {k:<32} {v}");
        }
        println!(
            "  served against the truth: main {} head {}",
            t.against[0], t.against[1]
        );
        println!(
            "  head lever shorter than the sampled region: {}",
            t.short_lever
        );
        println!("  head lever longer than main's: {}", t.longer_than_main);
        println!(
            "  good-input escalations: main {} head {}",
            t.good_esc[0], t.good_esc[1]
        );
        println!(
            "  served where main escalated (split families: served differently from main): {}",
            t.served_where_main_escalated.len()
        );
        println!(
            "  ... of which against the truth: {}",
            t.served_where_main_escalated
                .iter()
                .filter(|l| l.ends_with("AGAINST"))
                .count()
        );
        for l in t.served_where_main_escalated.iter().take(12) {
            println!("    {l}");
        }
    }
}

/// One split-row sample: the truth, main's and head's verdicts ("esc"
/// for an escalation), and whether head's lever fell short of the
/// sampled region or past main's.
struct Case {
    truth: &'static str,
    main: &'static str,
    head: &'static str,
    short: bool,
    longer: bool,
}

impl Case {
    /// A served verdict is against the truth where it is not the truth's
    /// own; "tilted" and "coplanar" stand for the plane row's definite
    /// and Zero.
    fn against(&self, v: &str) -> bool {
        match v {
            "esc" => false,
            "coplanar" => self.truth != "Zero",
            "tilted" => self.truth != "definite",
            v => v != self.truth,
        }
    }
}

fn classify(m: f64, b: geom_core::Band) -> &'static str {
    if m.abs() <= b.zero() {
        "Zero"
    } else if m.abs() >= b.escalate() {
        "definite"
    } else {
        "band"
    }
}

/// The cylinder patch between rulings `u0` and `u0 + du`, bounded by the
/// wall's sections with two planes, each through the axis point at
/// height `vᵢ` and tilted by `φᵢ` toward `x`: `φ = 0` is a rim circle,
/// any other an ellipse of semi-axes `r/cos φ` and `r` about that point,
/// the major along `(x − a·tan φ)·cos φ`.
fn oblique_patch(
    (o, a, x, r): (Point3<f64>, Vec3<f64>, Vec3<f64>, f64),
    (u0, du): (f64, f64),
    [(v0, f0), (v1, f1)]: [(f64, f64); 2],
) -> Face {
    let y = a.cross(x);
    let height = |v: f64, f: f64, u: f64| v - r * f.tan() * u.cos();
    let at = |u: f64, v: f64| o + (x * u.cos() + y * u.sin()) * r + a * v;
    let cut = |v: f64, f: f64| {
        let u_ref = (x - a * f.tan()) * f.cos();
        let carrier = if f == 0.0 {
            Curve3::Circle {
                center: o + a * v,
                axis: a,
                radius: r,
                u_ref: x,
            }
        } else {
            Curve3::Ellipse {
                center: o + a * v,
                axis: u_ref.cross(y),
                major: r / f.cos(),
                minor: r,
                u_ref,
            }
        };
        Reach::Span {
            carrier,
            t0: u0,
            t1: u0 + du,
        }
    };
    let c = [
        at(u0, height(v0, f0, u0)),
        at(u0 + du, height(v0, f0, u0 + du)),
        at(u0 + du, height(v1, f1, u0 + du)),
        at(u0, height(v1, f1, u0)),
    ];
    let n = 48;
    let grid = (0..=n)
        .flat_map(|i| (0..=n).map(move |j| (i, j)))
        .map(|(i, j)| {
            let u = u0 + du * f64::from(i) / f64::from(n);
            let (lo, hi) = (height(v0, f0, u), height(v1, f1, u));
            at(u, lo + (hi - lo) * f64::from(j) / f64::from(n))
        })
        .collect();
    Face {
        corners: c.to_vec(),
        edges: vec![cut(v0, f0), cut(v1, f1), line(c[1], c[2]), line(c[3], c[0])],
        grid,
    }
}

/// **A cylinder wall under the split lane's rule (a)**: a plane
/// near-tangent at a corner of a patch (rim circles or, half the time,
/// oblique ellipses), the coplanarity row, then the osculation and bend
/// rows of a grazed wall, each levered at the face extent from the
/// corner, against the same tree with each row's margin at the sampled
/// reach.
fn split_cyl_case(rng: &mut Rng, b: geom_core::Band, s: f64) -> Case {
    let z = b.zero();
    let kk = b.escalate() / z;
    let o = Point3::origin() + rng.unit() * 1e3;
    let a = rng.unit();
    let x = perp(a, rng);
    let du = rng.log(1e-6, 3.0);
    let h = s * rng.log(1e-6, 10.0);
    let u0 = rng.r(-3.0, 3.0);
    let v0 = rng.r(-1.0, 1.0) * s;
    let (f0, f1) = if rng.u() < 0.5 {
        (0.0, 0.0)
    } else {
        (rng.r(-1.2, 1.2), rng.r(-1.2, 1.2))
    };
    // The top cut clears the bottom one at every azimuth.
    let v1 = v0 + h + s * (f0.tan().abs() + f1.tan().abs());
    let face = oblique_patch((o, a, x, s), (u0, du), [(v0, f0), (v1, f1)]);
    let cyl = Surface::Cylinder {
        origin: o,
        axis: a,
        radius: s,
        u_ref: x,
    };
    let sense = rng.u() < 0.5;
    let base = face.corners[(rng.u() * 4.0) as usize % 4];
    let radial = {
        let w = base - o;
        (w - a * w.dot(a)).normalize()
    };
    let reach = face.sampled_reach(base).max(1e-300);
    let sin_t = if rng.u() < 0.15 {
        rng.r(0.0, 1.0)
    } else {
        (rng.log(0.05, 3.0 * kk) * z / reach).min(1.0)
    };
    let t = perp(radial, rng);
    let nsp = radial * (1.0 - sin_t * sin_t).max(0.0).sqrt() + t * sin_t;
    let outward = geom_brep::OutwardNormal::from_chart(radial, sense);
    let kappa = geom_brep::implicit_max_normal_curvature(&cyl, base);
    let tilt = radial.cross(nsp).norm();
    // Rule (a)'s tree at a lever: the verdict, or "esc".
    let tree = |arm: f64| -> &'static str {
        match decide("split_sector_coplanar", Margin::levered(tilt, arm), b) {
            Ok(Sign::Positive | Sign::Negative) => return "silent",
            Err(_) => return "esc",
            Ok(Sign::Zero) => {}
        }
        match decide("tangent_sector_osculation", Margin::sagitta(kappa, arm), b) {
            Ok(Sign::Positive) => {}
            Ok(_) => return "unsupported",
            Err(_) => return "esc",
        }
        match geom_brep::bends_into_material(&cyl, base, outward, arm, b) {
            Ok(WallBend::IntoMaterial) => "graze",
            Ok(WallBend::OutOfMaterial) => "knife",
            Ok(WallBend::Flat) => "unsupported",
            Err(_) => "esc",
        }
    };
    let truth = match classify(tilt * reach, b) {
        "definite" => "silent",
        "band" => "band",
        _ => match classify(0.5 * kappa * reach * reach, b) {
            "definite" => match classify(0.25 * kappa * reach * reach, b) {
                "definite" if sense => "graze",
                "definite" => "knife",
                "band" => "band",
                _ => "unsupported",
            },
            "band" => "band",
            _ => "unsupported",
        },
    };
    let (head_arm, main_arm) = (face.across(base, false), face.across(base, true));
    Case {
        truth,
        main: tree(main_arm),
        head: tree(head_arm),
        short: head_arm < reach - ulps(base, reach),
        longer: head_arm > main_arm,
    }
}

/// **A plane sector under the split lane's coplanarity row**: an annular
/// sector tilted about a line through a corner, the row levered at the
/// face extent from the corner, against the sector's farthest distance
/// off the tilted plane ([`sector_offset`]). Radii and offsets reach
/// 1e3 short of `1e12·ε`: past it a unit normal's own rounding, levered
/// at the face's extent, is a tenth of ε, and no f64 reading of the
/// tilt resolves the band.
fn split_plane_case(rng: &mut Rng, b: geom_core::Band, s: f64) -> Case {
    let z = b.zero();
    let kk = b.escalate() / z;
    let room = (1e12 * z).min(1e3);
    let s = s.min(room);
    let c = Point3::origin() + rng.unit() * room;
    let nf = rng.unit();
    let x = perp(nf, rng);
    let y = nf.cross(x);
    let r2 = s * rng.r(0.5, 2.0);
    let r1 = r2 * rng.r(0.0, 0.999);
    let dt = rng.log(1e-6, 3.0);
    let t0 = rng.r(-3.0, 3.0);
    let face = sector(c, (x, y), (r1, r2), (t0, dt));
    let corner = (rng.u() * 4.0) as usize % 4;
    let reach = face.sampled_reach(face.corners[corner]).max(1e-300);
    let sin_t = if rng.u() < 0.15 {
        rng.r(0.0, 1.0)
    } else {
        (rng.log(0.05, 3.0 * kk) * z / reach).min(1.0)
    };
    let hinge = perp(nf, rng);
    let nsp = nf * (1.0 - sin_t * sin_t).max(0.0).sqrt() + nf.cross(hinge) * sin_t;
    plane_case(b, (&face, corner), (x, y, nf), ((r1, r2), (t0, dt)), nsp)
}

/// The coplanarity row on `face` (the [`sector`] of `(x, y)` with normal
/// `nf`) at its corner `corner`, against the plane of normal `nsp`
/// through that corner.
fn plane_case(
    b: geom_core::Band,
    (face, corner): (&Face, usize),
    (x, y, nf): (Vec3<f64>, Vec3<f64>, Vec3<f64>),
    (radii, arc): ((f64, f64), (f64, f64)),
    nsp: Vec3<f64>,
) -> Case {
    let base = face.corners[corner];
    let reach = face.sampled_reach(base).max(1e-300);
    let read = |arm: f64| match decide(
        "split_sector_coplanar",
        Margin::levered(nf.cross(nsp).norm(), arm),
        b,
    ) {
        Ok(Sign::Zero) => "coplanar",
        Ok(_) => "tilted",
        Err(_) => "esc",
    };
    let (head_arm, main_arm) = (face.across(base, false), face.across(base, true));
    Case {
        truth: classify(sector_offset((x, y), nsp, radii, arc, corner), b),
        main: read(main_arm),
        head: read(head_arm),
        short: head_arm < reach - ulps(base, reach),
        longer: head_arm > main_arm,
    }
}

/// The farthest point of the [`sector`] `ρ ∈ [r1, r2]`, `t ∈ [t0, t0 +
/// dt]` of `(x, y)` off the plane of normal `n` through its corner `k`:
/// `(p − k)·n̂ = A·(x·n̂) + B·(y·n̂)`, the two near-zero dots compensated
/// ([`dot2`]) and no absolute coordinate rounded. The offset is linear
/// in `p`, so its extremes are the corners and the arcs' stationary
/// azimuths.
fn sector_offset(
    (x, y): (Vec3<f64>, Vec3<f64>),
    n: Vec3<f64>,
    (r1, r2): (f64, f64),
    (t0, dt): (f64, f64),
    k: usize,
) -> f64 {
    let (dx, dy) = (dot2(x, n) / n.norm(), dot2(y, n) / n.norm());
    let corners = [(r1, t0), (r2, t0), (r2, t0 + dt), (r1, t0 + dt)];
    let (rk, tk) = corners[k];
    let off = |(rho, t): (f64, f64)| {
        ((rho * t.cos() - rk * tk.cos()) * dx + (rho * t.sin() - rk * tk.sin()) * dy).abs()
    };
    let star = dy.atan2(dx);
    let arcs = [r1, r2].into_iter().flat_map(|rho| {
        (-2..=2)
            .map(move |j| (rho, star + f64::from(j) * core::f64::consts::PI))
            .filter(|(_, t)| (t0..=t0 + dt).contains(t))
    });
    corners.into_iter().chain(arcs).map(off).fold(0.0, f64::max)
}

/// `a·b` as if in twice the working precision (Ogita, Rump and Oishi's
/// Dot2): each product's and each partial sum's rounding error is
/// carried and added back.
fn dot2(a: Vec3<f64>, b: Vec3<f64>) -> f64 {
    let (mut s, mut c) = (0.0_f64, 0.0_f64);
    for (u, v) in [(a.x, b.x), (a.y, b.y), (a.z, b.z)] {
        let p = u * v;
        let t = s + p;
        let w = t - s;
        c += (s - (t - w)) + (p - w) + u.mul_add(v, -p);
        s = t;
    }
    s + c
}

/// The band the pinned counterexamples were drawn at: ε = 1e-12 and
/// the run's K.
#[allow(clippy::expect_used)]
fn drawn_at() -> geom_core::Band {
    geom_core::Band::linear_at(geom_core::Tol::witness(), 1e-12).expect("the 1e-12 band")
}

/// **Split #218 of `CAD_FUZZ_SEED=0x4a5a1477da7f8d8a` at ε = 1e-12**:
/// a sector of radii 1216 to 1702, 0.83 rad, 1e3 off the origin, under
/// a plane tilted 7.7e-16 about its corner. The span's lever (1253.136,
/// the corner's reach) reads the tilt at 0.90·ε and serves coplanar
/// where the whole turn's (2918) escalates. The sector's farthest point
/// is 0.993·ε off the plane, so the serving is on its truth; the grid's
/// rounded coordinates, up to 4e-13 off the face's own plane, read
/// 1.19·ε, in the band.
#[test]
fn a_sector_a_thousand_off_the_origin_serves_coplanar_on_its_exact_offset() {
    let b = drawn_at();
    let c = Point3::new(218.1321838584607, -903.8348395007782, 368.1045140578166);
    let x = Vec3::new(0.38137116318064646, 0.36602427219960004, 0.848871172826003);
    let y = Vec3::new(
        0.6865284265628431,
        -0.7270852465399547,
        0.005075803885127711,
    );
    let nf = Vec3::new(0.6190595733977365, 0.5808384254030431, -0.5285754138814308);
    let nsp = Vec3::new(0.619059573397736, 0.5808384254030433, -0.5285754138814313);
    let (radii, arc) = (
        (1216.2714342402082, 1701.9398722104595),
        (2.157531577593737, 0.8262122409773649),
    );
    let face = sector(c, (x, y), radii, arc);
    let case = plane_case(b, (&face, 0), (x, y, nf), (radii, arc), nsp);
    assert_eq!(
        (case.truth, case.main, case.head, case.short),
        ("Zero", "esc", "coplanar", false),
        "split #218: (truth, main, head, head's lever short of the face)"
    );
}

/// **The split's rows serve where main escalated only on their truth**,
/// and never on a lever short of the face: a few hundred seeded samples
/// of each split family (rim and oblique cylinder patches under rule
/// (a), plane sectors under the coplanarity row), at scales 1e-3, 1 and
/// 1e3 and 1e3 off the origin (the sectors' capped at `1e12·ε`). The
/// full families are
/// [`span_reach_differential`]'s.
#[test]
fn the_split_rows_serve_where_main_escalated_only_on_their_truth() {
    let mut g = fuzz::start("split_rows_served_where_main_escalated");
    split_rows(
        band(),
        &mut Rng(g.next_u64()),
        fuzz::scaled(300),
        &fuzz::replay(),
    );
}

/// **The sectors stop at `1e12·ε`.** Split #164 of
/// `CAD_FUZZ_SEED=0x344a6e7b2dc22080` at ε = 1e-12, before the cap, was
/// a sector of radius 1e3, 1e3 off the origin, under a plane its
/// normal's 8.7e-16 tilt levers at 0.999·ε exactly across the span's
/// 1143 (0.975·ε as read), so it served coplanar; the sector itself is
/// 1.10·ε off that plane, its rounded frame `x × y` 1.3e-16 off its
/// normal. This replays that stream: `Rng(0x74eb695e87f1661e)` is the
/// one the gated row draws from that seed.
#[test]
fn the_sectors_stop_where_the_tilt_outruns_a_unit_normal() {
    let b = drawn_at();
    split_rows(
        b,
        &mut Rng(0x74eb_695e_87f1_661e),
        165,
        "Rng(0x74eb695e87f1661e)",
    );
}

fn split_rows(b: geom_core::Band, rng: &mut Rng, n: usize, replay: &str) {
    for i in 0..n {
        let s = [1e-3, 1.0, 1e3][i % 3];
        for (family, case) in [
            ("split_cyl", split_cyl_case(rng, b, s)),
            ("split", split_plane_case(rng, b, s)),
        ] {
            assert!(
                !case.short,
                "{family} #{i}: head's lever falls short of the face; {replay}"
            );
            assert!(
                !(case.main == "esc" && case.head != "esc" && case.against(case.head)),
                "{family} #{i}: head serves {} where main escalated, against the truth {}; {replay}",
                case.head,
                case.truth,
            );
        }
    }
}
