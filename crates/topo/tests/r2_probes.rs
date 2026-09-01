//! **R2 review probes for PR #1443 (MATE-5)** — adversarial rows.
//!
//! These are REVIEW probes, not acceptance rows: a probe that PASSES
//! may be *demonstrating a defect* (its doc says which). They live on
//! the probe branch only.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{
    Body, ChartOverlap, ChartRegionError, ContactVerdict, FaceKey, FaceSurface, MefSite, MevSite,
    declared_pair_overlap,
};

fn band() -> Band {
    let tol = Tol::witness();
    Band::new(tol.eps(), tol.k() * tol.eps()).unwrap()
}

#[derive(Clone, Copy)]
struct CylFrame {
    origin: Point3<f64>,
    axis: Vec3<f64>,
    radius: f64,
    u_ref: Vec3<f64>,
}

impl CylFrame {
    fn surface(&self) -> Surface<f64> {
        Surface::Cylinder {
            origin: self.origin,
            axis: self.axis,
            radius: self.radius,
            u_ref: self.u_ref,
        }
    }

    fn at(&self, u: f64, v: f64) -> Point3<f64> {
        let w = self.axis.cross(self.u_ref);
        self.origin + (self.u_ref * u.cos() + w * u.sin()) * self.radius + self.axis * v
    }
}

/// The mate5 suite's `wall_sheet`, verbatim (the fixture builder the
/// unit itself uses — reused so the probes exercise the same door).
fn wall_sheet(
    body: &mut Body<f64>,
    frame: CylFrame,
    src_id: u64,
    u0: f64,
    u1: f64,
    v0: f64,
    v1: f64,
) -> FaceKey {
    let (p00, p10, p11, p01) = (
        frame.at(u0, v0),
        frame.at(u1, v0),
        frame.at(u1, v1),
        frame.at(u0, v1),
    );
    let seed = body.mvfs(p00).unwrap();
    let cyl = body
        .set_face_surface(seed.face, FaceSurface::New(frame.surface()))
        .unwrap();
    body.set_surface_source(cyl, topo::GeomSource::minted(src_id, 0))
        .unwrap();
    let rim = |body: &mut Body<f64>, v: f64, ccw: bool| {
        let center = frame.origin + frame.axis * v;
        let scaffold = body.mvfs(center).unwrap();
        let plane = body
            .set_face_surface(
                scaffold.face,
                FaceSurface::New(Surface::Plane {
                    origin: center,
                    normal: frame.axis,
                    u_ref: frame.u_ref,
                }),
            )
            .unwrap();
        let (carrier, t0, t1) = if ccw {
            (
                Curve3::Circle {
                    center,
                    axis: frame.axis,
                    radius: frame.radius,
                    u_ref: frame.u_ref,
                },
                u0,
                u1,
            )
        } else {
            // The radial direction at u1, from the frame fields
            // directly — the projection-based read cancels
            // catastrophically for tilted/small frames and mints a
            // non-structural chart image (the fix-pass port of the
            // unit builder's own repair).
            let w = frame.axis.cross(frame.u_ref);
            let s = frame.u_ref * u1.cos() + w * u1.sin();
            (
                Curve3::Circle {
                    center,
                    axis: -frame.axis,
                    radius: frame.radius,
                    u_ref: s,
                },
                0.0,
                u1 - u0,
            )
        };
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1: cyl,
                s2: plane,
                witness: frame.at((u0 + u1) * 0.5, v),
            },
            carrier,
            param_start: t0,
            param_end: t1,
        }
    };
    let bottom = rim(body, v0, true);
    let e_b = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p10,
            bottom,
            Tol::witness(),
        )
        .unwrap();
    let e_r = body
        .mev_line(
            MevSite::Fan {
                he1: e_b.he_minus,
                he2: e_b.he_minus,
            },
            p11,
            Tol::witness(),
        )
        .unwrap();
    let top = rim(body, v1, false);
    let e_t = body
        .mev(
            MevSite::Fan {
                he1: e_r.he_minus,
                he2: e_r.he_minus,
            },
            p01,
            top,
            Tol::witness(),
        )
        .unwrap();
    let he = body
        .find_half_edge(seed.face, e_t.vertex, e_r.vertex)
        .unwrap();
    let face = body
        .mef(
            MefSite::Chords {
                he1: he,
                he2: e_b.he_plus,
            },
            EdgeCurveSpec::line_between(p01, p00),
            FaceSurface::Shared(cyl),
            Tol::witness(),
        )
        .unwrap()
        .face;
    topo::pcurves::mint_pcurves(body, Tol::witness()).unwrap();
    face
}

fn frame_a(radius: f64) -> CylFrame {
    CylFrame {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius,
        u_ref: Vec3::unit_x(),
    }
}

/// A frame of the SAME radius through the SAME origin whose axis is
/// tilted by `theta` about +y (u_ref co-rotated so it stays a unit
/// vector perpendicular to the axis, delta = 0 against `frame_a`).
fn tilted_frame(radius: f64, theta: f64) -> CylFrame {
    CylFrame {
        origin: Point3::origin(),
        axis: Vec3::new(theta.sin(), 0.0, theta.cos()),
        radius,
        u_ref: Vec3::new(theta.cos(), 0.0, -theta.sin()),
    }
}

/// The builder, fallible at the MINT: a tilted frame's chart images
/// mint as exact structure only by bit-lottery above ~10·ε of tilt
/// (`r2_diag_mintable_tilts` maps it; the constants of the rows using
/// this door won the lottery at the default ε row, where their
/// demonstrations were measured). A row whose fixture cannot be
/// MINTED at this ε states that and stands down (the
/// `m5_pr7_split_meter` typed-fixture-refusal precedent) — the arm
/// never saw the pair, so neither outcome would be evidence about it.
fn try_wall_sheet(
    body: &mut Body<f64>,
    frame: CylFrame,
    src_id: u64,
    u0: f64,
    u1: f64,
    v0: f64,
    v1: f64,
) -> Option<FaceKey> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        wall_sheet(body, frame, src_id, u0, u1, v0, v1)
    }))
    .ok()
}

fn verdict_class(r: Result<ChartOverlap, ChartRegionError>) -> String {
    match r {
        Ok(ChartOverlap::PositiveArea) => "PositiveArea".into(),
        Ok(ChartOverlap::Empty) => "Empty".into(),
        Err(ChartRegionError::Escalated(d)) => {
            format!("Escalated({})", d.predicate.unwrap_or("?"))
        }
        Err(other) => format!("{other:?}")
            .split([' ', '{', '('])
            .next()
            .unwrap()
            .to_string(),
    }
}

/// PROBE (defect demonstration — claim 2 / deviation 5): a pair of
/// cylinder descriptions tilted by θ = 2.5e-5 rad about a shared
/// origin, radius 1, with hairline trims (axial extent ~2.6e-5 m):
/// every carrier gate passes (`tilt·extent ≈ 6.6e-10 < ε = 1e-9`),
/// the affine transfer maps B's chart window onto A's overlapping it,
/// and the arm certifies `PositiveArea` — while the two trims are
/// DEFINITELY DISJOINT in 3D by more than 5e-6 m (5000+ ε): every
/// point of B's trim sits below z = −5.2e-6 (exact bound computed
/// from the frame parameters below and cross-checked by sampling)
/// and every point of A's trim sits at z ≥ 0 on the unit cylinder.
///
/// The PRE-FIX tilt gate's lever (the pair's AXIAL reach) bounded the
/// RADIAL residual but not the transfer's v-error, which is
/// first-order r·θ·cos(φ) — levered by the RADIUS; on that head this
/// row demonstrated the false certification. The redesigned gate's
/// `hyp = √(reach² + r_max²)` lever prices the radius and the
/// `chart_region_cyl_transfer` row measures the residual at the trims
/// themselves — the pair now refuses typed (the acceptance
/// direction this row was adopted in).
#[test]
fn r2_tilted_disjoint_hairline_pair_certifies_false_positive() {
    // ε-RELATIVE (fix pass): θ = 3e4·ε, w = 2000·ε — the same band
    // multiples at every ε row (fixed-size hairlines are refused by
    // the euler ops once ε reaches their scale). The tilt gate fires
    // BEFORE extraction, so the mint's exact-structure lottery at
    // coarse-ε tilts cannot mask this row's refusal.
    let eps = Tol::witness().eps();
    let theta = 3e4 * eps;
    let r = 1.0_f64;
    let w = 2000.0 * eps;
    let mut ba = Body::<f64>::new();
    let Some(fa) = try_wall_sheet(&mut ba, frame_a(r), 9001, 0.2, 1.2, 0.0, w) else {
        println!("the fixture cannot be minted at this ε — standing down");
        return;
    };
    let bframe = tilted_frame(r, theta);
    let mut bb = Body::<f64>::new();
    let Some(fb) = try_wall_sheet(&mut bb, bframe, 9002, 0.3, 1.3, -0.3 * w, 0.7 * w) else {
        println!("the tilted fixture cannot be minted at this ε — standing down");
        return;
    };

    // --- 3D disjointness, exact: max z over B's whole trim.
    // z(u_b, v_b) = v_b·cosθ − r·sinθ·cos(u_b); max at v_b = 0.7w,
    // u_b = 1.3 (cos smallest). A's trim has z ∈ [0, w] exactly.
    let max_z_b = 0.7 * w * theta.cos() - r * theta.sin() * (1.3_f64).cos();
    assert!(
        max_z_b < -6000.0 * eps,
        "B's trim tops out at z = {max_z_b:.3e} — the trims are 3D-disjoint \
         by > 6000·ε (A's trim is at z >= 0)"
    );
    // Cross-check by dense sampling of both trims' 3D images.
    let mut min_d2 = f64::MAX;
    let samples_a: Vec<Point3<f64>> = (0..=60)
        .flat_map(|i| (0..=6).map(move |j| (0.2 + (i as f64) / 60.0, (j as f64) * w / 6.0)))
        .map(|(u, v)| frame_a(r).at(u, v))
        .collect();
    for i in 0..=60 {
        for j in 0..=6 {
            let (u, v) = (0.3 + (i as f64) / 60.0, -0.3 * w + (j as f64) * w / 6.0);
            let p = bframe.at(u, v);
            for q in &samples_a {
                let d = p - *q;
                min_d2 = min_d2.min(d.dot(d));
            }
        }
    }
    assert!(
        min_d2.sqrt() > 6000.0 * eps,
        "sampled min distance {} confirms 3D disjointness",
        min_d2.sqrt()
    );

    // --- The enclosure's verdict on the same pair (ADOPTED as a
    //     red-first acceptance row at the fix pass: on the pre-fix
    //     head this pair CERTIFIED PositiveArea — the review's central
    //     finding; the redesigned gate prices the radius through the
    //     `hyp` lever and refuses it typed).
    let got = declared_pair_overlap(&ba, fa, &bb, fb, ContactVerdict::Definite, band());
    assert!(
        matches!(got, Err(ChartRegionError::CarrierTilt)),
        "a 3D-disjoint tilted pair (gap > 5e-6 m ≈ 5000·eps) must refuse \
         typed at the carrier gates, got {got:?}"
    );
}

/// PROBE (census-reachable tier; adopted as a red-first acceptance
/// row): a tilt INSIDE Door 1's own band (θ = 8e-9 rad: at the 1 m
/// arm that is in [ε, k·ε), exactly what `Bridged` bridges) on a
/// LARGE cylinder (r = 100 m), trims 3D-disjoint by ~100·ε. On the
/// pre-fix head the gates passed and the arm CERTIFIED — the
/// composition through the census admitted a radius-levered error.
/// The fixed gate's `hyp` lever refuses it typed.
#[test]
fn r2_bridged_band_tilt_large_radius_certifies_beyond_eps() {
    // θ = 8·ε rad: at Door 1's pinned 1 m arm this is IN BAND
    // ([ε, k·ε)), i.e. exactly the residue `Bridged` bridges — this
    // pair is census-reachable with a declaration. ε-RELATIVE at the
    // fix pass, so the band-relative story holds at every ε row; the
    // tilt gate fires before extraction, so no mint-lottery masking.
    let eps = Tol::witness().eps();
    let theta = 8.0 * eps;
    let r = 100.0_f64;
    let w = 100.0 * eps;
    let mut ba = Body::<f64>::new();
    let fa = wall_sheet(&mut ba, frame_a(r), 9003, 0.2, 1.2, 0.0, w);
    let bframe = tilted_frame(r, theta);
    let mut bb = Body::<f64>::new();
    let fb = wall_sheet(&mut bb, bframe, 9004, 0.3, 1.3, -0.3 * w, 0.7 * w);
    // Max z over B's trim: 0.7w − r·sinθ·cos(1.3) — every point of B
    // below it; A's trim at z ∈ [0, w].
    let max_z_b = 0.7 * w * theta.cos() - r * theta.sin() * (1.3_f64).cos();
    assert!(
        max_z_b < -60.0 * eps,
        "B tops out at z = {max_z_b:.3e}: 3D-disjoint from A by > 60·ε"
    );
    // ADOPTED as a red-first acceptance row at the fix pass: on the
    // pre-fix head this census-reachable pair CERTIFIED PositiveArea.
    // The `hyp` lever prices the 100 m radius (sin θ · hyp ≈ 8e-7 m,
    // definitely apart), so the gate refuses typed — under `Bridged`
    // the premise band is additionally tightened, which can only move
    // the outcome further from certification.
    let got = declared_pair_overlap(&ba, fa, &bb, fb, ContactVerdict::Bridged, band());
    assert!(
        matches!(got, Err(ChartRegionError::CarrierTilt)),
        "a Door-1-band tilt on a 100 m cylinder must refuse typed at the \
         carrier gates, got {got:?}"
    );
}

/// PROBE (Q3, can-this-row-fail): the unit's
/// `one_axis_tilt_two_extents_two_answers` SHORT arm asserts only
/// `!matches!(short, Err(CarrierTilt))` — this row measures what the
/// short pair ACTUALLY returns. If it is any refusal other than
/// CarrierTilt (e.g. NonPlanarTrim from the tilted frame's trim
/// extraction), the unit row is vacuously green and "a peg-extent
/// pair absorbs the tilt" is not demonstrated by it.
#[test]
fn r2_what_does_the_short_tilt_pair_actually_return() {
    let eps = Tol::witness().eps();
    let tilt = 40.0 * Tol::witness().k() * eps;
    let mut ba = Body::<f64>::new();
    let fa = wall_sheet(&mut ba, frame_a(1.0), 9101, 0.2, 1.6, 0.0, 1e-3);
    let mut bb = Body::<f64>::new();
    let fb = wall_sheet(&mut bb, tilted_frame(1.0, tilt), 9102, 0.2, 1.6, 0.0, 1e-3);
    let short = declared_pair_overlap(&ba, fa, &bb, fb, ContactVerdict::Definite, band());
    println!("short tilt pair verdict: {short:?}");
    // RE-MEASURED at the fix pass. On the pre-fix head this pair
    // declined `TouchingBoundary` (identical windows share collinear
    // boundaries), which made the unit row's `!CarrierTilt` assertion
    // vacuous — the finding this probe recorded. The redesigned gate
    // prices the RADIUS (hyp ≈ r = 1 here), so a 4e-7 rad tilt is a
    // definite disagreement for this pair regardless of its 1 mm wall
    // height, and the gate refuses BEFORE the walk can touch. The
    // peg-absorbs-tilt demonstration moved to the unit row's genuinely
    // small-`hyp` fixture (radius AND height in millimetres).
    assert!(
        matches!(short, Err(ChartRegionError::CarrierTilt)),
        "re-measure: the r = 1 m 'short' tilted pair now returns {short:?}"
    );
}

/// PROBE (claim 3, adversarial frame-invariance fixture): trims
/// hugging the seam of ONE description (A's window straddles its own
/// u = 0 seam value; B's window sits mid-chart, u_ref rotated 0.7),
/// run both ways. Any verdict-class asymmetry is a finding.
#[test]
fn r2_seam_hugging_pair_is_frame_invariant_both_ways() {
    let d = 0.7_f64;
    let frame_b = CylFrame {
        origin: Point3::new(0.0, 0.0, 0.25),
        axis: -Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::new(d.cos(), d.sin(), 0.0),
    };
    // World window: azimuth [-0.15, 0.15] (A-chart: straddles 0;
    // B-chart: [0.55, 1.0] − mid-chart), z [0.1, 0.6] and [0.2, 0.5].
    let mut ba = Body::<f64>::new();
    let fa = wall_sheet(&mut ba, frame_a(1.0), 9005, -0.15, 0.15, 0.1, 0.6);
    let mut bb = Body::<f64>::new();
    // world θ = 0.7 − u_B, so world [-0.2, 0.3] is u_B [0.4, 0.9].
    let fb = wall_sheet(&mut bb, frame_b, 9006, 0.4, 0.9, 0.25 - 0.5, 0.25 - 0.2);
    let ab = verdict_class(declared_pair_overlap(
        &ba,
        fa,
        &bb,
        fb,
        ContactVerdict::Definite,
        band(),
    ));
    let ba_v = verdict_class(declared_pair_overlap(
        &bb,
        fb,
        &ba,
        fa,
        ContactVerdict::Definite,
        band(),
    ));
    assert_eq!(ab, ba_v, "seam-hugging pair: the two orders disagree");
    println!("seam-hugging: {ab} (both orders)");
}

/// Diagnostic: which (radius, tilt) combinations survive the fixture
/// builder's exact-structure pcurve minting (the C6 gate is
/// bit-fragile under tilted frames — this maps the usable space).
#[test]
fn r2_diag_mintable_tilts() {
    // ε-RELATIVE at the fix pass: the fixtures' sizes and tilts are
    // band multiples, so the map stays meaningful at every eps row
    // (absolutely-sized hairlines are refused by the euler ops
    // themselves once eps reaches their scale).
    let eps = Tol::witness().eps();
    let w = 2000.0 * eps;
    for r in [1.0_f64, 10.0, 100.0, 1000.0] {
        for theta_e in [
            0.0_f64, 1.5, 2.0, 2.5, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 9.5, 30.0, 100.0, 300.0,
            1e3, 3e3, 1e4, 3e4,
        ] {
            let theta = theta_e * eps;
            let got = std::panic::catch_unwind(|| {
                let mut bb = Body::<f64>::new();
                let fb = wall_sheet(
                    &mut bb,
                    tilted_frame(r, theta),
                    9300,
                    0.3,
                    1.3,
                    -0.3 * w,
                    0.7 * w,
                );
                let mut bb2 = Body::<f64>::new();
                let fb2 = wall_sheet(&mut bb2, tilted_frame(r, theta), 9301, 0.2, 1.2, 0.0, w);
                verdict_class(declared_pair_overlap(
                    &bb,
                    fb,
                    &bb2,
                    fb2,
                    ContactVerdict::Definite,
                    band(),
                ))
            });
            match got {
                Ok(v) => println!("r={r} theta={theta:e}: {v}"),
                Err(_) => println!("r={r} theta={theta:e}: fixture build panicked"),
            }
        }
    }
}

/// PROBE (claim 5, the disclosed decline (ii), unpinned by the unit):
/// a flush cylinder seat — rim-sharing trims through the GENERAL walk
/// (arc windows, not full wraps) — declines `TouchingBoundary` as the
/// arm's docs disclose. Pins the decline the PR states but never rows.
#[test]
fn r2_flush_cylinder_seat_declines_touching_boundary() {
    let d = 0.7_f64;
    let frame_b = CylFrame {
        origin: Point3::new(0.0, 0.0, 0.25),
        axis: -Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::new(d.cos(), d.sin(), 0.0),
    };
    let mut ba = Body::<f64>::new();
    let fa = wall_sheet(&mut ba, frame_a(1.0), 9007, 0.2, 1.6, 0.0, 0.5);
    let mut bb = Body::<f64>::new();
    // world z [0.5, 1.0] = v_B [0.25 − 1.0, 0.25 − 0.5]; azimuth
    // [0.5, 1.3] = u_B [0.7 − 1.3, 0.7 − 0.5].
    let fb = wall_sheet(&mut bb, frame_b, 9008, -0.6, 0.2, -0.75, -0.25);
    let got = declared_pair_overlap(&ba, fa, &bb, fb, ContactVerdict::Definite, band());
    match got {
        Err(ChartRegionError::TouchingBoundary) => {}
        other => panic!("expected the disclosed TouchingBoundary decline: {other:?}"),
    }
}
