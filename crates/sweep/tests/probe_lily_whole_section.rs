//! **Probe: what C3's whole-section readings say about lily wall 8's
//! face pairs**, asked directly of `PlaneSection::all` and
//! `Section::all`, outside any boolean. A designer's measurement for the
//! interior-loop guard's spline arms; it asserts nothing.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_brep::ssi::SsiDomain;
use geom_core::{Band, Point2, Point3, RigidMotion, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, SketchPlane};
use sweep::loft_body;

fn lens([width, ridge, keel]: [f64; 3]) -> Vec<ProfileLoop<f64>> {
    let half = 0.5 * width;
    vec![bulge_loop(vec![
        (Point2::new(half, 0.0), -ridge / half),
        (Point2::new(-half, 0.0), -keel / half),
    ])]
}

fn blade(
    d: Vec3<f64>,
    v: Vec3<f64>,
    (len, curl): (f64, f64),
    section: impl Fn(f64) -> [f64; 3],
    tol: Tol,
) -> topo::AtRestBody<f64> {
    const STATIONS: usize = 5;
    let band = Band::linear(tol).expect("the run's tolerance forms a band");
    let u = v.cross(d);
    let r = len / curl;
    let mut sections = Vec::with_capacity(STATIONS);
    let mut places = Vec::with_capacity(STATIONS);
    for k in 0..STATIONS {
        #[allow(clippy::cast_precision_loss)]
        let s = (k as f64) / ((STATIONS - 1) as f64);
        let (sa, ca) = (curl * s).sin_cos();
        let p = Point3::origin() + d * (r * sa) + v * (r * (1.0 - ca));
        let vk = v * ca - d * sa;
        sections.push(lens(section(s)));
        places.push(SketchPlane::from_frame(
            RigidMotion::gram_schmidt(p, u, vk, "sheath_blade", band)
                .expect("the station's two directions span a plane"),
        ));
    }
    let lofted = loft_body::<f64>(&sections, &places, 3, tol)
        .expect("the lens blade lofts")
        .body;
    topo::test_support::finished("a lens blade", lofted, tol)
}

/// The body's base cap (the plane through the origin normal to `x`) and
/// its NURBS walls, in face order.
fn parts(body: &topo::Body<f64>) -> (Surface<f64>, Vec<std::sync::Arc<NurbsSurface<f64>>>) {
    let mut cap = None;
    let mut walls = Vec::new();
    for (_, face) in body.faces() {
        match body.get_surface(face.surface).expect("a face has a surface") {
            s @ Surface::Plane { origin, normal, .. } => {
                if normal.y.abs() < 1e-9 && normal.z.abs() < 1e-9 && origin.x.abs() < 1e-9 {
                    cap = Some(s.clone());
                }
            }
            Surface::Nurbs(n) => walls.push(n.clone()),
            _ => {}
        }
    }
    (cap.expect("a base cap on x = 0"), walls)
}

fn domain(walls: &[&NurbsSurface<f64>]) -> SsiDomain {
    let extent = 2.0;
    let reach = walls
        .iter()
        .flat_map(|w| w.control().iter().map(|c| (*c - Point3::origin()).norm()))
        .fold(0.0, f64::max);
    SsiDomain {
        center: Point3::origin(),
        half_extent: 2.0 * reach + extent,
        extent,
        floor_scale: 1.0,
    }
}

fn report(what: &str, out: Result<geom_brep::ssi::SsiOutcome, geom_brep::ssi::SsiError>) {
    match out {
        Ok(o) => {
            let ends: Vec<String> = o.branches.iter().map(|b| format!("{:?}", b.end)).collect();
            eprintln!(
                "{what}: Ok: {} branches {ends:?}; {} boundary regions {:?}; receipt: examined {} excluded {} accounted {} contact {} refined {} depth {} floor {:e} m; seeds {}",
                o.branches.len(),
                o.boundary.len(),
                o.boundary,
                o.exhaustiveness.examined,
                o.exhaustiveness.excluded,
                o.exhaustiveness.accounted,
                o.exhaustiveness.contact,
                o.exhaustiveness.refined,
                o.exhaustiveness.max_depth,
                o.exhaustiveness.floor_meters(),
                o.seeds
            );
        }
        Err(e) => eprintln!("{what}: Err: {e:?}"),
    }
}

#[test]
fn probe_lilys_pairs_through_the_whole_section_readings() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let base = [0.170, 0.028, 0.020];
    let (d, v) = (Vec3::unit_x(), Vec3::unit_z());
    let leaf = blade(
        d,
        v,
        (1.2, -0.3),
        |s| [0, 1, 2].map(|i| base[i] + ([0.300, 0.034, 0.016][i] - base[i]) * s),
        tol,
    );
    let sheath = blade(-d, v, (0.34, 0.85), |_| base, tol);
    let (cap_a, walls_a) = parts(&leaf);
    let (cap_b, walls_b) = parts(&sheath);
    eprintln!("blade: {} walls; sheath: {} walls", walls_a.len(), walls_b.len());
    let t = std::time::Instant::now();
    for (i, w) in walls_b.iter().enumerate() {
        let dom = domain(&[w]);
        let out = geom_brep::PlaneSection::of(&cap_a, w, dom, band).and_then(|s| s.all());
        report(&format!("blade cap × sheath wall {i} ({:.2?})", t.elapsed()), out);
    }
    for (i, w) in walls_a.iter().enumerate() {
        let dom = domain(&[w]);
        let out = geom_brep::PlaneSection::of(&cap_b, w, dom, band).and_then(|s| s.all());
        report(&format!("sheath cap × blade wall {i} ({:.2?})", t.elapsed()), out);
    }
    for (i, wa) in walls_a.iter().enumerate() {
        for (j, wb) in walls_b.iter().enumerate() {
            let dom = domain(&[wa, wb]);
            let out = geom_brep::Section::of(wa, wb, dom, band).and_then(|s| s.all());
            report(&format!("blade wall {i} × sheath wall {j} ({:.2?})", t.elapsed()), out);
        }
    }
    eprintln!("total {:.2?}", t.elapsed());
}
