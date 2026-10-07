//! Review probes for PR 4256 (not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::common;
use common::meeting::{MEET, PLATE, Pose, posed_box, posed_pyramid, poses};
use geom_core::{Band, Point3, Tol};
use topo::{
    AtRestBody, BooleanError, BooleanResult, SolidContainment, intersect, mass_properties,
    point_in_solid, subtract, union, validate_geometric,
};

fn t() -> Tol {
    Tol::witness()
}

/// A tetrahedron: apex MEET and base corners, all relative to MEET (unposed).
#[derive(Clone)]
pub(crate) struct Tet {
    pub(crate) base: [[f64; 3]; 3],
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn det(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> f64 {
    a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
        + a[2] * (b[0] * c[1] - b[1] * c[0])
}

impl Tet {
    fn corner(bearing: f64, d: f64, r: f64, z: f64) -> [f64; 3] {
        let (s, c) = (bearing + d).to_radians().sin_cos();
        [r * c, r * s, z]
    }
    /// The test file's `pyramid(bearing, rise, r)`.
    pub(crate) fn pyr(bearing: f64, rise: f64, r: f64) -> Self {
        let turn = if rise > 0.0 { 15.0 } else { -15.0 };
        Self {
            base: [
                Self::corner(bearing, turn, r, rise),
                Self::corner(bearing, -turn, r, rise),
                Self::corner(bearing, 0.0, 0.6 * r, rise),
            ],
        }
    }
    /// A tetrahedron nested inside this one's cone, apex shared: each corner
    /// mixes 3:1:1 then scales by `s` (s < 1 keeps it inside).
    pub(crate) fn nest(&self, s: f64) -> Self {
        let b = self.base;
        Self {
            base: [0, 1, 2].map(|i| {
                [0, 1, 2].map(|k| (3.0 * b[i][k] + b[(i + 1) % 3][k] + b[(i + 2) % 3][k]) * s / 5.0)
            }),
        }
    }
    pub(crate) fn body(&self, pose: &Pose) -> AtRestBody<f64> {
        let abs = self.base.map(|c| [0, 1, 2].map(|k| MEET[k] + c[k]));
        // Counterclockwise from the apex: orient by the sign of det.
        let d = det(self.base[0], self.base[1], self.base[2]);
        let base = if d < 0.0 { abs } else { [abs[1], abs[0], abs[2]] };
        posed_pyramid(&base, MEET, pose, t())
    }
    pub(crate) fn volume(&self) -> f64 {
        det(self.base[0], self.base[1], self.base[2]).abs() / 6.0
    }
    /// Signed margin (in barycentric) of an unposed point relative to MEET:
    /// > 0 inside, < 0 outside.
    pub(crate) fn margin(&self, q: [f64; 3]) -> f64 {
        let [a, b, c] = self.base;
        let d = det(a, b, c);
        let la = det(q, b, c) / d;
        let lb = det(a, q, c) / d;
        let lc = det(a, b, q) / d;
        let l0 = 1.0 - la - lb - lc;
        la.min(lb).min(lc).min(l0)
    }
}

pub(crate) fn plate_margin(q: [f64; 3]) -> f64 {
    // q relative to MEET.
    let p = [0, 1, 2].map(|k| q[k] + MEET[k]);
    (0..3)
        .map(|k| (p[k] - PLATE[k].0).min(PLATE[k].1 - p[k]))
        .fold(f64::INFINITY, f64::min)
}

pub(crate) fn built(what: &str, r: Result<BooleanResult<f64>, BooleanError>) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{what}: {:?}", other.map(|_| ())),
    }
}

pub(crate) fn volume(b: &AtRestBody<f64>) -> f64 {
    mass_properties(b, t()).unwrap().volume
}

pub(crate) fn inside(body: &AtRestBody<f64>, q: Point3<f64>) -> Option<bool> {
    let band = Band::linear(t()).unwrap();
    match point_in_solid(body, q, band, t()).unwrap() {
        SolidContainment::In => Some(true),
        SolidContainment::Out => Some(false),
        SolidContainment::OnBoundary => None,
    }
}

/// Fibonacci directions on the sphere at radius r about MEET (unposed).
pub(crate) fn sphere(n: usize, r: f64) -> Vec<[f64; 3]> {
    (0..n)
        .map(|k| {
            let z = 1.0 - (2.0 * k as f64 + 1.0) / n as f64;
            let (s, c) = (2.399_963 * k as f64).sin_cos();
            let rr = z.mul_add(-z, 1.0).sqrt();
            [r * rr * c, r * rr * s, r * z]
        })
        .collect()
}

/// Compare each op's result against an analytic membership.
/// `x`/`y` margins: >0 in, <0 out (unposed, rel MEET). Returns a list of failures.
pub(crate) fn check_ops(
    label: &str,
    pose: &Pose,
    x: &AtRestBody<f64>,
    y: &AtRestBody<f64>,
    xm: &dyn Fn([f64; 3]) -> f64,
    ym: &dyn Fn([f64; 3]) -> f64,
    vols: Option<(f64, f64, f64)>, // |x|, |y|, |x∩y| analytic
) -> Vec<String> {
    let mut bad = Vec::new();
    let mut pts = sphere(600, 0.03);
    pts.extend(sphere(600, 0.1));
    let ops: [(&str, fn(bool, bool) -> bool, bool); 6] = [
        ("x − y", |a, b| a && !b, false),
        ("y − x", |a, b| b && !a, true),
        ("x ∪ y", |a, b| a || b, false),
        ("y ∪ x", |a, b| a || b, true),
        ("x ∩ y", |a, b| a && b, false),
        ("y ∩ x", |a, b| a && b, true),
    ];
    for (name, keep, swap) in ops {
        let (a, b) = if swap { (y, x) } else { (x, y) };
        let r = match name {
            n if n.contains('−') => subtract(a, b, t()),
            n if n.contains('∪') => union(a, b, t()),
            _ => intersect(a, b, t()),
        };
        let what = format!("{label}, {}, {name}", pose.label);
        let body = match r {
            Ok(BooleanResult::Body(r)) => {
                if pose.label == "at rest" {
                    let (am, bm): (&dyn Fn([f64; 3]) -> f64, &dyn Fn([f64; 3]) -> f64) =
                        if swap { (ym, xm) } else { (xm, ym) };
                    bad.extend(naming_check(&what, &r, a, b, am, bm));
                }
                if let Err(e) = validate_geometric(&r.body, t()) {
                    bad.push(format!("{what}: tier 3 {e:?}"));
                }
                Some(r.body)
            }
            Ok(BooleanResult::Empty) => None,
            Err(e) => {
                bad.push(format!("{what}: REFUSED {e:?}"));
                continue;
            }
        };
        let mut wrong = 0;
        let mut first = None;
        for q in &pts {
            let (mx, my) = (xm(*q), ym(*q));
            if mx.abs() < 1e-6 || my.abs() < 1e-6 {
                continue;
            }
            let want = keep(mx > 0.0, my > 0.0);
            let p = pose.at([0, 1, 2].map(|k| q[k] + MEET[k]));
            let got = body.as_ref().map_or(Some(false), |b| inside(b, p));
            if let Some(g) = got {
                if g != want {
                    wrong += 1;
                    first.get_or_insert((*q, want, g));
                }
            }
        }
        if wrong > 0 {
            bad.push(format!("{what}: {wrong} WRONG material points, first {first:?}"));
        }
        if let Some((vx, vy, vi)) = vols {
            let want = match name {
                "x − y" => vx - vi,
                "y − x" => vy - vi,
                n if n.contains('∪') => vx + vy - vi,
                _ => vi,
            };
            let got = body.as_ref().map_or(0.0, volume);
            if (got - want).abs() > 1e-9 {
                bad.push(format!("{what}: WRONG volume {got}, want {want}"));
            }
        }
    }
    bad
}

fn report(all: &[String]) {
    for b in all {
        eprintln!("PROBE: {b}");
    }
}

/// In-side nest: cavity K1 (void), K2 ⊂ K1 united back as an island,
/// then K3 ⊂ K2 against it.
#[test]
fn probe_in_side_nest() {
    let mut all = Vec::new();
    for pose in poses() {
        let k1 = Tet::pyr(120.0, -0.5, 0.4);
        let k2 = k1.nest(0.7);
        let k3 = k2.nest(0.7);
        let plate = posed_box("plate", PLATE, &pose, t());
        let cavity = built("cavity", subtract(&plate, &k1.body(&pose), t()));
        let xr = union(&cavity, &k2.body(&pose), t());
        let x = match xr {
            Ok(BooleanResult::Body(r)) => r.body,
            other => {
                all.push(format!("{}: X' = cavity ∪ K2 did not build: {:?}", pose.label, other.map(|_| ())));
                continue;
            }
        };
        let (k1c, k2c, k3c) = (k1.clone(), k2.clone(), k3.clone());
        let xm = move |q: [f64; 3]| {
            let p = plate_margin(q);
            let m1 = k1c.margin(q);
            let m2 = k2c.margin(q);
            // (plate \ K1) ∪ K2
            p.min(-m1).max(m2)
        };
        // Check X' itself against analytic.
        let pv = 3.0 * 2.0 * 1.0;
        let vx = pv - k1.volume() + k2.volume();
        let got = volume(&x);
        if (got - vx).abs() > 1e-9 {
            all.push(format!("{}: X' volume {got} want {vx}", pose.label));
        }
        let ym = move |q: [f64; 3]| k3c.margin(q);
        all.extend(check_ops("K3 inside island K2 inside void K1", &pose, &x, &k3.body(&pose), &xm, &ym,
            Some((vx, k3.volume(), k3.volume()))));
        // A probe between: K3' in K1 \ K2? use a nest of K1 at the other mix (2 corners)
        eprintln!("{}: X' vertices {}", pose.label, x.vertices().count());
    }
    report(&all);
    assert!(all.is_empty(), "{} failures", all.len());
}

/// Out-side nest: plate ∪ M1 (arch), less M2 ⊂ M1, then M3 ⊂ M2 against it.
#[test]
fn probe_out_side_nest() {
    let mut all = Vec::new();
    for pose in poses() {
        let m1 = Tet::pyr(60.0, 0.5, 0.4);
        let m2 = m1.nest(0.7);
        let m3 = m2.nest(0.7);
        let plate = posed_box("plate", PLATE, &pose, t());
        let one = built("one", union(&plate, &m1.body(&pose), t()));
        let x = match subtract(&one, &m2.body(&pose), t()) {
            Ok(BooleanResult::Body(r)) => r.body,
            other => {
                all.push(format!("{}: X'' = one − M2 did not build: {:?}", pose.label, other.map(|_| ())));
                continue;
            }
        };
        let (a, b, c) = (m1.clone(), m2.clone(), m3.clone());
        let xm = move |q: [f64; 3]| plate_margin(q).max(a.margin(q).min(-b.margin(q)));
        let vx = 6.0 + m1.volume() - m2.volume();
        let got = volume(&x);
        if (got - vx).abs() > 1e-9 {
            all.push(format!("{}: X'' volume {got} want {vx}", pose.label));
        }
        let ym = move |q: [f64; 3]| c.margin(q);
        all.extend(check_ops("M3 inside void M2 inside arch M1", &pose, &x, &m3.body(&pose), &xm, &ym,
            Some((vx, m3.volume(), 0.0))));
        eprintln!("{}: X'' vertices {}", pose.label, x.vertices().count());
    }
    report(&all);
    assert!(all.is_empty(), "{} failures", all.len());
}

/// A membership: a tree of analytic solids.
#[derive(Clone)]
pub(crate) enum M {
    Plate,
    T(Tet),
    U(Box<M>, Box<M>),
    D(Box<M>, Box<M>),
}
impl M {
    fn m(&self, q: [f64; 3]) -> f64 {
        match self {
            M::Plate => plate_margin(q),
            M::T(t) => t.margin(q),
            M::U(a, b) => a.m(q).max(b.m(q)),
            M::D(a, b) => a.m(q).min(-b.m(q)),
        }
    }
    fn build(&self, pose: &Pose) -> Result<AtRestBody<f64>, String> {
        Ok(match self {
            M::Plate => posed_box("plate", PLATE, pose, t()),
            M::T(t) => t.body(pose),
            M::U(a, b) => match union(&a.build(pose)?, &b.build(pose)?, t()) {
                Ok(BooleanResult::Body(r)) => r.body,
                o => return Err(format!("{:?}", o.map(|_| ()))),
            },
            M::D(a, b) => match subtract(&a.build(pose)?, &b.build(pose)?, t()) {
                Ok(BooleanResult::Body(r)) => r.body,
                o => return Err(format!("{:?}", o.map(|_| ()))),
            },
        })
    }
}
fn u(a: M, b: M) -> M {
    M::U(Box::new(a), Box::new(b))
}
fn d(a: M, b: M) -> M {
    M::D(Box::new(a), Box::new(b))
}
fn tt(t: Tet) -> M {
    M::T(t)
}

fn sweep(scenes: &[(&str, M, M)]) {
    let mut all = Vec::new();
    let mut cells = 0;
    for pose in poses() {
        for (label, xm, ym) in scenes {
            let (x, y) = match (xm.build(&pose), ym.build(&pose)) {
                (Ok(x), Ok(y)) => (x, y),
                (a, b) => {
                    all.push(format!("{label}, {}: operand did not build {:?} {:?}", pose.label, a.err(), b.err()));
                    continue;
                }
            };
            let (xa, ya) = (xm.clone(), ym.clone());
            cells += 6;
            all.extend(check_ops(label, &pose, &x, &y, &move |q| xa.m(q), &move |q| ya.m(q), None));
        }
    }
    report(&all);
    eprintln!("PROBE cells {cells}, findings {}", all.len());
    let wrong: Vec<_> = all.iter().filter(|s| s.contains("WRONG")).collect();
    assert!(wrong.is_empty(), "{} wrong", wrong.len());
}

fn arches() -> Vec<Tet> {
    [60.0, 180.0, 300.0].map(|b| Tet::pyr(b, 0.5, 0.4)).to_vec()
}

/// The PR's built scenes against the analytic oracle.
#[test]
fn probe_pr_scenes_analytic() {
    let plate = M::Plate;
    let a = arches();
    let bare = u(u(tt(a[0].clone()), tt(a[1].clone())), tt(a[2].clone()));
    let arches_m = u(plate.clone(), bare.clone());
    let one = u(plate.clone(), tt(a[0].clone()));
    let k1 = Tet::pyr(120.0, -0.5, 0.4);
    let cavity = d(plate.clone(), tt(k1.clone()));
    let cone = tt(Tet::pyr(240.0, 0.7, 0.5));
    let over = tt(Tet::pyr(50.0, 0.7, 0.5));
    let hang = tt(Tet::pyr(240.0, -0.6, 0.5));
    let hang_over = tt(Tet::pyr(130.0, -0.6, 0.5));
    let in_void = tt(k1.nest(1.4));
    sweep(&[
        ("the arches", cone.clone(), arches_m.clone()),
        ("one standing pyramid", cone.clone(), one.clone()),
        ("over the arch", over.clone(), one.clone()),
        ("over the arches", over.clone(), arches_m.clone()),
        ("hanging below the arch", hang.clone(), one.clone()),
        ("hanging across below the arch", hang_over.clone(), one.clone()),
        ("hanging below the arches", hang.clone(), arches_m.clone()),
        ("standing over the cavity", cone.clone(), cavity.clone()),
        ("hanging below the cavity", hang.clone(), cavity.clone()),
        ("hanging across the cavity", hang_over.clone(), cavity.clone()),
        ("hanging into the void", in_void.clone(), cavity.clone()),
        ("standing on the plate", cone.clone(), plate.clone()),
        ("hanging in the plate", hang.clone(), plate.clone()),
        ("the bare arches", cone.clone(), bare.clone()),
        ("hanging below the bare arches", hang.clone(), bare.clone()),
    ]);
}

/// New scenes.
#[test]
fn probe_new_scenes_analytic() {
    let plate = M::Plate;
    let a = arches();
    let m1 = a[0].clone();
    let one = u(plate.clone(), tt(m1.clone()));
    let k1 = Tet::pyr(120.0, -0.5, 0.4);
    // Both sides: the plate with an arch above and a void below.
    let both = d(one.clone(), tt(k1.clone()));
    let over = tt(Tet::pyr(50.0, 0.7, 0.5));
    let in_arch = tt(m1.nest(0.7));
    let in_arch_deep = tt(m1.nest(1.6));
    let hang_over = tt(Tet::pyr(130.0, -0.6, 0.5));
    let in_void = tt(k1.nest(0.7));
    let in_void_deep = tt(k1.nest(1.4));
    // Non-convex touchers: two standing pyramids at one apex; a standing
    // pyramid with a nested void at its apex.
    let two_up = u(tt(Tet::pyr(200.0, 0.6, 0.5)), tt(Tet::pyr(280.0, 0.6, 0.5)));
    let two_up_over = u(tt(Tet::pyr(40.0, 0.6, 0.5)), tt(Tet::pyr(280.0, 0.6, 0.5)));
    let big = Tet::pyr(60.0, 0.9, 0.7);
    let hollow_up = d(tt(big.clone()), tt(big.nest(0.6)));
    let two_down = u(tt(Tet::pyr(200.0, -0.6, 0.5)), tt(Tet::pyr(110.0, -0.6, 0.5)));
    // Two arches on the plate, one nested-overlapping region test: arch
    // containing the toucher partially.
    sweep(&[
        ("both sides, over the arch", over.clone(), both.clone()),
        ("both sides, inside the arch", in_arch.clone(), both.clone()),
        ("both sides, through the arch", in_arch_deep.clone(), both.clone()),
        ("both sides, hanging across the void", hang_over.clone(), both.clone()),
        ("both sides, in the void", in_void.clone(), both.clone()),
        ("both sides, through the void", in_void_deep.clone(), both.clone()),
        ("inside the arch", in_arch.clone(), one.clone()),
        ("through the arch", in_arch_deep.clone(), one.clone()),
        ("two up beside the arch", two_up.clone(), one.clone()),
        ("two up, one over the arch", two_up_over.clone(), one.clone()),
        ("hollow up around the arch", hollow_up.clone(), one.clone()),
        ("two down beside the void", two_down.clone(), both.clone()),
        ("two up vs both", two_up_over.clone(), both.clone()),
        ("hollow up vs both", hollow_up.clone(), both.clone()),
    ]);
}

/// Each naming class at MEET (at rest) against the analytic other operand.
fn naming_check(
    what: &str,
    r: &topo::BooleanBody<f64>,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    am: &dyn Fn([f64; 3]) -> f64,
    bm: &dyn Fn([f64; 3]) -> f64,
) -> Vec<String> {
    use topo::{Operand, SideCode, readback};
    let mut bad = Vec::new();
    let mut seen: std::collections::BTreeMap<String, Vec<SideCode>> = Default::default();
    for row in &r.naming.edge_classes {
        let (own, om): (_, &dyn Fn([f64; 3]) -> f64) = match row.operand {
            Operand::A => (a, bm),
            Operand::B => (b, am),
        };
        let Ok(p) = readback::vertex_point(own, row.vertex) else { continue };
        let pm = [p.x - MEET[0], p.y - MEET[1], p.z - MEET[2]];
        if pm.iter().any(|c| c.abs() > 1e-9) {
            continue;
        }
        let edge = own.get_edge(row.edge).unwrap();
        let plus = own.get_half_edge(edge.he_plus).unwrap();
        let far = if row.starts { own.get_half_edge(plus.next).unwrap().start } else { plus.start };
        let fp = readback::vertex_point(own, far).unwrap();
        let dd = [fp.x - p.x, fp.y - p.y, fp.z - p.z];
        let n = (dd[0] * dd[0] + dd[1] * dd[1] + dd[2] * dd[2]).sqrt();
        let q = [0, 1, 2].map(|k| pm[k] + dd[k] * 0.01 / n);
        let m = om(q);
        let want = if m > 1e-7 { SideCode::In } else if m < -1e-7 { SideCode::Out } else { SideCode::On };
        seen.entry(format!("{:?}{:?}{:?}", row.operand, row.edge, row.starts)).or_default().push(row.class);
        if row.class != want {
            bad.push(format!("{what}: WRONG naming class {row:?}, oracle {want:?}"));
        }
    }
    for (k, v) in seen {
        if v.len() > 1 && v.iter().any(|c| *c != v[0]) {
            bad.push(format!("{what}: CONFLICTING naming rows for {k}: {v:?}"));
        }
    }
    bad
}

/// Is the two-lump conflict pre-existing? No plate: pure VV pairs.
#[test]
fn probe_two_lumps_vs_bare_arch() {
    let m1 = arches()[0].clone();
    let two_up_over = u(tt(Tet::pyr(40.0, 0.6, 0.5)), tt(Tet::pyr(280.0, 0.6, 0.5)));
    let two_down = u(tt(Tet::pyr(200.0, -0.6, 0.5)), tt(Tet::pyr(110.0, -0.6, 0.5)));
    let one = u(M::Plate, tt(m1.clone()));
    let mut all = Vec::new();
    let pose = Pose::rest();
    for (label, xm, ym) in [
        ("two up vs the bare arch", two_up_over.clone(), tt(m1.clone())),
        ("two up vs one", two_up_over.clone(), one.clone()),
        ("two down vs plate", two_down.clone(), M::Plate),
    ] {
        let (x, y) = (xm.build(&pose).unwrap(), ym.build(&pose).unwrap());
        all.extend(check_ops(label, &pose, &x, &y, &move |q| xm.m(q), &move |q| ym.m(q), None));
    }
    report(&all);
}

/// A partner whose link touches the face along an edge: a tetrahedron
/// with apex at MEET and one base corner on the plate's top.
#[test]
fn probe_edge_partner() {
    let lie = Tet {
        base: [
            Tet::corner(60.0, 20.0, 0.4, 0.5),
            Tet::corner(60.0, -20.0, 0.4, 0.5),
            Tet::corner(60.0, 0.0, 0.5, 0.0),
        ],
    };
    let mut all = Vec::new();
    for pose in poses() {
        let xm = u(M::Plate, tt(lie.clone()));
        let x = match xm.build(&pose) {
            Ok(x) => x,
            Err(e) => {
                all.push(format!("{}: plate ∪ lying tet did not build: {e}", pose.label));
                continue;
            }
        };
        eprintln!("{}: X vertices {}", pose.label, x.vertices().count());
        for (label, ym) in [
            ("cone beside", tt(Tet::pyr(240.0, 0.7, 0.5))),
            ("over the lying tet", tt(Tet::pyr(50.0, 0.7, 0.5))),
            ("hanging", tt(Tet::pyr(240.0, -0.6, 0.5))),
        ] {
            let y = ym.build(&pose).unwrap();
            let xa = xm.clone();
            all.extend(check_ops(label, &pose, &y, &x, &move |q| ym.m(q), &move |q| xa.m(q), None));
        }
    }
    report(&all);
}

/// Panic grid: prisms (strut hangers) and pyramids against every
/// multi-contact operand; counts outcomes and panics; volume sums.
#[test]
fn probe_panic_grid() {
    use common::meeting::{leaned, posed_boxes, posed_prism, wedge};
    use std::collections::BTreeMap;
    let a = arches();
    let m1 = a[0].clone();
    let one = u(M::Plate, tt(m1.clone()));
    let k1 = Tet::pyr(120.0, -0.5, 0.4);
    let lie = Tet {
        base: [
            Tet::corner(60.0, 20.0, 0.4, 0.5),
            Tet::corner(60.0, -20.0, 0.4, 0.5),
            Tet::corner(60.0, 0.0, 0.5, 0.0),
        ],
    };
    let ops: Vec<(&str, M)> = vec![
        ("one", one.clone()),
        ("arches", u(M::Plate, u(u(tt(a[0].clone()), tt(a[1].clone())), tt(a[2].clone())))),
        ("cavity", d(M::Plate, tt(k1.clone()))),
        ("both", d(one.clone(), tt(k1.clone()))),
        ("bare", u(u(tt(a[0].clone()), tt(a[1].clone())), tt(a[2].clone()))),
        ("lie", u(M::Plate, tt(lie))),
        ("island", u(d(M::Plate, tt(k1.clone())), tt(k1.nest(0.7)))),
    ];
    let mut tally: BTreeMap<String, usize> = BTreeMap::new();
    let mut bad = Vec::new();
    let all_poses = poses();
    for pi in [0usize, 2, 4] {
        let pose = &all_poses[pi];
        let mut xs: Vec<(String, AtRestBody<f64>, Option<M>)> = Vec::new();
        for a0 in (0..8).map(|i| f64::from(i) * 45.0) {
            for w in [40.0, 90.0] {
                for k in [0usize, 2] {
                    xs.push((format!("wedge {a0}+{w} k{k}"), posed_prism(&wedge(a0, a0 + w, k), pose, t()), None));
                    xs.push((format!("leaned {a0}+{w} k{k}"), posed_prism(&leaned(a0, a0 + w, k, a0 + 180.0), pose, t()), None));
                }
            }
        }
        for b in (0..12).map(|i| f64::from(i) * 30.0 + 7.0) {
            for rise in [0.7, -0.6] {
                let tm = tt(Tet::pyr(b, rise, 0.5));
                xs.push((format!("pyr {b} {rise}"), tm.build(pose).unwrap(), Some(tm)));
            }
        }
        let mut ys: Vec<(String, AtRestBody<f64>, Option<M>)> = ops
            .iter()
            .map(|(n, m)| (n.to_string(), m.build(pose).unwrap(), Some(m.clone())))
            .collect();
        ys.push((
            "blocks".into(),
            posed_boxes("blocks", &[PLATE, [(0.5, 2.5), (0.5, 1.5), (1.0, 1.5)]], pose, t()),
            None,
        ));
        for (xl, x, xm) in &xs {
            for (yl, y, ym) in &ys {
                let what = format!("{xl} vs {yl}, {}", pose.label);
                let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut out = Vec::new();
                    for (name, r) in [
                        ("x−y", subtract(x, y, t())),
                        ("y−x", subtract(y, x, t())),
                        ("x∪y", union(x, y, t())),
                        ("y∪x", union(y, x, t())),
                        ("x∩y", intersect(x, y, t())),
                        ("y∩x", intersect(y, x, t())),
                    ] {
                        out.push((name, match r {
                            Ok(BooleanResult::Body(r)) => {
                                let ok = validate_geometric(&r.body, t()).is_ok();
                                (if ok { "body".to_string() } else { "BODY-NOT-TIER3".to_string() }, Some(volume(&r.body)))
                            }
                            Ok(BooleanResult::Empty) => ("empty".to_string(), Some(0.0)),
                            Err(e) => (format!("{e:?}").split([' ', '{', '(']).next().unwrap().to_string(), None),
                        }));
                    }
                    out
                }));
                match res {
                    Err(_) => {
                        *tally.entry("PANIC".into()).or_default() += 1;
                        bad.push(format!("PANIC {what}"));
                    }
                    Ok(out) => {
                        for (_, (k, _)) in &out {
                            *tally.entry(k.clone()).or_default() += 1;
                            if k.contains("NOT-TIER3") {
                                bad.push(format!("{k} {what}"));
                            }
                        }
                        let v: Vec<Option<f64>> = out.iter().map(|(_, (_, v))| *v).collect();
                        if let [Some(xy), Some(yx), Some(u1), Some(u2), Some(i1), Some(i2)] = v[..] {
                            let (vx, vy) = (volume(x), volume(y));
                            for (s, w, n) in [(u1 + i1, vx + vy, "u+i"), (xy + i1, vx, "x-y+i"), (yx + i1, vy, "y-x+i"), (u2, u1, "u2"), (i2, i1, "i2")] {
                                if (s - w).abs() > 1e-9 {
                                    bad.push(format!("VOLUME {what}: {n} {s} want {w}"));
                                }
                            }
                        }
                        if let (Some(xm), Some(ym)) = (xm, ym) {
                            if pi != 4 && out.iter().all(|(_, (_, v))| v.is_some()) {
                                let (xa, ya) = (xm.clone(), ym.clone());
                                bad.extend(check_ops(&what, pose, x, y, &move |q| xa.m(q), &move |q| ya.m(q), None)
                                    .into_iter().filter(|s| s.contains("WRONG material") || s.contains("WRONG volume")));
                            }
                        }
                    }
                }
            }
        }
    }
    eprintln!("PROBE tally {tally:?}");
    report(&bad);
    assert!(!tally.contains_key("PANIC"));
}

/// Y cones crossing the inner boundary of a nest at MEET.
#[test]
fn probe_nest_crossers() {
    let k1 = Tet::pyr(120.0, -0.5, 0.4);
    let m1 = arches()[0].clone();
    let island = u(d(M::Plate, tt(k1.clone())), tt(k1.nest(0.7)));
    let hollow_arch = d(u(M::Plate, tt(m1.clone())), tt(m1.nest(0.7)));
    let mut scenes = Vec::new();
    for db in [-12.0, 0.0, 9.0] {
        for r in [0.12, 0.2, 0.3] {
            scenes.push((format!("down {db} {r} vs island"), tt(Tet::pyr(120.0 + db, -0.45, r)), island.clone()));
            scenes.push((format!("up {db} {r} vs hollow arch"), tt(Tet::pyr(60.0 + db, 0.45, r)), hollow_arch.clone()));
        }
    }
    let s: Vec<(&str, M, M)> = scenes.iter().map(|(l, a, b)| (l.as_str(), a.clone(), b.clone())).collect();
    sweep(&s);
}
