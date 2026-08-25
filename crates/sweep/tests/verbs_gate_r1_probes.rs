//! Reviewer probes for PR #1001 (VERBS-GATE, head b2a8bad1) — an
//! independent consumer suite. Every row here is RED-able: each one
//! asserts an outcome the pair-scoped gate (or the boxes under it)
//! must produce, on bodies this suite authors itself.
//!
//! Rows 1–2 are the E2E the spec's acceptance only ran at
//! `boolean_reduce` depth: a body whose TORUS face is irrelevant to
//! the cut must union THROUGH THE FULL PIPELINE, and the result's
//! mass must be right; posed so the torus box genuinely overlaps, the
//! same body must refuse naming the pair.
//!
//! Row 3 documents what the full pipeline does when a torus-carrying
//! operand is admitted (correctly, per the ruling) but the operation
//! falls through to the containment fallback, whose `point_in_solid`
//! walks EVERY face of the other body regardless of box overlap.
//!
//! Rows 4–5 are behavioral box-soundness probes at TILTED axes — the
//! pose the in-tree cylinder/cone locus and ceiling rows never take
//! (they are all axis-aligned; only the torus rows tilt). The oracle
//! is the gate itself: a probe brick parked ON the described locus
//! must be refused (its box must reach the locus point); admitting it
//! would mean the box under-encloses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn vol(body: &Body<f64>) -> f64 {
    topo::mass_properties(body, Tol::witness()).unwrap().volume
}

/// A brick `[x0,x1] × [y0,y1] × [z0,z1]` via extrude.
fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(x.0, y.0), p2(x.1, y.0), p2(x.1, y.1), p2(x.0, y.1)]);
    let plane = SketchPlane::new(geom_core::Affine3::translation(Vec3::new(0.0, 0.0, z.0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(z.1 - z.0), Tol::witness())
        .unwrap()
        .body
}

/// A "vase": a solid of revolution about z with a straight cylinder
/// wall of radius 0.5 over z ∈ [0, 1] and a half-circle BULGE arc
/// (radius 0.25, centred at (0.5, 1.25) in the profile) over
/// z ∈ [1, 1.5] — the bulge revolves to a genuine TORUS band
/// (R = 0.5, r = 0.25, axis z), the kernel's own authorship, not a
/// relabel. Closed by discs at z = 0 and z = 1.5.
fn vase() -> Body<f64> {
    use sweep::{Revolution, RevolveAxis, revolve};
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(0.5, 0.0), 0.0),
        ProfileVertex::new(p2(0.5, 1.0), 1.0), // half-circle bulge to (0.5, 1.5)
        ProfileVertex::new(p2(0.5, 1.5), 0.0),
        ProfileVertex::new(p2(0.0, 1.5), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// The vase carries a torus face, minted by the kernel itself.
#[test]
fn the_vase_fixture_actually_carries_a_torus_face() {
    let v = vase();
    assert!(
        v.faces()
            .any(|(_, f)| matches!(v.get_surface(f.surface), Some(geom::Surface::Torus { .. }))),
        "the fixture must carry a real torus band"
    );
}

/// **E2E row 1 — the granted case, full pipeline, mass-checked.**
/// The vase's torus band lives in z ∈ [1, 1.5] (whole-torus box); the
/// brick crosses the CYLINDER wall over z ∈ [0.3, 0.7] and clears the
/// torus box, so the pair-scoped gate must admit the union, the
/// crossing pipeline must cut it, and the volume must be the analytic
/// one. Reverting the gate to a per-body kind scan reds this row with
/// a refusal; a wrong cut reds it on the number.
#[test]
fn a_union_whose_torus_face_is_out_of_reach_completes_with_the_exact_volume() {
    let a = vase();
    let b = brick((-1.0, 1.0), (-1.0, 1.0), (0.3, 0.7));
    let out = topo::union(&a, &b, Tol::witness())
        .expect("the torus band clears the brick; the pair gate must admit this union");
    let body = &out.body().expect("a non-empty union").body;
    // vase = cylinder (π r² · 1) + top segment: revolved half-disc
    // (Pappus: 2π · x̄ · A, x̄ = 0.5 + 4·0.25/(3π), A = π·0.25²/2)
    // + inner cylinder over the bulge's z-range (π r² · 0.5).
    let bulge = 2.0 * PI * (0.5 + 4.0 * 0.25 / (3.0 * PI)) * (PI * 0.25 * 0.25 / 2.0);
    let vase_vol = PI * 0.25 * 1.0 + PI * 0.25 * 0.5 + bulge;
    let brick_vol = 2.0 * 2.0 * 0.4;
    let overlap = PI * 0.25 * 0.4; // the cylinder's core inside the brick
    let want = vase_vol + brick_vol - overlap;
    let got = vol(body);
    assert!(
        (got - want).abs() < 1e-6,
        "union volume {got} != analytic {want} (vase {vase_vol}, brick {brick_vol}, \
         overlap {overlap})"
    );
}

/// **E2E row 2 — the refused pose.** The same brick raised into the
/// torus band's box (z ∈ [1.05, 1.45] overlaps the whole-torus box
/// z ∈ [1.0, 1.5]) must refuse naming the (Torus, _) pair — even
/// though the brick still genuinely intersects the solid only through
/// supported faces, the box MAY meet and the gate has no arm for the
/// pair.
#[test]
fn the_same_union_posed_into_the_torus_box_refuses_naming_the_pair() {
    let a = vase();
    let b = brick((-1.0, 1.0), (-1.0, 1.0), (1.05, 1.45));
    let err = topo::union(&a, &b, Tol::witness())
        .expect_err("a torus face whose box may meet the brick must gate the union");
    let BooleanError::CurvedPairUnsupported {
        op: None,
        kind: geom_brep::SurfaceKind::Torus,
        ..
    } = err
    else {
        panic!("expected the pair-scoped torus refusal, got {err:?}");
    };
}

/// **Row 3 — what the ruling's own blind spot answers today.** A
/// torus face whose box clears a DISJOINT other operand is admitted
/// (correct per the spec's ruling: it can enter no crossing). But
/// with no crossings at all the pipeline falls through to the
/// containment fallback, and `point_in_solid` walks EVERY face of the
/// classified-against body — box overlap never enters it — so the
/// admitted union dies in `face_geo`'s wildcard with an error that
/// claims the body is CORRUPT ("not planar (F5) or not walkable"),
/// which it is not.
///
/// This row pins the CURRENT behaviour so the frontier is visible; if
/// a torus containment arm ever lands, the union should instead
/// answer a two-solid assembly of volume vase + brick.
#[test]
fn a_disjoint_union_with_a_torus_face_is_admitted_then_mislabelled_corrupt() {
    let a = vase();
    let b = brick((5.0, 6.0), (0.0, 1.0), (0.0, 1.0));
    match topo::union(&a, &b, Tol::witness()) {
        Err(
            BooleanError::CurvedPairUnsupported { .. }
            | BooleanError::CurvedBooleanUnsupported { .. },
        ) => {
            panic!(
                "the torus band clears a body five units away — the pair-scoped \
                 gate must not refuse this"
            );
        }
        Err(BooleanError::Containment(e)) => {
            let msg = e.to_string();
            assert!(
                msg.contains("planar") || msg.contains("walkable") || msg.contains("corrupt"),
                "expected the fallback's mislabelled corruption refusal, got: {msg}"
            );
        }
        Err(other) => panic!("unexpected refusal shape for the fallback path: {other:?}"),
        Ok(out) => {
            // If this ever succeeds, it must be the honest assembly.
            let body = &out.body().expect("a non-empty union").body;
            let got = vol(body);
            let bulge = 2.0 * PI * (0.5 + 4.0 * 0.25 / (3.0 * PI)) * (PI * 0.25 * 0.25 / 2.0);
            let want = PI * 0.25 * 1.0 + PI * 0.25 * 0.5 + bulge + 1.0;
            assert!(
                (got - want).abs() < 1e-6,
                "a disjoint union that completes must be the assembly: {got} vs {want}"
            );
        }
    }
}

/// A brick whose `x = x1` face is relabelled to `surface` — the
/// operand-gate fixture: the gate reads the DESCRIPTION plus the
/// face's boundary, so a relabel is exactly what its box arithmetic
/// sees.
fn brick_with_face(surface: geom::Surface<f64>) -> Body<f64> {
    let mut b = brick((2.0, 3.0), (0.0, 1.0), (0.0, 1.0));
    let face = b
        .faces()
        .find(|(_, f)| match b.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => {
                (origin.x - 3.0).abs() < 1e-9 && normal.x.abs() > 0.5
            }
            _ => false,
        })
        .map(|(k, _)| k)
        .expect("the brick has an x = 3 face");
    b.set_face_surface(face, topo::FaceSurface::New(surface))
        .unwrap();
    b
}

/// A small probe brick centred at `p`.
fn probe_at(p: Point3<f64>) -> Body<f64> {
    let s = 0.02;
    brick((p.x - s, p.x + s), (p.y - s, p.y + s), (p.z - s, p.z + s))
}

/// **Row 4 — cone-slab soundness at a TILTED axis**, the pose no
/// in-tree cylinder/cone box row takes. The cone face's box must
/// contain every locus point whose axial coordinate lies inside the
/// boundary's own axial range; a probe brick parked on such a point
/// must therefore be REFUSED (box overlap), never admitted. The
/// azimuth sweep stresses the perpendicular room `r·√(1 − axisᵢ²)` at
/// axis components strictly between 0 and 1, where a wrong formula
/// (e.g. `1 − |aᵢ|`, or a swapped bound) parts company with the right
/// one.
#[test]
fn a_probe_on_a_tilted_cones_locus_is_always_refused() {
    let axis = Vec3::new(0.6, 0.0, 0.8); // unit
    let apex = Point3::new(2.5, 0.5, 2.0);
    let half_angle = 0.4_f64;
    let a = brick_with_face(geom::Surface::Cone {
        apex,
        axis,
        half_angle,
        u_ref: Vec3::new(0.8, 0.0, -0.6),
    });
    // The relabelled face's boundary is the x = 3 square,
    // y, z ∈ [0, 1]: axial range of (corner − apex)·axis over its
    // corners. Take a v safely INSIDE it.
    let corners = [
        Point3::new(3.0, 0.0, 0.0),
        Point3::new(3.0, 1.0, 0.0),
        Point3::new(3.0, 0.0, 1.0),
        Point3::new(3.0, 1.0, 1.0),
    ];
    let hs: Vec<f64> = corners.iter().map(|c| (*c - apex).dot(axis)).collect();
    let (h_lo, h_hi) = (
        hs.iter().cloned().fold(f64::INFINITY, f64::min),
        hs.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
    );
    let h_mid = 0.5 * (h_lo + h_hi);
    let v = h_mid / half_angle.cos(); // axial coord v·cos α = h_mid ∈ [h_lo, h_hi]
    let u_ref = Vec3::new(0.8, 0.0, -0.6);
    let v_ref = axis.cross(u_ref);
    for k in 0..8 {
        let t = 2.0 * PI * f64::from(k) / 8.0;
        let u_hat = u_ref * t.cos() + v_ref * t.sin();
        let p = apex + (axis * half_angle.cos() + u_hat * half_angle.sin()) * v;
        let err = topo::boolean_reduce(
            topo::boolean::BooleanOp::Union,
            &probe_at(p),
            &a,
            Tol::witness(),
        )
        .expect_err(&format!(
            "a probe ON the cone locus (azimuth {t:.2}) must overlap the cone's \
             box — admitting it means the slab under-encloses at a tilted axis"
        ));
        assert!(
            matches!(
                err,
                BooleanError::CurvedPairUnsupported {
                    kind: geom_brep::SurfaceKind::Cone,
                    ..
                }
            ),
            "azimuth {t:.2}: expected the cone pair refusal, got {err:?}"
        );
    }
}

/// **Row 5 — the torus twin.** Every point of the tube of a TILTED
/// torus must be inside its box; the extreme in-plane points
/// `center + (R + r)·û` are the ones a wrong perpendicular bound
/// (`(R + r)·√(1 − aᵢ²) + r·|aᵢ|` mis-derived) drops first.
#[test]
fn a_probe_on_a_tilted_toruss_locus_is_always_refused() {
    let axis = Vec3::new(1.0, 2.0, 2.0).normalize();
    let center = Point3::new(2.5, 0.5, 3.0);
    let (major, minor) = (0.8, 0.2);
    let a = brick_with_face(geom::Surface::Torus {
        center,
        axis,
        major_radius: major,
        minor_radius: minor,
        u_ref: axis.orthonormal_basis().0,
    });
    let u_ref = axis.orthonormal_basis().0;
    let v_ref = axis.cross(u_ref);
    for k in 0..8 {
        let t = 2.0 * PI * f64::from(k) / 8.0;
        let u_hat = u_ref * t.cos() + v_ref * t.sin();
        for (radial, axial) in [(major + minor, 0.0), (major, minor), (major - minor, 0.0)] {
            let p = center + u_hat * radial + axis * axial;
            let err = topo::boolean_reduce(
                topo::boolean::BooleanOp::Union,
                &probe_at(p),
                &a,
                Tol::witness(),
            )
            .expect_err(&format!(
                "a probe ON the torus tube (azimuth {t:.2}, radial {radial}) must \
                 overlap the torus box"
            ));
            assert!(
                matches!(
                    err,
                    BooleanError::CurvedPairUnsupported {
                        kind: geom_brep::SurfaceKind::Torus,
                        ..
                    }
                ),
                "azimuth {t:.2}: expected the torus pair refusal, got {err:?}"
            );
        }
    }
}

/// **Row 6 — the admit side at a tilted axis**: a probe parked well
/// OUTSIDE the tilted cone's ideal slab (beyond the perpendicular
/// room plus pad on the coordinate where the slab is thinnest) must
/// be admitted — the conservative direction has a ceiling too, and
/// this red-lines a box that quietly grew (e.g. the old axial `+ r`).
#[test]
fn a_probe_well_clear_of_the_tilted_cone_is_admitted() {
    let axis = Vec3::new(0.6, 0.0, 0.8);
    let apex = Point3::new(2.5, 0.5, 2.0);
    let half_angle = 0.4_f64;
    let a = brick_with_face(geom::Surface::Cone {
        apex,
        axis,
        half_angle,
        u_ref: Vec3::new(0.8, 0.0, -0.6),
    });
    // The slab's y extent: apex.y + h·axis.y ± r·√(1 − 0²) with
    // axis.y = 0 — i.e. y ∈ 0.5 ± r_max. r_max = max generator
    // length · sin α ≤ reach(apex → boundary corners) · sin α.
    let corners = [
        Point3::new(3.0, 0.0, 0.0),
        Point3::new(3.0, 1.0, 0.0),
        Point3::new(3.0, 0.0, 1.0),
        Point3::new(3.0, 1.0, 1.0),
    ];
    let r_max = corners
        .iter()
        .map(|c| (*c - apex).norm())
        .fold(0.0_f64, f64::max)
        * half_angle.sin();
    // Park the probe 0.5 beyond the widest possible slab in y, at an
    // x inside the slab's x-range — so y alone must separate.
    let p = Point3::new(2.5, 0.5 + r_max + 0.5, 2.0);
    let red = topo::boolean_reduce(
        topo::boolean::BooleanOp::Union,
        &probe_at(p),
        &a,
        Tol::witness(),
    );
    assert!(
        red.is_ok(),
        "a probe {:.2} beyond the cone slab's own widest y reach must be admitted \
         — refusing it means the box grew past its rule: {:?}",
        0.5,
        red.err()
    );
}
