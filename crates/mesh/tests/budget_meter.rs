//! **The budget meter's kernel half, end to end** (issue #320): the
//! meter is inert unless armed, measures every NURBS face when it is,
//! and does not change the mesh it measures.
//!
//! What the numbers MEAN is `tools/tess-meter`'s, and so are its
//! tests. What is checked here is what only this crate can check: that
//! arming is opt-in, that the measurements are attributed to the right
//! faces, and that taking them perturbs nothing.
//!
//! The body is the `loft_prism` corpus fixture (#212), constant for
//! constant with the trimmed-NURBS suite's — degree 1×2 walls, which
//! is exactly the anisotropic class the split-slack column is about.

// The meter is opt-in (`mesh`'s `budget` feature, gated at the module
// boundary — see `mesh::budget`), so this suite is too: without it
// there is no `arm`/`take` to drive. CI runs it in its own row, the
// way the `interval` lane's rows work.
#![cfg(feature = "budget")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Affine3, Vec3};
use mesh::budget::{self, Mode};
use sweep::loft_body;
use topo::Body;

mod common;
use common::quad;
use geom_core::Tol;

/// The tightness floor `the_deviation_pass_samples_and_stays_under_its_certificates`
/// asserts, and the argument for its value is there.
const RATIO_FLOOR: f64 = 0.1;

/// The `loft_prism` corpus body (#212): squares at z = 0 and 2, the
/// non-affine trapezoid at z = 1, v-degree 2.
fn loft_prism() -> Body<f64> {
    let sections = vec![
        quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        quad([(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
    ];
    let places: Vec<Affine3<f64>> = [0.0, 1.0, 2.0]
        .iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect();
    loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .expect("the corpus loft builds")
        .body
}

/// The body's described-NURBS faces, in arena order.
fn nurbs_faces(body: &Body<f64>) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface).expect("face surface"),
                Surface::Nurbs(_)
            )
        })
        .map(|(fk, _)| fk)
        .collect()
}

/// Disarmed is the normal state, and it records nothing — the meter
/// must not be a thing a caller can accidentally pay for.
#[test]
fn a_disarmed_meter_records_nothing() {
    let body = loft_prism();
    assert!(!budget::armed());
    assert!(budget::deviation_samples().is_none());
    mesh::tessellate(&body, 6e-3, Tol::witness()).expect("tessellates");
    assert!(budget::take().is_empty());
}

/// Armed: one measurement per NURBS face, attributed by key, with the
/// certified bounds and the built grid on it. Non-NURBS faces are the
/// consumer's to describe (`tools/tess-meter`) and the meter says
/// nothing about them.
#[test]
fn every_nurbs_face_is_measured_once_and_by_key() {
    let body = loft_prism();
    let walls = nurbs_faces(&body);
    assert!(!walls.is_empty(), "the loft's walls are NURBS faces");
    budget::arm(Mode::Sizing);
    mesh::tessellate(&body, 6e-3, Tol::witness()).expect("tessellates");
    let measures = budget::take();
    assert_eq!(
        measures.iter().map(|m| m.face).collect::<Vec<_>>(),
        walls,
        "one measurement per NURBS face, in face-arena order, keyed to its own face"
    );
    for m in &measures {
        assert!(m.grid_cells > 0, "the built grid is counted: {m:?}");
        assert!(
            !m.cells.is_empty(),
            "the analysis cells are reported: {m:?}"
        );
        assert!(
            m.muu.is_finite()
                && m.muv.is_finite()
                && m.mvv.is_finite()
                && m.mu1.is_finite()
                && m.mv1.is_finite(),
            "the whole-patch bound is certified and finite, first-derivative sups \
             included: {m:?}"
        );
        assert!(
            m.worst_cert.is_finite() && m.worst_cert > 0.0,
            "the face's worst certificate is recorded: {m:?}"
        );
        assert!(
            m.worst_dev.is_nan() && m.worst_ratio.is_nan() && m.dev_samples == 0,
            "Mode::Sizing does not resample: {m:?}"
        );
        assert!(
            m.u.1 > m.u.0 && m.v.1 > m.v.0,
            "the trim box is a box: {m:?}"
        );
    }

    // THE BOUND BELONGS TO THE FACE IT IS FILED UNDER.
    //
    // The whole-patch bound reaches this row through
    // `nurbs_cert::face_bound`'s per-tessellation memo, which is keyed
    // by `FaceKey` — and a memo that returns the wrong entry (or any
    // entry) is invisible to every assertion above, because each one
    // reads a single row in isolation. This fixture can see it: the
    // loft's walls carry genuinely different Hessians (the two
    // trapezoid-slanted walls, the two straight ones), so a lookup
    // that stopped keying would collapse them all onto one value.
    //
    // Distinctness rather than pinned numbers on purpose — the values
    // are a certified bound's business and may legitimately move; that
    // they DIFFER between differently-shaped walls may not.
    let bounds: Vec<(u64, u64, u64)> = measures
        .iter()
        .map(|m| (m.muu.to_bits(), m.muv.to_bits(), m.mvv.to_bits()))
        .collect();
    let distinct: std::collections::HashSet<_> = bounds.iter().collect();
    assert!(
        distinct.len() > 1,
        "every measured face got the same whole-patch bound, so the per-face \
         lookup is not keying on the face: {bounds:?}"
    );
}

/// The deviation pass samples, and every sample is dominated by its
/// own triangle's certificate — `worst_ratio` is that check for the
/// whole face, one number, carried out of the kernel instead of
/// asserted inside it.
///
/// # The ceiling is monotone in the safe direction, so there is a floor
///
/// `worst_ratio = d / (cert + ε) ≤ 1` gets EASIER as the certificate
/// grows: a bound loose enough to be worth #320's attention passes it
/// by a wider margin than a tight one, and the row would report green
/// hardest exactly where the budget question is sharpest. The floor
/// below is the other side of that bracket.
///
/// **It applies where the CERTIFICATE is the denominator, and ε is
/// what decides where that is.** This loft's fourth wall is planar: it
/// certifies at ~5e-17 while its sampled deviation is ~5e-16, so ε
/// dominates its ratio outright — measured 5e-7, 5e-10 and 5e-4 at the
/// default, 1e-6 and 1e-12 legs, three orders apart on one face with no
/// mesh change at all. `worst_cert > eps` separates that face from the
/// three curved walls with orders of margin at every leg (7e-4 against
/// 1e-6 on one side, 5e-17 against 1e-12 on the other), and it is the
/// condition under which a floor is about the certificate at all.
///
/// **The floor is BRACKETED from both sides, and the bracket is
/// narrow.** Measured on this fixture through this row's own arming,
/// over δ ∈ [1e-4, 1e3] — seven orders of magnitude — at four ε legs
/// (default, 1e-6, 1e-9, 1e-12): the gated ratio runs **0.1667 to
/// 0.4966**, rising monotonically as δ shrinks toward an asymptote
/// near 0.5 and **bottoming out at 1/6** once the sizing reaches its
/// coarsest grid, where it stays for every δ above ~10. So the
/// legitimate population has a floor it REACHES rather than
/// approaches, and [`RATIO_FLOOR`] sits **1.67×** under it.
///
/// The other side of the bracket is what the row must still catch: a
/// **5×** loosening of `grid.cert` drops the worst ratio to **0.0971**
/// (measured). The admissible interval is therefore
/// `[0.0971, 0.1667]` and 0.1 sits near its bottom. A safer-looking
/// 0.05 would triple the headroom and **stop catching the only
/// loosening anyone has demonstrated**, which is the trade this
/// constant is: sensitivity to a 10× loose certificate, bought with
/// 1.67× of margin against the coarsest mesh the sizing will build.
///
/// An earlier version of this doc claimed 4.5× from a δ ∈ [3e-4, 2e-2]
/// band; that figure was an artefact of the band, and the reviewer who
/// widened it was right.
///
/// # What this row is NOT about
///
/// Only the NURBS lane resamples (`trimmed`'s `dev_samples_per_edge`
/// is `None` for `Lane::Cylinder`), so `cert::cert_cylinder` — which
/// certifies every cylinder triangle in both tessellation lanes — is
/// falsified by nothing here, and by nothing in any build. The
/// assertion messages say NURBS for that reason. The gap is real and
/// is not this row's to close: giving the meter a cylinder row means a
/// `FaceMeasure` whose NURBS-only columns have no meaning, which is a
/// change to the consumer contract in `tools/`. **Known and unfixed** —
/// do not read this row's silence about cylinders as coverage.
#[test]
fn the_deviation_pass_samples_and_stays_under_its_certificates() {
    let body = loft_prism();
    budget::arm(Mode::Deviation {
        samples_per_edge: 6,
    });
    assert_eq!(budget::deviation_samples(), Some(6));
    mesh::tessellate(&body, 6e-3, Tol::witness()).expect("tessellates");
    let measures = budget::take();
    assert!(!measures.is_empty());
    // The run's own ε, read the way the kernel reads it rather than
    // transcribed: the floor's applicability test is a comparison
    // against it, and CI drives this suite at one leg while the ε
    // battery drives the crate at three.
    let eps = geom_core::Tol::witness().get().eps;
    for m in &measures {
        assert!(m.dev_samples > 0, "resampling ran: {m:?}");
        // The falsification, in the one form that carries it:
        // `worst_ratio` is the largest `d / (cert + eps)` over every
        // sample on every triangle, so `<= 1` says each sample was
        // dominated by ITS OWN triangle's certificate.
        //
        // `worst_dev <= worst_dev_cert + eps` is deliberately NOT
        // asserted beside it: `worst_dev_cert` is the certificate of
        // the triangle `worst_dev` came from, so that inequality is
        // one term of the maximum above and cannot fail unless this
        // one does.
        assert!(
            m.worst_ratio <= 1.0,
            "a NURBS triangle's samples exceeded its own certificate: {m:?}"
        );
        // The floor, on the faces whose ratio the certificate decides.
        if m.worst_cert > eps {
            assert!(
                m.worst_ratio >= RATIO_FLOOR,
                "a NURBS face's certificate is more than {:.0}x the deviation it \
                 bounds (worst d/(cert+eps) = {}, floor {RATIO_FLOOR}, measured \
                 minimum over seven orders of delta 0.1667) — the ceiling above \
                 cannot see this direction, which is the one #320 is about: {m:?}",
                1.0 / RATIO_FLOOR,
                m.worst_ratio
            );
        }
    }
}

/// The measurement does not perturb what it measures: the mesh is
/// byte-identical armed and disarmed (D9 determinism, which the meter
/// must not be an exception to).
#[test]
fn arming_the_meter_does_not_change_the_mesh() {
    let body = loft_prism();
    let plain = mesh::tessellate(&body, 6e-3, Tol::witness()).expect("tessellates");
    budget::arm(Mode::Deviation {
        samples_per_edge: 6,
    });
    let metered = mesh::tessellate(&body, 6e-3, Tol::witness()).expect("tessellates");
    let measures = budget::take();
    // Non-emptiness FIRST: `all()` on an empty slice is `true`, so a
    // meter that recorded nothing would sail through the next line.
    assert!(!measures.is_empty(), "the loft's NURBS walls were measured");
    assert!(measures.iter().all(|m| m.dev_samples > 0), "resampling ran");
    assert_eq!(plain.positions.len(), metered.positions.len());
    for (a, b) in plain.positions.iter().zip(&metered.positions) {
        assert_eq!(
            (a.x.to_bits(), a.y.to_bits(), a.z.to_bits()),
            (b.x.to_bits(), b.y.to_bits(), b.z.to_bits()),
            "a metered tessellation must be bit-identical to an unmetered one"
        );
    }
    // `zip` stops at the shorter side, so a metered run that dropped a
    // patch would compare only the survivors and pass.
    assert_eq!(plain.patches.len(), metered.patches.len());
    for (a, b) in plain.patches.iter().zip(&metered.patches) {
        assert_eq!(a.face, b.face);
        assert_eq!(a.triangles, b.triangles);
    }
}
