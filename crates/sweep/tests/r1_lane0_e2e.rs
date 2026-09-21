//! Review probe (LANE-0, R1): the PUBLIC doors — `validate_geometric`,
//! `validate_pseudomanifold`, `transform_rigid`, `replace_faces_offset` —
//! driven over a real box body whose top cap carries a certified `Approx`
//! surface, at `f64` and, the same subject lifted, at `Interval` and at the
//! telemetry probe. Every certificate limb is dumped by bits and every
//! refusal by `Debug` and `Display`, so the merge base and the head can be
//! diffed byte for byte. Run and diffed at review time: identical.

use std::sync::Arc;

use geom::Surface;
use geom_core::{Affine3, Tol, Vec3};
use topo::FaceSurface;

#[allow(unused_imports)]
use crate::common::approx::{band, box_with_approx_cap, planar_patch, pulled_back};

fn dump(tag: &str, c: &geom::OffsetCertificate) {
    println!(
        "{tag} distance={:016x} on_locus_max={:016x} hull_sup={:016x} normal_floor={:016x} \
         curvature_reach={:016x} cells={} samples={} rounds={}",
        c.distance.to_bits(),
        c.on_locus_max.to_bits(),
        c.hull_sup.to_bits(),
        c.normal_floor.to_bits(),
        c.curvature_reach.to_bits(),
        c.cells,
        c.samples,
        c.rounds
    );
}

fn rigid_f64() -> Affine3<f64> {
    Affine3::translation(Vec3::new(1.0, 0.0, 0.0))
}

#[test]
fn r1_lane0_f64_public_doors() {
    println!("--- R1 LANE0 f64 BEGIN ---");
    let d = 0.05;
    let (body, face) = box_with_approx_cap(d, 1e-9);
    let Some(Surface::Approx(a)) = body.get_surface(body.get_face(face).unwrap().surface) else {
        panic!("the cap wears the approximating surface");
    };
    dump("f64 at-rest", a.certificate());
    println!("f64 at-rest tolerance={:016x}", a.tolerance().to_bits());

    println!(
        "f64 validate_geometric      = {:?}",
        topo::validate_geometric(&body, Tol::witness())
    );
    let contacts = topo::boolean::ContactRecords::default();
    println!(
        "f64 validate_pseudomanifold = {:?}",
        topo::validate_pseudomanifold(&body, &contacts, Tol::witness())
    );

    match topo::transform_rigid(&body, &rigid_f64(), Tol::witness()) {
        Ok(moved) => {
            let Some(Surface::Approx(m)) = moved.get_surface(moved.get_face(face).unwrap().surface)
            else {
                panic!("the mapped cap is still approximating");
            };
            dump("f64 mapped", m.certificate());
            println!("f64 mapped tolerance={:016x}", m.tolerance().to_bits());
        }
        Err(e) => println!("f64 transform_rigid = Err({e:?}) | display={e}"),
    }

    // The offset door over the whole body: the mint that produces an
    // `Approx` for a kind not closed under offset.
    let (mut fresh, cap) = box_with_approx_cap(d, 1e-9);
    let planar = planar_patch(1.0);
    fresh
        .set_face_surface(cap, FaceSurface::New(Surface::Nurbs(Arc::new(planar))))
        .expect("the cap takes a NURBS surface");
    let r = topo::replace_faces_offset(&mut fresh, &[cap], 0.05, band(), Tol::witness());
    println!("f64 replace_faces_offset    = {r:?}");
    println!("--- R1 LANE0 f64 END ---");
}

#[cfg(feature = "interval")]
#[test]
fn r1_lane0_interval_public_doors() {
    use geom_core::{Bounds, Interval, Real};
    use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};

    println!("--- R1 LANE0 interval BEGIN ---");
    let approx_f64 = geom_brep::approx_offset_surface_at(
        Arc::new(pulled_back(&planar_patch(1.0), 0.05)),
        0.05,
        1e-9,
        band(),
    )
    .expect("a planar patch's offset fits");
    let Surface::Approx(a) = &approx_f64 else {
        panic!("the door mints the variant")
    };
    let lifted = a.map_scalar(Interval::from_f64);

    let iv = Interval::from_f64;
    let v = |x: f64, y: f64| ProfileVertex::new(geom_core::Point2::new(iv(x), iv(y)), iv(0.0));
    let lp = ProfileLoop::new(vec![v(0.0, 0.0), v(2.0, 0.0), v(2.0, 2.0), v(0.0, 2.0)]);
    let profile = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a square is a valid profile");
    let mut body = sweep::extrude(
        &profile,
        sweep::Extrusion::Distance(iv(1.0)),
        Tol::witness(),
    )
    .expect("a square prism extrudes at Interval")
    .body;
    let face = body
        .faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if Bounds::lo(normal.z) > 0.5 && Bounds::lo(origin.z) > 0.5
            )
        })
        .map(|(k, _)| k)
        .expect("the extruded box has a top cap");
    body.set_face_surface(face, FaceSurface::New(Surface::Approx(Arc::new(lifted))))
        .expect("the attach-layer door accepts a live face");

    let vg = topo::validate_geometric(&body, Tol::witness());
    println!("interval validate_geometric      = {vg:?}");
    if let Err(ref errs) = vg {
        for e in errs {
            println!("interval vg display = {e}");
        }
    }
    let contacts = topo::boolean::ContactRecords::default();
    let vp = topo::validate_pseudomanifold(&body, &contacts, Tol::witness());
    println!("interval validate_pseudomanifold = {vp:?}");
    if let Err(ref errs) = vp {
        for e in errs {
            println!("interval vp display = {e}");
        }
    }
    let t = topo::transform_rigid(
        &body,
        &Affine3::translation(geom_core::Vec3::new(iv(1.0), iv(0.0), iv(0.0))),
        Tol::witness(),
    );
    match t {
        Ok(_) => println!("interval transform_rigid         = Ok"),
        Err(e) => println!("interval transform_rigid         = Err({e:?}) | display={e}"),
    }
    let mut b2 = body.clone();
    let r = topo::replace_faces_offset(
        &mut b2,
        &[face],
        iv(0.05),
        geom_core::Band::linear(Tol::witness()).unwrap(),
        Tol::witness(),
    );
    match r {
        Ok(()) => println!("interval replace_faces_offset    = Ok"),
        Err(e) => println!("interval replace_faces_offset    = Err({e:?}) | display={e}"),
    }
    println!("--- R1 LANE0 interval END ---");
}

/// The mint's absence path through the PUBLIC offset door at a scalar with
/// no fit: a NURBS cap, offset at `Interval`.
#[cfg(feature = "interval")]
#[test]
fn r1_lane0_interval_mint_absence() {
    use geom_core::{Bounds, Interval, Real};
    use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};

    println!("--- R1 LANE0 interval-mint BEGIN ---");
    let iv = Interval::from_f64;
    let v = |x: f64, y: f64| ProfileVertex::new(geom_core::Point2::new(iv(x), iv(y)), iv(0.0));
    let lp = ProfileLoop::new(vec![v(0.0, 0.0), v(2.0, 0.0), v(2.0, 2.0), v(0.0, 2.0)]);
    let profile = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a square is a valid profile");
    let mut body = sweep::extrude(
        &profile,
        sweep::Extrusion::Distance(iv(1.0)),
        Tol::witness(),
    )
    .expect("a square prism extrudes at Interval")
    .body;
    let face = body
        .faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if Bounds::lo(normal.z) > 0.5 && Bounds::lo(origin.z) > 0.5
            )
        })
        .map(|(k, _)| k)
        .expect("the extruded box has a top cap");
    let nurbs = planar_patch(1.0).map_scalar(Interval::from_f64);
    body.set_face_surface(face, FaceSurface::New(Surface::Nurbs(Arc::new(nurbs))))
        .expect("the attach-layer door accepts a live face");
    let r = topo::replace_faces_offset(
        &mut body,
        &[face],
        iv(0.05),
        geom_core::Band::linear(Tol::witness()).unwrap(),
        Tol::witness(),
    );
    match r {
        Ok(()) => println!("interval-mint replace_faces_offset = Ok"),
        Err(e) => println!("interval-mint replace_faces_offset = Err({e:?}) | display={e}"),
    }
    println!("--- R1 LANE0 interval-mint END ---");
}

/// The same two absence paths at the telemetry probe scalar, through the
/// public doors.
#[cfg(feature = "probe")]
#[test]
fn r1_lane0_probe_absence() {
    use geom_core::{Probe, Real};

    println!("--- R1 LANE0 probe BEGIN ---");
    let approx_f64 = geom_brep::approx_offset_surface_at(
        Arc::new(pulled_back(&planar_patch(1.0), 0.05)),
        0.05,
        1e-9,
        band(),
    )
    .expect("a planar patch's offset fits");
    let Surface::Approx(a) = &approx_f64 else {
        panic!("the door mints the variant")
    };
    let lifted = a.map_scalar(Probe::from_f64);

    let mut body = topo::Body::<Probe>::new();
    let c = body
        .mvfs(geom_core::Point3::new(
            Probe::zero(),
            Probe::zero(),
            Probe::zero(),
        ))
        .unwrap();
    body.set_face_surface(c.face, FaceSurface::New(Surface::Approx(Arc::new(lifted))))
        .unwrap();

    match topo::transform_rigid(
        &body,
        &Affine3::translation(geom_core::Vec3::new(
            Probe::from_f64(1.0),
            Probe::zero(),
            Probe::zero(),
        )),
        Tol::witness(),
    ) {
        Ok(_) => println!("probe transform_rigid = Ok"),
        Err(e) => println!("probe transform_rigid = Err({e:?}) | display={e}"),
    }

    let nurbs = planar_patch(1.0).map_scalar(Probe::from_f64);
    let mut b2 = topo::Body::<Probe>::new();
    let c2 = b2
        .mvfs(geom_core::Point3::new(
            Probe::zero(),
            Probe::zero(),
            Probe::zero(),
        ))
        .unwrap();
    b2.set_face_surface(c2.face, FaceSurface::New(Surface::Nurbs(Arc::new(nurbs))))
        .unwrap();
    match topo::replace_faces_offset(
        &mut b2,
        &[c2.face],
        Probe::from_f64(0.05),
        geom_core::Band::linear(Tol::witness()).unwrap(),
        Tol::witness(),
    ) {
        Ok(()) => println!("probe replace_faces_offset = Ok"),
        Err(e) => println!("probe replace_faces_offset = Err({e:?}) | display={e}"),
    }
    println!("--- R1 LANE0 probe END ---");
}
