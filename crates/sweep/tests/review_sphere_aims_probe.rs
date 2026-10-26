//! Review probes (PR 5730): (a) every aimed ray at a sphere face of a
//! body with sphere patches, aimed anywhere on that face's sphere (in
//! the face or not), reads the solid's side or nothing — never the
//! other side; (b) what each body's faces, solids and shells read, for
//! a diff between two trees. Run:
//! `cargo nextest run -p sweep --run-ignored only -E 'test(review_sphere_aims)' --no-capture`.

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout, clippy::unwrap_used)]

use geom_core::{Band, Point2, Point3, Tol, Vec3};
use sweep::test_support::{
    ball_poled_y, ball_poled_z, bowl, brick, dome, domed_cavity, finished,
    hemisphere_on_flat_base, lantern, prism_at, sphere_zone,
};
use topo::{AtRestBody, BooleanError, BooleanResult};

fn moved(b: AtRestBody<f64>, c: Vec3<f64>, tol: Tol) -> AtRestBody<f64> {
    let m = topo::transform_rigid(
        &b,
        &geom_core::RigidMotion::translation(c).expect("a translation"),
        tol,
    )
    .expect("move");
    finished("moved", m, tol)
}

fn ball(r: f64, c: Vec3<f64>, tol: Tol) -> AtRestBody<f64> {
    moved(
        finished("ball", ball_poled_y(r, Vec3::new(0.0, 0.0, 0.0), tol), tol),
        c,
        tol,
    )
}

fn zball(r: f64, c: Vec3<f64>, tol: Tol) -> AtRestBody<f64> {
    finished("zball", ball_poled_z(r, c, tol), tol)
}

fn body(out: Result<BooleanResult<f64>, BooleanError>) -> Result<AtRestBody<f64>, String> {
    match out {
        Ok(r) => match r.body() {
            Some(b) => Ok(b.body.clone()),
            None => Err("empty".into()),
        },
        Err(e) => Err(format!("refused: {e:?}")),
    }
}

/// A z-axis rod of radius `r` from `z0` up `h`, at `(x, y)`.
fn rod(r: f64, (x, y): (f64, f64), z0: f64, h: f64, tol: Tol) -> AtRestBody<f64> {
    let p = prism_at(
        vec![(Point2::new(r, 0.0), 1.0), (Point2::new(-r, 0.0), 1.0)],
        z0,
        h,
        tol,
    );
    moved(finished("rod", p, tol), Vec3::new(x, y, 0.0), tol)
}

type Fx = (String, Result<AtRestBody<f64>, String>);

fn fixtures(tol: Tol) -> Vec<Fx> {
    let z = |d: f64| Vec3::new(0.0, 0.0, d);
    let o = Vec3::new(0.0, 0.0, 0.0);
    let unit = || ball(1.0, z(0.0), tol);
    let other = || ball(0.8, z(0.9), tol);
    let off = || ball(0.8, Vec3::new(0.13, 0.07, 0.9), tol);
    let block = || finished("block", brick((-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0), tol), tol);
    let mut out: Vec<Fx> = Vec::new();
    let mut push = |what: &str, f: &dyn Fn() -> Result<AtRestBody<f64>, String>| {
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f))
            .unwrap_or_else(|_| Err("panicked".into()));
        out.push((what.to_owned(), r));
    };
    push("lens", &|| body(topo::intersect(&unit(), &other(), tol)));
    push("two balls", &|| body(topo::union(&unit(), &other(), tol)));
    push("bitten ball", &|| body(topo::subtract(&unit(), &other(), tol)));
    push("bite", &|| body(topo::subtract(&other(), &unit(), tol)));
    push("lens off", &|| body(topo::intersect(&unit(), &off(), tol)));
    push("bitten off", &|| body(topo::subtract(&unit(), &off(), tol)));
    push("block less lens", &|| {
        let lens = body(topo::intersect(&unit(), &other(), tol))?;
        body(topo::subtract(&block(), &lens, tol))
    });
    push("block less two balls", &|| {
        let two = body(topo::union(&unit(), &other(), tol))?;
        body(topo::subtract(&block(), &two, tol))
    });
    // Symmetric poses: equal balls at z = ±0.5 (the aims' rays about the
    // plane z = 0 and every plane through the axis).
    push("sym lens", &|| {
        body(topo::intersect(&ball(1.0, z(-0.5), tol), &ball(1.0, z(0.5), tol), tol))
    });
    push("sym union", &|| {
        body(topo::union(&ball(1.0, z(-0.5), tol), &ball(1.0, z(0.5), tol), tol))
    });
    push("sym bite", &|| {
        body(topo::subtract(&ball(1.0, z(-0.5), tol), &ball(1.0, z(0.5), tol), tol))
    });
    push("block less sym lens", &|| {
        let lens = body(topo::intersect(&ball(1.0, z(-0.5), tol), &ball(1.0, z(0.5), tol), tol))?;
        body(topo::subtract(&block(), &lens, tol))
    });
    // A sliver lens: two unit balls 1.999 apart.
    push("sliver lens", &|| {
        body(topo::intersect(&ball(1.0, z(0.0), tol), &ball(1.0, z(1.999), tol), tol))
    });
    push("block less sliver lens", &|| {
        let lens = body(topo::intersect(&ball(1.0, z(0.0), tol), &ball(1.0, z(1.999), tol), tol))?;
        body(topo::subtract(&block(), &lens, tol))
    });
    // Very different radii.
    push("ball less pea", &|| body(topo::subtract(&unit(), &ball(0.05, z(1.0), tol), tol)));
    push("pea less ball", &|| body(topo::subtract(&ball(0.05, z(1.0), tol), &unit(), tol)));
    push("ball and pea lens", &|| body(topo::intersect(&unit(), &ball(0.05, z(1.0), tol), tol)));
    // A ball with a cylinder: coaxial with its pole, and off it.
    push("zball less coaxial rod", &|| body(topo::subtract(&zball(1.0, o, tol), &rod(0.3, (0.0, 0.0), -2.0, 4.0, tol), tol)));
    push("zball and coaxial rod", &|| body(topo::intersect(&zball(1.0, o, tol), &rod(0.3, (0.0, 0.0), -2.0, 4.0, tol), tol)));
    push("zball less off rod", &|| body(topo::subtract(&zball(1.0, o, tol), &rod(0.3, (0.2, 0.1), -2.0, 4.0, tol), tol)));
    // A ball and a box.
    push("zball less slab", &|| body(topo::subtract(&zball(1.0, o, tol), &finished("slab", brick((-2.0, 2.0), (-2.0, 2.0), (0.3, 2.0), tol), tol), tol)));
    push("zball and slab", &|| body(topo::intersect(&zball(1.0, o, tol), &finished("slab", brick((-2.0, 2.0), (-2.0, 2.0), (-0.3, 0.3), tol), tol), tol)));
    push("block less zball", &|| body(topo::subtract(&block(), &zball(1.0, o, tol), tol)));
    push("block less lens and a far ball", &|| {
        let lens = body(topo::intersect(&unit(), &other(), tol))?;
        let far = body(topo::union(&lens, &ball(0.3, Vec3::new(1.4, 1.4, -1.4), tol), tol))?;
        body(topo::subtract(&block(), &far, tol))
    });
    // An assembly: a lens and a far ball, two solids.
    push("lens and far ball", &|| {
        let lens = body(topo::intersect(&unit(), &other(), tol))?;
        body(topo::union(&lens, &ball(0.5, Vec3::new(5.0, 0.0, 0.0), tol), tol))
    });
    // Revolves with sphere patches.
    push("dome", &|| Ok(finished("dome", dome(1.0, tol), tol)));
    push("domed cavity", &|| Ok(finished("domed", domed_cavity(tol), tol)));
    push("sphere zone", &|| Ok(finished("zone", sphere_zone(0.5, sweep::Revolution::Full, tol), tol)));
    push("lantern", &|| Ok(finished("lantern", lantern(tol), tol)));
    push("bowl", &|| Ok(finished("bowl", bowl(tol), tol)));
    push("hemisphere", &|| Ok(finished("hemi", hemisphere_on_flat_base(1.0, tol), tol)));
    out
}

fn classify(b: &AtRestBody<f64>, tol: Tol) -> String {
    match topo::classify_shells(b, tol) {
        Ok(v) => format!("{:?}", v.iter().map(|c| format!("{:?}", c.role)).collect::<Vec<_>>()),
        Err(e) => format!("refused: {e:?}"),
    }
}

/// (b) Each body's readings, for a diff between two trees.
#[test]
#[ignore = "probe"]
fn review_sphere_aims_readings() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let eps = tol.eps();
    for (what, b) in fixtures(tol) {
        let b = match b {
            Ok(b) => b,
            Err(e) => {
                println!("RSA eps={eps:e} {what}: {e}");
                continue;
            }
        };
        let census = topo::validate_pseudomanifold(&b, &topo::ContactRecords::default(), tol).is_ok();
        println!("RSA eps={eps:e} {what}: census {census} faces {} shells {}", b.faces().count(), b.shells().count());
        for l in topo::test_support::each_aim(&b, band) {
            println!("RSA eps={eps:e} {what}:   face {l}");
        }
        for l in topo::test_support::aimed_rays(&b, band) {
            println!("RSA eps={eps:e} {what}:   solid {l}");
        }
        println!("RSA eps={eps:e} {what}:   roles {}", classify(&b, tol));
    }
}

/// (a) Aims anywhere on each sphere face's sphere, in the face or not:
/// a solid's aimed ray reads `Out` or nothing.
#[test]
#[ignore = "probe"]
fn review_sphere_aims_anywhere() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let eps = tol.eps();
    let n = 300;
    for (what, b) in fixtures(tol) {
        let Ok(b) = b else { continue };
        let mut tally: std::collections::BTreeMap<String, usize> = Default::default();
        for (face, f) in b.faces() {
            let Some(geom::Surface::Sphere { center, radius, .. }) = b.get_surface(f.surface).cloned() else {
                continue;
            };
            for i in 0..n {
                // A Fibonacci point on the sphere.
                let y = 1.0 - 2.0 * (i as f64 + 0.5) / n as f64;
                let r = (1.0 - y * y).sqrt();
                let a = i as f64 * geom_core::GOLDEN_ANGLE;
                let dir = Vec3::new(r * a.cos(), y, r * a.sin());
                let aim: Point3<f64> = center + dir * radius;
                let normal = if f.sense { dir } else { -dir };
                let read = topo::test_support::ray_at(&b, face, aim, normal, band);
                let key = if read.contains("Side(Out)") {
                    "Side(Out)".to_owned()
                } else if read.contains("Side(In)") {
                    "Side(In)".to_owned()
                } else {
                    read.split(['(', ' ', '{']).take(3).collect::<Vec<_>>().join("/")
                };
                *tally.entry(key).or_default() += 1;
            }
        }
        println!("RSA-ANY eps={eps:e} {what}: {tally:?}");
    }
}
