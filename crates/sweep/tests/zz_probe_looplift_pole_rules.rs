//! **Probe (ONEDOOR loop lift, PR 6851, probe a): the two pole rules
//! on the pole fixtures.** A measurement, not a product test: it prints
//! and asserts nothing about the answer.
//!
//! For every sphere face of the pole fixtures (`pole_slit_window.rs`'s
//! dome and slits, the mesh sphere-arm suites' balls, wedges and the
//! tilted pair) it reports:
//!
//! - `chart_window`: `face_azimuth_window_traces` (the walk whose pole
//!   joints `pole_branch` decides), its window or its refusal;
//! - `stored`: the stored joint elements at the loop's pole vertices;
//! - `area`: the rule of `mesh::trimmed::choose_pole_azimuths`,
//!   transcribed (it is private) and run on the stored lift
//!   (`Body::loop_lift`) sampled at 64 points per image: each run
//!   between poles tried at `−2π, 0, +2π`, kept where the equal-area
//!   shoelace times the sense is in `(0, 4π)`, ties broken by the
//!   shortest pole sides. Its placed polygon's azimuth hull;
//! - `arm`: whether the real mesh runs `choose_pole_azimuths` on the
//!   face at all (the trimmed sphere arm is taken only where the outer
//!   loop stores a `Projected` image, `trimmed::takes_sphere_arm`);
//! - `mesh`: the real tessellation's patch, read in 3-D: the azimuth arc
//!   its triangle centroids cover (the complement of their largest
//!   azimuth gap) and its area against the face's closed-form area.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use geom_brep::Pcurve;
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{ball_poled_y, finished, revolved_about_y_at};
use topo::construction::Construct;
use topo::{Body, FaceKey, LoopBoundary, face_azimuth_window_traces, validate};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// `pole_slit_window.rs`'s unit dome, its base merged to one disc.
fn dome() -> Body<f64> {
    let bulge = (FRAC_PI_2 / 4.0).tan();
    let mut body = revolved_about_y_at(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), bulge),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    );
    body.merge_coplanar_faces(Tol::witness()).unwrap();
    body
}

/// `pole_slit_window.rs`'s slits: each band meridian killed from each
/// half.
fn slits(body: &Body<f64>) -> Vec<(String, Body<f64>)> {
    let on_sphere = |he| {
        body.face_of_half_edge(he)
            .and_then(|face| body.face_surface(face))
            .is_some_and(|s| matches!(s, geom::Surface::Sphere { .. }))
    };
    let meridians: Vec<_> = body
        .edges()
        .filter(|(_, e)| on_sphere(e.he_plus) && on_sphere(e.he_minus))
        .map(|(_, e)| (e.he_plus, e.he_minus))
        .collect();
    meridians
        .into_iter()
        .enumerate()
        .flat_map(|(i, (plus, minus))| [(i, "plus", plus), (i, "minus", minus)])
        .map(|(i, half, he)| {
            let mut slit = body.clone();
            slit.kef(he).unwrap();
            assert_eq!(validate(&slit), Ok(()));
            (format!("dome slit (meridian {i}, killed from {half})"), slit)
        })
        .collect()
}

/// The mesh suites' ball (`common::ball`, = `ball_poled_y(1, 0)`).
fn ball() -> Body<f64> {
    ball_poled_y(1.0, Vec3::new(0.0, 0.0, 0.0), Tol::witness())
}

/// The mesh suites' `common::sphere_wedge`: the half-disc revolved
/// `theta` about `y`.
fn sphere_wedge(theta: f64) -> Body<f64> {
    revolved_about_y_at(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Partial(theta),
        Tol::witness(),
    )
}

/// `mesh::trimmed`'s `tilted_sphere_pair`: two unit balls 1.4 apart,
/// poles on `y`, unioned. The fixture `choose_pole_azimuths`' own row
/// runs on.
fn tilted_sphere_pair() -> Body<f64> {
    let tol = Tol::witness();
    let b = |c: Vec3<f64>| finished("the ball", ball_poled_y(1.0, c, tol), tol);
    topo::boolean::union(
        &b(Vec3::new(0.0, 0.0, 0.0)),
        &b(Vec3::new(1.4, 0.0, 0.0)),
        tol,
    )
    .unwrap()
    .body()
    .unwrap()
    .body
    .clone()
    .into_body()
}

fn hemisphere() -> Body<f64> {
    sweep::test_support::hemisphere_on_flat_base(1.0, Tol::witness())
}

/// The sphere's `(center, radius, axis, u_ref)`.
fn sphere_of(s: &geom::Surface<f64>) -> Option<(Point3<f64>, f64, Vec3<f64>, Vec3<f64>)> {
    match *s {
        geom::Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => Some((center, radius, axis, u_ref)),
        _ => None,
    }
}

fn at_pole(v: f64) -> bool {
    FRAC_PI_2 - v.abs() <= 64.0 * f64::EPSILON
}

/// The outer loop's lift, sampled: one traversal of `(u, v)` per
/// half-edge in walk order, 65 points each.
fn sampled_lift(body: &Body<f64>, fk: FaceKey) -> Result<Vec<Vec<(f64, f64)>>, String> {
    let face = body.get_face(fk).unwrap();
    let rows = body
        .loop_lift(face.outer)
        .map_err(|e| format!("loop_lift: {e:?}"))?;
    Ok(rows
        .iter()
        .map(|row| {
            let he = body.get_half_edge(row.half_edge).unwrap();
            let forward = body.get_edge(he.edge).unwrap().he_plus == row.half_edge;
            let (t0, t1) = row.row.params();
            let (a, b) = if forward { (t0, t1) } else { (t1, t0) };
            (0..=64)
                .map(|i| {
                    let t = a + (b - a) * f64::from(i) / 64.0;
                    let p = row.pcurve.eval(t);
                    (p.x, p.y)
                })
                .collect()
        })
        .collect())
}

/// `choose_pole_azimuths`' rule, transcribed, on `travs`: the per-run
/// shift (in turns) it picks, or why it refuses.
fn area_rule(travs: &[Vec<(f64, f64)>], sense: bool) -> Result<Vec<f64>, String> {
    let m = travs.len();
    let junctions: Vec<usize> = (0..m)
        .filter(|&k| at_pole(travs[k][travs[k].len() - 1].1))
        .collect();
    let j = junctions.len();
    if j < 2 {
        return Ok(vec![0.0; m]);
    }
    if j > 6 {
        return Err(format!("PoleJunctions ({j})"));
    }
    let run: Vec<usize> = (0..m)
        .map(|k| junctions.iter().filter(|&&p| p < k).count() % j)
        .collect();
    let placements = 3usize.pow(u32::try_from(j - 1).unwrap());
    let shift = |placement: usize, s: usize| -> f64 {
        if s == 0 {
            return 0.0;
        }
        let digit = (placement / 3usize.pow(u32::try_from(s - 1).unwrap())) % 3;
        f64::from(u32::try_from(digit).unwrap()) - 1.0
    };
    let want = if sense { 1.0 } else { -1.0 };
    let (mut best, mut tied): (Option<(f64, usize)>, bool) = (None, false);
    let mut fits = Vec::new();
    for placement in 0..placements {
        let pts = flat(travs, |k| shift(placement, run[k]) * TAU);
        let n = pts.len();
        let (mut area2, mut pole_len) = (0.0, 0.0);
        for i in 0..n {
            let (a, b) = (pts[i], pts[(i + 1) % n]);
            area2 += a.0 * b.1.sin() - b.0 * a.1.sin();
            if at_pole(a.1) && at_pole(b.1) && (a.1 - b.1).abs() < 1e-9 {
                pole_len += (b.0 - a.0).abs();
            }
        }
        let area = want * 0.5 * area2;
        if !(area > 0.0 && area < 2.0 * TAU) {
            continue;
        }
        fits.push((placement, area, pole_len));
        match best {
            Some((len, _)) if (pole_len - len).abs() < PI => tied = true,
            Some((len, _)) if pole_len > len => {}
            _ => {
                best = Some((pole_len, placement));
                tied = false;
            }
        }
    }
    let Some((_, placement)) = best else {
        return Err("PoleAzimuth (no placement fits the area window)".into());
    };
    if tied {
        return Err(format!("PoleAzimuth (tie): fits {fits:?}"));
    }
    Ok((0..m).map(|k| shift(placement, run[k])).collect())
}

/// The traversals joined, each moved by `du(k)`, a pole kept as a side
/// (both ends of a pole junction kept, as `flat_points` keeps them).
fn flat(travs: &[Vec<(f64, f64)>], du: impl Fn(usize) -> f64) -> Vec<(f64, f64)> {
    let m = travs.len();
    let mut out = Vec::new();
    for k in 0..m {
        let t = &travs[k];
        out.extend(t[..t.len() - 1].iter().map(|&(u, v)| (u + du(k), v)));
        let last = (t[t.len() - 1].0 + du(k), t[t.len() - 1].1);
        let next = (travs[(k + 1) % m][0].0 + du((k + 1) % m), travs[(k + 1) % m][0].1);
        #[allow(clippy::float_cmp)]
        if at_pole(last.1) && last != next {
            out.push(last);
        }
    }
    out
}

fn hull(pts: &[(f64, f64)]) -> (f64, f64) {
    pts.iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| {
            (lo.min(p.0), hi.max(p.0))
        })
}

/// The azimuth of a point on the sphere, in the chart's own convention
/// (read off `Surface::eval`).
fn azimuth(s: &geom::Surface<f64>, p: Point3<f64>) -> f64 {
    let (c, _, axis, u_ref) = sphere_of(s).unwrap();
    let q = s.eval(FRAC_PI_2, 0.0) - c;
    let w = q - axis * q.dot(axis);
    let d = p - c;
    d.dot(w).atan2(d.dot(u_ref))
}

/// The arc a set of azimuths covers: `(start, width)`, the complement of
/// the largest gap between them.
fn covered_arc(mut az: Vec<f64>) -> (f64, f64) {
    az.sort_by(f64::total_cmp);
    let n = az.len();
    let (mut gap, mut at) = (TAU - (az[n - 1] - az[0]), 0);
    for i in 1..n {
        if az[i] - az[i - 1] > gap {
            gap = az[i] - az[i - 1];
            at = i;
        }
    }
    (az[at], TAU - gap)
}

fn wrap(x: f64) -> f64 {
    x.rem_euclid(TAU)
}

/// Whether the face's outer loop stores a `Projected` image: the mesh
/// takes its trimmed sphere arm (and so `choose_pole_azimuths`) only
/// then.
fn takes_sphere_arm(body: &Body<f64>, fk: FaceKey) -> bool {
    let face = body.get_face(fk).unwrap();
    let LoopBoundary::Cycle { first } = body.get_loop(face.outer).unwrap().boundary else {
        return false;
    };
    body.loop_cycle(first).unwrap().into_iter().any(|he| {
        body.pcurve(he)
            .is_some_and(|c| matches!(c.pcurve(), Pcurve::Projected(_)))
    })
}

fn pole_elements(body: &Body<f64>, fk: FaceKey) -> Vec<String> {
    let face = body.get_face(fk).unwrap();
    let s = body.face_surface(fk).unwrap();
    let (c, r, axis, _) = sphere_of(s).unwrap();
    let mut out = Vec::new();
    for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
            continue;
        };
        for he in body.loop_cycle(first).unwrap() {
            let h = body.get_half_edge(he).unwrap();
            let p = body
                .vertex_points()
                .find(|(v, _)| *v == h.start)
                .map(|(_, p)| p)
                .unwrap();
            let d = (p - c).dot(axis);
            if (r - d.abs()).abs() < 1e-9 {
                out.push(format!(
                    "{}:{:?}",
                    if d > 0.0 { "N" } else { "S" },
                    body.joint(he)
                ));
            }
        }
    }
    out
}

fn report(name: &str, body: &Body<f64>) {
    let mesh = mesh::tessellate(body, 0.02, Tol::witness());
    for (fk, face) in body.faces() {
        let Some(s) = body.face_surface(fk) else {
            continue;
        };
        if sphere_of(s).is_none() {
            continue;
        }
        println!("== {name} / face {fk:?} sense {}", face.sense);
        println!("   stored pole elements: {:?}", pole_elements(body, fk));
        let cw = face_azimuth_window_traces(body, fk, band());
        let cw_win = match &cw {
            Ok(Some((lo, hi))) => {
                println!(
                    "   chart_window: [{lo:.6}, {hi:.6}] width {:.6} (lo mod τ {:.6})",
                    hi - lo,
                    wrap(*lo)
                );
                Some((*lo, *hi))
            }
            other => {
                println!("   chart_window: {other:?}");
                None
            }
        };
        let arm = takes_sphere_arm(body, fk);
        let area_win = match sampled_lift(body, fk) {
            Err(e) => {
                println!("   area rule: no lift: {e}");
                None
            }
            Ok(travs) => {
                let lift_hull = hull(&travs.concat());
                match area_rule(&travs, face.sense) {
                    Ok(shifts) => {
                        let placed: Vec<(f64, f64)> = travs
                            .iter()
                            .zip(&shifts)
                            .flat_map(|(t, s)| t.iter().map(move |&(u, v)| (u + s * TAU, v)))
                            .collect();
                        let (lo, hi) = hull(&placed);
                        println!(
                            "   area rule (transcribed, on the stored lift): shifts {shifts:?} \
                             -> [{lo:.6}, {hi:.6}] width {:.6} (lo mod τ {:.6}); lift as \
                             stored [{:.6}, {:.6}]",
                            hi - lo,
                            wrap(lo),
                            lift_hull.0,
                            lift_hull.1
                        );
                        Some((lo, hi))
                    }
                    Err(e) => {
                        println!("   area rule (transcribed): refuses {e}");
                        None
                    }
                }
            }
        };
        println!("   mesh runs choose_pole_azimuths on it: {arm}");
        match &mesh {
            Ok(m) => {
                let patch = m.patches.iter().find(|p| p.face == fk);
                match patch {
                    Some(p) if !p.triangles.is_empty() => {
                        let (c, r, _, _) = sphere_of(s).unwrap();
                        let mut area = 0.0;
                        let az: Vec<f64> = p
                            .triangles
                            .iter()
                            .map(|t| {
                                let [a, b, cc] = t.map(|i| m.positions[i as usize]);
                                area += 0.5 * (b - a).cross(cc - a).norm();
                                let g = Point3::new(
                                    (a.x + b.x + cc.x) / 3.0,
                                    (a.y + b.y + cc.y) / 3.0,
                                    (a.z + b.z + cc.z) / 3.0,
                                );
                                let g = c + (g - c) * (r / (g - c).norm());
                                azimuth(s, g)
                            })
                            .collect();
                        let (start, width) = covered_arc(az);
                        println!(
                            "   mesh patch: {} triangles, area {area:.4}, covers azimuth from \
                             {:.4} width {width:.4}",
                            p.triangles.len(),
                            wrap(start)
                        );
                    }
                    _ => println!("   mesh patch: none / empty"),
                }
            }
            Err(e) => println!("   mesh: tessellate refuses: {e:?}"),
        }
        let verdict = match (cw_win, area_win) {
            (Some((a, b)), Some((c, d))) => {
                let same_width = ((b - a) - (d - c)).abs() < 1e-6;
                let same_place = {
                    let x = wrap(a - c);
                    x < 1e-6 || TAU - x < 1e-6
                };
                if same_width && same_place {
                    "AGREE".to_string()
                } else {
                    format!("DISAGREE (width {} vs {}, lo {} vs {})", b - a, d - c, a, c)
                }
            }
            _ => "NO COMPARISON (one side refuses or has no lift)".to_string(),
        };
        println!("   verdict chart_window vs area rule: {verdict}");
    }
}

#[test]
fn probe_pole_rule_agreement() {
    let d = dome();
    report("dome", &d);
    for (name, slit) in slits(&d) {
        report(&name, &slit);
    }
    report("ball", &ball());
    for theta in [PI / 4.0, PI, 1.5 * PI] {
        report(&format!("sphere_wedge({theta:.4})"), &sphere_wedge(theta));
    }
    report("hemisphere_on_flat_base", &hemisphere());
    report("tilted_sphere_pair", &tilted_sphere_pair());
}
