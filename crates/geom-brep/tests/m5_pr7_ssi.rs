//! **M5 PR 7 acceptance** — rung 3 of the C1 ladder: march-then-certify
//! with in-op exhaustiveness (spec §6).
//!
//! The rows, in the spec's order:
//!
//! 1. **Shape (iv), the milestone's signature.** A planted fixture whose
//!    whole intersection is two *interior* loops that touch no domain
//!    boundary: boundary-curve seeding provably cannot reach either, and
//!    a marcher launched from one of them finds exactly one. Both are
//!    found — because the subdivision enumerated them — and both certify.
//! 2. **The found-AND-floor-refused variant.** The same fixture with the
//!    accounting floor clamped above the certified tube radius: the
//!    branches are still found, and the operation nonetheless refuses
//!    `SsiExhaustivenessInconclusive` rather than claim they are all of
//!    them. Never silence, in both directions.
//! 3. **Shape (iii) substrate.** A directly-authored NURBS wall cut by a
//!    plane: marched in ℝ⁴, fitted, all three limbs.
//! 4. **The OQ4 demonstration.** The ℝ⁴ trace's two pcurves are
//!    coordinate projections of one parameterized object, so
//!    `|S(P(t)) − C(t)| ≤ ε` holds on the **carrier's own parameter** at
//!    the PR 6 cache schedule — the parameter-identity contract, checked
//!    the way PR 6's door checks it.
//! 5. **The corrupted-cache limb rows.** Each limb refuses separately.
//! 6. **The σ₂-sliver row**, refusing toward C7.
//! 7. **The closure trio**, exercised as verdicts: a closed loop, a
//!    boundary-terminated branch, and the tangent-match arm.
//! 8. **Idealized vs realized**, the T4 differential pin.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::CERT_SAMPLES;
use geom_brep::ssi::BranchEnd;
use geom_brep::ssi::{self, SsiDomain, SsiError, SsiLimb, SsiOperand};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tolerance, Vec3};
use geom_curves::{Curve3, NurbsCurve3};
use geom_surfaces::{NurbsSurface, Surface};

fn eps() -> f64 {
    Tolerance::get().eps
}

fn band() -> Band {
    Band::linear().unwrap()
}

// ---------------------------------------------------------------------
// Shape (iv): the planted two-interior-loop fixture
// ---------------------------------------------------------------------

/// The unit sphere at the origin.
fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// A thin cylinder threaded through the sphere, offset from the axis so
/// the locus is a genuine quartic and not a pair of exact circles.
///
/// Because `radius + offset = 0.11 < 1`, the cylinder wall never reaches
/// the sphere's equator: the intersection is **two disjoint small loops**
/// near the poles (`z ≈ ±0.996`, each about 0.5 m long), and *nothing*
/// else. Neither touches the session slab's boundary, so
/// boundary-curve×surface seeding — the only seeding a marcher gets for
/// free — finds nothing at all. This is the classic silent disaster the
/// banked principle names, planted deliberately.
fn threaded_cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.03, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 0.08,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn slab() -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.0, 0.0, 0.0),
        half_extent: 1.5,
        extent: 2.0,
        eps: eps(),
        floor_scale: 1.0,
    }
}

/// The planted fixture's outcome, computed **once per process**.
///
/// A rung-3 intersection at ε = 1e-9 legitimately produces a carrier
/// with several hundred control points (the geometry's own requirement:
/// a cubic needs that many spans to stay inside ε on a loop of 0.08 m
/// radius), and the interpolation solve is cubic in that. Re-running it
/// per row would make the suite's cost a multiple of the operation's
/// for no extra coverage — the rows below ask different questions of
/// the *same* result.
fn fixture() -> geom_brep::SsiOutcome {
    static ONCE: std::sync::OnceLock<geom_brep::SsiOutcome> = std::sync::OnceLock::new();
    ONCE.get_or_init(|| {
        let (s, c) = (sphere(), threaded_cylinder());
        ssi::cylinder_sphere_ssi(&c, &s, slab(), band()).expect("the planted fixture")
    })
    .clone()
}

#[test]
fn shape_iv_both_interior_loops_are_found_and_certified() {
    let out = fixture();
    assert_eq!(
        out.branches.len(),
        2,
        "expected two interior loops, got {} (exhaustiveness {:?})",
        out.branches.len(),
        out.exhaustiveness
    );
    for b in out.branches.iter() {
        assert_eq!(b.end, BranchEnd::Closed, "an interior loop must close");
        // The full three-limb certificate rode along.
        assert!(b.certificate.on_locus_max <= eps(), "{:?}", b.certificate);
        assert!(b.certificate.hull_sup <= eps(), "{:?}", b.certificate);
        assert!(b.certificate.tube_boxes >= 1, "{:?}", b.certificate);
        assert!(
            b.certificate.tube_transversality > 0.0,
            "the uniqueness tube must have positive headroom: {:?}",
            b.certificate
        );
        // The witness is carrier(mid), unchanged from M2.
        let Curve3::Nurbs(ref n) = b.carrier else {
            panic!("a rung-3 carrier is a NURBS curve");
        };
        let (t0, t1) = n.domain();
        let mid = n.eval(0.5 * (t0 + t1));
        assert!((b.witness - mid).norm() < 1e-15);
    }
    // The two loops are near opposite poles, so they are genuinely
    // distinct components and not the same one found twice.
    let z: Vec<f64> = out.branches.iter().map(|b| b.witness.z).collect();
    assert!(z[0] * z[1] < 0.0, "expected one loop per pole, got {z:?}");
    // The never-silence receipt.
    assert!(out.exhaustiveness.excluded > 0, "{:?}", out.exhaustiveness);
    assert!(out.exhaustiveness.accounted > 0, "{:?}", out.exhaustiveness);
    assert!(out.seeds >= 2, "seeds = {}", out.seeds);
}

#[test]
fn a_single_seeded_march_finds_only_one_of_the_two_loops() {
    // The row that makes row 1 mean something: marching is a candidate
    // generator, and a candidate generator launched once returns one
    // candidate. Found-ness comes from the subdivision, not from luck.
    let (s, c) = (sphere(), threaded_cylinder());
    let seed = Point3::new(0.11, 0.0, 0.994);
    let (pts, end) = ssi::idealized_trace_r3(&c, &s, seed, slab(), band()).expect("a seeded trace");
    assert_eq!(end, BranchEnd::Closed);
    // Every sample of the single trace sits on ONE pole's loop.
    assert!(
        pts.iter().all(|p| p.z > 0.0),
        "one march cannot reach the other component"
    );
}

#[test]
fn the_floor_clamped_variant_refuses_typed_even_though_branches_were_found() {
    // Same fixture, accounting floor clamped far above any certifiable
    // tube radius: cells along the locus can be neither excluded nor
    // accounted, so the operation refuses instead of reporting an
    // intersection it cannot prove complete.
    let (s, c) = (sphere(), threaded_cylinder());
    // One loop's neighbourhood: the row is about the floor, not about
    // finding both components, and a rung-3 op is not cheap.
    let mut d = SsiDomain {
        center: Point3::new(0.03, 0.0, 0.996),
        half_extent: 0.2,
        extent: 0.4,
        eps: eps(),
        floor_scale: 1.0,
    };
    d.floor_scale = 1.0e8; // floor = 0.1 m, far wider than any tube
    let err = ssi::cylinder_sphere_ssi(&c, &s, d, band()).expect_err("must refuse");
    let msg = format!("{err}");
    match err {
        SsiError::ExhaustivenessInconclusive {
            cell_width, floor, ..
        } => {
            assert!(cell_width <= floor, "{cell_width} vs {floor}");
        }
        other => panic!("expected the exhaustiveness refusal, got {other}"),
    }
    // The refusal says what it means and what to do.
    assert!(msg.contains("exhaustiveness inconclusive"), "{msg}");
    assert!(msg.contains("refuses"), "{msg}");
}

// ---------------------------------------------------------------------
// The σ₂-sliver row
// ---------------------------------------------------------------------

#[test]
fn a_tangent_pair_refuses_toward_the_c7_regime_and_never_desingularizes() {
    // A cylinder of the sphere's own radius, coaxial: the two surfaces
    // are TANGENT along the equator. Transversality dies on the whole
    // candidate locus, which is C7's construction, not a locus to march.
    let s = sphere();
    let c = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let err = ssi::cylinder_sphere_ssi(&c, &s, slab(), band()).expect_err("must refuse");
    let msg = format!("{err}");
    match err {
        SsiError::TransversalityBand { sin_theta, .. } => {
            assert!(sin_theta < 1.0e-6, "sin θ = {sin_theta}");
        }
        // The tube's own straddle is the same verdict reached one stage
        // later; either is a correct refusal toward C7.
        SsiError::TubeStraddles { .. } | SsiError::CertificateLimb { .. } => {}
        other => panic!("expected a tangency-shaped refusal, got {other}"),
    }
    // The refusal names the regime it points at.
    assert!(
        msg.contains("TangentIntersection") || msg.contains("tube"),
        "{msg}"
    );
}

#[test]
fn the_uniqueness_tube_margin_dies_on_a_tangent_pair() {
    // The limb-3 mechanism in isolation: on a tangent pair the graph
    // criterion's enclosure straddles zero at every box size, because
    // the gradients are parallel there. This is what makes the tube
    // refuse rather than certify a component it cannot separate.
    let s = sphere();
    let c = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    // A carrier that IS the tangency circle (the equator).
    // Interpolated, not approximated, and densely: the row is about
    // limb 3, so limbs 1 and 2 must pass on their own merits. A cubic
    // interpolant through 400 exact circle points deviates by
    // ≈ ((π/2)/200)⁴/384 ≈ 1e-11 m, comfortably inside ε even after the
    // control-hull bound's conservatism.
    // A quarter arc, not the whole circle: the row is about limb 3, so
    // limbs 1 and 2 must pass on their own merits, and a short arc
    // reaches the same interpolation accuracy with a quarter of the
    // samples — which matters, because the interpolation solve is cubic
    // in the sample count.
    let pts: Vec<Point3<f64>> = (0..=200)
        .map(|i| {
            let t = std::f64::consts::FRAC_PI_2 * (f64::from(i) / 200.0);
            Point3::new(t.cos(), t.sin(), 0.0)
        })
        .collect();
    let carrier =
        NurbsCurve3::<f64>::interpolate(&pts, 3).expect("a cubic interpolant of the equator");
    let err = ssi::certify_rung3(
        &carrier,
        None,
        &SsiOperand::Analytic(&c),
        &SsiOperand::Analytic(&s),
        1.0,
        2.0,
        eps(),
        band(),
    )
    .expect_err("a tangency cannot certify a uniqueness tube");
    match err {
        SsiError::TubeStraddles { .. } => {}
        SsiError::CertificateLimb { limb, .. } => {
            assert_eq!(limb, SsiLimb::Tube, "the tube limb is the one that dies");
        }
        // A coarse fit of the equator may trip an earlier limb first;
        // that is still a refusal, but it is not the row's point, so
        // name it loudly if it happens.
        other => panic!("expected a limb-3 refusal, got {other}"),
    }
}

// ---------------------------------------------------------------------
// The corrupted-cache limb rows
// ---------------------------------------------------------------------

/// A good carrier for one loop of the planted fixture. Computed once
/// per process: the operation runs a full exhaustiveness sweep, and the
/// corruption rows only need the carrier it produced.
fn good_carrier() -> NurbsCurve3<f64> {
    static ONCE: std::sync::OnceLock<NurbsCurve3<f64>> = std::sync::OnceLock::new();
    ONCE.get_or_init(|| {
        let out = fixture();
        let Curve3::Nurbs(ref n) = out.branches[0].carrier else {
            panic!("a rung-3 carrier is a NURBS curve");
        };
        (**n).clone()
    })
    .clone()
}

/// The same carrier with control point `i` displaced by `d` metres in
/// `+z` — a planted corruption.
///
/// `+z` deliberately: the fixture's loops sit near the sphere's poles,
/// where the sphere normal is essentially `+z`, so this displacement is
/// *normal* to the locus and shows up in the residual at full size. An
/// `+x` displacement there is nearly tangent to the sphere and would
/// plant a corruption the certificate is right not to see.
fn displaced(curve: &NurbsCurve3<f64>, i: usize, d: f64) -> NurbsCurve3<f64> {
    let mut control = curve.control().to_vec();
    control[i] = control[i] + Vec3::new(0.0, 0.0, d);
    NurbsCurve3::new(curve.knots().clone(), control, curve.weights().to_vec())
        .expect("structure unchanged")
}

fn certify_against(carrier: &NurbsCurve3<f64>) -> Result<geom_brep::SsiCertificate, SsiError> {
    let (s, c) = (sphere(), threaded_cylinder());
    ssi::certify_rung3(
        carrier,
        None,
        &SsiOperand::Analytic(&c),
        &SsiOperand::Analytic(&s),
        0.08,
        2.0,
        eps(),
        band(),
    )
}

#[test]
fn a_good_carrier_certifies_all_three_limbs() {
    let carrier = good_carrier();
    let cert = certify_against(&carrier).expect("the branch this came from certified");
    assert_eq!(cert.samples, CERT_SAMPLES);
    assert!(cert.hull_sup <= eps());
    assert!(cert.tube_transversality > 0.0);
}

#[test]
fn corrupting_a_carrier_grossly_fails_the_on_locus_limb() {
    let carrier = good_carrier();
    let n = carrier.control().len() / 2;
    // A micron is a thousand ε: the schedule sees it immediately.
    let bad = displaced(&carrier, n, 1.0e-6);
    match certify_against(&bad) {
        Err(SsiError::CertificateLimb { limb, value }) => {
            assert_eq!(limb, SsiLimb::OnLocus, "value = {value}");
        }
        other => panic!("expected limb 1 to refuse, got {other:?}"),
    }
}

#[test]
fn a_between_samples_excursion_is_caught_by_the_hull_limb_alone() {
    // THE limb-2 row (C2.2's whole reason to exist): a displacement
    // small enough that the nine-point schedule still passes, but large
    // enough that the certified control-hull bound does not. The
    // sampled max steers; the hull bound certifies.
    let carrier = good_carrier();
    // Deliberately NOT the middle: a control point at the parameter
    // midpoint sits on a schedule sample, so its bump would be seen by
    // limb 1 and the row would be testing nothing. Three sixteenths
    // lands between samples 1 and 2 of the nine.
    let n = carrier.control().len() * 3 / 16;
    let mut found = None;
    // A deterministic ascending scan — the planted excursion is looked
    // for, not guessed at.
    for k in 1..=200u32 {
        let d = eps() * 0.05 * f64::from(k);
        let bad = displaced(&carrier, n, d);
        match certify_against(&bad) {
            Err(SsiError::CertificateLimb {
                limb: SsiLimb::HullSup,
                value,
            }) => {
                found = Some((d, value));
                break;
            }
            // A hull bound that lands just ABOVE ε is inside the
            // escalation band, so limb 2 speaks as an F6 escalation
            // rather than a definite refusal. Same limb, same meaning —
            // and the predicate name is how they are told apart.
            Err(SsiError::Escalated(ref diag)) if diag.predicate == Some("ssi_hull_sup") => {
                found = Some((d, f64::NAN));
                break;
            }
            // Still inside both limbs, or already gross enough to trip
            // limb 1 — keep scanning / stop.
            Ok(_) => {}
            Err(SsiError::CertificateLimb {
                limb: SsiLimb::OnLocus,
                ..
            }) => break,
            // The scan walks a displacement across ε, so the schedule's
            // own trilean legitimately lands in the escalation band on
            // the way past: that is limb 1 speaking, and the scan is
            // over.
            Err(SsiError::Escalated(ref diag)) if diag.predicate == Some("ssi_on_locus") => {
                break;
            }
            Err(e) => panic!("unexpected refusal while scanning: {e}"),
        }
    }
    let (d, value) = found.expect(
        "no displacement made the hull bound refuse while the sampled schedule still \
         passed — either the hull bound is not tighter than the schedule, or the scan \
         range is wrong; both are defects worth failing on",
    );
    assert!(
        value.is_nan() || value > eps(),
        "the hull bound must exceed ε: {value}"
    );
    // And the same displacement passes limb 1 on its own schedule.
    let bad = displaced(&carrier, n, d);
    let (t0, t1) = bad.domain();
    let c = threaded_cylinder();
    for i in 0..CERT_SAMPLES {
        let t = t0 + (t1 - t0) * (f64::from(i) / f64::from(CERT_SAMPLES - 1));
        let r = geom_brep::implicit_residual(&c, bad.eval(t)).abs();
        assert!(r <= eps(), "the schedule must NOT see it: sample {i} = {r}");
    }
}

// ---------------------------------------------------------------------
// Shape (iii): the NURBS wall, the ℝ⁴ trace, and OQ4
// ---------------------------------------------------------------------

/// A directly-authored NURBS wall (loft/sweep *definitions* are PR 10):
/// a bicubic × linear patch, curved in `x`–`y`, extruded in `z`. The
/// cutting plane meets it in a single open branch that runs wall-edge to
/// wall-edge.
fn nurbs_wall() -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    // Four control columns in u, two rows in v (height).
    // Gently curved: a wall whose curvature *swings* violently makes
    // ‖C⁗‖ far exceed κ³ and the step rule's fit budget (which assumes
    // slowly-varying curvature) then understates what the fit needs.
    // The acceptance shape wants a NURBS wall, not a pathological one.
    let cols = [(0.0, 0.0), (0.35, 0.18), (0.70, -0.12), (1.05, 0.04)];
    let mut control = Vec::with_capacity(8);
    for (x, y) in cols {
        control.push(Point3::new(x, y, 0.0));
        control.push(Point3::new(x, y, 0.8));
    }
    NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap()
}

/// A plane slicing the wall at mid height, tilted so the cut is not a
/// parameter line.
fn cutting_plane() -> Surface<f64> {
    let n = Vec3::new(0.0, 0.25, 1.0);
    let n = n / n.norm();
    // u_ref must be unit and ⊥ n.
    let u = Vec3::new(1.0, 0.0, 0.0);
    let u = (u - n * u.dot(n)) / (u - n * u.dot(n)).norm();
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.4),
        normal: n,
        u_ref: u,
    }
}

fn wall_domain() -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.5, 0.0, 0.4),
        half_extent: 2.0,
        extent: 1.5,
        eps: eps(),
        floor_scale: 1.0,
    }
}

#[test]
fn the_r4_trace_runs_and_its_certificate_refuses_at_the_named_limb() {
    // The plane×NURBS arm is NOT retired, and this row pins exactly
    // where it stops: the trace, the fit and the tube all run, and the
    // certificate refuses at limb 2 — the between-samples sup bound
    // against a NURBS operand, which needs a surface composite this PR
    // does not have. A refusal that names its own missing mechanism is
    // the C12.1 posture; silently shipping the carrier would be the
    // "sampled max pretending to be a bound" C2.2 forbids.
    let (p, w) = (cutting_plane(), nurbs_wall());
    let err = ssi::plane_nurbs_ssi(&p, &w, wall_domain(), band()).expect_err("not retired yet");
    match err {
        SsiError::CertificateLimb {
            limb: SsiLimb::HullSup,
            ..
        } => {}
        SsiError::Escalated(ref d) if d.predicate == Some("ssi_hull_sup_chart") => {}
        other => panic!("expected limb 2 to be the blocker, got {other}"),
    }
    // And the table says so where a caller reads it.
    let r = geom_brep::route(geom_brep::SurfaceKind::Plane, geom_brep::SurfaceKind::Nurbs);
    assert!(!r.implemented);
    assert!(r.note.contains("Bernstein composition"), "{}", r.note);
}

#[test]
fn oq4_the_two_pcurves_share_the_carriers_own_parameter() {
    // THE OQ4-discharge demonstration, and it does not depend on the
    // arm being retired: the ℝ⁴ trace yields the 3-D curve and both
    // pcurves as projections of ONE parameterized object, so
    // `S(P(t)) = C(t)` holds at the SAME t — the parameter-identity
    // contract PR 6 ratified for caches — by construction rather than
    // by coincidence. Checked on PR 6's own schedule (`CERT_SAMPLES`
    // over the carrier's interval), which is the statement
    // `PcurveCache::certify` makes.
    let (p, w) = (cutting_plane(), nurbs_wall());
    let (carrier, pa, pb) =
        ssi::trace_plane_nurbs_uncertified(&p, &w, (0.5, 0.5), wall_domain(), band())
            .expect("the ℝ⁴ trace");
    let (t0, t1) = carrier.domain();
    // Same parameter interval, not merely the same shape.
    assert!((pa.domain().0 - t0).abs() < 1e-15 && (pa.domain().1 - t1).abs() < 1e-15);
    assert!((pb.domain().0 - t0).abs() < 1e-15 && (pb.domain().1 - t1).abs() < 1e-15);
    let Surface::Plane {
        origin,
        normal,
        u_ref,
    } = p
    else {
        unreachable!()
    };
    let v_ref = normal.cross(u_ref);
    for i in 0..CERT_SAMPLES {
        let t = t0 + (t1 - t0) * (f64::from(i) / f64::from(CERT_SAMPLES - 1));
        let c = carrier.eval(t);
        // The wall chart, through the NURBS map.
        let q = pb.eval(t);
        let on_wall = (w.eval(q.x, q.y) - c).norm();
        assert!(on_wall <= eps(), "wall pcurve, sample {i}: {on_wall:e}");
        // The plane chart, through the affine map.
        let a = pa.eval(t);
        let on_plane = ((origin + u_ref * a.x + v_ref * a.y) - c).norm();
        assert!(on_plane <= eps(), "plane pcurve, sample {i}: {on_plane:e}");
    }
    // And the identity is not vacuous: the pcurves genuinely move.
    let a0 = pb.eval(t0);
    let a1 = pb.eval(t1);
    assert!(
        (a1.x - a0.x).abs() > 0.5,
        "the wall pcurve must span its chart"
    );
}

// ---------------------------------------------------------------------
// The closure trio and the differential pin
// ---------------------------------------------------------------------

#[test]
fn a_clipped_domain_ends_the_branch_on_the_boundary() {
    // The third closure trilean: `ssi_branch_open_end`. The slab is
    // shrunk so the loop cannot stay inside it, and the branch ends
    // open on the named domain instead of closing.
    let (s, c) = (sphere(), threaded_cylinder());
    let mut d = slab();
    d.center = Point3::new(0.06, 0.0, 0.996);
    d.half_extent = 0.05;
    d.extent = 0.2;
    let out = ssi::cylinder_sphere_ssi(&c, &s, d, band());
    match out {
        Ok(o) => {
            assert!(!o.branches.is_empty());
            assert!(
                o.branches.iter().all(|b| b.end != BranchEnd::Closed),
                "a clipped slab cannot contain a closed loop"
            );
        }
        // A slab that clips this tightly may also fail to prove itself
        // exhausted; that is a typed refusal and equally acceptable —
        // what must never happen is a silently-closed loop.
        Err(SsiError::ExhaustivenessInconclusive { .. }) => {}
        Err(e) => panic!("unexpected: {e}"),
    }
}

#[test]
fn idealized_and_realized_steppers_agree_on_the_locus_they_trace() {
    // The T4 / PERF-PLAN §4.4 differential pin. The two steppers place
    // different samples at different arc lengths by construction, so the
    // pin is on the LOCUS: same branch topology, and every idealized
    // sample within the realized branch's own certified band of the
    // realized carrier.
    let (s, c) = (sphere(), threaded_cylinder());
    let out = fixture();
    for b in out.branches.iter() {
        let Curve3::Nurbs(ref carrier) = b.carrier else {
            panic!("a rung-3 carrier is a NURBS curve");
        };
        let seed = b.witness;
        let (pts, end) =
            ssi::idealized_trace_r3(&c, &s, seed, slab(), band()).expect("the idealized trace");
        assert_eq!(end, b.end, "the two steppers must agree on branch topology");
        // The certified band the realized branch actually earned, plus
        // the tolerance itself for the idealized stepper's own Newton
        // residual.
        let tol = b.certificate.hull_sup + 2.0 * eps();
        for (i, p) in pts.iter().enumerate() {
            let pr = carrier.project(*p).expect("a foot on the realized carrier");
            assert!(
                pr.distance <= tol,
                "idealized sample {i} is {:e} m off the realized carrier (band {tol:e})",
                pr.distance
            );
        }
    }
}

#[test]
fn the_c5_table_retires_the_arm_whose_proof_is_complete() {
    use geom_brep::{Rung, SurfaceKind, route};
    // Retired: all three C2 limbs certify.
    for (a, b) in [
        (SurfaceKind::Cylinder, SurfaceKind::Sphere),
        (SurfaceKind::Sphere, SurfaceKind::Cylinder),
    ] {
        let r = route(a, b);
        assert_eq!(r.rung, Rung::General);
        assert!(r.implemented, "{a:?}×{b:?} should be retired by PR 7");
        assert!(r.note.contains("IMPLICIT PAIR"), "{}", r.note);
    }
    // Not retired, and the note says what is missing rather than
    // "unimplemented": C12.1 retires an arm WITH its proof.
    for (a, b) in [
        (SurfaceKind::Plane, SurfaceKind::Nurbs),
        (SurfaceKind::Nurbs, SurfaceKind::Plane),
    ] {
        let r = route(a, b);
        assert_eq!(r.rung, Rung::General);
        assert!(!r.implemented);
        assert!(r.note.contains("PARAMETRIC PAIR"), "{}", r.note);
        assert!(r.note.contains("Bernstein composition"), "{}", r.note);
    }
    // Everything else on the general rung still refuses, and now names
    // its trace shape rather than a bare "unimplemented".
    for (a, b) in [
        (SurfaceKind::Cylinder, SurfaceKind::Torus),
        (SurfaceKind::Nurbs, SurfaceKind::Nurbs),
        (SurfaceKind::Cone, SurfaceKind::Sphere),
    ] {
        let r = route(a, b);
        assert!(!r.implemented, "{a:?}×{b:?} must stay refused");
        assert!(
            r.note.contains("TRACE") || r.note.contains("trace shape"),
            "{a:?}×{b:?} must cite its trace shape: {}",
            r.note
        );
    }
}

#[test]
fn the_accounting_receipt_is_bounded_and_reported() {
    // The exhaustiveness contract is only useful if it is also
    // *affordable*: a subdivision that must refine to ε along the whole
    // locus is a hang, not a proof. What makes it terminate is the
    // certified tube radius — cells are accounted at the geometry's own
    // scale — so this row pins both the accounting numbers and the
    // radius they depend on, and would fail loudly if the tube ladder
    // ever started bottoming out.
    let out = fixture();
    let e = out.exhaustiveness;
    println!("exhaustiveness = {e:?}, seeds = {}", out.seeds);
    for b in out.branches.iter() {
        println!("certificate = {:?}", b.certificate);
    }
    assert!(e.examined > 0);
    assert_eq!(
        e.examined,
        e.excluded + e.accounted + e.refined,
        "the receipt must add up: every cell excluded, accounted, or split"
    );
    assert!(
        e.excluded > 0 && e.accounted > 0,
        "both terminal states must be exercised: {e:?}"
    );
    assert!(
        e.examined < 100_000,
        "the accounting should terminate at the tube's scale, not at ε: {e:?}"
    );
}
