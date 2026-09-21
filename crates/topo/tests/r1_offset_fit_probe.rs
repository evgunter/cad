//! Review probe (LANE-0, R1): the `mvfs`-seed body with one `Approx` face —
//! the shape `topo::fixtures::approx_faced_body` builds — driven through the
//! PUBLIC doors, with every certificate limb dumped by bits and every refusal
//! by `Debug`. The same file compiles at the merge base and at the head, so
//! the dumps are diffable.
//!
//! **What it records:** this body never reaches check 1 through
//! `validate_geometric` or `validate_pseudomanifold` — tier 1 refuses it
//! (`ScaffoldingEmptyLoop`) first, which is what
//! `n2r1_probes.rs`'s own comment already says about the same fixture. So the
//! offset-fit door's `Some` arm at f64 is reachable here only through
//! `transform_rigid`; the validator half is exercised by calling the tier-3
//! battery directly. A production-body exercise of all three doors is the
//! sibling probe in `crates/sweep/tests/r1_lane0_e2e.rs`.

use geom_core::{Affine3, Band, Point3, Tol, Vec3};
use topo::{Body, FaceSurface};

fn bowed(bow: f64) -> geom::NurbsSurface<f64> {
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut control = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            let (u, v) = (f64::from(i) * 0.5, f64::from(j) * 0.5);
            control.push(Point3::new(
                u,
                v,
                bow * u * (1.0 - u) + (bow * 2.0 / 3.0) * v * v,
            ));
        }
    }
    geom::NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 9]).unwrap()
}

fn minted(bow: f64, d: f64, tol: Tol, band: Band) -> geom::ApproxSurface<f64> {
    let s = geom_brep::approx_offset_surface(std::sync::Arc::new(bowed(bow)), d, tol, band)
        .expect("R1 subject mints");
    let geom::Surface::Approx(a) = s else {
        panic!("mint gives Approx")
    };
    std::sync::Arc::try_unwrap(a).unwrap_or_else(|a| (*a).clone())
}

fn body_of<T: geom_core::Decide>(a: &geom::ApproxSurface<f64>) -> Body<T> {
    let mut b = Body::<T>::new();
    let c = b
        .mvfs(Point3::new(T::zero(), T::zero(), T::zero()))
        .unwrap();
    b.set_face_surface(
        c.face,
        FaceSurface::New(geom::Surface::Approx(std::sync::Arc::new(
            a.map_scalar(T::from_f64),
        ))),
    )
    .unwrap();
    b
}

fn dump_cert(tag: &str, c: &geom::OffsetCertificate) {
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

fn doors<T: topo::props::PropsQuadLane + geom_core::CertifiedBounds + topo::AtRestPolicy>(
    tag: &str,
    a: &geom::ApproxSurface<f64>,
    tol: Tol,
) {
    let body = body_of::<T>(a);
    println!(
        "{tag} validate_geometric      = {:?}",
        topo::validate_geometric(&body, tol)
    );
    let contacts = topo::boolean::ContactRecords::default();
    println!(
        "{tag} validate_pseudomanifold = {:?}",
        topo::validate_pseudomanifold(&body, &contacts, tol)
    );
    let map: Affine3<T> = Affine3::rotation_about_axis(
        Point3::new(T::zero(), T::zero(), T::zero()),
        Vec3::new(T::zero(), T::zero(), T::one()),
        T::from_f64(0.7),
    );
    match topo::transform_rigid(&body, &map, tol) {
        Ok(_) => println!("{tag} transform_rigid         = Ok"),
        Err(e) => println!("{tag} transform_rigid         = Err({e:?}) | display={e}"),
    }
}

#[test]
fn r1_public_doors_on_an_approx_faced_body() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let a = minted(1.5e-5, 0.05, tol, band);
    println!("--- R1 PROBE BEGIN ---");
    dump_cert("subject", a.certificate());
    println!("subject tolerance={:016x}", a.tolerance().to_bits());

    doors::<f64>("f64", &a, tol);

    let body = body_of::<f64>(&a);
    let map = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), 0.7);
    match topo::transform_rigid(&body, &map, tol) {
        Ok(moved) => {
            let mut printed = 0;
            for (_k, f) in moved.faces() {
                if let Some(geom::Surface::Approx(x)) = moved.get_surface(f.surface) {
                    dump_cert("f64 mapped", x.certificate());
                    println!("f64 mapped tolerance={:016x}", x.tolerance().to_bits());
                    printed += 1;
                }
            }
            println!("f64 mapped approx faces={printed}");
        }
        Err(e) => println!("f64 mapped = Err({e:?})"),
    }
    println!("--- R1 PROBE END ---");
}

#[cfg(feature = "probe")]
#[test]
fn r1_public_doors_at_probe() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let a = minted(1.5e-5, 0.05, tol, band);
    println!("--- R1 PROBE(probe) BEGIN ---");
    doors::<geom_core::Probe>("probe", &a, tol);
    println!("--- R1 PROBE(probe) END ---");
}

#[cfg(feature = "interval")]
#[test]
fn r1_public_doors_at_interval() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let a = minted(1.5e-5, 0.05, tol, band);
    println!("--- R1 PROBE(interval) BEGIN ---");
    doors::<geom_core::interval::Interval>("interval", &a, tol);
    println!("--- R1 PROBE(interval) END ---");
}
