//! **`shell_open` through a curved designated face.**

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol, Vec2};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::test_support::{finished, revolved_about_y};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, FaceKey, LoopBoundary};

/// `lp` revolved a full turn about the `y` axis.
fn revolved(lp: ProfileLoop<f64>) -> Body<f64> {
    let tol = Tol::witness();
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .expect("the meridian validates");
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&profile, axis, Revolution::Full, tol)
        .expect("the meridian revolves")
        .body
}

/// A vessel of revolution: a cylinder of radius `r` and height `h`
/// capped by a hemisphere of the same radius.
fn dome_vessel(r: f64, h: f64) -> Body<f64> {
    let bulge = (core::f64::consts::FRAC_PI_8).tan();
    revolved(
        bulge_loop(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(r, h), bulge),
            (Point2::new(0.0, h + r), 0.0),
        ])
        .with_tangent_joints(vec![2]),
    )
}

/// A cylinder of radius `r`, height `h`, capped by a spherical cap
/// spanning `deg` degrees of arc (not tangent to the wall).
fn cap_vessel(r: f64, h: f64, deg: f64) -> Body<f64> {
    let half = (deg / 2.0).to_radians();
    let rho = r / (2.0 * half).sin();
    let rise = rho * (1.0 - (2.0 * half).cos());
    revolved(bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(r, 0.0), 0.0),
        (Point2::new(r, h), (half / 2.0).tan()),
        (Point2::new(0.0, h + rise), 0.0),
    ]))
}

/// A cylinder of radius `r`, height `h`, then a sphere zone up `deg`
/// degrees of arc from a non-tangent junction, then a flat top.
fn zone_vessel(r: f64, h: f64, deg: f64) -> Body<f64> {
    // A sphere centred on the axis at height h - r/2, through (r, h).
    let c = h - 0.5 * r;
    let rho = (r * r + 0.25 * r * r).sqrt();
    let a0 = (0.5 * r / rho).asin();
    let a1 = a0 + deg.to_radians();
    let top = (rho * a1.cos(), c + rho * a1.sin());
    revolved(bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(r, 0.0), 0.0),
        (Point2::new(r, h), (deg.to_radians() / 4.0).tan()),
        (Point2::new(top.0, top.1), 0.0),
        (Point2::new(0.0, top.1), 0.0),
    ]))
}

/// A frustum-capped cylinder: a cone zone from `(r, h)` to `(r2, h + k)`.
fn frustum_vessel(r: f64, h: f64, r2: f64, k: f64) -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(r, h), 0.0),
            (Point2::new(r2, h + k), 0.0),
            (Point2::new(0.0, h + k), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

/// A vessel of revolution: a cylinder of radius `r` and height `h`
/// capped by a cone of height `k`.
fn cone_vessel(r: f64, h: f64, k: f64) -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(r, h), 0.0),
            (Point2::new(0.0, h + k), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

fn chart_of(body: &Body<f64>, kind: geom::SurfaceKind) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| body.get_surface(f.surface).map(|s| s.kind()) == Some(kind))
        .map(|(k, _)| k)
        .collect()
}

fn dump(body: &Body<f64>, faces: &[FaceKey]) {
    let pts: std::collections::HashMap<_, _> = body.vertex_points().collect();
    for &f in faces {
        let data = body.get_face(f).unwrap();
        eprintln!(
            "face {f:?} {:?} sense {} rings {}",
            body.get_surface(data.surface).map(|s| s.kind()),
            data.sense,
            data.rings.len()
        );
        for lk in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
            let LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
                eprintln!("  loop {lk:?} lone");
                continue;
            };
            eprintln!("  loop {lk:?}");
            for he in body.loop_cycle(first).unwrap() {
                let h = body.get_half_edge(he).unwrap();
                let p = pts[&h.start];
                eprintln!(
                    "    he {he:?} edge {:?} from {:?} ({:.4},{:.4},{:.4})",
                    h.edge, h.start, p.x, p.y, p.z
                );
            }
        }
    }
}

fn probe(what: &str, body: Body<f64>, kind: geom::SurfaceKind, t: f64) {
    let tol = Tol::witness();
    let chart = chart_of(&body, kind);
    eprintln!("==== {what}: designated {kind:?} chart");
    dump(&body, &chart);
    match topo::shell_open(&finished("the operand", body.clone(), tol), t, &chart, tol) {
        Ok(s) => {
            eprintln!("built: shells {}", s.body.shells().count());
            let faces: Vec<FaceKey> = s.body.faces().map(|(k, _)| k).collect();
            dump(&s.body, &faces);
        }
        Err(e) => eprintln!("refused: {e}\n  {e:?}"),
    }
}

fn cap(rho: f64, h: f64) -> f64 {
    core::f64::consts::PI * h * h * (3.0 * rho - h) / 3.0
}

#[test]
fn probe_cap_volume() {
    let tol = Tol::witness();
    let (r, h, deg, t) = (0.5, 0.6, 60.0_f64, 0.05);
    let body = cap_vessel(r, h, deg);
    let chart = chart_of(&body, geom::SurfaceKind::Sphere);
    let s = topo::shell_open(&finished("the operand", body.clone(), tol), t, &chart, tol).unwrap();
    let half = (deg / 2.0).to_radians();
    let rho = r / (2.0 * half).sin();
    let rise = rho * (1.0 - (2.0 * half).cos());
    let cy = h + rise - rho;
    let a = r - t;
    let ya = cy + (rho * rho - a * a).sqrt();
    let outer = core::f64::consts::PI * r * r * h + cap(rho, rise);
    let cavity =
        core::f64::consts::PI * a * a * (ya - t) + cap(rho, rho - (rho * rho - a * a).sqrt());
    let props = topo::mass_properties(&s.body, tol).unwrap();
    eprintln!(
        "volume {} pad {} want {}",
        props.volume,
        props.volume_pad,
        outer - cavity
    );
    for d in [1e-2, 1e-3] {
        match mesh::tessellate(&s.body, d, tol) {
            Ok(_) => eprintln!("mesh {d}: ok"),
            Err(e) => eprintln!("mesh {d}: {e:?}"),
        }
    }
    eprintln!("rims {:?}", s.naming.rims);
}

#[test]
fn probe_curved_mouths() {
    probe(
        "cap60",
        cap_vessel(0.5, 0.6, 60.0),
        geom::SurfaceKind::Sphere,
        0.05,
    );
    probe(
        "zone",
        zone_vessel(0.5, 0.6, 30.0),
        geom::SurfaceKind::Sphere,
        0.05,
    );
    probe(
        "frustum",
        frustum_vessel(0.5, 0.6, 0.3, 0.3),
        geom::SurfaceKind::Cone,
        0.05,
    );
}
