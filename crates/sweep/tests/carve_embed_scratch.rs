//! MEASUREMENT SCRATCH (carve-embed phase 1; delete before landing).
//!
//! Runs `topo::clearance`'s injectivity question (the body-level
//! embedding certificate) on loft and sweep bodies today's doors build,
//! at `f64` (the door lifts every carrier to `Interval`) and on the body
//! built at `Interval`, timing the build and the certificate. Asserts
//! nothing; run with
//! `cargo nextest run --release -p sweep --run-ignored only -E 'test(carve_embed_scratch)' --no-capture`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::cast_precision_loss, clippy::too_many_lines)]

use core::f64::consts::TAU;
use std::time::Instant;

use geom::NurbsCurve3;
use geom_core::k_stats::decide;
use geom_core::spline::KnotVector;
use geom_core::{
    Affine3, Band, Bounds, CertifiedBounds, Decide, Interval, Margin, Mat3, Point2, Point3, Real,
    Sign, Tol, Vec3,
};
use profile::{ProfileLoop, RawLoop, SketchPlane};
use sweep::test_support::{lift_places, lift_sections, stacked_at, with_fold_bypassed};
use sweep::{LoftError, Lofted, loft_body, sweep_body};
use topo::FaceKey;
use topo::clearance::{
    BoundReport, BoundVerdict, Budgets, Candidate, ClearanceBound, DEFAULT_MAX_CELL_DEPTH,
    EngineRefusal, Side, Sweep, Which,
};

use crate::common::{normal_start_place, plane_of, quad};

fn budget() -> usize {
    std::env::var("CARVE_EMBED_BUDGET")
        .ok()
        .and_then(|b| b.parse().ok())
        .unwrap_or(65_536)
}

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

trait Scalar: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy {}
impl<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy> Scalar for T {}

fn mid<T: Bounds>(x: T) -> f64 {
    0.5 * (x.lo() + x.hi())
}

#[derive(Debug, Clone, Copy)]
struct Witness {
    a: FaceKey,
    b: FaceKey,
    uva: (f64, f64),
    uvb: (f64, f64),
    pa: Point3<f64>,
    d: f64,
}

fn verify<T: Scalar>(
    body: &topo::Body<T>,
) -> impl FnMut(Candidate<'_, '_, T>) -> Result<Witness, String> + '_ {
    move |candidate| {
        let (fa, fb) = candidate.faces();
        let surf = |f: FaceKey| {
            body.get_surface(body.get_face(f).unwrap().surface)
                .unwrap()
                .clone()
        };
        let (sa, sb) = (surf(fa), surf(fb));
        let (la, lb) = candidate.on_face_stations();
        let at = T::from_f64;
        let mut best: Option<(Witness, T)> = None;
        for &(au, av) in &la {
            let pa = sa.eval(at(au), at(av));
            for &(bu, bv) in &lb {
                let pb = sb.eval(at(bu), at(bv));
                let d = (pa - pb).norm();
                if best.is_none_or(|(w, _)| d.hi() < w.d) {
                    best = Some((
                        Witness {
                            a: fa,
                            b: fb,
                            uva: (au, av),
                            uvb: (bu, bv),
                            pa: pa.map(mid),
                            d: d.hi(),
                        },
                        d,
                    ));
                }
            }
        }
        let (w, d) = best.ok_or("no station")?;
        let bound = ClearanceBound::StrictlyPositive;
        let definite = match decide(bound.predicate(), Margin::of(d - at(bound.c())), band()) {
            Ok(Sign::Negative) => true,
            Ok(Sign::Zero) => !bound.zero_discharges(),
            _ => false,
        };
        if definite {
            Ok(w)
        } else {
            Err(format!("the closest station pair is {} apart", w.d))
        }
    }
}

fn injective<T: Scalar>(body: &topo::Body<T>) -> Result<BoundReport<Witness>, EngineRefusal> {
    let faces = body.faces().map(|(k, _)| k).collect();
    let side = Side { body, faces };
    let windows = side.windows(Which::First)?;
    let sweep = Sweep {
        bound: ClearanceBound::StrictlyPositive,
        band: band(),
        budgets: Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: budget(),
        },
    };
    Ok(sweep.run(&windows, &windows, true, verify(body)))
}

/// Tier 3's verdict and the signed volume, timed (the "inside" price).
fn volume<T: Scalar>(body: &topo::Body<T>) -> String {
    let t3 = topo::validate_geometric(body, tol())
        .map_err(|e| format!("{e:?}").chars().take(80).collect::<String>());
    let t = Instant::now();
    let m = topo::test_support::measure_unvalidated(
        body,
        tol(),
        topo::test_support::pinned_resolution(tol()),
    );
    let c = t.elapsed().as_secs_f64();
    let v = match m {
        Ok(m) => format!("{:.4}", mid(m.enclosure.volume)),
        Err(e) => format!("refused {e:?}").chars().take(80).collect(),
    };
    format!("\n            tier3 {t3:?} | signed volume {v} in {c:.3}s")
}

fn line<T: Scalar>(l: &Lofted<T>, l64: Option<&Lofted<f64>>, build: f64) -> String {
    let t = Instant::now();
    let r = injective(&l.body);
    let c = t.elapsed().as_secs_f64();
    match r {
        Ok(r) => format!(
            "{} | cert {c:.3}s build {build:.3}s | {:?}{}",
            say(l, l64, &r),
            r.receipt,
            if matches!(r.verdict, BoundVerdict::Holds) {
                volume(&l.body)
            } else {
                String::new()
            }
        ),
        Err(EngineRefusal::Unsupported { carrier, face, .. }) => format!(
            "WINDOW REFUSED {} unsupported: {carrier} | cert {c:.3}s build {build:.3}s",
            role(l, face)
        ),
        Err(e) => format!("WINDOW REFUSED {}: {}", e.name(), e.payload()),
    }
}

/// A face's role in the loft: `top`, `bottom`, or `wall[l][j][s]`.
fn role<T: Real>(l: &Lofted<T>, f: FaceKey) -> String {
    if f == l.top {
        return "top".into();
    }
    if f == l.bottom {
        return "bottom".into();
    }
    for (li, lp) in l.side_faces.iter().enumerate() {
        for (j, seg) in lp.iter().enumerate() {
            for (s, &w) in seg.iter().enumerate() {
                if w == f {
                    return format!("wall[{li}][{j}][{s}]");
                }
            }
        }
    }
    format!("{f:?}")
}

/// The sections a wall's `v` lies between.
fn between(l: &Lofted<f64>, v: f64) -> String {
    let p = &l.section_params;
    match p.iter().position(|&x| x >= v) {
        Some(0) => "at section 0".into(),
        Some(i) => format!("v={v:.4}, sections {}..{}", i - 1, i),
        None => format!("v={v:.4} past the last section"),
    }
}

fn say<T: Real>(l: &Lofted<T>, l64: Option<&Lofted<f64>>, r: &BoundReport<Witness>) -> String {
    match &r.verdict {
        BoundVerdict::Holds => "Holds".into(),
        BoundVerdict::Violated(w) => {
            let at = |f: FaceKey, (u, v): (f64, f64)| {
                let r = role(l, f);
                let s = match (l64, r.starts_with("wall")) {
                    (Some(l64), true) => between(l64, v),
                    _ => format!("u={u:.3} v={v:.3}"),
                };
                format!("{r} ({s})")
            };
            format!(
                "Violated: {} x {} at ({:.3},{:.3},{:.3}) d<={:.2e}",
                at(w.a, w.uva),
                at(w.b, w.uvb),
                w.pa.x,
                w.pa.y,
                w.pa.z,
                w.d
            )
        }
        BoundVerdict::Refused(EngineRefusal::Uncertified { a, b, shared, arm }) => format!(
            "Refused Uncertified {{{} x {}, {}, {}}}",
            role(l, *a),
            role(l, *b),
            shared.payload(),
            arm.name()
        ),
        BoundVerdict::Refused(e) => format!("Refused {}: {}", e.name(), e.payload()),
    }
}

/// One row: build at f64 and Interval, certify both, print.
fn row(
    what: &str,
    bypass: bool,
    f: impl Fn() -> Result<Lofted<f64>, LoftError>,
    g: impl Fn() -> Result<Lofted<Interval>, LoftError>,
) {
    let t = Instant::now();
    let built = if bypass { with_fold_bypassed(&f) } else { f() };
    let build64 = t.elapsed();
    let line64 = match &built {
        Err(e) => format!("BUILD REFUSED {e:?}"),
        Ok(l) => line(l, Some(l), build64.as_secs_f64()),
    };
    let t = Instant::now();
    let built_i = if bypass { with_fold_bypassed(&g) } else { g() };
    let build_i = t.elapsed();
    let line_i = match &built_i {
        Err(e) => format!("BUILD REFUSED {e:?}"),
        Ok(l) => line(l, built.as_ref().ok(), build_i.as_secs_f64()),
    };
    println!(
        "[embed] {what}{}\n  f64:      {line64}\n  Interval: {line_i}",
        if bypass { " (fold bypassed)" } else { "" }
    );
}

// ------------------------------------------------------------ corpora

fn arc_spine_places(curl: f64, stations: usize) -> Vec<SketchPlane<f64>> {
    (0..stations)
        .map(|i| {
            let theta = curl * (i as f64) / ((stations - 1) as f64);
            let (s, c) = theta.sin_cos();
            let linear = Mat3::from_cols(
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(-s, 0.0, c),
                Vec3::new(c, 0.0, s),
            );
            plane_of(&Affine3::from_parts(linear, Vec3::new(s, 0.0, 1.0 - c)))
        })
        .collect()
}

fn loft_at<T: Scalar>(
    sections: &[sweep::Section],
    places: &[SketchPlane<f64>],
    deg: usize,
) -> Result<Lofted<T>, LoftError> {
    loft_body::<T>(
        &lift_sections::<T>(sections),
        &lift_places::<T>(places, tol()),
        deg,
        tol(),
    )
}

fn sweep_at<T: Scalar>(
    section: &sweep::Section,
    start: SketchPlane<f64>,
    path: &NurbsCurve3<f64>,
    stations: usize,
    deg: usize,
) -> Result<Lofted<T>, LoftError> {
    let s = &lift_sections::<T>(core::slice::from_ref(section))[0];
    let p = lift_places::<T>(&[start], tol())[0];
    sweep_body::<T>(s, p, path, stations, deg, tol())
}

fn loft_row(
    what: &str,
    bypass: bool,
    sections: &[sweep::Section],
    places: &[SketchPlane<f64>],
    deg: usize,
) {
    row(
        what,
        bypass,
        || loft_at::<f64>(sections, places, deg),
        || loft_at::<Interval>(sections, places, deg),
    );
}

fn sweep_row(
    what: &str,
    bypass: bool,
    section: &sweep::Section,
    start: SketchPlane<f64>,
    path: &NurbsCurve3<f64>,
    stations: usize,
    deg: usize,
) {
    row(
        what,
        bypass,
        || sweep_at::<f64>(section, start, path, stations, deg),
        || sweep_at::<Interval>(section, start, path, stations, deg),
    );
}

fn helix_path(turns: f64) -> NurbsCurve3<f64> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = ((32.0 * turns).ceil() as usize).max(32);
    let pts: Vec<Point3<f64>> = (0..=n)
        .map(|k| {
            let a = TAU * turns * (k as f64) / (n as f64);
            Point3::new(a.cos(), a.sin(), 0.4 * turns * (k as f64) / (n as f64))
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

fn turned_about_x(origin: Vec3<f64>, cos_a: f64) -> SketchPlane<f64> {
    let sin_a = (1.0 - cos_a * cos_a).sqrt();
    let linear = Mat3::from_cols(
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, cos_a, sin_a),
        Vec3::new(0.0, -sin_a, cos_a),
    );
    plane_of(&Affine3::from_parts(linear, origin))
}

fn poly(pts: &[(f64, f64)]) -> sweep::Section {
    vec![ProfileLoop::polygon(
        pts.iter().map(|&(x, y)| Point2::new(x, y)),
    )]
}

#[test]
#[ignore = "measurement scratch: prints, asserts nothing"]
fn carve_embed_scratch_arc_spines() {
    let sq = quad([(-0.05, -0.05), (0.05, -0.05), (0.05, 0.05), (-0.05, 0.05)]);
    for curl in [3.5, 6.0, 9.0, 13.0] {
        loft_row(
            &format!("arc spine {curl} rad / 17 stations"),
            false,
            &vec![sq.clone(); 17],
            &arc_spine_places(curl, 17),
            3,
        );
    }
}

#[test]
#[ignore = "measurement scratch: prints, asserts nothing"]
fn carve_embed_scratch_helices() {
    let sq = quad([(-0.08, -0.08), (0.08, -0.08), (0.08, 0.08), (-0.08, 0.08)]);
    for (turns, stations) in [(0.5, 17), (0.5, 33), (1.0, 33), (1.0, 65), (2.0, 65)] {
        let path = helix_path(turns);
        sweep_row(
            &format!("m8_14 helix {turns} turn / {stations} stations"),
            false,
            &sq,
            normal_start_place(&path),
            &path,
            stations,
            3,
        );
    }
}

#[test]
#[ignore = "measurement scratch: prints, asserts nothing"]
fn carve_embed_scratch_fold_arms() {
    let unit = quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]);
    // The fold's own refusals, built past it.
    loft_row(
        "top facing down (z 0,1; +z,-z)",
        true,
        &vec![unit.clone(); 2],
        &[
            stacked_at(&[0.0])[0],
            turned_about_x(Vec3::new(0.0, 0.0, 1.0), -1.0),
        ],
        1,
    );
    loft_row(
        "interior back-facing (z 0,1,0.5; +z,-z,-z)",
        true,
        &vec![unit.clone(); 3],
        &[
            stacked_at(&[0.0])[0],
            turned_about_x(Vec3::new(0.0, 0.0, 1.0), -1.0),
            turned_about_x(Vec3::new(0.0, 0.0, 0.5), -1.0),
        ],
        2,
    );
    loft_row(
        "reversed list: z 0,1 stacked 1 then 0",
        true,
        &vec![unit.clone(); 2],
        &stacked_at(&[1.0, 0.0]),
        1,
    );
    // The two embedded bodies the far decide over-refuses.
    let top = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(0.0, 1.0, 0.0),
        100f64.to_radians(),
    );
    let top = plane_of(&Affine3::from_parts(top.linear, Vec3::new(0.0, 0.0, 6.0)));
    loft_row(
        "hood with a downturned top",
        true,
        &[
            quad([(-5.0, -5.0), (5.0, -5.0), (5.0, 5.0), (-5.0, 5.0)]),
            quad([(-2.0, -2.0), (2.0, -2.0), (2.0, 2.0), (-2.0, 2.0)]),
        ],
        &[stacked_at(&[0.0])[0], top],
        1,
    );
    let (s120, c120) = 120f64.to_radians().sin_cos();
    let path = NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 5.0 * 60f64.to_radians().tan()),
            Point3::new(5.0 - 5.0 * c120, 0.0, 5.0 * s120),
        ],
        vec![1.0, 0.5, 1.0],
    )
    .unwrap();
    let start = plane_of(&Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(0.0, 1.0, 0.0),
        70f64.to_radians(),
    ));
    sweep_row(
        "oblique arc sweep",
        true,
        &quad([(-0.2, -0.2), (0.2, -0.2), (0.2, 0.2), (-0.2, 0.2)]),
        start,
        &path,
        3,
        2,
    );
    // The y-prism: embedded, inside out.
    let (s, c) = (-core::f64::consts::FRAC_PI_3).sin_cos();
    let tilted = plane_of(&Affine3::from_parts(
        Mat3::from_cols(
            Vec3::new(c, 0.0, -s),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(s, 0.0, c),
        ),
        Vec3::new(3.0, 0.5, 0.3),
    ));
    loft_row(
        "y-prism (embedded, inside out)",
        true,
        &[
            quad([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]),
            quad([(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5), (-0.5, 0.5)]),
        ],
        &[stacked_at(&[0.0])[0], tilted],
        1,
    );
}

#[test]
#[ignore = "measurement scratch: prints, asserts nothing"]
fn carve_embed_scratch_coincident_and_others() {
    let sq = sweep::test_support::loft_prism_sections()[0].clone();
    for step in [1e-3, 1e-6, 5e-10, 1e-12, 1e-15, 0.0] {
        loft_row(
            &format!("middle slab step {step:e} (4 squares, deg 2)"),
            true,
            &vec![sq.clone(); 4],
            &stacked_at(&[-1.0, 0.0, step, 1.0]),
            2,
        );
    }
    // The closed spine (CONTACTHOLD): ends coincide, fold reads adjacent
    // slabs only, so it builds without a bypass.
    let pts: Vec<Point3<f64>> = (0..=32)
        .map(|k| {
            let a = TAU * f64::from(k % 32) / 32.0;
            Point3::new(3.0 * a.cos(), 3.0 * a.sin(), 0.0)
        })
        .collect();
    let closed = NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap();
    sweep_row(
        "closed spine, r=3 circle, 9 stations",
        false,
        &quad([(-0.25, -0.25), (0.25, -0.25), (0.25, 0.25), (-0.25, 0.25)]),
        normal_start_place(&closed),
        &closed,
        9,
        3,
    );
    // CARVETAIL: opposite-turning joints, v_degree 1, z 0 and 1.
    loft_row(
        "carvetail opposite-turning joints",
        false,
        &[
            poly(&[
                (0.0, 0.0),
                (2.0, 0.0),
                (0.03, 0.347),
                (-2.0, 1.0),
                (-2.0, -1.0),
            ]),
            poly(&[
                (0.0, 0.0),
                (2.0, 0.0),
                (0.03, -0.347),
                (3.0, -3.0),
                (3.0, 3.0),
            ]),
        ],
        &stacked_at(&[0.0, 1.0]),
        1,
    );
}
