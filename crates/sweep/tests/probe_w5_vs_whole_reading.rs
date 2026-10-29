//! **Probe: does C3's whole plane × NURBS reading ever refuse a pair
//! that W5 clears?** For each pair, W5's plane form is read honestly
//! (`geom_brep::ssi::w5_probe::w5_plane`: one closed derivative box per
//! span cell, `(S_u × S_v) × n` in certification arithmetic), then
//! `PlaneSection::all` at ε 1e-9, 1e-6 and 1e-12. SPLICE's measurement
//! for PR 5990's question 2; it asserts nothing. Run with
//! `--ignored --nocapture`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{NurbsSurface, Surface};
use geom_brep::ssi::w5_probe::w5_plane;
use geom_brep::ssi::{SsiDomain, SsiError, SsiOutcome};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point2, Point3, RigidMotion, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, SketchPlane};
use sweep::loft_body;

const EPS: [f64; 3] = [1e-9, 1e-6, 1e-12];

fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> Surface<f64> {
    let n = normal * (1.0 / normal.norm());
    let pick = if n.x.abs() < 0.9 {
        Vec3::unit_x()
    } else {
        Vec3::unit_y()
    };
    let u = pick - n * pick.dot(n);
    Surface::Plane {
        origin,
        normal: n,
        u_ref: u * (1.0 / u.norm()),
    }
}

/// A wall over `[0, 1]²`, degree `(control rows − 1, 1)`, ruled in `v`
/// between the `bottom` and `top` rows.
fn ruled(bottom: &[Point3<f64>], top: &[Point3<f64>]) -> NurbsSurface<f64> {
    let p = bottom.len() - 1;
    let mut ku = vec![0.0; p + 1];
    ku.extend(vec![1.0; p + 1]);
    let ku = KnotVector::clamped(ku, p).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::new();
    for (b, t) in bottom.iter().zip(top) {
        control.push(*b);
        control.push(*t);
    }
    let n = control.len();
    NurbsSurface::new(ku, kv, control, vec![1.0; n]).unwrap()
}

fn domain_for(p: &Surface<f64>, wall: &NurbsSurface<f64>, extent: f64) -> SsiDomain {
    let Surface::Plane { origin, .. } = p else {
        unreachable!()
    };
    let reach = wall
        .control()
        .iter()
        .map(|c| (*c - *origin).norm())
        .fold(0.0, f64::max);
    SsiDomain {
        center: *origin,
        half_extent: 2.0 * reach + extent,
        extent,
        floor_scale: 1.0,
    }
}

fn verdict(out: &Result<SsiOutcome, SsiError>) -> String {
    match out {
        Ok(o) => format!(
            "Ok: {} branches {:?}; {} regions {:?}; cells examined {} depth {}",
            o.branches.len(),
            o.branches
                .iter()
                .map(|b| format!("{:?}", b.end))
                .collect::<Vec<_>>(),
            o.boundary.len(),
            o.boundary,
            o.exhaustiveness.examined,
            o.exhaustiveness.max_depth,
        ),
        Err(e) => format!("Err: {e:?}"),
    }
}

fn run(what: &str, p: &Surface<f64>, wall: &NurbsSurface<f64>, extent: f64) {
    let Surface::Plane { normal, .. } = p else {
        unreachable!()
    };
    let t = std::time::Instant::now();
    let w5 = w5_plane(wall, *normal);
    let w5_time = t.elapsed();
    eprintln!(
        "== {what}\n   W5: clears {} (gradient form {}), {} spans, first failure {:?} ({w5_time:.2?})",
        w5.clears, w5.gradient_clears, w5.spans, w5.first_failure
    );
    let tol = Tol::witness();
    for eps in EPS {
        let band = Band::linear_at(tol, eps).expect("band");
        let t = std::time::Instant::now();
        let out = geom_brep::PlaneSection::of(p, wall, domain_for(p, wall, extent), band)
            .and_then(|s| s.all());
        eprintln!("   ε {eps:e} ({:.2?}): {}", t.elapsed(), verdict(&out));
    }
}

fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// The planar wall in `y = 0` whose bottom edge is the quadratic with
/// heights `z` at its three controls, its top edge at `z = 1`.
fn bowed(z: [f64; 3]) -> NurbsSurface<f64> {
    ruled(
        &[pt(0.0, 0.0, z[0]), pt(0.5, 0.0, z[1]), pt(1.0, 0.0, z[2])],
        &[pt(0.0, 0.0, 1.0), pt(0.5, 0.0, 1.0), pt(1.0, 0.0, 1.0)],
    )
}

#[test]
#[ignore = "probe: SPLICE's W5 measurement, asserts nothing"]
fn probe_w5_against_the_whole_reading_on_synthetic_walls() {
    let floor = |h: f64| plane(pt(0.5, 0.0, h), Vec3::unit_z());

    // 1. An exterior graze: the bottom edge z = 0.4(u − ½)² touches z = 0
    //    at u = ½ from above; the section is that one boundary point.
    let w = bowed([0.1, -0.1, 0.1]);
    run(
        "1. exterior graze, edge touches z = 0 at u = ½ (section: one corner-free side point)",
        &floor(0.0),
        &w,
        1.0,
    );
    // 2. An interior graze: the edge z = −0.4(u − ½)² touches z = 0 from
    //    below; the section is the line z = 0 across the wall, tangent
    //    to the bottom side at its middle.
    let w = bowed([-0.1, 0.1, -0.1]);
    run(
        "2. interior graze, open branch tangent to the bottom side at u = ½",
        &floor(0.0),
        &w,
        1.0,
    );
    // 3. Near grazes either side of 1 and 2.
    for d in [1e-3, 1e-5, 1e-7, 1e-10] {
        run(
            &format!("3a. exterior near-graze, plane z = +{d:e} (a short arc)"),
            &floor(d),
            &bowed([0.1, -0.1, 0.1]),
            1.0,
        );
        run(
            &format!(
                "3b. interior near-graze, plane z = +{d:e} (a line clearing the side by {d:e})"
            ),
            &floor(d),
            &bowed([-0.1, 0.1, -0.1]),
            1.0,
        );
        run(
            &format!("3c. interior near-graze, plane z = −{d:e} (two arcs, crossings close)"),
            &floor(-d),
            &bowed([-0.1, 0.1, -0.1]),
            1.0,
        );
    }
    // 4. A side lying in the plane, the wall leaving it at angle θ.
    for th in [std::f64::consts::FRAC_PI_2, 0.3, 1e-3, 1e-6, 1e-8] {
        let (s, c) = th.sin_cos();
        let w = ruled(
            &[pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0)],
            &[pt(0.0, c, s), pt(1.0, c, s)],
        );
        run(
            &format!("4. bottom side in z = 0, wall at θ = {th:e} to the plane"),
            &floor(0.0),
            &w,
            1.0,
        );
    }
    // 4b. A curved side lying in a plane: the edge z = 0 bowed in y, wall up.
    let w = ruled(
        &[pt(0.0, 0.0, 0.0), pt(0.5, 0.3, 0.0), pt(1.0, 0.0, 0.0)],
        &[pt(0.0, 0.0, 1.0), pt(0.5, 0.3, 1.0), pt(1.0, 0.0, 1.0)],
    );
    run(
        "4b. a bowed bottom side lying in z = 0 (lily's cap shape)",
        &floor(0.0),
        &w,
        1.0,
    );
    // 5. A corner on the plane: the flat wall y = 0, [0,1]², and planes
    //    through its (0, 0) corner.
    let flat = ruled(
        &[pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0)],
        &[pt(0.0, 0.0, 1.0), pt(1.0, 0.0, 1.0)],
    );
    run(
        "5a. plane x + z = 0 touches only the corner",
        &plane(pt(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 1.0)),
        &flat,
        1.0,
    );
    run(
        "5b. plane x − z = 0, the diagonal from corner to corner",
        &plane(pt(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, -1.0)),
        &flat,
        1.0,
    );
    run(
        "5c. plane x + 1e-3·z = 0, near the u = 0 side through the corner",
        &plane(pt(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 1e-3)),
        &flat,
        1.0,
    );
    // 6. An inflected wall, large and far: an S-shaped cubic edge ruled
    //    upward, 50 m across, 1 km from the origin, cut by z = const.
    let (o, s) = (pt(1000.0, 1000.0, 1000.0), 50.0);
    let at = |x: f64, y: f64, z: f64| pt(o.x + s * x, o.y + s * y, o.z + s * z);
    let w = ruled(
        &[
            at(0.0, 0.0, 0.0),
            at(0.33, 0.6, 0.0),
            at(0.66, -0.6, 0.0),
            at(1.0, 0.0, 0.0),
        ],
        &[
            at(0.0, 0.0, 1.0),
            at(0.33, 0.6, 1.0),
            at(0.66, -0.6, 1.0),
            at(1.0, 0.0, 1.0),
        ],
    );
    run(
        "6a. inflected S wall, 50 m at 1 km, plane z = mid",
        &plane(at(0.5, 0.0, 0.5), Vec3::unit_z()),
        &w,
        s,
    );
    run(
        "6b. same, plane tilted (normal z + 0.3x)",
        &plane(at(0.5, 0.0, 0.5), Vec3::new(0.3, 0.0, 1.0)),
        &w,
        s,
    );
    // 6c. A twisted (doubly curved, inflected) wall: a cubic in u whose
    //     top row is the bottom row's S mirrored, cut by a tilted plane.
    let w = ruled(
        &[
            at(0.0, 0.0, 0.0),
            at(0.33, 0.6, 0.1),
            at(0.66, -0.6, -0.1),
            at(1.0, 0.0, 0.0),
        ],
        &[
            at(0.0, 0.0, 1.0),
            at(0.33, -0.6, 0.9),
            at(0.66, 0.6, 1.1),
            at(1.0, 0.0, 1.0),
        ],
    );
    run(
        "6c. twisted inflected wall at 1 km, plane z = mid",
        &plane(at(0.5, 0.0, 0.5), Vec3::unit_z()),
        &w,
        s,
    );
    // 7. A graze far from the origin: case 1 scaled by 50 at 1 km.
    let w = ruled(
        &[at(0.0, 0.0, 0.1), at(0.5, 0.0, -0.1), at(1.0, 0.0, 0.1)],
        &[at(0.0, 0.0, 1.0), at(0.5, 0.0, 1.0), at(1.0, 0.0, 1.0)],
    );
    run(
        "7. exterior graze at 1 km, 50 m",
        &plane(at(0.5, 0.0, 0.0), Vec3::unit_z()),
        &w,
        s,
    );
}

#[test]
#[ignore = "probe: SPLICE's W5 measurement, asserts nothing"]
fn probe_w5_against_the_whole_reading_on_inflected_walls_at_the_origin() {
    // 6d/6e. The S wall of 6b at the origin (50 m, then 1 m), to part
    //     the far-from-origin cost from the inflection.
    for (o2, s2) in [(pt(0.0, 0.0, 0.0), 50.0), (pt(0.0, 0.0, 0.0), 1.0)] {
        let at2 = |x: f64, y: f64, z: f64| pt(o2.x + s2 * x, o2.y + s2 * y, o2.z + s2 * z);
        let w = ruled(
            &[
                at2(0.0, 0.0, 0.0),
                at2(0.33, 0.6, 0.0),
                at2(0.66, -0.6, 0.0),
                at2(1.0, 0.0, 0.0),
            ],
            &[
                at2(0.0, 0.0, 1.0),
                at2(0.33, 0.6, 1.0),
                at2(0.66, -0.6, 1.0),
                at2(1.0, 0.0, 1.0),
            ],
        );
        run(
            &format!("6d. inflected S wall, {s2} m at the origin, plane tilted"),
            &plane(at2(0.5, 0.0, 0.5), Vec3::new(0.3, 0.0, 1.0)),
            &w,
            s2,
        );
        let w = ruled(
            &[
                at2(0.0, 0.0, 0.0),
                at2(0.33, 0.6, 0.1),
                at2(0.66, -0.6, -0.1),
                at2(1.0, 0.0, 0.0),
            ],
            &[
                at2(0.0, 0.0, 1.0),
                at2(0.33, -0.6, 0.9),
                at2(0.66, 0.6, 1.1),
                at2(1.0, 0.0, 1.0),
            ],
        );
        run(
            &format!("6e. twisted wall, {s2} m at the origin, plane z = mid"),
            &plane(at2(0.5, 0.0, 0.5), Vec3::unit_z()),
            &w,
            s2,
        );
    }
}

// ---- the lofted blade and lily's caps, as on splice/fork-interior-loop-fable ----

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

fn parts(body: &topo::Body<f64>) -> (Surface<f64>, Vec<Arc<NurbsSurface<f64>>>) {
    let mut cap = None;
    let mut walls = Vec::new();
    for (_, face) in body.faces() {
        match body
            .get_surface(face.surface)
            .expect("a face has a surface")
        {
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

#[test]
#[ignore = "probe: SPLICE's W5 measurement, asserts nothing"]
fn probe_w5_against_the_whole_reading_on_the_blade_and_lilys_caps() {
    let tol = Tol::witness();
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
    for (i, w) in walls_b.iter().enumerate() {
        run(
            &format!("L. lily: blade cap × sheath wall {i}"),
            &cap_a,
            w,
            2.0,
        );
    }
    for (i, w) in walls_a.iter().enumerate() {
        run(
            &format!("L. lily: sheath cap × blade wall {i}"),
            &cap_b,
            w,
            2.0,
        );
    }
    for (i, w) in walls_a.iter().enumerate() {
        run(
            &format!("B. plane x = 0.5 across the spine × blade wall {i}"),
            &plane(pt(0.5, 0.0, 0.0), Vec3::unit_x()),
            w,
            2.0,
        );
        run(
            &format!("B. plane z = −0.01 lengthwise × blade wall {i}"),
            &plane(pt(0.0, 0.0, -0.01), Vec3::unit_z()),
            w,
            2.0,
        );
        // A cap across the spine at the blade's tip end, near where the
        // walls meet the far cap.
        run(
            &format!("B. plane x = 1.15 near the tip × blade wall {i}"),
            &plane(pt(1.15, 0.0, 0.0), Vec3::unit_x()),
            w,
            2.0,
        );
        // A plane through the blade's ridge rim, tilted: the rim (a
        // side of each wall) crosses it.
        run(
            &format!("B. plane tilted 30° across the spine × blade wall {i}"),
            &plane(pt(0.6, 0.0, 0.0), Vec3::new(0.866, 0.0, 0.5)),
            w,
            2.0,
        );
    }
}
