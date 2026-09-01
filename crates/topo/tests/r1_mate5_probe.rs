//! **R1 review probes for MATE-5 (PR #1443).** Not a deliverable — an
//! adversarial lane's demonstrations. Each row is named for the claim
//! it attacks. Rows that FAIL are the findings.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

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

fn frame_a() -> CylFrame {
    CylFrame {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    }
}

/// Verbatim from `tests/mate5_cyl_eps_rung.rs` (the unit's own fixture
/// builder) so the probes stand on the unit's own ground.
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
            let s = frame.at(u1, v)
                - center
                - frame.axis * ((frame.at(u1, v) - center).dot(frame.axis));
            (
                Curve3::Circle {
                    center,
                    axis: -frame.axis,
                    radius: frame.radius,
                    u_ref: s.normalize(),
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

fn verdict_class(r: Result<ChartOverlap, ChartRegionError>) -> String {
    match r {
        Ok(ChartOverlap::PositiveArea) => "PositiveArea".into(),
        Ok(ChartOverlap::Empty) => "Empty".into(),
        Err(ChartRegionError::Escalated(d)) => format!("Escalated({})", d.predicate.unwrap_or("?")),
        Err(other) => format!("{other:?}")
            .split([' ', '{', '('])
            .next()
            .unwrap()
            .to_string(),
    }
}

// =====================================================================
// PROBE 1 (claim 1 + 2 + 10): the tilt gate's lever omits the RADIUS,
// but the chart transfer's axial error is levered by ‖P − o_b‖, which
// CONTAINS the radius. Consequence: a pair whose gates all certify
// Zero can have its transferred trims displaced axially far outside
// the band — and certify PositiveArea on a trim pair that is
// definitely, separably EMPTY in world space.
// =====================================================================

/// The tilted B description: same locus to within `r(1−cos θ)`, axis
/// tilted by `θ` about ŷ through the shared origin.
fn tilted_frame(theta: f64) -> CylFrame {
    CylFrame {
        origin: Point3::origin(),
        axis: Vec3::new(theta.sin(), 0.0, theta.cos()),
        radius: 1.0,
        u_ref: Vec3::new(theta.cos(), 0.0, -theta.sin()),
    }
}

#[test]
fn probe1_tilt_lever_omits_the_radius_and_certifies_a_separated_pair() {
    let eps = Tol::witness().eps();
    let theta = 5e-6_f64;
    let fb = tilted_frame(theta);
    // Azimuth window [0.2, 1.4] (cos u > 0 throughout, so the tilt
    // pushes B's TRUE world z strictly DOWN); no seam crossing.
    let (u0, u1) = (0.2_f64, 1.4_f64);
    let ha = 1e-4;
    let (mut body_a, mut body_b) = (Body::<f64>::new(), Body::<f64>::new());
    let fa_key = wall_sheet(&mut body_a, frame_a(), 8001, u0, u1, 0.0, ha);

    // B's chart-v window straddles 0, so the TRANSFER (identity here:
    // δ = 0, σ = +1, c = 0) reads it as overlapping A's [0, ha].
    let vb1 = 0.08 * theta.sin();
    let vb0 = vb1 - 5e-5;
    let fb_key = wall_sheet(&mut body_b, fb, 8002, u0, u1, vb0, vb1);

    // --- The TRUTH, from the two chart maps.
    let mut b_zmax = f64::NEG_INFINITY;
    let mut b_zmin = f64::INFINITY;
    for i in 0..=400 {
        let u = u0 + (u1 - u0) * (i as f64) / 400.0;
        for v in [vb0, vb1] {
            let z = fb.at(u, v).z;
            b_zmax = b_zmax.max(z);
            b_zmin = b_zmin.min(z);
        }
    }
    let gap = 0.0 - b_zmax;
    println!(
        "TRUTH: A world z in [0, {ha:e}]; B world z in [{b_zmin:e}, {b_zmax:e}]; \
         separation = {gap:e} m = {:.0}*eps",
        gap / eps
    );
    println!(
        "carrier radial residual = {:e} m = {:.3e}*eps  (the carrier premise HOLDS)",
        1.0 - theta.cos(),
        (1.0 - theta.cos()) / eps
    );
    println!(
        "transferred B band = [{vb0:e}, {vb1:e}] -> overlaps A's [0, {ha:e}]"
    );

    let got = declared_pair_overlap(
        &body_a,
        fa_key,
        &body_b,
        fb_key,
        ContactVerdict::Definite,
        band(),
    );
    println!("ANSWER: {got:?}");

    assert!(
        gap > 100.0 * eps,
        "fixture sanity: trims must be separated far outside the band (got {gap:e})"
    );
    assert!(
        !matches!(got, Ok(ChartOverlap::PositiveArea)),
        "FINDING: the arm CERTIFIES PositiveArea for a declared cylinder \
         pair whose trims are separated in world space by {:.0}*eps, on a \
         carrier pair whose radial residual is 5e-4*eps. The tilt gate's \
         lever is the AXIAL reach; the transfer's axial error is levered \
         by the RADIUS.",
        gap / eps
    );
}

// =====================================================================
// PROBE 2 (claim 4 + 10): the full-wrap band fast path's structural
// detector demands BIT-EXACT chart points. `uv_b` is always the
// TRANSFERRED polygon (`δ + σu`, `c + σv`), so at a non-point scalar
// its coordinates are never exact — the fast path the PR calls "the
// class's most ordinary member's only non-declining route" cannot fire
// on the interval lane the gate was pinned to.
// =====================================================================

#[test]
fn probe2_band_fast_path_exactness_gate_is_f64_only() {
    // Demonstrated arithmetically: the transfer adds `delta`, an
    // `atan2` result. Under any scalar whose `lo() != hi()` for a
    // rounded transcendental, `wrap_band`'s `xl == xh` test fails.
    // At f64 lo()==hi() always, so the gate is a no-op there.
    let a = frame_a();
    let b = tilted_frame(0.0);
    let w_a = a.axis.cross(a.u_ref);
    let delta = b.u_ref.dot(w_a).atan2(b.u_ref.dot(a.u_ref));
    println!("delta (f64) = {delta:e}; f64 lo()==hi() by construction");
    // The structural claim under review, restated for the report:
    // `wrap_band` reads `p.x.lo() == p.x.hi()`; `uv_b`'s x is
    // `delta + sigma*p.x`. See the report's finding text.
    assert_eq!(delta, 0.0);
}

// =====================================================================
// PROBE 3 (claim 3): adversarial frame invariance — a LARGE u_ref
// offset with both trims hugging the seam, run both ways.
// =====================================================================

fn sheet(frame: CylFrame, src: u64, u0: f64, u1: f64, v0: f64, v1: f64) -> (Body<f64>, FaceKey) {
    let mut body = Body::<f64>::new();
    let f = wall_sheet(&mut body, frame, src, u0, u1, v0, v1);
    (body, f)
}

/// A divergent description with an ARBITRARY seam rotation `d` and the
/// axis reversed — the fixture family the unit ships, parameterised.
fn frame_b_at(d: f64) -> CylFrame {
    CylFrame {
        origin: Point3::new(0.0, 0.0, 0.25),
        axis: -Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::new(d.cos(), d.sin(), 0.0),
    }
}

#[test]
fn probe3_large_seam_offset_with_trims_hugging_the_seam_both_ways() {
    // World azimuth windows that straddle / hug A's seam at u = 0.
    let cases: Vec<(&str, f64, (f64, f64), (f64, f64))> = vec![
        ("seam-straddling, d = 3.0", 3.0, (-0.3, 0.3), (-0.15, 0.45)),
        ("seam-hugging, d = 6.0", 6.0, (0.0, 0.6), (-0.2, 0.2)),
        ("d = 6.2 (near τ)", 6.2, (-0.4, 0.4), (0.1, 0.9)),
        ("d = -2.9", -2.9, (-0.5, 0.1), (-0.05, 0.5)),
        ("d = 3.14159", 3.141_59, (-0.25, 0.25), (-0.25, 0.25)),
    ];
    let mut disagreements = vec![];
    for (name, d, (t0, t1), (s0, s1)) in cases {
        let fb = frame_b_at(d);
        let (a, fa) = sheet(frame_a(), 8101, t0, t1, 0.0, 1.0);
        // Same world region, expressed in B's chart:
        // θ_world = d − u_B, z_world = 0.25 − v_B.
        let (b, fbk) = sheet(fb, 8102, d - s1, d - s0, 0.25 - 0.7, 0.25 - 0.3);
        let ab = verdict_class(declared_pair_overlap(
            &a,
            fa,
            &b,
            fbk,
            ContactVerdict::Definite,
            band(),
        ));
        let ba = verdict_class(declared_pair_overlap(
            &b,
            fbk,
            &a,
            fa,
            ContactVerdict::Definite,
            band(),
        ));
        println!("{name}: A→B = {ab}, B→A = {ba}");
        if ab != ba {
            disagreements.push(format!("{name}: {ab} vs {ba}"));
        }
    }
    assert!(
        disagreements.is_empty(),
        "FINDING: frame-invariance asymmetry: {disagreements:?}"
    );
}

// =====================================================================
// PROBE 4 (claim 5): can the three declines fire on decidable
// geometry, and does the arm ever answer differently than the truth
// near the seam? Also: does PeriodFold reach f64 at all?
// =====================================================================

#[test]
fn probe4_declines_on_decidable_geometry_are_reachable_and_typed() {
    // The unit's own SeamBranch row generalised: spans summing past τ.
    let (a, fa) = sheet(frame_a(), 8201, 0.0, 3.5, 0.0, 1.0);
    let (b, fbk) = sheet(frame_b_at(0.7), 8202, 0.7 - 6.5, 0.7 - 3.3, -0.45, -0.05);
    println!(
        "un-windowable pair: {}",
        verdict_class(declared_pair_overlap(
            &a,
            fa,
            &b,
            fbk,
            ContactVerdict::Definite,
            band()
        ))
    );
    // A FLUSH seat (shared rim): TouchingBoundary is claimed.
    let (a2, fa2) = sheet(frame_a(), 8203, 0.2, 1.6, 0.0, 0.5);
    let (b2, fb2) = sheet(frame_b_at(0.7), 8204, 0.7 - 1.3, 0.7 - 0.5, 0.25 - 1.0, -0.25);
    println!(
        "flush (rim-sharing) seat: {}",
        verdict_class(declared_pair_overlap(
            &a2,
            fa2,
            &b2,
            fb2,
            ContactVerdict::Definite,
            band()
        ))
    );
}

// =====================================================================
// PROBE 5 (claim 7 + 9): does the arm ignoring `door_one` admit a pair
// Door 1 would have refused? Run the enclosure at EVERY verdict.
// =====================================================================

#[test]
fn probe5_door_one_verdict_is_ignored_at_every_variant() {
    let (a, fa) = sheet(frame_a(), 8301, 0.2, 1.6, 0.0, 1.0);
    let (b, fbk) = sheet(frame_b_at(0.7), 8302, 0.7 - 1.3, 0.7 - 0.5, -0.45, -0.05);
    // NOTE: `ContactVerdict` has only PASSING variants (Definite /
    // Bridged) — a refusal is `ContactRefusal`, a separate type — so
    // ignoring `door_one` cannot admit a pair Door 1 refused.
    for v in [ContactVerdict::Definite, ContactVerdict::Bridged] {
        println!(
            "door_one = {v:?} -> {}",
            verdict_class(declared_pair_overlap(&a, fa, &b, fbk, v, band()))
        );
    }
}

// =====================================================================
// PROBE 6: what does the unit's OWN `one_axis_tilt_two_extents` short
// pair actually answer? Its assertion is only `!CarrierTilt`.
// =====================================================================

#[test]
fn probe6_what_the_units_own_tilt_row_actually_answers() {
    let tol = Tol::witness();
    let tilt = 40.0 * tol.k() * tol.eps();
    for (label, ha) in [("short (unit's own A/B z 0..1e-3)", 1e-3), ("long (0..4)", 4.0)] {
        let (a, fa) = sheet(frame_a(), 8402, 0.2, 1.6, 0.0, ha);
        let (b, fb) = sheet(tilted_frame(tilt), 8401, 0.2, 1.6, 0.0, ha);
        let r = declared_pair_overlap(&a, fa, &b, fb, ContactVerdict::Definite, band());
        println!("{label}: {r:?}");
    }
    for (label, ha, vb0, vb1, theta) in [
        ("untilted baseline", 1e-3, 0.0, 1e-3, 0.0_f64),
        ("tilt only", 1e-3, 0.0, 1e-3, 5e-6),
        ("tilt + negative vb0", 1e-3, -5e-5, 4e-7, 5e-6),
        ("tilt + thin A", 1e-4, 0.0, 1e-3, 5e-6),
        ("untilted + negative vb0", 1e-3, -5e-5, 4e-7, 0.0),
    ] {
        let (a, fa) = sheet(frame_a(), 8403, 0.2, 1.4, 0.0, ha);
        let (b, fb) = sheet(tilted_frame(theta), 8404, 0.2, 1.4, vb0, vb1);
        let r = declared_pair_overlap(&a, fa, &b, fb, ContactVerdict::Definite, band());
        println!("{label}: {r:?}");
    }
}

// =====================================================================
// PROBE 7 (claim 1/2/10, the demonstration): sweep (radius, tilt,
// wall height) for a pair whose carrier gates all say Zero and whose
// trims are DEFINITELY separated in world space, and see what the arm
// answers. A `PositiveArea` here is a false certification.
// =====================================================================

fn tilted_at(theta: f64, radius: f64) -> CylFrame {
    CylFrame {
        origin: Point3::origin(),
        axis: Vec3::new(theta.sin(), 0.0, theta.cos()),
        radius,
        u_ref: Vec3::new(theta.cos(), 0.0, -theta.sin()),
    }
}

fn frame_a_r(radius: f64) -> CylFrame {
    CylFrame {
        radius,
        ..frame_a()
    }
}

#[test]
fn probe7_a_definitely_separated_pair_is_falsely_certified() {
    let eps = Tol::witness().eps();
    let (u0, u1) = (0.2_f64, 1.4_f64); // cos u > 0 throughout
    let mut false_positives: Vec<String> = vec![];
    for (r, theta, h) in [
        (10.0_f64, 2e-7_f64, 3e-3_f64),
        (10.0, 2e-7, 1e-3),
        (5.0, 4e-7, 1e-3),
        (2.0, 1e-6, 5e-4),
        (20.0, 1e-7, 5e-3),
        (50.0, 4e-8, 1e-2),
    ] {
        let fb = tilted_at(theta, r);
        let vb1 = 0.08 * theta.sin() * r;
        let vb0 = vb1 - h;
        let (a, fa) = sheet(frame_a_r(r), 8501, u0, u1, 0.0, h);
        let (b, fbk) = sheet(fb, 8502, u0, u1, vb0, vb1);
        // The truth, from the two chart maps.
        let mut b_zmax = f64::NEG_INFINITY;
        for i in 0..=400 {
            let u = u0 + (u1 - u0) * (i as f64) / 400.0;
            for v in [vb0, vb1] {
                b_zmax = b_zmax.max(fb.at(u, v).z);
            }
        }
        let gap = 0.0 - b_zmax;
        let got = declared_pair_overlap(&a, fa, &b, fbk, ContactVerdict::Definite, band());
        let tag = format!(
            "r={r} theta={theta:e} h={h:e}: true separation = {:.0}*eps, \
             carrier radial residual = {:.2e}*eps, ANSWER = {got:?}",
            gap / eps,
            (r * (1.0 - theta.cos())) / eps
        );
        println!("{tag}");
        if gap > 20.0 * eps && matches!(got, Ok(ChartOverlap::PositiveArea)) {
            false_positives.push(tag);
        }
    }
    assert!(
        false_positives.is_empty(),
        "FINDING: the arm certifies PositiveArea on definitely-separated \
         trims:\n{}",
        false_positives.join("\n")
    );
}

/// PROBE 7b: as probe 7, but with B's azimuth window strictly INSIDE
/// A's, so the overlap has no collinear side boundaries.
#[test]
fn probe7b_a_definitely_separated_pair_is_falsely_certified() {
    let eps = Tol::witness().eps();
    let (bu0, bu1) = (0.2_f64, 1.4_f64); // cos u > 0 throughout
    let (au0, au1) = (0.15_f64, 1.45_f64);
    let mut false_positives: Vec<String> = vec![];
    for (r, theta, h, frac) in [
        (5.0_f64, 4e-7_f64, 1e-3_f64, 0.08_f64),
        (5.0, 4e-7, 1e-3, 0.05),
        (5.0, 8e-7, 5e-4, 0.08),
        (5.0, 2e-7, 2e-3, 0.08),
        (4.0, 5e-7, 1e-3, 0.08),
        (8.0, 2.5e-7, 1e-3, 0.08),
    ] {
        let fb = tilted_at(theta, r);
        let vb1 = frac * theta.sin() * r;
        let vb0 = vb1 - h;
        let (a, fa) = sheet(frame_a_r(r), 8601, au0, au1, 0.0, h);
        let (b, fbk) = sheet(fb, 8602, bu0, bu1, vb0, vb1);
        let mut b_zmax = f64::NEG_INFINITY;
        for i in 0..=400 {
            let u = bu0 + (bu1 - bu0) * (i as f64) / 400.0;
            for v in [vb0, vb1] {
                b_zmax = b_zmax.max(fb.at(u, v).z);
            }
        }
        let gap = 0.0 - b_zmax;
        let got = declared_pair_overlap(&a, fa, &b, fbk, ContactVerdict::Definite, band());
        let tag = format!(
            "r={r} theta={theta:e} h={h:e} frac={frac}: true separation = \
             {:.0}*eps ({:.1}*k*eps), carrier radial residual = {:.2e}*eps, \
             ANSWER = {got:?}",
            gap / eps,
            gap / (10.0 * eps),
            (r * (1.0 - theta.cos())) / eps
        );
        println!("{tag}");
        if gap > 20.0 * eps && matches!(got, Ok(ChartOverlap::PositiveArea)) {
            false_positives.push(tag);
        }
    }
    assert!(
        false_positives.is_empty(),
        "FINDING: the arm certifies PositiveArea on definitely-separated \
         trims:\n{}",
        false_positives.join("\n")
    );
}
