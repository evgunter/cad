//! **Sweeping and rolling through the façade**: a section carried
//! along a path from a written start frame turned onto the path's
//! start tangent, then the same section lofted through the frames the
//! sweep carries, with a roll composed about the tangent — the answer
//! to "is a caller wanting a different roll owed a second door",
//! written as a program rather than as prose.
//!
//! It also walks the turn's refusals, and tangents a few band-widths
//! off the world +z, where the roll does not jump.
//!
//! Run: `cargo run -p pncad --example sweep_start_frame_and_roll`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::authoring::polygon;
use pncad::geom_core::{Band, Point3, RigidMotion, Tol, UnitVec3, Vec3};
use pncad::profile::SketchPlane;

const TURNS: f64 = 0.4;
fn main() {
    let tol = Tol::witness();
    // Every volume below is read at 1.024e-6 m — 1024·ε at this
    // tolerance, the resolution the suites' pinned numbers are stated at.
    // Spelled as a literal: an example reaches the kernel only through
    // the façade, which carries no test vocabulary in a plain build.
    // A helix-ish path a user might author: 33 points, degree 3.
    let pts: Vec<Point3<f64>> = (0..=32)
        .map(|k| {
            let s = f64::from(k) / 32.0;
            let a = std::f64::consts::TAU * TURNS * s;
            Point3::new(a.cos(), a.sin(), 0.6 * s)
        })
        .collect();
    let path = pncad::geom::NurbsCurve3::interpolate(&pts, 3).expect("path");
    let (t0, t1) = path.domain();

    // 1. The start frame, as a user writes it: the world frame at the
    // path's start, turned onto its start tangent by the least turn.
    let band = Band::linear(tol).expect("band");
    let unit = |v: Vec3<f64>| UnitVec3::new(v, "example_tangent", band);
    let (p, tau) = path.ders1(t0);
    let place = SketchPlane::from_frame(
        RigidMotion::axes_xy(p)
            .turned_onto(unit(tau).expect("a tangent"), "example_start", band)
            .expect("start frame"),
    );
    println!("start frame: {place:?}");
    let section =
        vec![polygon(&[(-0.1, -0.1), (0.1, -0.1), (0.1, 0.1), (-0.1, 0.1)], tol).expect("square")];
    for stations in [9usize, 13, 17, 25, 33] {
        match pncad::sweep::sweep_body::<f64>(&section, place, &path, stations, 3, tol) {
            Ok(swept) => {
                let m = pncad::topo::mass_properties(
                    &pncad::topo::AtRestBody::validate(pncad::topo::Body::clone(&swept.body), tol)
                        .expect("the measured body passes tier 3"),
                    tol,
                    pncad::topo::Settle::Resolution(pncad::quantity::Length::from_meters(1.024e-6)),
                )
                .expect("props")
                .met()
                .expect("the resolution is met");
                println!(
                    "{stations} stations: swept volume {} ± {:.1e}; validate {:?}",
                    m.volume,
                    m.volume_pad,
                    pncad::topo::validate_closed(&swept.body)
                );
            }
            Err(e) => println!("{stations} stations: refused {e:?}"),
        }
    }

    // 2. What happens if a user hands it a zero tangent, or the pole?
    let world = RigidMotion::axes_xy(Point3::<f64>::origin());
    println!("zero tangent: {:?}", unit(Vec3::new(0.0, 0.0, 0.0)).err());
    println!(
        "pole tangent: {:?}",
        world
            .turned_onto(-UnitVec3::exact_z(), "example_start", band)
            .err()
    );
    // The recourse at the pole is to write a different frame.
    println!(
        "pole tangent, frame written upside down: {:?}",
        RigidMotion::turned(
            Point3::<f64>::origin(),
            UnitVec3::exact_x(),
            pncad::geom_core::Turns::quarters(2)
        )
        .turned_onto(-UnitVec3::exact_z(), "example_start", band)
        .map(RigidMotion::linear)
    );
    // Tangents a few band-widths off +z: the roll moves with them.
    let eps = tol.eps();
    for k in [0.5, 1.0, 5.0, 10.0, 20.0] {
        let t = Vec3::new(k * eps, 0.0, 1.0);
        let r = world.turned_onto(unit(t).expect("a tangent"), "example_start", band);
        println!(
            "tangent ({k}·eps, 0, 1): {}",
            match &r {
                Ok(f) => format!("x̂ = {:?}", f.u()),
                Err(e) => format!("refused {e:?}"),
            }
        );
    }

    // 3. Composed roll, the way the narration says: the start frame
    // carried to each station by the sweep's own carry, then a
    // rotation about the tangent.
    let stations: u32 = 6;
    let carried = pncad::sweep::sweep_places(
        place,
        &path,
        usize::try_from(stations).expect("a count"),
        tol,
    )
    .expect("the path carries its start frame");
    let places: Vec<SketchPlane<f64>> = (0..stations)
        .zip(carried)
        .map(|(i, plane)| {
            let u = f64::from(i) / f64::from(stations - 1);
            // The roll turns the AXES about the frame's own tangent
            // axis and leaves the origin the station point: a world
            // rotation about the tangent line would rebuild the
            // translation and drift off the spine.
            let roll = RigidMotion::rotation(UnitVec3::exact_z(), 0.8 * u).expect("roll");
            SketchPlane::from_frame(plane.frame().compose(roll).expect("rolled frame"))
        })
        .collect();
    let sections: Vec<_> = (0..stations).map(|_| section.clone()).collect();
    let loft = pncad::sweep::loft_body::<f64>(&sections, &places, 3, tol).expect("loft");
    let ml = pncad::topo::mass_properties(
        &pncad::topo::AtRestBody::validate(pncad::topo::Body::clone(&loft.body), tol)
            .expect("the measured body passes tier 3"),
        tol,
        pncad::topo::Settle::Resolution(pncad::quantity::Length::from_meters(1.024e-6)),
    )
    .expect("props")
    .met()
    .expect("the resolution is met");
    println!(
        "rolled loft volume {} ± {:.1e}; validate {:?}",
        ml.volume,
        ml.volume_pad,
        pncad::topo::validate_closed(&loft.body)
    );
    let mut drift = 0.0f64;
    for (i, pl) in (0u32..).zip(places.iter()) {
        let u = f64::from(i) / f64::from(stations - 1);
        let t = (t1 - t0).mul_add(u, t0);
        let p = path.eval(t);
        drift = drift.max((pl.origin() - p).norm());
    }
    println!("composed-roll origin drift off the spine: {drift:.2e}");
}
