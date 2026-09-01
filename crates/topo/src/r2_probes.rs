//! R2 REVIEW PROBES for MATE-3 (PR 1423). Not part of the unit under
//! review; committed to the reviewer's own branch only.
//!
//! The point of this module is that it does NOT call
//! `classify_material_pairing` or `material_kappa_rel`. It re-derives
//! the cusp/slit question from an independent datum — the body's own
//! SIGNED VOLUME, and a point-membership test built from
//! `implicit_residual` and the face sense bits — and then asks whether
//! the validator's verdict agrees.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::MaterialWedge;
use geom_core::{Point3, Tol, Vec3};

use crate::validate::{ValidationError, validate_geometric, validate_geometric_declared};

fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// The face's outward-normal sign, read off the sense BIT (the D6
/// per-file inventory counts the `\u{00b1}1` accessor by name, and a review
/// probe must not perturb that census).
fn sgn(b: &crate::Body<f64>, f: crate::entity::FaceKey) -> f64 {
    if b.get_face(f).unwrap().sense { 1.0 } else { -1.0 }
}

/// **Oracle 1 — the volume says which region is material.**
///
/// The cusp prism's cross-section is the crescent between the circles
/// (centre (0,1), r 1) and (centre (0,2), r 2), cut at x = 0 and kept
/// on x ≥ 0. Areas: half of (π·2² − π·1²) = 3π/2. Extruded 1 in z, the
/// solid's volume must be +3π/2 ≈ 4.712389.
///
/// A POSITIVE volume of exactly that number is the independent fact
/// that the material really is the vanishing crescent between the two
/// kissing walls — i.e. that the kissing edge's material wedge is 0,
/// a CUSP — with no reference to the sign chain under review.
#[test]
fn r2_the_cusp_prisms_material_is_the_crescent_by_volume() {
    let tol = Tol::witness();
    let p = crate::tier3_tests::cusp_prism(tol);
    let props = crate::mass_properties(&p.body, tol).expect("volume meters");
    let expect = 1.5 * std::f64::consts::PI;
    assert!(
        (props.volume - expect).abs() < 1e-9,
        "crescent lip volume: got {}, hand-derived {expect}",
        props.volume
    );
    // And the validator, asked the same body, calls the kissing edge a
    // CUSP. The two agree.
    let errs = validate_geometric(&p.body, tol).unwrap_err();
    assert_eq!(
        errs,
        vec![ValidationError::UndeclaredCusp {
            edge: p.ev[0],
            wedge: MaterialWedge::Cusp,
        }]
    );
}

/// **Oracle 2 — point membership at the kissing edge, from the sense
/// bits and the implicit residuals only.**
///
/// With the material region known from oracle 1 (the crescent), a
/// point strictly inside the crescent must satisfy `sense·F < 0` for
/// BOTH walls, and a point just on the other side of either wall must
/// not. That is the local picture "material is the vanishing crescent
/// between two tangent faces" — wedge 0 — checked against the actual
/// stored senses rather than asserted.
///
/// The same test run on the REVERTED body must invert every one of
/// those memberships, which is what makes `Cusp ↔ Slit` a fact about
/// the geometry rather than about the formula.
#[test]
fn r2_membership_near_the_kiss_is_the_crescent_and_reverts_to_its_complement() {
    let tol = Tol::witness();
    let p = crate::tier3_tests::cusp_prism(tol);
    let inner = Surface::Cylinder {
        origin: pt(0.0, 1.0, 0.0),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let outer = Surface::Cylinder {
        origin: pt(0.0, 2.0, 0.0),
        axis: Vec3::unit_z(),
        radius: 2.0,
        u_ref: Vec3::unit_x(),
    };
    // Read the two walls' senses off the body rather than assuming.
    let s_inner = sgn(&p.body, p.face_side[2]);
    let s_outer = sgn(&p.body, p.face_side[0]);
    let material = |q: Point3<f64>, si: f64, so: f64| {
        si * geom_brep::implicit_residual(&inner, q) < 0.0
            && so * geom_brep::implicit_residual(&outer, q) < 0.0
    };
    // x = 0.1 off the kiss: the crescent there is y ∈ (x²/4, x²/2) =
    // (0.0025, 0.005). Midpoint 0.00375 is material; 0.001 (below the
    // outer wall) and 0.01 (above the inner wall) are void.
    let x = 0.1;
    let inside = pt(x, 0.00375, 0.5);
    let below = pt(x, 0.001, 0.5);
    let above = pt(x, 0.01, 0.5);
    assert!(material(inside, s_inner, s_outer), "the crescent is material");
    assert!(!material(below, s_inner, s_outer), "below the outer wall is void");
    assert!(!material(above, s_inner, s_outer), "inside the inner wall is void");
    // Revert negates every outward normal, so every membership flips.
    let r = p.body.revert().unwrap();
    let r_inner = sgn(&r, p.face_side[2]);
    let r_outer = sgn(&r, p.face_side[0]);
    assert_eq!(r_inner, -s_inner);
    assert_eq!(r_outer, -s_outer);
    assert!(!material(inside, r_inner, r_outer));
    // The reverted body's verdict is the SLIT, and the geometry agrees:
    // the crescent is now void from both walls' point of view, so the
    // material is everything else — wedge 2π.
    let errs = validate_geometric(&r, tol).unwrap_err();
    assert_eq!(
        errs,
        vec![ValidationError::UndeclaredCusp {
            edge: p.ev[0],
            wedge: MaterialWedge::Slit,
        }]
    );
}

/// **Probe — a declaration that names nothing real costs nothing.**
/// The doc claims it; here it is executed on a real solid, and on the
/// cusp prism where a foreign declaration must not legalize the kiss.
#[test]
fn r2_a_foreign_declaration_asserts_nothing() {
    let tol = Tol::witness();
    let p = crate::tier3_tests::cusp_prism(tol);
    let mut d = vec![crate::contact::DeclaredContact {
        a: p.face_top,
        b: p.face_bottom,
        class: crate::contact::ContactClass::Tangent,
    }];
    // Still refuses.
    assert!(validate_geometric_declared(&p.body, &d, tol).is_err());
    // Add the real one alongside: green, and the noise still costs
    // nothing.
    d.push(crate::contact::DeclaredContact {
        a: p.face_side[0],
        b: p.face_side[2],
        class: crate::contact::ContactClass::Tangent,
    });
    assert_eq!(validate_geometric_declared(&p.body, &d, tol), Ok(()));
}

/// **Probe — the declaration gate, every axis at once.** The PR claims
/// a `Rest` claim and a foreign pair both fail to legalize. Executed
/// here with the ERROR VECTOR compared, not merely `is_err()`: the
/// committed row only asserts `is_err()`, which a differently-typed
/// refusal would also satisfy.
#[test]
fn r2_declaration_gating_is_by_class_and_by_pair_and_stays_the_same_refusal() {
    let tol = Tol::witness();
    let p = crate::tier3_tests::cusp_prism(tol);
    let want = vec![ValidationError::UndeclaredCusp {
        edge: p.ev[0],
        wedge: MaterialWedge::Cusp,
    }];
    let mk = |a, b, class| [crate::contact::DeclaredContact { a, b, class }];
    use crate::contact::ContactClass::{Rest, Tangent};
    // Right pair, wrong class.
    assert_eq!(
        validate_geometric_declared(&p.body, &mk(p.face_side[0], p.face_side[2], Rest), tol)
            .unwrap_err(),
        want
    );
    // Right class, wrong pair (two other faces).
    assert_eq!(
        validate_geometric_declared(&p.body, &mk(p.face_top, p.face_bottom, Tangent), tol)
            .unwrap_err(),
        want
    );
    // Right class, HALF the right pair — one wall against a cap.
    assert_eq!(
        validate_geometric_declared(&p.body, &mk(p.face_side[0], p.face_top, Tangent), tol)
            .unwrap_err(),
        want
    );
    // Right class, right pair, order swapped: legal (unordered).
    assert_eq!(
        validate_geometric_declared(&p.body, &mk(p.face_side[2], p.face_side[0], Tangent), tol),
        Ok(())
    );
}

/// **Probe — the `LaminaWedge` message against the body it now fires
/// on.** The message asserts "a lamina: a zero-volume geometric
/// defect". This probe measures the volume of a body the arm calls a
/// lamina and reports it, so the claim is checkable rather than
/// asserted.
#[test]
fn r2_lamina_message_versus_the_measured_volume() {
    let tol = Tol::witness();
    // Two coplanar faces of one solid with one face's sense flipped is
    // the committed lamina row; a same-surface split on a CURVED wall
    // with one side flipped is the corpus shape (step-export's conic
    // trim, the rimless ball). Build the curved one here on a prism
    // whose two adjacent side faces share one cylinder.
    let p = crate::tier3_tests::cusp_prism(tol);
    let v_ok = crate::mass_properties(&p.body, tol).unwrap().volume;
    let flipped = p.body.flipped_face_sense_for_tests(p.face_side[1]).unwrap();
    let errs = validate_geometric(&flipped, tol).unwrap_err();
    let v_flipped = crate::mass_properties(&flipped, tol).map(|m| m.volume);
    println!("R2: cusp prism volume {v_ok}; flat-face-flipped volume {v_flipped:?}");
    println!("R2: flipped-flat verdict {errs:?}");
}
