//! LANE-0 review probes (R2), the closed-body half: a box whose cap
//! wears a certified `Approx` surface reaches tier 3's check 1 through
//! the PUBLIC validators, the transform door's `Approx` arm through
//! `transform_rigid`, and the offset mint through `replace_face_offset`
//! on a NURBS-faced box. Every row prints what the door answered so
//! the merge base and the head diff byte for byte. Public API only.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::Surface;
use geom_core::{Affine3, Band, Point3, Tol, Vec3};
use topo::{Body, ContactRecords, FaceSurface};

use crate::common::approx::{band, box_with_approx_cap, planar_patch, top_face, unit_box};

fn tol() -> Tol {
    Tol::witness()
}

fn turned() -> Affine3<f64> {
    Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), 0.7)
}

fn dump_approx_bits(label: &str, body: &Body<f64>) {
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

#[test]
fn dump_closed_body_f64() {
    let _ = Band::linear(tol()).unwrap();
    let (body, face) = box_with_approx_cap(0.05, 1e-9);
    println!("cap face {face:?}");
    dump_approx_bits("seed", &body);
    dump_validation("validate_geometric", &topo::validate_geometric(&body, tol()));
    dump_validation(
        "validate_geometric_structural",
        &topo::validate_geometric_structural(&body, tol()),
    );
    dump_validation(
        "validate_pseudomanifold",
        &topo::validate_pseudomanifold(&body, &ContactRecords::default(), tol()),
    );
    dump_validation(
        "validate_pseudomanifold_certified",
        &topo::validate_pseudomanifold_certified(&body, &ContactRecords::default(), tol()),
    );
    match topo::transform_rigid(&body, &turned(), tol()) {
        Ok(mapped) => {
            println!("transform_rigid: Ok");
            dump_approx_bits("mapped", &mapped);
            dump_validation(
                "mapped validate_geometric",
                &topo::validate_geometric(&mapped, tol()),
            );
        }
        Err(e) => println!("transform_rigid: Err {e:?} | display: {e}"),
    }

    // The mint through the public door: the box's cap becomes a NURBS
    // plane (its edges already lie on it), and the offset of a NURBS
    // is the fit door's business.
    let mut nurbs_box = unit_box();
    let cap = top_face(&nurbs_box);
    nurbs_box
        .set_face_surface(cap, FaceSurface::New(Surface::Nurbs(Arc::new(planar_patch(1.0)))))
        .expect("the cap takes a NURBS plane");
    match topo::replace_face_offset(&mut nurbs_box, cap, 0.05, band(), tol()) {
        Ok(()) => {
            println!("replace_face_offset: Ok");
            dump_approx_bits("offset", &nurbs_box);
        }
        Err(e) => println!("replace_face_offset: Err {e:?} | display: {e}"),
    }
}
