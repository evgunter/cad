//! R1 review probes for PR 6345 (cone apex B1, vertex–vertex). Not for
//! merge: each probe runs the four ops on a pose whose operands meet at a
//! cone's apex, and holds each result to an independent oracle — the
//! directional integral `∫ ⅓ min(r₁, r₂)³ dΩ` of both operands' reaches
//! from the shared apex (both are star-shaped from it), and point
//! membership by ray parity over the result's tessellation, never the
//! kernel's classification. A typed refusal is logged, not failed; a
//! wrong volume, a wrong lattice point or a panic is a failure.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::wrap_tip::{NORTH, one_wrap_edge};
use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, BooleanResult, intersect, subtract, union};

fn tol() -> Tol {
    Tol::witness()
}

type Reach = Rc<dyn Fn(Vec3<f64>) -> f64>;
type Region = Rc<dyn Fn(Point3<f64>) -> f64>;

fn cone_body(alpha: f64) -> topo::Body<f64> {
    let r = alpha.to_radians().tan();
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

fn form(alpha: f64, tipped: bool) -> topo::Body<f64> {
    if tipped {
        one_wrap_edge(cone_body(alpha), true).0
    } else {
        crate::revolve_common::needle(alpha, None, tol()).into_body()
    }
}

fn moved(body: &topo::Body<f64>, m: Affine3<f64>) -> AtRestBody<f64> {
    finished(
        "moved",
        topo::transform_rigid(body, &geom_core::test_support::rigid(&m, tol()), tol()).unwrap(),
        tol(),
    )
}

/// A cone of half-angle `alpha` (deg), height 1, apex NORTH, axis `m·(−y)`.
fn cone_reach(alpha: f64, m: Affine3<f64>) -> Reach {
    let a = m.transform_vec(Vec3::new(0.0, -1.0, 0.0)).normalize();
    let c = alpha.to_radians().cos();
    Rc::new(move |d: Vec3<f64>| {
        let k = d.dot(a);
        if k >= c { 1.0 / k } else { 0.0 }
    })
}

fn cone_region(alpha: f64, m: Affine3<f64>) -> Region {
    let r = alpha.to_radians().tan();
    let back = m.inverse();
    Rc::new(move |p: Point3<f64>| {
        let p = back.transform_point(p);
        (r * (1.0 - p.y) - p.x.hypot(p.z)).min(p.y).min(1.0 - p.y)
    })
}

/// Box `[0,l]` edges from NORTH along `m`'s images of x, y, z.
fn box_reach(l: [f64; 3], m: Affine3<f64>) -> Reach {
    let e = [
        m.transform_vec(Vec3::new(1.0, 0.0, 0.0)),
        m.transform_vec(Vec3::new(0.0, 1.0, 0.0)),
        m.transform_vec(Vec3::new(0.0, 0.0, 1.0)),
    ];
    Rc::new(move |d: Vec3<f64>| {
        let mut r = f64::INFINITY;
        for i in 0..3 {
            let k = d.dot(e[i]);
            if k < 0.0 {
                return 0.0;
            }
            if k > 0.0 {
                r = r.min(l[i] / k);
            }
        }
        r
    })
}

fn box_region(l: [f64; 3], m: Affine3<f64>) -> Region {
    let back = m.inverse();
    Rc::new(move |p: Point3<f64>| {
        let q = back.transform_point(p) - NORTH;
        let q = [q.x, q.y, q.z];
        (0..3).map(|i| q[i].min(l[i] - q[i])).fold(f64::INFINITY, f64::min)
    })
}

/// Box `[0,l]` with its corner at NORTH, its edges along `m`'s axes.
fn box_body(l: [f64; 3], m: Affine3<f64>) -> AtRestBody<f64> {
    let b = topo::test_support::brick((0.0, l[0]), (0.0, l[1]), (0.0, l[2]), tol());
    moved(&b, m * Affine3::translation(NORTH - Point3::origin()))
}

/// `∫ ⅓ min(ra, rb)³ dΩ` over the cap of directions within `half` of `axis`
/// (which must hold `ra`'s support).
fn shared(axis: Vec3<f64>, half: f64, ra: &Reach, rb: &Reach) -> f64 {
    let n = 1600;
    let (dt, dp) = (half / f64::from(n), 2.0 * PI / f64::from(n));
    let u = if axis.x.abs() < 0.9 { Vec3::new(1.0, 0.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) };
    let u = (u - axis * u.dot(axis)).normalize();
    let v = axis.cross(u);
    let mut sum = 0.0;
    for i in 0..n {
        let t = (f64::from(i) + 0.5) * dt;
        let (st, ct) = t.sin_cos();
        for j in 0..n {
            let p = (f64::from(j) + 0.5) * dp;
            let d = axis * ct + (u * p.cos() + v * p.sin()) * st;
            let r = ra(d).min(rb(d));
            if r > 0.0 && r.is_finite() {
                sum += r.powi(3) / 3.0 * st * dt * dp;
            }
        }
    }
    sum
}

/// Ray parity of `p` against the mesh's triangles.
fn parity(tris: &[[Point3<f64>; 3]], p: Point3<f64>) -> bool {
    let dir = Vec3::new(0.5377, 0.8341, 0.1234).normalize();
    let mut hits = 0;
    for [a, b, c] in tris {
        let (e1, e2) = (*b - *a, *c - *a);
        let h = dir.cross(e2);
        let det = e1.dot(h);
        if det.abs() < 1e-14 {
            continue;
        }
        let s = p - *a;
        let uu = s.dot(h) / det;
        if !(0.0..=1.0).contains(&uu) {
            continue;
        }
        let q = s.cross(e1);
        let vv = dir.dot(q) / det;
        if vv < 0.0 || uu + vv > 1.0 {
            continue;
        }
        if e2.dot(q) / det > 0.0 {
            hits += 1;
        }
    }
    hits % 2 == 1
}

thread_local! { static KERNEL_MEMBERSHIP: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

struct Case {
    name: String,
    a: AtRestBody<f64>,
    b: AtRestBody<f64>,
    ra: Region,
    rb: Region,
    va: f64,
    vb: f64,
    share: f64,
    vtol: f64,
    lattice: (Point3<f64>, Point3<f64>),
}

#[derive(Default)]
struct Tally {
    built: usize,
    refused: Vec<String>,
    failed: Vec<String>,
}

fn check_body(what: &str, body: &AtRestBody<f64>, want: f64, vtol: f64, region: &Region, lattice: (Point3<f64>, Point3<f64>)) -> Result<(), String> {
    let mp = topo::mass_properties(body, tol(), topo::test_support::pinned_resolution(tol()))
        .map_err(|e| format!("{what}: mass properties {e:?}"))?;
    let v = mp.met().map_err(|e| format!("{what}: volume unmet {e}"))?.volume;
    if (v - want).abs() > vtol {
        return Err(format!("{what}: WRONG volume {v} against oracle {want} (tol {vtol})"));
    }
    if let Err(e) = topo::validate_geometric(body, tol()) {
        return Err(format!("{what}: tier 3 {e:?}"));
    }
    // Independent membership by mesh parity where the tessellator covers
    // the result; the kernel's point classification otherwise (counted).
    let tris: Option<Vec<[Point3<f64>; 3]>> = mesh::tessellate(body, 2e-3, tol()).ok().map(|mesh| {
        mesh.patches
            .iter()
            .flat_map(|p| p.triangles.iter())
            .map(|t| t.map(|i| mesh.positions[i as usize]))
            .collect()
    });
    if tris.is_none() {
        KERNEL_MEMBERSHIP.with(|c| c.set(c.get() + 1));
    }
    let band = geom_core::Band::linear(tol()).unwrap();
    let inside = |p: Point3<f64>| match &tris {
        Some(t) => parity(t, p),
        None => topo::point_in_solid(body, p, band, tol()) == Ok(topo::SolidContainment::In),
    };
    let (lo, hi) = lattice;
    let span = hi - lo;
    let n = 9;
    let mut wrong = Vec::new();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let f = |t: usize, o: f64| (t as f64 + o) / (n as f64 - 1.0 + 0.37);
                let p = Point3::new(lo.x + span.x * f(i, 0.11), lo.y + span.y * f(j, 0.23), lo.z + span.z * f(k, 0.07));
                let s = region(p);
                if s.abs() < 0.02 {
                    continue;
                }
                if inside(p) != (s > 0.0) {
                    wrong.push((p, s));
                }
            }
        }
    }
    if !wrong.is_empty() {
        return Err(format!("{what}: WRONG membership at {} points, first {:?}", wrong.len(), wrong[0]));
    }
    Ok(())
}

fn run(case: &Case, tally: &mut Tally) {
    let (ra, rb) = (case.ra.clone(), case.rb.clone());
    let both: Region = { let (x, y) = (ra.clone(), rb.clone()); Rc::new(move |p| x(p).min(y(p))) };
    let either: Region = { let (x, y) = (ra.clone(), rb.clone()); Rc::new(move |p| x(p).max(y(p))) };
    let a_less: Region = { let (x, y) = (ra.clone(), rb.clone()); Rc::new(move |p| x(p).min(-y(p))) };
    let b_less: Region = { let (x, y) = (ra.clone(), rb.clone()); Rc::new(move |p| y(p).min(-x(p))) };
    let ops: [(&str, &AtRestBody<f64>, &AtRestBody<f64>, u8, f64, Region); 6] = [
        ("a∩b", &case.a, &case.b, 0, case.share, both.clone()),
        ("b∩a", &case.b, &case.a, 0, case.share, both),
        ("a∪b", &case.a, &case.b, 1, case.va + case.vb - case.share, either.clone()),
        ("b∪a", &case.b, &case.a, 1, case.va + case.vb - case.share, either),
        ("a∖b", &case.a, &case.b, 2, case.va - case.share, a_less),
        ("b∖a", &case.b, &case.a, 2, case.vb - case.share, b_less),
    ];
    for (op, x, y, kind, want, region) in ops {
        let what = format!("{} {op}", case.name);
        let got = catch_unwind(AssertUnwindSafe(|| match kind {
            0 => intersect(x, y, tol()),
            1 => union(x, y, tol()),
            _ => subtract(x, y, tol()),
        }));
        match got {
            Err(p) => {
                let msg = p.downcast_ref::<String>().cloned().or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default();
                tally.failed.push(format!("{what}: PANIC {msg}"));
            }
            Ok(Err(e)) => tally.refused.push(format!("{what}: {e:?}")),
            Ok(Ok(BooleanResult::Empty)) => {
                if want.abs() > case.vtol {
                    tally.failed.push(format!("{what}: WRONG empty, oracle {want}"));
                } else {
                    tally.built += 1;
                }
            }
            Ok(Ok(BooleanResult::Body(b))) => {
                match catch_unwind(AssertUnwindSafe(|| check_body(&what, &b.body, want, case.vtol, &region, case.lattice))) {
                    Ok(Ok(())) => tally.built += 1,
                    Ok(Err(e)) => tally.failed.push(e),
                    Err(_) => tally.failed.push(format!("{what}: PANIC in the check")),
                }
            }
        }
    }
}

fn report(name: &str, tally: &Tally) {
    eprintln!("=== {name} @ eps {}: built {} refused {} failed {} (kernel-membership fallbacks {})", tol().eps(), tally.built, tally.refused.len(), tally.failed.len(), KERNEL_MEMBERSHIP.with(std::cell::Cell::get));
    for r in &tally.refused {
        eprintln!("  REFUSED {r}");
    }
    for f in &tally.failed {
        eprintln!("  FAILED {f}");
    }
}

fn down() -> Vec3<f64> {
    Vec3::new(0.0, -1.0, 0.0)
}

fn rot(axis: Vec3<f64>, angle: f64) -> Affine3<f64> {
    Affine3::rotation_about_axis(NORTH, axis.normalize(), angle)
}

fn cone_volume(alpha: f64) -> f64 {
    PI * alpha.to_radians().tan().powi(2) / 3.0
}

/// Two cones at one apex: `alpha` against `beta` with `beta`'s turned by
/// `m` about the apex.
fn cone_pair(name: String, alpha: f64, beta: f64, m: Affine3<f64>, tipped: (bool, bool)) -> Case {
    let a = finished("a", form(alpha, tipped.0), tol());
    let b = moved(&form(beta, tipped.1), m);
    let (rca, rcb) = (cone_reach(alpha, Affine3::identity()), cone_reach(beta, m));
    let share = shared(down(), alpha.to_radians(), &rca, &rcb);
    let ext = 1.0f64.max(alpha.to_radians().tan()).max(beta.to_radians().tan()) + 0.2;
    Case {
        name,
        a,
        b,
        ra: cone_region(alpha, Affine3::identity()),
        rb: cone_region(beta, m),
        va: cone_volume(alpha),
        vb: cone_volume(beta),
        share,
        vtol: 3e-3 * cone_volume(alpha).max(cone_volume(beta)).max(1.0),
        lattice: (Point3::new(-ext, 1.0 - ext, -ext), Point3::new(ext, 1.0 + ext, ext)),
    }
}

/// Probe 1: two cones at one apex, equal and unequal half-angles,
/// coaxial (same and opposite nappes), tilted, crossing.
#[test]
#[ignore = "R1 probe"]
fn r1_two_cones_at_one_apex() {
    let mut tally = Tally::default();
    let angles = [10.0f64, 30.0, 45.0, 60.0, 80.0];
    let poses: Vec<(&str, Affine3<f64>)> = vec![
        ("copy", Affine3::identity()),
        ("over", rot(Vec3::new(1.0, 0.0, 0.0), PI)),
        ("tilt.25", rot(Vec3::new(0.0, 0.0, 1.0), 0.25)),
        ("tilt.7skew", rot(Vec3::new(0.3, 0.1, 1.0), 0.7)),
        ("tilt1.3", rot(Vec3::new(1.0, 0.0, 0.4), 1.3)),
        ("tilt2.2", rot(Vec3::new(0.2, 0.0, 1.0), 2.2)),
        ("over+tilt.4", rot(Vec3::new(0.0, 0.0, 1.0), 0.4) * rot(Vec3::new(1.0, 0.0, 0.0), PI)),
    ];
    let only = std::env::var("R1_FORMS").unwrap_or_default();
    for &alpha in &angles {
        for &beta in &angles {
            for (pn, m) in &poses {
                for tipped in [(true, true), (false, false), (true, false)] {
                    if only == "tt" && tipped != (true, true) {
                        continue;
                    }
                    let name = format!("{alpha}°/{beta}° {pn} forms {tipped:?}");
                    match catch_unwind(AssertUnwindSafe(|| cone_pair(name.clone(), alpha, beta, *m, tipped))) {
                        Ok(case) => run(&case, &mut tally),
                        Err(_) => tally.failed.push(format!("{name}: PANIC building the case")),
                    }
                }
            }
        }
    }
    report("two cones at one apex", &tally);
    assert!(tally.failed.is_empty());
}

/// Probe 3: near-coaxial fans — the copy turned by a tiny angle about the
/// apex, and near-tangent fans (axes tilted by just under / over the sum of
/// the half-angles).
#[test]
#[ignore = "R1 probe"]
fn r1_near_coaxial_and_near_tangent_fans() {
    let mut tally = Tally::default();
    for alpha in [30.0f64, 45.0] {
        for delta in [1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 1e-8, 1e-9, 3e-11] {
            for tipped in [(true, true), (false, false)] {
                let m = rot(Vec3::new(0.0, 0.0, 1.0), delta);
                let name = format!("{alpha}° copy turned {delta:e} forms {tipped:?}");
                match catch_unwind(AssertUnwindSafe(|| cone_pair(name.clone(), alpha, alpha, m, tipped))) {
                    Ok(case) => run(&case, &mut tally),
                    Err(_) => tally.failed.push(format!("{name}: PANIC building the case")),
                }
            }
        }
    }
    for (alpha, beta) in [(30.0f64, 30.0f64), (30.0, 45.0)] {
        let sum = (alpha + beta).to_radians();
        for off in [-1e-3, -1e-6, -1e-9, 1e-9, 1e-6, 1e-3] {
            let m = rot(Vec3::new(0.0, 0.0, 1.0), sum + off);
            let name = format!("{alpha}°/{beta}° near tangent {off:e}");
            match catch_unwind(AssertUnwindSafe(|| cone_pair(name.clone(), alpha, beta, m, (true, true)))) {
                Ok(case) => run(&case, &mut tally),
                Err(_) => tally.failed.push(format!("{name}: PANIC building the case")),
            }
        }
    }
    report("near-coaxial and near-tangent fans", &tally);
    assert!(tally.failed.is_empty());
}

/// Probe 2: a box corner at the apex in many poses and sizes.
#[test]
#[ignore = "R1 probe"]
fn r1_box_corner_at_the_apex() {
    let mut tally = Tally::default();
    // Deterministic pseudo-random rotations.
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    let mut rnd = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    let mut poses: Vec<(String, Affine3<f64>)> = Vec::new();
    // The witness's pose family, turned about the axis, then flipped so the
    // box sits below with the axis along an edge, then random.
    for turn in [0.0, 0.4, 1.0, 2.0, 3.0] {
        // brick [0,2]x[-1,1]x[0,2]-like: corner at apex, x,z along e1,e3, y down.
        poses.push((format!("down-edge turn {turn}"), rot(Vec3::new(0.0, 1.0, 0.0), turn) * rot(Vec3::new(1.0, 0.0, 0.0), PI / 2.0)));
    }
    for k in 0..14 {
        let axis = Vec3::new(rnd() - 0.5, rnd() - 0.5, rnd() - 0.5);
        let ang = rnd() * PI;
        poses.push((format!("random {k}"), rot(axis, ang)));
    }
    for alpha in [20.0f64, 45.0, 70.0] {
        for (pn, m) in &poses {
            for l in [[2.0, 2.0, 2.0], [0.3, 0.5, 0.4], [0.6, 2.0, 0.2]] {
                for tipped in [true, false] {
                    let name = format!("{alpha}° box {l:?} {pn} tipped {tipped}");
                    let built = catch_unwind(AssertUnwindSafe(|| {
                        let a = finished("a", form(alpha, tipped), tol());
                        let b = box_body(l, *m);
                        let (rca, rcb) = (cone_reach(alpha, Affine3::identity()), box_reach(l, *m));
                        let share = shared(down(), alpha.to_radians(), &rca, &rcb);
                        let ext = 2.2f64.max(l[0] + l[1] + l[2]);
                        Case {
                            name: name.clone(),
                            a,
                            b,
                            ra: cone_region(alpha, Affine3::identity()),
                            rb: box_region(l, *m),
                            va: cone_volume(alpha),
                            vb: l[0] * l[1] * l[2],
                            share,
                            vtol: 3e-3 * cone_volume(alpha).max(1.0),
                            lattice: (Point3::new(-ext, 1.0 - ext, -ext), Point3::new(ext, 1.0 + ext, ext)),
                        }
                    }));
                    match built {
                        Ok(case) => run(&case, &mut tally),
                        Err(_) => tally.failed.push(format!("{name}: PANIC building the case")),
                    }
                }
            }
        }
    }
    report("box corner at the apex", &tally);
    assert!(tally.failed.is_empty());
}

/// Probe 4: an edge through the apex (B2's, must refuse typed), and a box
/// corner just off the apex (inside the band and outside it).
#[test]
#[ignore = "R1 probe"]
fn r1_edge_through_and_near_the_apex() {
    let mut tally = Tally::default();
    let eps = tol().eps();
    let alpha = 45.0f64;
    let a = finished("a", form(alpha, true), tol());
    // A box whose edge runs through the apex: [-1,1] along x through NORTH.
    let through = topo::test_support::brick((-1.0, 1.0), (1.0, 2.0), (0.0, 1.0), tol());
    let through = finished("through", through, tol());
    for (x, y, op) in [(&a, &through, 0u8), (&through, &a, 0), (&a, &through, 1), (&a, &through, 2), (&through, &a, 2)] {
        let got = catch_unwind(AssertUnwindSafe(|| match op {
            0 => intersect(x, y, tol()),
            1 => union(x, y, tol()),
            _ => subtract(x, y, tol()),
        }));
        match got {
            Err(_) => tally.failed.push(format!("through-edge op {op}: PANIC")),
            Ok(Err(e)) => tally.refused.push(format!("through-edge op {op}: {e:?}")),
            Ok(Ok(r)) => tally.refused.push(format!("through-edge op {op}: BUILT {}", match r { BooleanResult::Empty => "empty".to_string(), BooleanResult::Body(b) => format!("vol {:?}", topo::mass_properties(&b.body, tol(), topo::test_support::pinned_resolution(tol())).ok().and_then(|m| m.met().ok()).map(|m| m.volume)) })),
        }
    }
    // A box corner at an offset from the apex, the box's edges along the
    // witness's frame (one edge down the axis).
    let m0 = rot(Vec3::new(1.0, 0.0, 0.0), PI / 2.0);
    for f in [0.1, 0.5, 0.9, 1.5, 3.0, 10.0, 1e3] {
        for dir in [Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(-0.6, 0.0, 0.8), Vec3::new(-1.0, -1.0, -1.0).normalize()] {
            let off = dir * (f * eps);
            let m = Affine3::translation(off) * m0;
            let l = [2.0, 2.0, 2.0];
            let name = format!("box corner off apex {f}ε along {dir:?}");
            let built = catch_unwind(AssertUnwindSafe(|| {
                let b = box_body(l, m);
                let (rca, rcb) = (cone_reach(alpha, Affine3::identity()), box_reach(l, m0));
                let share = shared(down(), alpha.to_radians(), &rca, &rcb);
                Case {
                    name: name.clone(),
                    a: a.clone(),
                    b,
                    ra: cone_region(alpha, Affine3::identity()),
                    rb: box_region(l, m),
                    va: cone_volume(alpha),
                    vb: 8.0,
                    share,
                    vtol: 3e-3,
                    lattice: (Point3::new(-2.5, -1.5, -2.5), Point3::new(2.5, 3.5, 2.5)),
                }
            }));
            match built {
                Ok(case) => run(&case, &mut tally),
                Err(_) => tally.failed.push(format!("{name}: PANIC building")),
            }
        }
    }
    report("edge through / near the apex", &tally);
    assert!(tally.failed.is_empty());
}

/// Probe 5: a cone apex on a pyramid's tip.
#[test]
#[ignore = "R1 probe"]
fn r1_cone_apex_on_a_pyramid_tip() {
    use topo::test_support::meeting::{Pose, posed_pyramid};
    let mut tally = Tally::default();
    // Pyramids with apex at NORTH, base below or above, corners CCW seen
    // from the apex.
    let bases: Vec<(&str, Vec<[f64; 3]>, bool)> = vec![
        ("square below wide", vec![[-1.0, 0.0, -1.0], [-1.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 0.0, -1.0]], false),
        ("square below narrow", vec![[-0.2, 0.0, -0.2], [-0.2, 0.0, 0.2], [0.2, 0.0, 0.2], [0.2, 0.0, -0.2]], false),
        ("triangle off-axis", vec![[0.1, 0.0, -0.3], [-0.5, 0.2, 0.6], [0.9, -0.1, 0.4]], false),
        ("square above", vec![[-0.6, 2.0, -0.6], [0.6, 2.0, -0.6], [0.6, 2.0, 0.6], [-0.6, 2.0, 0.6]], true),
        ("pentagon sideways", (0..5).map(|k| { let t = 2.0 * PI * f64::from(k) / 5.0; [1.2, 1.0 + 0.5 * t.cos(), 0.5 * t.sin()] }).collect(), false),
    ];
    let id = Pose::turn("id", [0.0, 0.0, 1.0], 0.0, [0.0, 0.0, 0.0]);
    for alpha in [20.0f64, 45.0, 70.0] {
        for (bn, base, _) in &bases {
            for tipped in [true, false] {
                for flip in [false, true] {
                    let name = format!("{alpha}° pyramid {bn} tipped {tipped} flip {flip}");
                    let built = catch_unwind(AssertUnwindSafe(|| {
                        let mut base = base.clone();
                        if flip {
                            base.reverse();
                        }
                        posed_pyramid(&base, [0.0, 1.0, 0.0], &id, tol())
                    }));
                    let Ok(b) = built else {
                        if !flip { eprintln!("  (pyramid {bn} unbuildable unflipped)"); }
                        continue;
                    };
                    let base: Vec<Point3<f64>> = {
                        let mut v = base.clone();
                        if flip { v.reverse(); }
                        v.iter().map(|p| Point3::new(p[0], p[1], p[2])).collect()
                    };
                    // Pyramid reach: inside every side plane, up to the base plane.
                    let apex = NORTH;
                    let nb = base.len();
                    let mut planes: Vec<(Vec3<f64>, f64)> = Vec::new(); // (outward n, offset) n·x <= off
                    let centroid = base.iter().fold(Vec3::new(0.0, 0.0, 0.0), |s, p| s + (*p - Point3::origin())) * (1.0 / nb as f64);
                    let cen = Point3::origin() + (centroid + (apex - Point3::origin())) * 0.5;
                    let mut push = |p0: Point3<f64>, p1: Point3<f64>, p2: Point3<f64>| {
                        let mut n = (p1 - p0).cross(p2 - p0).normalize();
                        if n.dot(cen - p0) > 0.0 { n = -n; }
                        planes.push((n, n.dot(p0 - Point3::origin())));
                    };
                    for i in 0..nb {
                        push(apex, base[i], base[(i + 1) % nb]);
                    }
                    push(base[0], base[1], base[2]);
                    let planes = Rc::new(planes);
                    let pl = planes.clone();
                    let reach: Reach = Rc::new(move |d: Vec3<f64>| {
                        let mut r = f64::INFINITY;
                        for &(n, off) in pl.iter() {
                            let k = n.dot(d);
                            let room = off - n.dot(apex - Point3::origin());
                            if room.abs() < 1e-12 {
                                if k > 0.0 { return 0.0; }
                            } else if k > 0.0 {
                                r = r.min(room / k);
                            }
                        }
                        r
                    });
                    let pl = planes.clone();
                    let region: Region = Rc::new(move |p: Point3<f64>| pl.iter().map(|&(n, off)| off - n.dot(p - Point3::origin())).fold(f64::INFINITY, f64::min));
                    let vb = {
                        // volume = ∫ ⅓ r³ over the sphere, via the integrator over a hemisphere-ish cap
                        shared(down(), PI, &reach, &(Rc::new(|_d: Vec3<f64>| f64::INFINITY) as Reach))
                    };
                    let a = finished("a", form(alpha, tipped), tol());
                    let share = shared(down(), alpha.to_radians(), &cone_reach(alpha, Affine3::identity()), &reach);
                    let case = Case {
                        name,
                        a,
                        b,
                        ra: cone_region(alpha, Affine3::identity()),
                        rb: region,
                        va: cone_volume(alpha),
                        vb,
                        share,
                        vtol: 4e-3 * cone_volume(alpha).max(vb).max(1.0),
                        lattice: (Point3::new(-3.0, -2.0, -3.0), Point3::new(3.0, 3.0, 3.0)),
                    };
                    run(&case, &mut tally);
                }
            }
        }
    }
    report("cone apex on a pyramid tip", &tally);
    assert!(tally.failed.is_empty());
}

/// Probe 3b: `cone_cone_apex_meeting` at near-coaxial axes, equal
/// half-angles. The true common rulings lie at `α` off each axis; the
/// closed form's `(c₁ − k·c₂)/s²` cancels as `k → 1`.
#[test]
#[ignore = "R1 probe"]
fn r1_fan_meeting_near_coaxial() {
    for half in [0.5236f64, 0.7854] {
        for delta in [1e-2f64, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 3e-8, 1e-8, 1e-9] {
            let a1 = Vec3::new(0.0, -1.0, 0.0);
            let a2 = Vec3::new(delta.sin(), -delta.cos(), 0.0);
            let m = geom_brep::cone_cone_apex_meeting((a1, half.cos()), (a2, half.cos()));
            let [r0, r1] = m.rulings();
            let off = |d: Vec3<f64>| ((d.dot(a1) - half.cos()).abs()).max((d.dot(a2) - half.cos()).abs()).max((d.norm() - 1.0).abs());
            eprintln!("half {half} delta {delta:e}: disc {:.6e} (true {:.6e}) worst off-cone residual {:.3e}", m.disc, {
                // exact: rulings at angle α from each axis; ‖base‖² = c²/cos²(δ/2)
                let c = half.cos();
                1.0 - c * c / (delta / 2.0).cos().powi(2)
            }, off(r0).max(off(r1)));
        }
    }
}

/// Probe 4b: a box edge lying along a ruling of the cone and ending at
/// the apex (the `Degenerate` arm of the end-at-apex reading), and edges
/// just off a ruling.
#[test]
#[ignore = "R1 probe"]
fn r1_box_edge_along_a_ruling_to_the_apex() {
    let mut tally = Tally::default();
    let m0 = rot(Vec3::new(1.0, 0.0, 0.0), PI / 2.0);
    for alpha in [30.0f64, 45.0, 60.0] {
        for off in [0.0, 1e-10, 1e-8, 1e-6, 1e-3] {
            for tipped in [true, false] {
                let m = rot(Vec3::new(0.0, 0.0, 1.0), alpha.to_radians() + off) * m0;
                let l = [0.7, 0.6, 0.5];
                let name = format!("{alpha}° box edge along a ruling +{off:e} tipped {tipped}");
                let built = catch_unwind(AssertUnwindSafe(|| {
                    let a = finished("a", form(alpha, tipped), tol());
                    let b = box_body(l, m);
                    let share = shared(down(), alpha.to_radians(), &cone_reach(alpha, Affine3::identity()), &box_reach(l, m));
                    Case {
                        name: name.clone(), a, b,
                        ra: cone_region(alpha, Affine3::identity()), rb: box_region(l, m),
                        va: cone_volume(alpha), vb: l[0] * l[1] * l[2], share,
                        vtol: 3e-3 * cone_volume(alpha).max(1.0),
                        lattice: (Point3::new(-2.0, -1.0, -2.0), Point3::new(2.0, 2.0, 2.0)),
                    }
                }));
                match built {
                    Ok(case) => run(&case, &mut tally),
                    Err(_) => tally.failed.push(format!("{name}: PANIC building")),
                }
            }
        }
    }
    report("box edge along a ruling", &tally);
    assert!(tally.failed.is_empty());
}
