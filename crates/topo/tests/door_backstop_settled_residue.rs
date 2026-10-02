//! **A declared coincidence the door settles inside the band moves a
//! correct result's volume, and at a tight bound the volume backstop
//! refuses it** — a correct body refused, the safe direction, filed as
//! `work/reach/a-settled-declared-coincidence-crosses-a-tight-volume-bound.md`.
//!
//! The fixture is a block and a parallelepiped cornered on its top face
//! by a 5° wedge angle, the wedge's face there tilted about its one edge
//! by `θ`, an angle the door reads in band across both faces, so the two
//! are declared one plane (a `Rest` contact standing on the block, a
//! continuation sunk flush into it). The door glues the pair onto one
//! carrier, and the result's volume stands off the box arithmetic by at
//! most the gap between the two faces, `½·|θ|·sin² φ`, in the direction
//! the tilt sends it:
//!
//! - standing, tilted up: the union crosses `vol(A) + vol(B)`;
//! - sunk, tilted down: the intersect crosses `vol(B)` and `A ∖ B`
//!   crosses `vol(A) − vol(B)`;
//! - every other op and tilt builds at the box arithmetic.
//!
//! A crossing refuses wherever the interval margin certifies it. Each
//! row pins the verdict measured at ε = 1e-9, 1e-6 and 1e-12: the sunk
//! intersect refuses at all three, and the standing union and the sunk
//! `A ∖ B` (13.5 m³ bodies, whose sums round coarser) refuse down to
//! 1e-9 and build within the gap at 1e-12.
//!
//! Sunk at `2ε` the intersect and subtract refuse in the join instead
//! (`work/join/a-declared-flush-wedge-sunk-in-a-block-refuses-its-intersect-join-desync.md`).
//!
//! Oracle: box arithmetic — the block `3 × 4.5 × 1`, the parallelepiped
//! `sin φ · h` (its base parallelogram's area, whatever the tilt, times
//! its vertical height).

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

/// What one op is expected to do.
#[derive(Clone, Copy, Debug)]
enum Want {
    /// Build at this volume (`0` is empty), within the gap.
    Builds(f64),
    /// The result crosses this bound by up to the gap. At every ε
    /// down to the third field the volume backstop refuses it naming
    /// the bound; below it the crossing is under the interval's own
    /// rounding and builds within the gap at this volume.
    Crosses(&'static str, f64, f64),
    /// The join refuses: neither section loop's regions hold a
    /// decisive witness.
    Join,
}

#[test]
fn a_settled_in_band_coincidence_refuses_where_it_crosses_a_tight_bound() {
    use Want::{Builds, Crosses, Join};
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let phi = 5.0_f64.to_radians();
    let p = Point3::new(0.5, 0.2, 1.0);
    let block = brick::<f64>((0.0, 3.0), (-2.0, 2.5), (0.0, 1.0), tol);
    let block_volume = 3.0 * 4.5;
    let wedge = |h: f64| phi.sin() * h;
    let (standing, sunk) = ((1.0, 0.0, -1.0), (0.5, 0.5, 1.0));
    let (rest, cont) = (BooleanCoincidence::REST, BooleanCoincidence::Continuation);
    let (union_cap, cap, floor) = (
        "vol(A ∪ B) ≤ vol(A) + vol(B)",
        "vol(A ∩ B) ≤ vol(B)",
        "vol(A ∖ B) ≥ vol(A) − vol(B)",
    );
    // (pose, θ over ε, (height, depth, the wedge face's facing), class,
    //  [∪, ∩, A ∖ B, B ∖ A])
    let rows = [
        (
            "standing",
            1.2,
            standing,
            rest,
            [
                Crosses(union_cap, block_volume + wedge(1.0), 1e-9),
                Builds(0.0),
                Builds(block_volume),
                Builds(wedge(1.0)),
            ],
        ),
        (
            "standing",
            -1.2,
            standing,
            rest,
            [
                Builds(block_volume + wedge(1.0)),
                Builds(0.0),
                Builds(block_volume),
                Builds(wedge(1.0)),
            ],
        ),
        (
            "sunk",
            1.2,
            sunk,
            cont,
            [
                Builds(block_volume),
                Builds(wedge(0.5)),
                Builds(block_volume - wedge(0.5)),
                Builds(0.0),
            ],
        ),
        (
            "sunk",
            -1.2,
            sunk,
            cont,
            [
                Builds(block_volume),
                Crosses(cap, wedge(0.5), 1e-12),
                Crosses(floor, block_volume - wedge(0.5), 1e-9),
                Builds(0.0),
            ],
        ),
        (
            "sunk",
            2.0,
            sunk,
            cont,
            [Builds(block_volume), Join, Join, Builds(0.0)],
        ),
    ];
    for (pose, over_eps, (height, depth, facing), class, wants) in rows {
        let theta = over_eps * band.zero();
        let gap = 0.5 * theta.abs() * phi.sin().powi(2);
        let (ea, eb) = (
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(phi.cos(), phi.sin(), theta * phi.sin()),
        );
        let tool = mapped_cube::<f64>(
            move |u, v, w| p + ea * u + eb * v + Vec3::new(0.0, 0.0, height * w - depth),
            tol,
        );
        let (top, face) = (face_facing(&block, 1.0), face_facing(&tool, facing));
        let ab = declared(top, face, class);
        let ba = declared(face, top, class);
        let ops = [
            ("A ∪ B", topo::union_with(&block, &tool, &ab, tol)),
            ("A ∩ B", topo::intersect_with(&block, &tool, &ab, tol)),
            ("A ∖ B", topo::subtract_with(&block, &tool, &ab, tol)),
            ("B ∖ A", topo::subtract_with(&tool, &block, &ba, tol)),
        ];
        for ((op, out), want) in ops.into_iter().zip(wants) {
            let label = format!("{pose} at θ = {over_eps}ε, {op}");
            let builds = |out: Result<BooleanResult<f64>, BooleanError>, want: f64| match out {
                Ok(BooleanResult::Empty) => assert_eq!(want, 0.0, "{label}: empty"),
                Ok(BooleanResult::Body(bb)) => {
                    assert_eq!(
                        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol),
                        Ok(()),
                        "{label}: tier 3′"
                    );
                    let got = topo::mass_properties(&bb.body, tol)
                        .expect("its volume")
                        .volume;
                    assert!(
                        (got - want).abs() <= gap,
                        "{label}: {got} vs {want}, the gap {gap}"
                    );
                }
                Err(e) => panic!("{label}: it builds: {e:?}"),
            };
            match (want, out) {
                (Builds(want), out) => builds(out, want),
                (Crosses(bound, _, down_to), out) if band.zero() >= down_to => assert!(
                    matches!(out, Err(BooleanError::ResultVolumeImplausible { which, .. })
                        if which == bound),
                    "{label}: refuses {bound}: {out:?}"
                ),
                (Crosses(_, want, _), out) => builds(out, want),
                (Join, out) => assert!(
                    matches!(
                        out,
                        Err(BooleanError::Join(
                            topo::SplitJoinError::SectionLoopUndecided { .. }
                        ))
                    ),
                    "{label}: the join refuses: {out:?}"
                ),
            }
        }
    }
}
