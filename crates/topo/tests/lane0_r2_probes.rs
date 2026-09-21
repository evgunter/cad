//! LANE-0 review probes (R2): the public doors over a body carrying an
//! `Approx` face, at `f64`, `Dual64`, and behind the features `Probe`
//! and `Interval`. Every row PRINTS what the door answered — the
//! certificate limbs by bits at `f64`, the refusal's `Debug` and
//! `Display` elsewhere — so the same file run at the merge base and at
//! the head can be diffed byte for byte. Public API only, on purpose:
//! the file has to compile on both trees.
//!
//! Run with `--nocapture` and diff the stdout.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{ApproxSurface, NurbsSurface, Surface};
use geom_core::{Affine3, Band, Decide, Point3, Real, Tol, Vec3};
use topo::{Body, ContactRecords, FaceKey, FaceSurface};

/// The bowed patch the head's fixture uses, copied so the base can
/// build it too.
fn bowed_patch() -> NurbsSurface<f64> {
    const BOW: f64 = 1.5e-5;
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut control = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            let (u, v) = (f64::from(i) * 0.5, f64::from(j) * 0.5);
            control.push(Point3::new(
                u,
                v,
                BOW * u * (1.0 - u) + (BOW * 2.0 / 3.0) * v * v,
            ));
        }
    }
    NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 9]).unwrap()
}

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

fn bowed_offset_approx<T: Real>() -> ApproxSurface<T> {
    let minted = geom_brep::approx_offset_surface(Arc::new(bowed_patch()), 0.05, tol(), band())
        .expect("the bowed patch's offset fits at the run's eps");
    let Surface::Approx(approx) = minted else {
        panic!("the mint door produces Surface::Approx");
    };
    approx.map_scalar(T::from_f64)
}

fn seed_with<T: Decide>(surface: Surface<T>) -> (Body<T>, FaceKey) {
    let mut body = Body::<T>::new();
    let created = body
        .mvfs(Point3::new(T::zero(), T::zero(), T::zero()))
        .expect("mvfs has no preconditions");
    body.set_face_surface(created.face, FaceSurface::New(surface))
        .expect("the seed face takes a fresh surface");
    (body, created.face)
}

fn approx_seed<T: Decide>() -> (Body<T>, FaceKey) {
    seed_with(Surface::Approx(Arc::new(bowed_offset_approx::<T>())))
}

fn nurbs_seed<T: Decide>() -> (Body<T>, FaceKey) {
    seed_with(Surface::Nurbs(Arc::new(bowed_patch().map_scalar(T::from_f64))))
}

fn turned<T: Real>() -> Affine3<T> {
    Affine3::rotation_about_axis(
        Point3::new(T::zero(), T::zero(), T::zero()),
        Vec3::new(T::zero(), T::zero(), T::from_f64(1.0)),
        T::from_f64(0.7),
    )
}

/// Every `Approx` surface of `body`, its certificate limb for limb by
/// bits, plus the stored tolerance.
fn dump_approx_bits<T: Real>(label: &str, body: &Body<T>) {
    for (key, s) in body.surfaces() {
        if let Surface::Approx(a) = s {
            let c = a.certificate();
            println!(
                "{label} {key:?} approx: distance={:016x} on_locus_max={:016x} hull_sup={:016x} \
                 normal_floor={:016x} curvature_reach={:016x} cells={} samples={} rounds={} \
                 tolerance={:016x}",
                c.distance.to_bits(),
                c.on_locus_max.to_bits(),
                c.hull_sup.to_bits(),
                c.normal_floor.to_bits(),
                c.curvature_reach.to_bits(),
                c.cells,
                c.samples,
                c.rounds,
                a.tolerance().to_bits(),
            );
        }
    }
}

fn dump_validation(label: &str, r: &Result<(), Vec<topo::ValidationError>>) {
    match r {
        Ok(()) => println!("{label}: Ok(())"),
        Err(errors) => {
            for e in errors {
                println!("{label}: Err {e:?} | display: {e}");
            }
        }
    }
}

/// The public doors at one scalar. `structural_only` says whether the
/// scalar may form the certified doors at all (a dual may not).
fn doors_at<T: topo::PropsQuadLane + geom_core::Bounds + topo::AtRestPolicy>(label: &str) {
    let (body, face) = approx_seed::<T>();
    println!("{label}: approx face {face:?}");
    dump_approx_bits(&format!("{label} seed"), &body);
    dump_validation(
        &format!("{label} validate_geometric_structural"),
        &topo::validate_geometric_structural(&body, tol()),
    );
    dump_validation(
        &format!("{label} validate_pseudomanifold"),
        &topo::validate_pseudomanifold(&body, &ContactRecords::default(), tol()),
    );
    match topo::transform_rigid(&body, &turned::<T>(), tol()) {
        Ok(mapped) => {
            println!("{label} transform_rigid: Ok");
            dump_approx_bits(&format!("{label} mapped"), &mapped);
        }
        Err(e) => println!("{label} transform_rigid: Err {e:?} | display: {e}"),
    }
    let (mut nurbs, nface) = nurbs_seed::<T>();
    match topo::replace_faces_offset(&mut nurbs, &[nface], T::from_f64(0.05), band(), tol()) {
        Ok(()) => {
            println!("{label} replace_faces_offset: Ok");
            dump_approx_bits(&format!("{label} offset"), &nurbs);
        }
        Err(e) => println!("{label} replace_faces_offset: Err {e:?} | display: {e}"),
    }
}

fn certified_doors_at<T: topo::PropsQuadLane + geom_core::CertifiedBounds + topo::AtRestPolicy>(label: &str) {
    let (body, _) = approx_seed::<T>();
    dump_validation(
        &format!("{label} validate_geometric"),
        &topo::validate_geometric(&body, tol()),
    );
    dump_validation(
        &format!("{label} validate_pseudomanifold_certified"),
        &topo::validate_pseudomanifold_certified(&body, &ContactRecords::default(), tol()),
    );
}

#[test]
fn dump_f64() {
    doors_at::<f64>("f64");
    certified_doors_at::<f64>("f64");
}

#[test]
fn dump_dual64() {
    doors_at::<geom_core::Dual64>("dual64");
}

#[test]
fn dump_sym_f64() {
    doors_at::<geom_core::Sym<f64>>("sym<f64>");
    certified_doors_at::<geom_core::Sym<f64>>("sym<f64>");
}

#[cfg(feature = "probe")]
#[test]
fn dump_probe() {
    doors_at::<geom_core::Probe>("probe");
    certified_doors_at::<geom_core::Probe>("probe");
}

#[cfg(feature = "interval")]
#[test]
fn dump_interval() {
    doors_at::<geom_core::interval::Interval>("interval");
    certified_doors_at::<geom_core::interval::Interval>("interval");
}
