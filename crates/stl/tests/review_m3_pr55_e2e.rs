//! M3 PR 5.5 review, assignment D (e2e half): THE DIE built from RAW
//! extrude outputs — profile → extrude → 21 sequential boolean pip
//! subtracts across all six faces — with NO caller-side edge
//! normalization (the demo's `normalize_edges_to_chords` workaround
//! must be unnecessary after PR 5's extrude-operand remap), exact
//! volume oracle, tiers, tessellation, and a binary STL written for
//! the external admesh watertight gate (`scripts/check_admesh.sh`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::type_complexity)] // fixture tables read clearer inline

use geom_core::{Point2, Point3, Vec3};
use mesh::validate::{check_mesh, signed_volume};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanResult, mass_properties, subtract, validate, validate_closed};
use geom_core::Tol;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn validated(plane: SketchPlane<f64>, lp: ProfileLoop<f64>) -> ValidatedProfile<f64> {
    Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap()
}

/// A rectangular raw-extrude brick on an arbitrary sketch frame.
fn slab(
    origin: Point3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    w: (f64, f64),
    h: (f64, f64),
    depth: f64,
) -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(w.0, h.0), p2(w.1, h.0), p2(w.1, h.1), p2(w.0, h.1)]);
    extrude(
        &validated(SketchPlane::from_frame(origin, u, v), lp),
        Extrusion::Distance(depth),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// THE DIE from raw extrude operands: cube [0,2]^3, 21 pips (0.25
/// square, 0.125 deep, {0.5,1,1.5} grid, opposite faces sum 7).
#[test]
fn die_from_raw_extrudes_watertight_stl() {
    let o = Point3::new(0.0, 0.0, 0.0);
    let (ex, ey, ez) = (
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );
    let mut acc = slab(o, ex, ey, (0.0, 2.0), (0.0, 2.0), 2.0);
    let g = [0.5, 1.0, 1.5];
    // (sketch origin, u, v, pip centers (u,v)) per face; sketch plane
    // sits 0.125 under the face, extrude 0.625 outward.
    let faces: [(Point3<f64>, Vec3<f64>, Vec3<f64>, Vec<(f64, f64)>); 6] = [
        (Point3::new(0.0, 0.0, 1.875), ex, ey, vec![(g[1], g[1])]),
        (
            Point3::new(0.0, 0.0, 0.125),
            ey,
            ex, // normal -z
            vec![
                (g[0], g[0]),
                (g[1], g[1]),
                (g[2], g[2]),
                (g[0], g[2]),
                (g[2], g[0]),
                (g[0], g[1]),
            ],
        ),
        (
            Point3::new(1.875, 0.0, 0.0),
            ey,
            ez, // normal +x
            vec![(g[0], g[0]), (g[2], g[2])],
        ),
        (
            Point3::new(0.125, 0.0, 0.0),
            ez,
            ey, // normal -x
            vec![
                (g[0], g[0]),
                (g[2], g[2]),
                (g[0], g[2]),
                (g[2], g[0]),
                (g[1], g[1]),
            ],
        ),
        (
            Point3::new(0.0, 1.875, 0.0),
            ez,
            ex, // normal +y
            vec![(g[0], g[0]), (g[1], g[1]), (g[2], g[2])],
        ),
        (
            Point3::new(0.0, 0.125, 0.0),
            ex,
            ez, // normal -y
            vec![(g[0], g[0]), (g[2], g[2]), (g[0], g[2]), (g[2], g[0])],
        ),
    ];
    let mut pips = 0u32;
    for (origin, u, v, centers) in faces {
        for (cu, cv) in centers {
            let pip = slab(
                origin,
                u,
                v,
                (cu - 0.125, cu + 0.125),
                (cv - 0.125, cv + 0.125),
                0.625,
            );
            let r = subtract(&acc, &pip, Tol::witness()).unwrap();
            let BooleanResult::Body(bb) = r else {
                panic!("pip subtract emptied the die");
            };
            acc = bb.body;
            pips += 1;
            assert_eq!(validate(&acc), Ok(()));
            assert_eq!(validate_closed(&acc), Ok(()));
            let expect = 8.0 - f64::from(pips) * 0.25 * 0.25 * 0.125;
            assert_eq!(mass_properties(&acc, Tol::witness()).unwrap().volume, expect, "pip {pips}");
        }
    }
    assert_eq!(pips, 21);
    assert_eq!(mass_properties(&acc, Tol::witness()).unwrap().volume, 7.8359375);
    // Tessellate + self-checked mesh + STL for the admesh gate.
    let mesh = mesh::tessellate(&acc, 1e-2, Tol::witness()).unwrap();
    check_mesh(&mesh).unwrap();
    let v = signed_volume(&mesh);
    assert!((v - 7.8359375).abs() < 1e-9, "mesh volume {v}");
    let dir = std::env::var("REVIEW_STL_DIR").unwrap_or_else(|_| {
        std::env::temp_dir()
            .join("review_m3_pr55_stl")
            .to_string_lossy()
            .into_owned()
    });
    std::fs::create_dir_all(&dir).unwrap();
    let path = format!("{dir}/die21.stl");
    let mut file = std::fs::File::create(&path).unwrap();
    stl::write_binary(&mesh, &stl::BinaryOptions::default(), &mut file).unwrap();
}
