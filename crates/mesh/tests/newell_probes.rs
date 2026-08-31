//! R1 adversarial review probes for the #284 boundary-anchored Newell
//! chart frame (PR #301). Each probe constructs a planar boundary
//! adversarial to the stated convention (anchor = first walk point,
//! Newell normal, u toward farthest point) and demands the documented
//! contract: tessellate, or refuse TYPED (`TessellateError`) — never a
//! panic, never a silently wrong mesh.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Point2, Point3, Vec3};
use mesh::tessellate;
use mesh::validate::{check_mesh, signed_volume, triangle_count};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::Body;

fn lp(poly: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::polygon(
        poly.iter()
            .map(|&(x, y)| Point2::new(x, y))
            .collect::<Vec<_>>(),
    )
}

fn validated(plane: SketchPlane<f64>, loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(plane, loops)
        .validate(Tol::witness())
        .expect("profile validation")
}

fn prism_on(plane: SketchPlane<f64>, poly: &[(f64, f64)], h: f64) -> Body<f64> {
    extrude(
        &validated(plane, vec![lp(poly)]),
        Extrusion::Distance(h),
        Tol::witness(),
    )
    .expect("extrude")
    .body
}

fn prism(poly: &[(f64, f64)], h: f64) -> Body<f64> {
    prism_on(SketchPlane::xy(), poly, h)
}

fn dump(m: &mesh::Mesh) -> String {
    let mut s = String::new();
    for p in &m.positions {
        s.push_str(&format!(
            "{:016x}{:016x}{:016x}",
            p.x.to_bits(),
            p.y.to_bits(),
            p.z.to_bits()
        ));
    }
    for patch in &m.patches {
        for t in &patch.triangles {
            s.push_str(&format!("{};{};{}|", t[0], t[1], t[2]));
        }
    }
    s
}

/// Tessellate-or-typed: the only two acceptable outcomes. Returns the
/// mesh when it tessellates so callers can pile on oracles.
fn tessellate_or_typed(body: &Body<f64>, delta: f64, label: &str) -> Option<mesh::Mesh> {
    match tessellate(body, delta, Tol::witness()) {
        Ok(m) => {
            assert_eq!(check_mesh(&m), Ok(()), "{label}: mesh not watertight");
            assert!(signed_volume(&m) > 0.0, "{label}: non-positive volume");
            Some(m)
        }
        Err(e) => {
            // Typed refusal: acceptable by contract; record which.
            eprintln!("{label}: typed refusal {e:?}");
            None
        }
    }
}

/// (a) First input point nearly coincident with the antipode of the
/// farthest point: a near-regular polygon walked from one end, with
/// the far vertex almost diametrically opposite the anchor. u's
/// pre-normalization length is ~ the diameter — must be fine.
#[test]
fn probe_a_anchor_antipodal_to_far() {
    let n = 17usize;
    let poly: Vec<(f64, f64)> = (0..n)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let t = std::f64::consts::PI * (2.0 * i as f64 / n as f64 + 1.0);
            (t.cos(), t.sin())
        })
        .collect();
    let body = prism(&poly, 1.0);
    let m = tessellate_or_typed(&body, 1e-2, "antipodal").expect("must tessellate");
    // Area of the regular 17-gon: n/2 sin(2pi/n); volume = area * 1.
    #[allow(clippy::cast_precision_loss)]
    let area = n as f64 / 2.0 * (2.0 * std::f64::consts::PI / n as f64).sin();
    assert!((signed_volume(&m) - area).abs() < 1e-9);
}

/// (b) Farthest-point near-tie: anchor at the center of one edge of a
/// rectangle so the two opposite corners are EXACTLY equidistant
/// (strict > keeps the first in walk order) — plus a 1-ulp-perturbed
/// variant. Both must be deterministic run-to-run (byte-identical).
#[test]
fn probe_b_far_tie_determinism() {
    // First point (0,0); corners (1,2) and (-1,2) both at d^2 = 5.
    let tie = [(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (-1.0, 2.0), (-1.0, 0.0)];
    // 1-ulp nudge on the SECOND candidate: the pick must flip to
    // whichever is strictly larger, deterministically.
    let nudged = [
        (0.0, 0.0),
        (1.0, 0.0),
        (1.0, 2.0),
        (-1.0, f64::from_bits(2.0f64.to_bits() + 1)),
        (-1.0, 0.0),
    ];
    for (label, poly) in [("tie", &tie[..]), ("nudged", &nudged[..])] {
        let body = prism(poly, 1.0);
        let m1 = tessellate_or_typed(&body, 1e-2, label).expect("must tessellate");
        let m2 = tessellate(&body, 1e-2, Tol::witness()).unwrap();
        assert_eq!(dump(&m1), dump(&m2), "{label}: rebuild not byte-identical");
        assert!((signed_volume(&m1) - 4.0).abs() < 1e-6, "{label}");
    }
}

/// (c) Needle slivers: extent ratios 1e3..1e9 (long axis huge, short
/// axis 1). Profile validation refuses the extreme ratios upstream
/// (typed `DegenerateSegment` — the public path bounds the ratio);
/// whatever validation admits must tessellate watertight with the
/// exact prism volume or refuse typed.
#[test]
fn probe_c_needle_extent_ratio() {
    for (label, len) in [
        ("long-1e3", 1.0e3),
        ("long-1e6", 1.0e6),
        ("long-1e9", 1.0e9),
    ] {
        let poly = [(0.0, 0.0), (len, 0.0), (len, 1.0), (0.0, 1.0)];
        let profile = Profile::new(SketchPlane::xy(), vec![lp(&poly)]);
        let Ok(vp) = profile.validate(Tol::witness()) else {
            eprintln!("{label}: profile validation refuses (typed, upstream)");
            continue;
        };
        let body = extrude(&vp, Extrusion::Distance(1.0), Tol::witness())
            .expect("extrude")
            .body;
        if let Some(m) = tessellate_or_typed(&body, 1e-2, label) {
            let v = signed_volume(&m);
            assert!(((v - len) / len).abs() < 1e-9, "{label}: volume {v}");
        }
    }
}

/// (d) Huge ambient coordinates (1e8 m offset on all axes), tiny face
/// (1e-3): chart coordinates are differences of input floats, so the
/// offset must cancel. Volume must survive at the relative precision
/// the coordinates themselves allow.
#[test]
fn probe_d_huge_offset_tiny_face() {
    let plane = SketchPlane::from_frame(
        Point3::new(1.0e8, 1.0e8, 1.0e8),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let poly = [(0.0, 0.0), (1.0e-3, 0.0), (1.0e-3, 1.0e-3), (0.0, 1.0e-3)];
    let body = prism_on(plane, &poly, 1.0e-3);
    let m = tessellate_or_typed(&body, 1e-6, "huge-offset").expect("must tessellate");
    // `signed_volume` recentres on the mesh's bbox centre, so its
    // fold operands scale with the body, not the placement — it is
    // the honest oracle here directly.
    let v = signed_volume(&m);
    // Positions are ~1e8 with ulp ~1.5e-8 against 1e-3 edges: ~1.5e-5
    // relative slop per coordinate — accept 1e-4.
    assert!(
        ((v - 1.0e-9) / 1.0e-9).abs() < 1e-4,
        "huge-offset: volume {v}"
    );
}

/// (e) The original failure class, synthesized fresh: a skewed sketch
/// frame injects noise-scale out-of-plane components (z = x·nu) into
/// the boundary positions of an axis-aligned-ish rectangle, at noise
/// scales riding from 1e-30 down to 1e-60 (below, at, and above
/// spade's MIN_ALLOWED_VALUE = 2^-142 ~ 1.79e-43). Every scale must
/// tessellate or refuse typed; a panic is a finding.
#[test]
fn probe_e_noise_scale_sweep() {
    for exp in [-15i32, -20, -22, -30, -40, -43, -45, -50, -60] {
        let nu = 10.0f64.powi(exp);
        let plane = SketchPlane::from_frame(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, nu),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let poly = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)];
        let body = prism_on(plane, &poly, 1.0);
        let label = format!("noise-1e{exp}");
        if let Some(m) = tessellate_or_typed(&body, 1e-2, &label) {
            let v = signed_volume(&m);
            assert!((v - 2.0).abs() < 1e-9, "{label}: volume {v}");
        }
    }
}

/// (e-pin) The position-noise refusal class, PINNED (R1 MAJOR-1 of
/// PR #301; probe (e) above records the contract, this row records
/// the POSTURE). Mechanism, stated as the invariant: with off-plane
/// noise ν on the boundary POSITIONS themselves (z = x·ν — no
/// exactly-equal input column), the far point's own v-coordinate is
/// an engineered exact-zero whose float residue is ~ν²; for
/// ν ≲ √(2⁻¹⁴²) ≈ 4e-22 that residue is nonzero below spade's
/// coordinate floor and the face refuses TYPED
/// (`TessellateError::Triangulation`), while scales whose residue
/// clears the floor tessellate exactly. Deliberate, recorded posture
/// — not accidental: `mesh::planar`'s module docs scope the
/// input-exactness guarantee to the noisy-AXES class #284 actually
/// closed (wild-corpus noise is axis noise), and this synthetic
/// residual class is banked as a follow-up candidate (same "valid
/// body refuses tessellation" shape as #284). If this row starts
/// TESSELLATING, the class was closed: retire the pin and the
/// follow-up together.
#[test]
fn position_noise_subfloor_refusal_is_pinned_typed() {
    let body_at = |nu: f64| {
        let plane = SketchPlane::from_frame(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, nu),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let poly = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)];
        prism_on(plane, &poly, 1.0)
    };
    // Well inside the refusal band (~ν² sub-floor): typed, exactly.
    for exp in [-30i32, -45, -60] {
        match tessellate(&body_at(10.0f64.powi(exp)), 1e-2, Tol::witness()) {
            Err(mesh::TessellateError::Triangulation { .. }) => {}
            other => panic!(
                "noise-1e{exp}: pinned typed Triangulation refusal drifted: \
                 {:?}",
                other.map(|m| triangle_count(&m))
            ),
        }
    }
    // Well clear of the floor (~ν² ≥ 2⁻¹⁴²): tessellates, exact volume.
    for exp in [-15i32, -20] {
        let label = format!("noise-1e{exp}");
        let m = tessellate_or_typed(&body_at(10.0f64.powi(exp)), 1e-2, &label)
            .expect("must tessellate above the floor");
        let v = signed_volume(&m);
        assert!((v - 2.0).abs() < 1e-9, "{label}: volume {v}");
    }
}

/// (e') The cancellation corner the module docs' argument does not
/// literally cover: a boundary point ON the anchor->far diagonal (its
/// exact v-coordinate is 0) while the noise column rides at 1e-50.
/// If the real-sized products cancel exactly, the surviving term is
/// ~1e-100 — sub-floor nonzero. Contract: tessellate or typed.
#[test]
fn probe_e2_collinear_midpoint_on_diagonal() {
    for exp in [-33i32, -45, -50, -60] {
        let nu = 10.0f64.powi(exp);
        let plane = SketchPlane::from_frame(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, nu),
            Vec3::new(0.0, 1.0, 0.0),
        );
        // (1, 0.5) sits exactly on the segment (0,0)->(2,1) = far.
        let poly = [
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 0.5),
            (0.0, 1.0),
            (0.0, 0.5),
        ];
        let body = prism_on(plane, &poly, 1.0);
        let label = format!("collinear-1e{exp}");
        if let Some(m) = tessellate_or_typed(&body, 1e-2, &label) {
            // Area: shoelace of the hexagon = 1.5.
            let v = signed_volume(&m);
            assert!((v - 1.5).abs() < 1e-9, "{label}: volume {v}");
        }
    }
}

/// Seam-slit prerequisite (claim 2): repeated 3-D points must project
/// bitwise-identically — the full-2pi revolve annulus wall is a slit
/// polygon traversing its seam twice. Byte-identical rebuild plus
/// watertightness of the washer exercises the cancellation on the
/// PLANAR slit faces (top/bottom annuli).
#[test]
fn probe_slit_washer_rebuild() {
    use geom_core::Vec2;
    use sweep::{Revolution, RevolveAxis, revolve};
    let profile = validated(
        SketchPlane::xy(),
        vec![lp(&[(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)])],
    );
    let body = revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("revolve")
    .body;
    let m1 = tessellate(&body, 1e-2, Tol::witness()).unwrap();
    let m2 = tessellate(&body, 1e-2, Tol::witness()).unwrap();
    assert_eq!(dump(&m1), dump(&m2), "washer rebuild not byte-identical");
    assert_eq!(check_mesh(&m1), Ok(()), "washer not watertight");
    assert!(triangle_count(&m1) > 0);
}
