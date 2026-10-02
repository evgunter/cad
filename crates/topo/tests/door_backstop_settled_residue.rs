//! **A declared coincidence the door settles inside the band builds
//! every op, whichever way its residue falls** — and the volume
//! backstop reads the residue as what the door may move, not as a
//! defect.
//!
//! The fixture is a block and a parallelepiped cornered on its top face
//! by a 5° wedge angle, the wedge's face there tilted about its one edge
//! by an angle inside the band, so the two faces are declared one plane
//! (a `Rest` contact standing on the block, a continuation sunk flush
//! into it). The door glues the pair onto one carrier, and the face it
//! drops leaves a residue of at most the band's displacement over the
//! wedge's face: each result's volume stands off the box arithmetic by
//! that much, in the direction the tilt sends it. Tilted up, the union
//! standing on the block exceeds `vol(A) + vol(B)`; tilted down, the
//! sunk intersect exceeds `vol(B)` and the sunk `A ∖ B` falls short of
//! `vol(A) − vol(B)`. Each is a correct result at a bound it legitimately
//! passes by less than the band, and each builds.
//!
//! Oracle: box arithmetic — the block `3 × 4.5 × 1`, the parallelepiped
//! `sin φ · h` (its base parallelogram's area, whatever the tilt, times
//! its vertical height) — within the band's displacement over the
//! wedge's face, `escalate · sin φ`.

#![allow(clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point3, Tol, Vec3};
use topo::test_support::{brick, mapped_cube};
use topo::{
    Body, BooleanCoincidence, BooleanDeclarations, BooleanError, BooleanResult, CarrierDesc,
    FacePairDeclaration, face_carrier,
};

/// The one face of `body` whose plane's normal leans along `facing`
/// (`+1` up, `−1` down).
fn face_facing(body: &Body<f64>, facing: f64) -> topo::FaceKey {
    let hits: Vec<_> = body
        .faces()
        .map(|(k, _)| k)
        .filter(|&k| {
            matches!(face_carrier(body, k),
                Some(CarrierDesc::Plane { normal, .. }) if normal.z * facing > 0.99)
        })
        .collect();
    assert_eq!(hits.len(), 1, "one face faces {facing}");
    hits[0]
}

fn declared(a: topo::FaceKey, b: topo::FaceKey, class: BooleanCoincidence) -> BooleanDeclarations {
    BooleanDeclarations {
        coincident_faces: vec![FacePairDeclaration::new(a, b, class)],
        ..BooleanDeclarations::none()
    }
}

/// The result's volume, or `0` for an empty one, after the result
/// passes its at-rest gate.
fn built(label: &str, out: Result<BooleanResult<f64>, BooleanError>, tol: Tol) -> f64 {
    match out {
        Ok(BooleanResult::Empty) => 0.0,
        Ok(BooleanResult::Body(bb)) => {
            assert_eq!(
                topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol),
                Ok(()),
                "{label}: tier 3′"
            );
            topo::mass_properties(&bb.body, tol)
                .unwrap_or_else(|e| panic!("{label}: its volume: {e:?}"))
                .volume
        }
        Err(e) => panic!("{label}: it builds: {e:?}"),
    }
}

#[test]
fn a_settled_in_band_coincidence_builds_every_op_whichever_way_its_residue_falls() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let phi = 5.0_f64.to_radians();
    let p = Point3::new(0.5, 0.2, 1.0);
    let block = brick::<f64>((0.0, 3.0), (-2.0, 2.5), (0.0, 1.0), tol);
    let block_volume = 3.0 * 4.5;
    let wedge_volume = |height: f64| phi.sin() * height;
    let residue = band.escalate() * phi.sin();
    for tilt in [1.0, -1.0] {
        let theta = tilt * (band.zero() + band.escalate()) / 2.0;
        let (ea, eb) = (
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(phi.cos(), phi.sin(), theta * phi.sin()),
        );
        let standing = mapped_cube::<f64>(
            move |u, v, w| p + ea * u + eb * v + Vec3::new(0.0, 0.0, w),
            tol,
        );
        let sunk = mapped_cube::<f64>(
            move |u, v, w| p + ea * u + eb * v + Vec3::new(0.0, 0.0, 0.5 * (w - 1.0)),
            tol,
        );
        // (pose, wedge, the wedge face on the block's top, the class,
        //  ∪, ∩, A ∖ B, B ∖ A)
        let poses = [
            (
                "standing on the block",
                standing,
                -1.0,
                BooleanCoincidence::REST,
                [
                    block_volume + wedge_volume(1.0),
                    0.0,
                    block_volume,
                    wedge_volume(1.0),
                ],
            ),
            (
                "sunk into the block",
                sunk,
                1.0,
                BooleanCoincidence::Continuation,
                [
                    block_volume,
                    wedge_volume(0.5),
                    block_volume - wedge_volume(0.5),
                    0.0,
                ],
            ),
        ];
        for (pose, wedge, facing, class, oracle) in poses {
            let (top, face) = (face_facing(&block, 1.0), face_facing(&wedge, facing));
            let ab = declared(top, face, class);
            let ba = declared(face, top, class);
            let ops = [
                ("A ∪ B", topo::union_with(&block, &wedge, &ab, tol)),
                ("A ∩ B", topo::intersect_with(&block, &wedge, &ab, tol)),
                ("A ∖ B", topo::subtract_with(&block, &wedge, &ab, tol)),
                ("B ∖ A", topo::subtract_with(&wedge, &block, &ba, tol)),
            ];
            for ((op, out), want) in ops.into_iter().zip(oracle) {
                let label = format!("{pose}, tilted {tilt}, {op}");
                let got = built(&label, out, tol);
                assert!(
                    (got - want).abs() <= residue,
                    "{label}: {got} vs {want}, residue {residue}"
                );
            }
        }
    }
}
