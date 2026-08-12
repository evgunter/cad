//! **RW2 blinded-review probes for PR #353** (M8-3 half 1, the
//! `Pcurve::IsoArc` mint). Three attacks the shipped rows do not run:
//!
//! 1. round-trip volume BIT-identity (the PR table's claim is stronger
//!    than the shipped overlap assertion) plus a REORDERED-file rerun;
//! 2. dense residual sampling of every minted `IsoArc` against its own
//!    carrier and chart, concentrated at the sub-arc JOINS, checked
//!    against the cache's own certified numbers;
//! 3. a FOREIGN segmentation (4 uniform sub-arcs where the native form
//!    is 2) certified through the same public door the mint uses —
//!    carrier-keyed means the class accepts any chart that really is
//!    this circle's uniform rational-quadratic form.
//!
//! **Adopted at the fix pass as the class's NEGATIVE CONTROL (R1
//! MAJOR-1).** Before this file, skipping `run_iso_arc_checks`
//! entirely — early-returning a zero certificate — turned nothing red
//! anywhere in the workspace: every shipped row mints an
//! exact-by-construction chart whose checks pass trivially, so the
//! rows prove the class ACCEPTS the truth and never that it REFUSES a
//! lie. Re-applying that mutation on the fix head now fails probe 3's
//! negative control ("a wrong interior weight must refuse", answering
//! a fabricated `envelope: 0.0` certificate); reverted, green.
//!
//! Probe 2 alone does NOT kill that mutation and is not meant to: a
//! perfect mint's envelope is legitimately `0`, so a zero certificate
//! is not by itself evidence of anything, and probe 2's bound collapses
//! onto its own f64-rounding allowance. The lock is the REFUSAL side.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Vec3};
use profile::RawLoop;
use profile::{ProfileLoop, ProfileVertex};
use sweep::{Section, loft_body};

fn arc_section(s: f64) -> Section {
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex {
        pos: Point2::new(x, y),
        bulge,
    };
    vec![ProfileLoop::new(vec![
        v(-s, -s, 0.0),
        v(s, -s, 0.4142135623730951),
        v(s, s, 0.0),
        v(-s, s, 0.0),
    ])]
}

fn stack(z: [f64; 3]) -> Vec<Affine3<f64>> {
    z.map(|h| Affine3::translation(Vec3::new(0.0, 0.0, h)))
        .into()
}

fn native_arc_loft() -> topo::Body<f64> {
    loft_body::<f64>(
        &[arc_section(1.0), arc_section(1.25), arc_section(1.0)],
        &stack([0.0, 1.0, 2.0]),
        2,
    )
    .expect("the arc loft builds")
    .body
}

/// Probe 1: the PR's round-trip claim, at full strength (bit identity),
/// then again on a file whose DATA section is REVERSED — the reader's
/// fixed-point discipline should not depend on entity order.
///
/// **ε posture** (added at the fix pass; the probe as pushed was
/// ε-BLIND and went red on the hosted 1e-12 row). The schedule is
/// fixed (D9) against a `1024·ε` target, so at a tight enough ε the
/// arc loft's flux honestly cannot certify and there is no volume to
/// compare — the bit-identity claim has no SUBJECT there. That is not
/// a reason to widen anything: the row keeps the full-strength
/// assertion wherever the subject exists, and where it does not it
/// asserts the honest thing instead — that the reader refuses the SAME
/// way the builder does, which is the writer/reader symmetry this
/// probe is really about.
#[test]
fn probe_round_trip_bit_identity_and_reorder() {
    let native = native_arc_loft();
    let text =
        step_export::step_string(&native, &step_export::StepOptions::default()).expect("exports");
    let Ok(want) = topo::mass_properties(&native) else {
        // No subject for bit identity. Pin the symmetry that survives:
        // the native body refuses at the quadrature budget, and so must
        // the round trip — through the import gate, on the same fact.
        let native_err = topo::mass_properties(&native).expect_err("just refused");
        assert!(
            matches!(
                native_err,
                topo::MassPropsError::Face {
                    source: geom_brep::props::PropsError::QuadratureBudget { .. },
                    ..
                }
            ),
            "the only honest non-certifying posture is the fixed schedule's budget: \
             {native_err:?}"
        );
        let refusal = step_import::import_step(&text, &step_import::ImportOptions::default())
            .expect_err("the import gate refuses what the builder refuses");
        let shown = format!("{refusal:?}");
        assert!(
            shown.contains("QuadratureBudget"),
            "the reader must refuse on the SAME fact as the builder: {shown}"
        );
        println!(
            "RW2 probe 1: quadrature budget at eps={:e} — symmetry pinned, \
             bit identity has no subject",
            geom_core::Tolerance::get().eps
        );
        return;
    };

    let import = |t: &str, who: &str| -> topo::MassProperties<f64> {
        match step_import::import_step(t, &step_import::ImportOptions::default()) {
            Ok(step_import::StepImport::Solid { body, .. }) => {
                topo::mass_properties(&body).expect("imported body certifies")
            }
            other => panic!("{who}: expected a first-class Solid, got {other:?}"),
        }
    };

    let got = import(&text, "as-written");
    assert_eq!(
        got.volume.to_bits(),
        want.volume.to_bits(),
        "round-trip volume must be BIT-identical (the PR table's claim): \
         native {:?} vs imported {:?}",
        want.volume,
        got.volume,
    );

    // Reorder: reverse every `#k = ...;` statement in DATA. Multi-line
    // statements are glued to their opener first.
    let mut head = Vec::new();
    let mut data = Vec::new();
    let mut tail = Vec::new();
    let mut in_data = false;
    let mut done = false;
    for line in text.lines() {
        if line.trim() == "DATA;" {
            in_data = true;
            head.push(line.to_string());
            continue;
        }
        if in_data && line.trim() == "ENDSEC;" {
            in_data = false;
            done = true;
            tail.push(line.to_string());
            continue;
        }
        if in_data {
            if line.trim_start().starts_with('#') || data.is_empty() {
                data.push(line.to_string());
            } else {
                let last = data.last_mut().unwrap();
                last.push('\n');
                last.push_str(line);
            }
        } else if done {
            tail.push(line.to_string());
        } else {
            head.push(line.to_string());
        }
    }
    assert!(!data.is_empty(), "no DATA section parsed");
    data.reverse();
    let reordered = format!(
        "{}\n{}\n{}",
        head.join("\n"),
        data.join("\n"),
        tail.join("\n")
    );
    let got2 = import(&reordered, "reversed-DATA");
    assert_eq!(
        got2.volume.to_bits(),
        got.volume.to_bits(),
        "a reordered file must import to the same volume bits: {:?} vs {:?}",
        got2.volume,
        got.volume,
    );
    println!(
        "RW2 probe 1: bit-identical round trip AND reversed-DATA reimport, volume {}",
        got.volume
    );
}

/// Probe 2: every minted `IsoArc` on the arc prism and the arc loft,
/// sampled densely (1009 points, plus 41-point windows of width 1e-6
/// around every interior sub-arc JOIN): `|S(P(t)) − C(t)|` must be
/// within the cache's own certified statement (envelope + sampled max)
/// at every point.
#[test]
fn probe_dense_isoarc_residuals_at_joins() {
    let bodies = [
        (
            "arc prism",
            loft_body::<f64>(
                &[arc_section(1.0), arc_section(1.0), arc_section(1.0)],
                &stack([0.0, 1.0, 2.0]),
                2,
            )
            .expect("lofts")
            .body,
        ),
        ("arc loft", native_arc_loft()),
    ];
    let mut seen = 0usize;
    for (who, body) in &bodies {
        for (hek, cache) in body.pcurves() {
            let geom_brep::Pcurve::IsoArc {
                breaks,
                t0: at0,
                angle,
                ..
            } = cache.pcurve()
            else {
                continue;
            };
            seen += 1;
            let spans = breaks.control_count() - 1;
            let he = body.get_half_edge(hek).unwrap();
            let lp = body.get_loop(he.parent_loop).unwrap();
            let face = body.get_face(lp.face).unwrap();
            let surface = body.get_surface(face.surface).unwrap();
            let edge = body.get_edge(he.edge).unwrap();
            let curve = body
                .get_curve_geom(edge.curve)
                .unwrap()
                .certified()
                .expect("certified carrier");
            let (t0, t1) = cache.params();
            let cert = cache.certificate();
            // The certificate is an EXACT-arithmetic sup statement about
            // the stored f64 structure; sampling it through f64
            // evaluation adds a few ulps of rounding the interval lane
            // (not this probe) accounts for. Allow that, and nothing
            // more: 5e-15 m is ~50 ulps at these coordinates and six
            // decades below the 1e-9 band.
            let bound = cert.envelope + cert.max_residual + 5e-15;
            let mut worst = 0.0f64;
            let mut check = |t: f64| {
                let uv = cache.pcurve().eval(t);
                let s = surface.eval(uv.x, uv.y);
                let c = curve.carrier().eval(t);
                let d = (s - c).norm();
                if d > worst {
                    worst = d;
                }
                assert!(
                    d <= bound,
                    "{who}: residual {d:e} at t={t} exceeds certified envelope+max \
                     {bound:e} (envelope {:e}, sampled max {:e})",
                    cert.envelope,
                    cert.max_residual,
                );
            };
            for i in 0..=1008 {
                check(t0 + (t1 - t0) * (f64::from(i) / 1008.0));
            }
            // Join neighborhoods: τ = k/m ⇒ t = at0 + angle·k/m.
            #[allow(clippy::cast_precision_loss)]
            for k in 1..spans {
                let tj = at0 + angle * (k as f64) / (spans as f64);
                for j in -20i32..=20 {
                    let t = tj + f64::from(j) * 1e-6;
                    if t >= t0 && t <= t1 {
                        check(t);
                    }
                }
            }
            println!(
                "RW2 probe 2: {who} IsoArc ({spans} sub-arcs) worst residual {worst:e} \
                 vs certified {bound:e}"
            );
        }
    }
    assert!(seen >= 4, "expected at least 4 minted IsoArcs, saw {seen}");
}

/// Probe 3: the FOREIGN-segmentation attack on the carrier-keyed
/// claim. The native quarter-circle bulge mints 2 uniform sub-arcs;
/// here the SAME circle's chart column is authored as **4** uniform
/// sub-arcs (weights `1, cos(h/2), 1, …` with `h = θ/4`) and offered
/// to the same public `certify` door with the matching 4-span
/// `breaks`. Carrier-keyed means this certifies; description-keyed
/// (or native-form-keyed) would refuse a chart that is honestly this
/// circle's uniform rational-quadratic form.
#[test]
fn probe_foreign_segmentation_certifies_through_the_same_door() {
    use geom_brep::{ChartWindow, PcurveCache};
    use geom_core::Point3;

    let radius = 1.5f64;
    let theta = core::f64::consts::FRAC_PI_2; // the native bulge's sweep
    let spans = 4usize; // FOREIGN: native construction would use 2
    #[allow(clippy::cast_precision_loss)]
    let h = theta / spans as f64;
    let w = (h / 2.0).cos();

    let center = Point3::new(0.3, -0.2, 0.0);
    let axis = Vec3::unit_z();
    let u_ref = Vec3::unit_x();
    let vref = axis.cross(u_ref);
    let on =
        |a: f64, z: f64| center + u_ref * (radius * a.cos()) + vref * (radius * a.sin()) + axis * z;
    let tan_pt = |a: f64, z: f64| {
        let r = radius / w;
        center + u_ref * (r * a.cos()) + vref * (r * a.sin()) + axis * z
    };

    // The wall chart: u = the 4-sub-arc rational-quadratic parameter,
    // v = height 0..1 (degree 1) — exactly the loft-wall shape, only
    // the segmentation differs from the native mint's.
    let mut knots_u = vec![0.0, 0.0, 0.0];
    #[allow(clippy::cast_precision_loss)]
    for k in 1..spans {
        let t = k as f64 / spans as f64;
        knots_u.push(t);
        knots_u.push(t);
    }
    knots_u.extend([1.0, 1.0, 1.0]);
    let ku = geom_core::spline::KnotVector::clamped(knots_u, 2).unwrap();
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    // u-column descriptors: (angle-or-tangent, weight), then laid out
    // row-major over u then v (`iu·nv + iv`, the module's binding
    // layout) with nv = 2 height rows.
    let mut cols: Vec<(bool, f64, f64)> = vec![(false, 0.0, 1.0)]; // (is_tangent, angle, weight)
    #[allow(clippy::cast_precision_loss)]
    for k in 0..spans {
        let base = h * k as f64;
        cols.push((true, base + h / 2.0, w));
        cols.push((false, base + h, 1.0));
    }
    let mut ctrl: Vec<Point3<f64>> = Vec::new();
    let mut weights: Vec<f64> = Vec::new();
    for (is_tan, a, cw) in &cols {
        for z in [0.0f64, 1.0] {
            ctrl.push(if *is_tan { tan_pt(*a, z) } else { on(*a, z) });
            weights.push(*cw);
        }
    }
    let nurbs = geom_surfaces::NurbsSurface::new(ku, kv, ctrl, weights).unwrap();
    let surface = geom_surfaces::Surface::Nurbs(std::sync::Arc::new(nurbs));

    let carrier = geom_curves::Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    };
    let mut knots_b = vec![0.0, 0.0];
    #[allow(clippy::cast_precision_loss)]
    for k in 1..spans {
        knots_b.push(k as f64 / spans as f64);
    }
    knots_b.extend([1.0, 1.0]);
    let breaks = geom_core::spline::KnotVector::clamped(knots_b, 1).unwrap();
    let pcurve = geom_brep::Pcurve::IsoArc {
        p0: Point2::new(0.0, 0.0),
        pd: geom_core::Vec2::new(1.0, 0.0),
        t0: 0.0,
        angle: theta,
        breaks,
    };
    let window = ChartWindow {
        u_min: 0.0,
        u_max: 1.0,
        v_min: 0.0,
        v_max: 1.0,
    };
    let band = geom_core::Band::linear().unwrap();
    let cache = PcurveCache::certify(pcurve, 0.0, theta, &carrier, &surface, window, band)
        .expect("a FOREIGN 4-sub-arc chart certifies through the same door (carrier-keyed)");
    println!(
        "RW2 probe 3: foreign 4-sub-arc segmentation certified, envelope {:e}",
        cache.certificate().envelope
    );

    // Negative control: a chart whose interior weight is NOT cos(h/2)
    // must refuse typed (the decided margin), not certify.
    let bad_w = w * (1.0 + 1e-3);
    let mut ctrl2: Vec<Point3<f64>> = Vec::new();
    let mut weights2: Vec<f64> = Vec::new();
    let mut knots_u2 = vec![0.0, 0.0, 0.0];
    #[allow(clippy::cast_precision_loss)]
    for k in 1..spans {
        let t = k as f64 / spans as f64;
        knots_u2.push(t);
        knots_u2.push(t);
    }
    knots_u2.extend([1.0, 1.0, 1.0]);
    let ku2 = geom_core::spline::KnotVector::clamped(knots_u2, 2).unwrap();
    let kv2 = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    for (is_tan, a, cw) in &cols {
        for z in [0.0f64, 1.0] {
            ctrl2.push(if *is_tan { tan_pt(*a, z) } else { on(*a, z) });
            weights2.push(if *is_tan { bad_w * (cw / w) } else { *cw });
        }
    }
    let nurbs2 = geom_surfaces::NurbsSurface::new(ku2, kv2, ctrl2, weights2).unwrap();
    let surface2 = geom_surfaces::Surface::Nurbs(std::sync::Arc::new(nurbs2));
    let mut knots_b2 = vec![0.0, 0.0];
    #[allow(clippy::cast_precision_loss)]
    for k in 1..spans {
        knots_b2.push(k as f64 / spans as f64);
    }
    knots_b2.extend([1.0, 1.0]);
    let breaks2 = geom_core::spline::KnotVector::clamped(knots_b2, 1).unwrap();
    let pcurve2 = geom_brep::Pcurve::IsoArc {
        p0: Point2::new(0.0, 0.0),
        pd: geom_core::Vec2::new(1.0, 0.0),
        t0: 0.0,
        angle: theta,
        breaks: breaks2,
    };
    let refusal = PcurveCache::certify(pcurve2, 0.0, theta, &carrier, &surface2, window, band)
        .expect_err("a wrong interior weight must refuse");
    let text = format!("{refusal:?}");
    assert!(
        text.contains("cos(h/2)") || text.contains("Envelope") || text.contains("Residual"),
        "the refusal should be the decided weight margin or a residual, got: {text}"
    );
    println!("RW2 probe 3 negative control: wrong weight refused: {text}");
}
