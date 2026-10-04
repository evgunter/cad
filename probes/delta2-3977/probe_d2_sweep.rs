//! Delta-2 reviewer probe (reach-delta2-3977): check 7, the shell role
//! and containment on far, thin bodies, curved and planar, upright and
//! inside out, run on the PR head and on main. Oracle: the sign is the
//! construction's (upright +, reverted −); the magnitude is a closed
//! form (π r² h, Pappus, the spherical-shell sector, n-gon area × h),
//! printed beside the kernel's f64 sum, and `V/A` over the band says
//! whether the sign is above the band at all.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::test_support::{bored_block_of_arcs, cylinder_of_arcs_at, prism_at, revolved_about_y};
use topo::{classify_shells, mass_properties, transform_rigid, validate_geometric, Body};

fn place(b: &Body<f64>, angle: f64, at: Vec3<f64>, tol: Tol) -> Option<Body<f64>> {
    let b = if angle == 0.0 {
        b.clone()
    } else {
        let r = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.3, 0.5, 0.81), angle);
        transform_rigid(b, &r, tol).ok()?
    };
    transform_rigid(&b, &Affine3::translation(at), tol).ok()
}

fn verdict(tag: &str, body: &Body<f64>, exact: f64, area: f64, tol: Tol) {
    let band = Band::linear(tol).unwrap();
    let v = mass_properties(body, tol).map(|p| p.volume).unwrap_or(f64::NAN);
    let g = validate_geometric(body, tol);
    let cls = classify_shells(body, tol).map(|v| v.iter().map(|c| format!("{:?}", c.role)).collect::<Vec<_>>());
    let flag = match (&g, exact > 0.0) {
        (Ok(()), false) => " <-- INSIDE-OUT PASSES",
        (Err(_), true) => " <-- VALID REFUSED",
        _ => "",
    };
    let gs: String = format!("{g:?}").chars().take(80).collect();
    let cs: String = format!("{cls:?}").chars().take(60).collect();
    println!(
        "D2|{tag}|exact={exact:+.3e}|f64={v:+.3e}|VA/esc={:.1}|tier3={gs}|classify={cs}{flag}",
        (exact / area).abs() / band.escalate()
    );
}

fn both(tag: &str, b: Option<Body<f64>>, exact: f64, area: f64, tol: Tol) {
    let Some(b) = b else {
        println!("D2|{tag}|no build");
        return;
    };
    verdict(&format!("{tag}|upright"), &b, exact, area, tol);
    match b.revert() {
        Ok(inv) => verdict(&format!("{tag}|INVERTED"), &inv, -exact, area, tol),
        Err(e) => println!("D2|{tag}|revert {e:?}"),
    }
}

fn built(f: impl FnOnce() -> Body<f64> + std::panic::UnwindSafe) -> Option<Body<f64>> {
    std::panic::catch_unwind(f).ok()
}

const DIRS: [(f64, f64, f64); 3] = [(0.6, 0.48, 0.64), (-0.6, -0.48, -0.64), (0.0, 0.0, 1.0)];
const DISTS: [f64; 4] = [1e3, 5e3, 1e4, 2e4];
const ANGLES: [f64; 2] = [0.0, 0.7];

/// The n-arc disc family over directions, rotations and arc counts.
#[test]
fn d2_disc_family() {
    let tol = Tol::witness();
    println!("D2|disc|eps={:e}", tol.eps());
    for &n in &[4usize, 64] {
        for &(r, h) in &[(1e-3, 1e-7), (1e-3, 1e-6), (1e-3, 1e-5), (1e-2, 1e-6)] {
            if h < 15.0 * tol.eps() {
                continue;
            }
            let Some(origin) = built(move || cylinder_of_arcs_at(n, r, Point2::new(0.0, 0.0), 0.0, h, tol)) else {
                println!("D2|disc n={n} r={r:e} h={h:e}|no build at origin");
                continue;
            };
            let exact = std::f64::consts::PI * r * r * h;
            let area = 2.0 * std::f64::consts::PI * r * r;
            for dir in DIRS {
                for d in DISTS {
                    for angle in ANGLES {
                        let at = Vec3::new(dir.0 * d, dir.1 * d, dir.2 * d);
                        let tag = format!("disc n={n} r={r:e} h={h:e} dir={dir:?} d={d:e} a={angle}");
                        both(&tag, place(&origin, angle, at, tol), exact, area, tol);
                    }
                }
            }
        }
    }
}

/// Thin revolved bodies: a washer (planes + cylinders), a spherical
/// shell sector (spheres + cones), a thin torus tube.
#[test]
fn d2_revolved_kinds() {
    use std::f64::consts::PI;
    let tol = Tol::witness();
    println!("D2|rev|eps={:e}", tol.eps());
    let mut shapes: Vec<(String, Vec<(Point2<f64>, f64)>, f64, f64)> = Vec::new();
    for &h in &[1e-7, 1e-6] {
        if h < 15.0 * tol.eps() {
            continue;
        }
        let (rr, w) = (1e-3, 1e-3);
        shapes.push((
            format!("washer h={h:e}"),
            vec![
                (Point2::new(rr, 0.0), 0.0),
                (Point2::new(rr + w, 0.0), 0.0),
                (Point2::new(rr + w, h), 0.0),
                (Point2::new(rr, h), 0.0),
            ],
            PI * ((rr + w).powi(2) - rr * rr) * h,
            2.0 * PI * ((rr + w).powi(2) - rr * rr),
        ));
        // spherical shell sector between latitudes 30° and 60°, radii ρ, ρ+t
        let (rho, t) = (1e-3, h);
        let (a1, a2) = (PI / 6.0, PI / 3.0);
        let b = ((a2 - a1) / 4.0).tan();
        let p = |r: f64, a: f64| Point2::new(r * a.cos(), r * a.sin());
        shapes.push((
            format!("sphshell t={t:e}"),
            vec![
                (p(rho, a1), 0.0),
                (p(rho + t, a1), b),
                (p(rho + t, a2), 0.0),
                (p(rho, a2), -b),
            ],
            2.0 * PI / 3.0 * ((rho + t).powi(3) - rho.powi(3)) * (a2.sin() - a1.sin()),
            2.0 * 2.0 * PI * rho * rho * (a2.sin() - a1.sin()),
        ));
    }
    for &a in &[2e-7, 2e-6] {
        if a < 15.0 * tol.eps() {
            continue;
        }
        let rr = 1e-3;
        shapes.push((
            format!("torus a={a:e}"),
            vec![(Point2::new(rr - a, 0.0), 1.0), (Point2::new(rr + a, 0.0), 1.0)],
            2.0 * PI * PI * rr * a * a,
            4.0 * PI * PI * rr * a,
        ));
    }
    for (name, prof, exact, area) in shapes {
        let p2 = prof.clone();
        let Some(origin) = built(move || revolved_about_y(p2, sweep::Revolution::Full, tol)) else {
            println!("D2|{name}|no build at origin");
            continue;
        };
        for dir in DIRS {
            for d in [0.0, 1e3, 5e3, 2e4] {
                for angle in ANGLES {
                    let at = Vec3::new(dir.0 * d, dir.1 * d, dir.2 * d);
                    both(&format!("{name} dir={dir:?} d={d:e} a={angle}"), place(&origin, angle, at, tol), exact, area, tol);
                }
            }
        }
    }
}

/// Planar prisms the fan formula reads: many loop points, a sliver
/// triangle, a holed plate (ring faces and an arc bore), all thin,
/// rotated and far.
#[test]
fn d2_planar_fans() {
    use std::f64::consts::PI;
    let tol = Tol::witness();
    println!("D2|fan|eps={:e}", tol.eps());
    for &h in &[1e-7, 1e-6] {
        if h < 15.0 * tol.eps() {
            continue;
        }
        let mut shapes: Vec<(String, Box<dyn Fn() -> Body<f64> + std::panic::RefUnwindSafe>, f64, f64)> = Vec::new();
        for &n in &[3usize, 17, 256] {
            let r = 1e-3;
            let a = 0.5 * n as f64 * r * r * (2.0 * PI / n as f64).sin();
            let verts: Vec<(Point2<f64>, f64)> = (0..n)
                .map(|i| {
                    let th = 2.0 * PI * i as f64 / n as f64;
                    (Point2::new(r * th.cos(), r * th.sin()), 0.0)
                })
                .collect();
            shapes.push((format!("ngon n={n} h={h:e}"), Box::new(move || prism_at(verts.clone(), 0.0, h, tol)), a * h, 2.0 * a));
        }
        // sliver triangle: base 2 mm, apex 1e-6 rad
        let (l, s) = (2e-3, 2e-3 * 1e-3);
        let tri = vec![(Point2::new(0.0, 0.0), 0.0), (Point2::new(l, 0.0), 0.0), (Point2::new(l, s), 0.0)];
        shapes.push((format!("sliver h={h:e}"), Box::new(move || prism_at(tri.clone(), 0.0, h, tol)), 0.5 * l * s * h, l * s));
        // holed plate: 4 mm square, 1 mm bore of 8 arcs
        let (pl, pr) = (4e-3, 1e-3);
        let pa = pl * pl - PI * pr * pr;
        shapes.push((format!("holed h={h:e}"), Box::new(move || bored_block_of_arcs(8, pl, h, pr, tol)), pa * h, 2.0 * pa));
        for (name, mk, exact, area) in shapes {
            let Some(origin) = built(std::panic::AssertUnwindSafe(|| mk())) else {
                println!("D2|{name}|no build at origin");
                continue;
            };
            for dir in DIRS {
                for d in [0.0, 1e3, 5e3, 2e4] {
                    for angle in ANGLES {
                        let at = Vec3::new(dir.0 * d, dir.1 * d, dir.2 * d);
                        both(&format!("{name} dir={dir:?} d={d:e} a={angle}"), place(&origin, angle, at, tol), exact, area, tol);
                    }
                }
            }
        }
    }
}

/// A quadrature face: a cylinder cut by a tilted thin slab, so its wall
/// is trimmed by two ellipses (the certified-quadrature lane), placed far.
/// The arm `VolumeSignUnresolved` guards.
#[test]
fn d2_quadrature_kind() {
    let tol = Tol::witness();
    println!("D2|quad|eps={:e}", tol.eps());
    for &h in &[1e-7, 1e-6, 1e-5] {
        if h < 15.0 * tol.eps() {
            continue;
        }
        for &tilt in &[0.3, 0.8] {
            let r = 1e-3;
            let mk = move || {
                let cyl = cylinder_of_arcs_at(4, r, Point2::new(0.0, 0.0), -3e-3, 6e-3, tol);
                let slab = sweep::test_support::brick::<f64>((-4e-3, 4e-3), (-4e-3, 4e-3), (0.0, h), tol);
                let rot = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), tilt);
                let slab = transform_rigid(&slab, &rot, tol).unwrap();
                topo::intersect(&cyl, &slab, tol).unwrap().body().unwrap().body.clone()
            };
            let Some(origin) = built(mk) else {
                println!("D2|quad h={h:e} tilt={tilt}|no build at origin");
                continue;
            };
            // the slab's thickness is h along its normal; the cut area is the
            // ellipse π r² / cos(tilt)
            let exact = std::f64::consts::PI * r * r * h / tilt.cos();
            let area = 2.0 * std::f64::consts::PI * r * r / tilt.cos();
            for dir in DIRS {
                for d in [0.0, 1e3, 5e3, 2e4] {
                    for angle in ANGLES {
                        let at = Vec3::new(dir.0 * d, dir.1 * d, dir.2 * d);
                        both(&format!("quad h={h:e} tilt={tilt} dir={dir:?} d={d:e} a={angle}"), place(&origin, angle, at, tol), exact, area, tol);
                    }
                }
            }
        }
    }
}
