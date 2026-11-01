//! **Review probe for PR 6221** (`splitdoor/wrap-tip-boolean`): wrap
//! tips through the Boolean at poses the PR's rows do not take, read
//! against an independent oracle — each result's closed-form volume and
//! `point_in_solid` membership at sample points against the analytic
//! region. Every case is reported; the row fails if any case does.
//! `RV_UNTIPPED=1` builds each body without its wrap edge (two meridians
//! at the pole) as the baseline.
//!
//! Measured at a5352925 (head) and 5f0a527e (main), each at ε 1e-6,
//! 1e-9 and 1e-12 (all ε alike):
//! - The full sphere's lone pole-to-pole wrap edge, ∖/∩ the half-space
//!   `z > 0` (the plane holding the wrap edge): **main builds all three
//!   ops at the closed form, membership right; head refuses
//!   `SectionLoopUndecided`** (with the strut arm reverted,
//!   `CurvedBooleanUnsupported`: the `SlitTips` reading alone moves it).
//!   The quarter: main `BisectorSide`, head `SectionLoopUndecided`; the
//!   untipped sphere builds the quarter. `x > 0`: main `BisectorSide`,
//!   head builds.
//! - The bullet (a profile ending on the axis on a sphere): main refuses
//!   x>0, z>0 and the quarter; head builds all at the closed form.
//! - The hemisphere: off-axis planes near the pole and caps build on
//!   both. A box corner, edge or face on the pole refuses
//!   `CurvedPierceUnsupported` / `CurvedBooleanUnsupported` on head, on
//!   main and untipped alike. Every split of a sphere refuses
//!   `CurvedBooleanUnsupported` everywhere.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::wrap_tip::{hemisphere, leaving, one_wrap_edge, vertex_at};
use core::f64::consts::PI;
use geom_brep::EdgeDescriptionSpec;
use geom_core::{Band, Point2, Point3, Tol};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::construction::Construct;
use topo::{
    AtRestBody, Body, BooleanResult, ContactRecords, SolidContainment, intersect, point_in_solid,
    subtract, validate_geometric, validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

type Region = Box<dyn Fn(Point3<f64>) -> f64>;

/// Signed distance-ish: > 0 inside, < 0 outside.
fn ball(c: Point3<f64>, r: f64) -> Region {
    Box::new(move |p| r - (p - c).norm())
}
fn halfy(y0: f64) -> Region {
    Box::new(move |p: Point3<f64>| p.y - y0)
}
fn bx(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Region {
    Box::new(move |p: Point3<f64>| {
        (p.x - x.0)
            .min(x.1 - p.x)
            .min(p.y - y.0)
            .min(y.1 - p.y)
            .min(p.z - z.0)
            .min(z.1 - p.z)
    })
}
fn and(a: Region, b: Region) -> Region {
    Box::new(move |p| a(p).min(b(p)))
}
fn not(a: Region) -> Region {
    Box::new(move |p| -a(p))
}
fn clone_box(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> (Region, Region) {
    (bx(x, y, z), bx(x, y, z))
}

/// Checks volume, tiers and membership; returns a failure line or None.
fn check(
    what: &str,
    r: Result<BooleanResult<f64>, topo::BooleanError>,
    want: f64,
    region: &Region,
) -> Option<String> {
    let b = match r {
        Ok(BooleanResult::Body(b)) => b.body,
        Ok(BooleanResult::Empty) if want.abs() < 1e-12 => {
            eprintln!("ok   {what}: empty");
            return None;
        }
        other => return Some(format!("FAIL {what}: {other:?}")),
    };
    let mut fails = Vec::new();
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        topo::test_support::pinned_volume(what, &b, tol())
    })) {
        Ok(mp) => {
            let rounding = 64.0 * f64::EPSILON * want.abs().max(1.0);
            if (mp.volume - want).abs() > mp.volume_pad + rounding {
                fails.push(format!(
                    "volume {} ± {:e}, want {want} (off {:e})",
                    mp.volume,
                    mp.volume_pad,
                    mp.volume - want
                ));
            }
        }
        Err(_) => fails.push("volume panicked".into()),
    }
    if let Err(e) = validate_geometric(&b, tol()) {
        fails.push(format!("tier 3: {e:?}"));
    }
    if let Err(e) = validate_pseudomanifold(&b, &ContactRecords::default(), tol()) {
        fails.push(format!("tier 3′: {e:?}"));
    }
    // Membership: a deterministic lattice, skipping points within 1e-3
    // of the region's boundary.
    let band = Band::linear(tol()).unwrap();
    let (mut asked, mut wrong, mut refused) = (0, 0, 0);
    let mut first_wrong = None;
    let n = 9;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let f = |t: usize, o: f64| -1.15 + 2.3 * (t as f64 + o) / (n as f64 - 1.0 + 0.37);
                let p = Point3::new(f(i, 0.11), f(j, 0.23) * 0.9 + 0.1, f(k, 0.07));
                let s = region(p);
                if s.abs() < 1e-3 {
                    continue;
                }
                asked += 1;
                match point_in_solid(&b, p, band, tol()) {
                    Ok(SolidContainment::In) if s > 0.0 => {}
                    Ok(SolidContainment::Out) if s < 0.0 => {}
                    Ok(got) => {
                        wrong += 1;
                        first_wrong.get_or_insert(format!("{p:?} read {got:?}, region {s}"));
                    }
                    Err(e) => {
                        refused += 1;
                        first_wrong.get_or_insert(format!("{p:?} refused {e:?}"));
                    }
                }
            }
        }
    }
    if wrong > 0 || refused > 0 {
        fails.push(format!(
            "membership {wrong} wrong, {refused} refused of {asked}: {}",
            first_wrong.unwrap()
        ));
    }
    if fails.is_empty() {
        eprintln!("ok   {what} ({asked} probes)");
        None
    } else {
        Some(format!("FAIL {what}: {}", fails.join("; ")))
    }
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished("a box", topo::test_support::brick(x, y, z, tol()), tol())
}

/// The cases `body ∩ tool`, `body ∖ tool`, `tool ∖ body`.
#[allow(clippy::too_many_arguments)]
fn cuts(
    out: &mut Vec<String>,
    what: &str,
    body: &AtRestBody<f64>,
    solid: &dyn Fn() -> Region,
    bvol: f64,
    (x, y, z): ((f64, f64), (f64, f64), (f64, f64)),
    inside: f64,
) {
    let tool = brick(x, y, z);
    let size = (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0);
    let (b1, b2) = clone_box(x, y, z);
    let b3 = bx(x, y, z);
    for (op, r, want, region) in [
        ("∩", intersect(body, &tool, tol()), inside, and(solid(), b1)),
        (
            "∖",
            subtract(body, &tool, tol()),
            bvol - inside,
            and(solid(), not(b2)),
        ),
        (
            "tool∖",
            subtract(&tool, body, tol()),
            size - inside,
            and(b3, not(solid())),
        ),
    ] {
        if let Some(f) = check(&format!("{what} {op}"), r, want, &region) {
            out.push(f);
        }
    }
}

/// A cap of height `h` of a ball of radius `r`.
fn cap(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}

/// Wraps the meridians at `at` into one wrap edge (the `one_wrap_edge`
/// recipe at any pole), without joining.
fn wrap_at(mut body: Body<f64>, at: Point3<f64>) -> Body<f64> {
    let tip = vertex_at(&body, at);
    let out = leaving(&body, tip);
    assert_eq!(out.len(), 2, "two meridians leave {at:?}");
    let wraps = body.get_half_edge(out[0]).unwrap().edge;
    let face = body.face_of_half_edge(out[0]).unwrap();
    let surface = body.get_face(face).unwrap().surface;
    body.kef_describing(
        out[1],
        &[(wraps, EdgeDescriptionSpec::wrap(surface))],
        tol(),
    )
    .expect("restated as a wrap edge");
    body
}

fn untipped() -> bool {
    std::env::var("RV_UNTIPPED").is_ok()
}

fn tipped_hemi() -> AtRestBody<f64> {
    if untipped() {
        finished("untipped", hemisphere(), tol())
    } else {
        finished("tipped", one_wrap_edge(hemisphere(), true).0, tol())
    }
}

const HEMI: f64 = 2.0 * PI / 3.0;
const BIG: (f64, f64) = (-2.0, 2.0);
const Y: (f64, f64) = (-0.5, 2.0);

#[test]
fn rv_hemisphere_poses() {
    let body = tipped_hemi();
    let hemi = || and(ball(Point3::origin(), 1.0), halfy(0.0));
    let mut out = Vec::new();
    // Off-axis half-spaces near the pole: the section misses the pole.
    for d in [0.05, 0.2, -0.05, 1e-4] {
        let x = (d, 2.0);
        // ∩ volume of the hemisphere with x > d: half of a sphere cap
        // of height 1 - d (for d>0), mirrored for d<0.
        let full = if d >= 0.0 {
            cap(1.0, 1.0 - d)
        } else {
            4.0 * PI / 3.0 - cap(1.0, 1.0 + d)
        };
        cuts(
            &mut out,
            &format!("hemi x>{d}"),
            &body,
            &hemi,
            HEMI,
            (x, Y, BIG),
            full / 2.0,
        );
        let z = (d, 2.0);
        cuts(
            &mut out,
            &format!("hemi z>{d}"),
            &body,
            &hemi,
            HEMI,
            (BIG, Y, z),
            full / 2.0,
        );
    }
    // A cap through near the pole, the tip kept in the tool's material.
    for h in [0.1, 0.01] {
        cuts(
            &mut out,
            &format!("hemi y>{}", 1.0 - h),
            &body,
            &hemi,
            HEMI,
            (BIG, (1.0 - h, 2.0), BIG),
            cap(1.0, h),
        );
    }
    // A tool vertex on the pole: the box's top corner at (0,1,0).
    cuts(
        &mut out,
        "hemi corner-on-pole quarter",
        &body,
        &hemi,
        HEMI,
        ((0.0, 2.0), (-0.5, 1.0), (0.0, 2.0)),
        HEMI / 4.0,
    );
    // A tool edge on the pole, square to the axis.
    cuts(
        &mut out,
        "hemi edge-on-pole half",
        &body,
        &hemi,
        HEMI,
        ((0.0, 2.0), (-0.5, 1.0), BIG),
        HEMI / 2.0,
    );
    // A tool face touching only at the pole.
    cuts(
        &mut out,
        "hemi face-on-pole",
        &body,
        &hemi,
        HEMI,
        (BIG, (1.0, 2.0), BIG),
        0.0,
    );
    // A tool corner touching only at the pole.
    cuts(
        &mut out,
        "hemi corner-touch-pole",
        &body,
        &hemi,
        HEMI,
        ((0.0, 2.0), (1.0, 2.0), (0.0, 2.0)),
        0.0,
    );
    // Diagonal quarter: through the axis, off both chart planes (not a
    // box: the quarter x>0,z>0 turned 45° is not a brick; use a thin
    // slab off the wrap plane instead).
    cuts(
        &mut out,
        "hemi slab x∈(-0.3,0.3)",
        &body,
        &hemi,
        HEMI,
        ((-0.3, 0.3), Y, BIG),
        HEMI - 2.0 * cap(1.0, 0.7) / 2.0,
    );
    for f in &out {
        eprintln!("{f}");
    }
    assert!(out.is_empty(), "{} cases failed", out.len());
}

#[test]
fn rv_full_sphere_two_tips() {
    let t = (PI / 8.0).tan();
    let s = revolved_about_y(
        vec![
            (Point2::new(0.0, -1.0), t),
            (Point2::new(1.0, 0.0), t),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let s = if untipped() {
        s
    } else {
        wrap_at(s, Point3::new(0.0, 1.0, 0.0))
    };
    let tips = s
        .vertices()
        .filter(|(v, _)| leaving(&s, *v).len() == 1)
        .count();
    eprintln!(
        "full sphere: {} vertices, {} with one leaving half-edge, {} faces",
        s.vertices().count(),
        tips,
        s.faces().count()
    );
    let body = finished("tipped sphere", s, tol());
    let sph = || ball(Point3::origin(), 1.0);
    let v = 4.0 * PI / 3.0;
    let mut out = Vec::new();
    let yy = (-2.0, 2.0);
    cuts(
        &mut out,
        "sphere x>0",
        &body,
        &sph,
        v,
        ((0.0, 2.0), yy, BIG),
        v / 2.0,
    );
    cuts(
        &mut out,
        "sphere z>0",
        &body,
        &sph,
        v,
        (BIG, yy, (0.0, 2.0)),
        v / 2.0,
    );
    cuts(
        &mut out,
        "sphere quarter",
        &body,
        &sph,
        v,
        ((0.0, 2.0), yy, (0.0, 2.0)),
        v / 4.0,
    );
    cuts(
        &mut out,
        "sphere x>0.3",
        &body,
        &sph,
        v,
        ((0.3, 2.0), yy, BIG),
        cap(1.0, 0.7),
    );
    cuts(
        &mut out,
        "sphere y>0.5",
        &body,
        &sph,
        v,
        (BIG, (0.5, 2.0), BIG),
        cap(1.0, 0.5),
    );
    cuts(
        &mut out,
        "sphere y<-0.5",
        &body,
        &sph,
        v,
        (BIG, (-2.0, -0.5), BIG),
        cap(1.0, 0.5),
    );
    cuts(
        &mut out,
        "sphere slab y∈(-0.5,0.5)",
        &body,
        &sph,
        v,
        (BIG, (-0.5, 0.5), BIG),
        v - 2.0 * cap(1.0, 0.5),
    );
    for f in &out {
        eprintln!("{f}");
    }
    assert!(out.is_empty(), "{} cases failed", out.len());
}

#[test]
fn rv_bullet_profile_ends_on_axis() {
    // A cylinder r = 0.5 to y = 0.5, capped by a hemisphere of radius 0.5
    // whose pole (0,1,0) is the profile's end on the axis.
    let t = (PI / 8.0).tan();
    let b = revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(0.5, 0.0), 0.0),
            (Point2::new(0.5, 0.5), t),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let b = if untipped() {
        b
    } else {
        one_wrap_edge(b, true).0
    };
    let body = finished("tipped bullet", b, tol());
    let c = Point3::new(0.0, 0.5, 0.0);
    let solid = move || -> Region {
        Box::new(move |p: Point3<f64>| {
            let cyl = (0.5 - (p.x * p.x + p.z * p.z).sqrt())
                .min(p.y)
                .min(0.5 - p.y);
            let dome = (0.5 - (p - c).norm()).min(p.y - 0.5);
            cyl.max(dome)
        })
    };
    let v = PI * 0.25 * 0.5 + 2.0 / 3.0 * PI * 0.125;
    let mut out = Vec::new();
    cuts(
        &mut out,
        "bullet x>0",
        &body,
        &solid,
        v,
        ((0.0, 2.0), Y, BIG),
        v / 2.0,
    );
    cuts(
        &mut out,
        "bullet z>0",
        &body,
        &solid,
        v,
        (BIG, Y, (0.0, 2.0)),
        v / 2.0,
    );
    cuts(
        &mut out,
        "bullet quarter",
        &body,
        &solid,
        v,
        ((0.0, 2.0), Y, (-2.0, 0.0)),
        v / 4.0,
    );
    cuts(
        &mut out,
        "bullet y>0.9",
        &body,
        &solid,
        v,
        (BIG, (0.9, 2.0), BIG),
        cap(0.5, 0.1),
    );
    for f in &out {
        eprintln!("{f}");
    }
    assert!(out.is_empty(), "{} cases failed", out.len());
}

fn split_case(
    out: &mut Vec<String>,
    what: &str,
    body: &AtRestBody<f64>,
    o: Point3<f64>,
    n: geom_core::Vec3<f64>,
    want: (f64, f64),
) {
    match topo::split(body, &topo::test_support::split_plane(o, n, tol()), tol()) {
        Err(e) => out.push(format!("FAIL {what}: split {e:?}")),
        Ok(r) => {
            for (side, part, w) in [("above", &r.above, want.0), ("below", &r.below, want.1)] {
                let topo::SplitPart::Body(b) = part else {
                    out.push(format!("FAIL {what} {side}: no body"));
                    continue;
                };
                let mp = topo::test_support::pinned_volume(what, b, tol());
                let mut f = Vec::new();
                if (mp.volume - w).abs() > mp.volume_pad + 64.0 * f64::EPSILON {
                    f.push(format!(
                        "volume {} ± {:e} want {w}",
                        mp.volume, mp.volume_pad
                    ));
                }
                if let Err(e) = validate_geometric(b, tol()) {
                    f.push(format!("tier 3 {e:?}"));
                }
                if let Err(e) = validate_pseudomanifold(b, &ContactRecords::default(), tol()) {
                    f.push(format!("tier 3′ {e:?}"));
                }
                if f.is_empty() {
                    eprintln!("ok   {what} {side}");
                } else {
                    out.push(format!("FAIL {what} {side}: {}", f.join("; ")));
                }
            }
        }
    }
}

#[test]
fn rv_splits_through_the_pole() {
    use geom_core::Vec3;
    let body = tipped_hemi();
    let pole = Point3::new(0.0, 1.0, 0.0);
    let mut out = Vec::new();
    let s = 0.5f64.sqrt();
    for (what, n) in [
        ("x", Vec3::new(1.0, 0.0, 0.0)),
        ("z", Vec3::new(0.0, 0.0, 1.0)),
        ("diag", Vec3::new(s, 0.0, s)),
        ("-z", Vec3::new(0.0, 0.0, -1.0)),
    ] {
        split_case(
            &mut out,
            &format!("hemi split {what}"),
            &body,
            Point3::origin(),
            n,
            (HEMI / 2.0, HEMI / 2.0),
        );
    }
    // Near the pole, off it.
    split_case(
        &mut out,
        "hemi split x=0.05",
        &body,
        Point3::new(0.05, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        (cap(1.0, 0.95) / 2.0, HEMI - cap(1.0, 0.95) / 2.0),
    );
    split_case(
        &mut out,
        "hemi split y=0.95",
        &body,
        Point3::new(0.0, 0.95, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        (cap(1.0, 0.05), HEMI - cap(1.0, 0.05)),
    );
    // Tangent at the pole: the plane y = 1 touches only there.
    split_case(
        &mut out,
        "hemi split tangent y=1",
        &body,
        pole,
        Vec3::new(0.0, 1.0, 0.0),
        (0.0, HEMI),
    );
    for f in &out {
        eprintln!("{f}");
    }
    assert!(out.is_empty(), "{} cases failed", out.len());
}
