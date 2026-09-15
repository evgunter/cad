//! **R1's end-to-end exercise for SENSE-FOLD (review probe).** Not a
//! pinned row: it drives the kernel the way a user would — extrude,
//! subtract, fillet a convex edge set and a concave one, validate a
//! body with a reversed face — and PRINTS a bitwise digest of every
//! answer, so the same file run at the merge base and at the head is a
//! differential over the folded sites rather than a re-run of the
//! unit's own suites.
//!
//! Sites it reaches: `join::ring_run_ccw`, `rest::face_carrier`,
//! `solid_contain::{face_plane, face_geo}`, `merge_faces` (both),
//! `validate.rs` check 6, `blend::build::outward_of`,
//! `blend::battery::outward` and `arms.rs`'s `SupportTrace::side` on
//! BOTH ball sides (the convex fillet and the concave one).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::cavity::{brick, cavity_edges, edges_with_corners, rod, vented_cavity};
use geom_core::{Point2, Point3, Tol};
use sweep::blend::fillet_edges;
use topo::{Body, ValidationError, mass_properties, subtract, validate_geometric};

/// A body's whole geometric content as one hex digest: every point's
/// three coordinate bit patterns and every face's sense bit, folded in
/// the arena's own order (which is deterministic per D2), plus the
/// Euler counts.
fn digest(body: &Body<f64>) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut mix = |x: u64| {
        h ^= x;
        h = h.wrapping_mul(0x1000_0000_01b3);
    };
    let mut n_pt = 0u64;
    for (_, p) in body.points() {
        n_pt += 1;
        mix(p.x.to_bits());
        mix(p.y.to_bits());
        mix(p.z.to_bits());
    }
    let mut n_face = 0u64;
    for (_, f) in body.faces() {
        n_face += 1;
        mix(u64::from(f.sense));
    }
    let n_edge = body.edges().count() as u64;
    let n_vert = body.vertices().count() as u64;
    let mp = mass_properties(body, Tol::witness());
    let vol = mp.as_ref().map_or(0u64, |m| m.volume.to_bits());
    let area = mp.as_ref().map_or(0u64, |m| m.surface_area.to_bits());
    format!("V={n_vert} E={n_edge} F={n_face} P={n_pt} h={h:016x} vol={vol:016x} area={area:016x}")
}

/// **The reversed face, through the front door.** A user builds a
/// brick, flips one face's orientation bit, and asks the kernel to
/// validate it: check 6 reads the planar door and falsifies the loop's
/// stored winding against the bit.
#[test]
fn e2e_r1_a_reversed_face_is_refused_by_the_geometric_validator() {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 1.0, 3.0));
    assert_eq!(
        validate_geometric(&block, Tol::witness()),
        Ok(()),
        "the honest brick validates"
    );
    println!("R1-E2E brick {}", digest(&block));
    let mut reversed = Vec::new();
    for (fk, _) in block.faces() {
        let flipped = block
            .flipped_face_sense_for_tests(fk)
            .expect("the face is live");
        let errors = validate_geometric(&flipped, Tol::witness())
            .expect_err("a lone reversed face is a corrupt body");
        let named = errors
            .iter()
            .filter(|e| matches!(e, ValidationError::LoopRoleInverted { .. }))
            .count();
        reversed.push(format!("{named}/{}", errors.len()));
        println!("R1-E2E reversed-face {}", digest(&flipped));
    }
    println!("R1-E2E check6-refusals {}", reversed.join(" "));
    assert!(
        reversed.iter().all(|r| r.starts_with("1/")),
        "every reversed face names exactly one inverted loop; got {reversed:?}"
    );
}

/// **A boolean a user would write**: a round vent cut through a block,
/// and the same cut where the tool's wall is TANGENT to two of the
/// block's side faces — the configuration the contact verifier is
/// there for. Both answers are digested bitwise.
#[test]
fn e2e_r1_a_subtract_and_a_tangent_subtract() {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let vent = rod(Point2::new(2.0, 2.0), 0.75, -1.0, 5.0);
    let cut = subtract(&block, &vent, Tol::witness())
        .expect("a through-vent cut succeeds")
        .body()
        .expect("it leaves material")
        .body
        .clone();
    println!("R1-E2E vented-block {}", digest(&cut));
    assert_eq!(validate_geometric(&cut, Tol::witness()), Ok(()));

    // Tangent: the rod's wall touches the block's x = 0 and y = 0
    // faces exactly.
    let tool = rod(Point2::new(2.0, 2.0), 2.0, -1.0, 5.0);
    let tangent = subtract(&block, &tool, Tol::witness());
    match tangent {
        Ok(r) => match r.body() {
            Some(b) => println!("R1-E2E tangent-cut ok {}", digest(&b.body)),
            None => println!("R1-E2E tangent-cut ok empty"),
        },
        Err(e) => println!("R1-E2E tangent-cut refused {e:?}"),
    }
}

/// **A convex fillet and a concave fillet, in one row.** The ball
/// rests on opposite sides of its supports in the two, so the two
/// `SupportTrace::side` bits differ and every `sided(..)` in the sheet
/// reduction is exercised both ways. Both results are digested.
#[test]
fn e2e_r1_a_convex_and_a_concave_fillet_digest() {
    // Convex: the four vertical edges of a brick.
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let all12 = edges_with_corners(&block, |_| true);
    assert_eq!(all12.len(), 12, "the brick has twelve edges");
    let convex = fillet_edges(&block, &all12, 0.5, Tol::witness())
        .expect("the convex fillet lands")
        .body;
    println!("R1-E2E convex-fillet {}", digest(&convex));
    assert_eq!(validate_geometric(&convex, Tol::witness()), Ok(()));

    // Concave: the twelve edges of a vented cavity.
    let cavity = vented_cavity();
    let edges = cavity_edges(&cavity);
    assert_eq!(edges.len(), 12, "the cavity has twelve concave edges");
    let concave = fillet_edges(&cavity, &edges, 0.25, Tol::witness())
        .expect("the concave fillet lands")
        .body;
    println!("R1-E2E concave-fillet {}", digest(&concave));
    assert_eq!(validate_geometric(&concave, Tol::witness()), Ok(()));
}

/// **The contact verifier, at both faces' bits.** Site 12–13's fold:
/// the tangency verifier forms each face's outward normal from the
/// implicit gradient and the face's bit, and refuses a declared
/// tangency whose normals do not point at each other. A plane and a
/// cylinder resting on it, tangent along the x-axis, at all four bit
/// pairs — the verdict differs by pair, which is the anti-vacuity, and
/// the whole table is printed for the base/head differential.
#[test]
fn e2e_r1_the_contact_verifier_reads_both_face_bits() {
    use geom::{Curve3, Surface as S};
    use geom_core::{Band, Vec3};
    use topo::boolean::contact_verify::tangent_locus_relation;

    let band = Band::linear(Tol::witness()).unwrap();
    let cyl = S::Cylinder {
        origin: Point3::new(0.0, 0.0, 1.0),
        axis: Vec3::new(1.0, 0.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let flat = S::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let locus = Curve3::Line {
        origin: Point3::new(0.0, 0.0, 0.0),
        dir: Vec3::new(1.0, 0.0, 0.0),
    };
    let mut table = Vec::new();
    for s1 in [true, false] {
        for s2 in [true, false] {
            let out = tangent_locus_relation(&cyl, s1, &flat, s2, &locus, -1.0, 1.0, true, band);
            table.push(format!("({s1},{s2})={out:?}"));
        }
    }
    println!("R1-E2E contact-verify {}", table.join(" | "));
    assert!(
        table.iter().collect::<std::collections::BTreeSet<_>>().len() > 1,
        "the two bits must change the verdict somewhere; got {table:?}"
    );
}
