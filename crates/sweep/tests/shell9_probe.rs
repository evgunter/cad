//! SHELL-9 probe rows: localize the void-insertion refusal of a cavity
//! whose chart carries a same-surface LATITUDE seam (collinear cap
//! plane, two-arc sphere). Every row prints what it measures; the
//! asserts pin the stage at which the pipeline's output first goes
//! wrong. Diagnosis only — no kernel change rides on these rows.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom::Surface;
use profile::{ProfileLoop, ProfileVertex, RawLoop};
use sweep::Revolution;
use topo::{Body, EdgeKey, HalfEdgeKey, VoidContainment, VoidEvidence};

use super::shell7_common::*;

fn collinear_cap_drum() -> Body<f64> {
    let (r, h) = (1.0, 2.0);
    polyline(
        &[(0.0, 0.0), (r, 0.0), (r, h), (r / 2.0, h), (0.0, h)],
        Revolution::Full,
    )
}

fn two_arc_sphere() -> Body<f64> {
    let r = 1.0;
    let v = PI / 4.0;
    let (s, c) = v.sin_cos();
    revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -r), ((FRAC_PI_2 + v) / 4.0).tan()),
            ProfileVertex::new(p2(r * c, r * s), ((FRAC_PI_2 - v) / 4.0).tan()),
            ProfileVertex::new(p2(0.0, r), 0.0),
        ]),
        Revolution::Full,
    )
}

/// The door's cavity, tier-3 valid (the fact SHELL-7 measured).
fn door_cavity(body: &Body<f64>, t: f64) -> Body<f64> {
    let mut cavity = body.clone();
    let band = geom_core::Band::linear(tol()).expect("band");
    topo::offset_charts_together(&mut cavity, &hollow_moves(body, t), band, tol())
        .expect("the door takes it");
    assert_eq!(
        topo::validate_geometric(&cavity, tol()),
        Ok(()),
        "cavity tier 3"
    );
    cavity
}

/// Re-certify every edge of `body` exactly as the graft does
/// (`combine.rs`'s recertify arm: description with the image verbatim,
/// carrier and params verbatim, endpoints from he_plus, surfaces from
/// the body itself). Returns the failures.
fn recertify_like_the_graft(
    label: &str,
    body: &Body<f64>,
) -> Vec<(EdgeKey, geom_brep::CertifyError)> {
    let band = geom_core::Band::linear(tol()).expect("band");
    let mut failures = Vec::new();
    for (ek, e) in body.edges() {
        let curve = body
            .get_curve_geom(e.curve)
            .and_then(|g| g.certified())
            .unwrap();
        let description = match *curve.description() {
            geom_brep::EdgeDescription::Intersection { s1, s2, witness } => {
                geom_brep::EdgeDescriptionSpec::Intersection { s1, s2, witness }
            }
            geom_brep::EdgeDescription::TangentIntersection { s1, s2, witness } => {
                geom_brep::EdgeDescriptionSpec::TangentIntersection { s1, s2, witness }
            }
            geom_brep::EdgeDescription::Chart(ref c) => geom_brep::EdgeDescriptionSpec::Chart {
                surface: c.surface,
                image: Some(c.pcurve.clone()),
                seam: c.seam,
                declared: match curve.authority() {
                    geom_brep::EdgeAuthority::Declared(mc) => Some(mc),
                    geom_brep::EdgeAuthority::Derived => None,
                },
            },
            geom_brep::EdgeDescription::Scaffold(_) => continue,
        };
        let start_v = body.get_half_edge(e.he_plus).unwrap().start;
        let end_v = body.half_edge_end(e.he_plus).unwrap();
        let spec = geom_brep::EdgeCurveSpec {
            description,
            carrier: curve.carrier().clone(),
            param_start: curve.params().0,
            param_end: curve.params().1,
        };
        let out = geom_brep::EdgeCurve::certify(
            spec,
            point(body, start_v),
            point(body, end_v),
            |sk| body.get_surface(sk).cloned(),
            band,
        );
        if let Err(err) = out {
            println!("[probe] {label}: edge {ek:?} refuses: {err:?}");
            failures.push((ek, err));
        }
    }
    failures
}

/// One edge, every quantity the graft's meter reads: carrier, params,
/// description, endpoints, and the sampled `|C(t) − S(P(t))|` on the
/// certification schedule against the body's own surface.
fn describe_edge(label: &str, body: &Body<f64>, ek: EdgeKey) {
    let e = body.get_edge(ek).unwrap();
    let curve = body
        .get_curve_geom(e.curve)
        .and_then(|g| g.certified())
        .unwrap();
    let (t0, t1) = curve.params();
    let start_v = body.get_half_edge(e.he_plus).unwrap().start;
    let end_v = body.half_edge_end(e.he_plus).unwrap();
    println!(
        "[probe] {label}: edge {ek:?} carrier={:?} params=({t0},{t1}) start={start_v:?}@{:?} end={end_v:?}@{:?}",
        curve.carrier(),
        point(body, start_v),
        point(body, end_v)
    );
    println!(
        "[probe] {label}: edge {ek:?} description={:?}",
        curve.description()
    );
    let fa = face_of_he(body, e.he_plus);
    let fb = face_of_he(body, e.he_minus);
    println!(
        "[probe] {label}: edge {ek:?} faces plus={fa:?}(sense {}) minus={fb:?}(sense {}) same_surface={}",
        body.get_face(fa).unwrap().sense,
        body.get_face(fb).unwrap().sense,
        same_surface(body, ek)
    );
    if let geom_brep::EdgeDescription::Chart(c) = curve.description() {
        let surface = body.get_surface(c.surface).unwrap();
        println!(
            "[probe] {label}: edge {ek:?} chart surface {:?} = {surface:?}",
            c.surface
        );
        for i in 0..9u32 {
            let t = t0 + (t1 - t0) * f64::from(i) / 8.0;
            let q = c.pcurve.eval(t);
            let p = curve.carrier().eval(t);
            let sp = surface.eval(q.x, q.y);
            println!(
                "[probe] {label}: edge {ek:?} sample {i}: t={t:.6} P(t)=({:.6},{:.6}) C(t)={:?} S(P(t))={:?} residual={:.6e}",
                q.x,
                q.y,
                p,
                sp,
                p.distance(sp)
            );
        }
    }
}

/// The pcurve rows of every half-edge on the loop of `he`, in `next`
/// order: the stored cache's image and its entry/exit chart points.
fn describe_loop(label: &str, body: &Body<f64>, he: HalfEdgeKey) {
    let cycle = body.loop_cycle(he).unwrap();
    let face = face_of_he(body, he);
    let f = body.get_face(face).unwrap();
    println!(
        "[probe] {label}: loop of {he:?}: face {face:?} sense={} surface {:?} = {:?}",
        f.sense,
        f.surface,
        body.get_surface(f.surface).unwrap()
    );
    let mut prev_exit: Option<geom_core::Point2<f64>> = None;
    let mut first_entry: Option<geom_core::Point2<f64>> = None;
    for h in cycle {
        let hd = body.get_half_edge(h).unwrap();
        let e = body.get_edge(hd.edge).unwrap();
        let plus = e.he_plus == h;
        let curve = body
            .get_curve_geom(e.curve)
            .and_then(|g| g.certified())
            .unwrap();
        let desc = match curve.description() {
            geom_brep::EdgeDescription::Chart(c) => {
                format!(
                    "Chart(surface {:?}, seam {}, pcurve {:?})",
                    c.surface, c.seam, c.pcurve
                )
            }
            other => format!("{other:?}"),
        };
        match body.pcurve(h) {
            Some(cache) => {
                let (t0, t1) = cache.params();
                let (entry_t, exit_t) = if plus { (t0, t1) } else { (t1, t0) };
                let (entry, exit) = (cache.pcurve().eval(entry_t), cache.pcurve().eval(exit_t));
                if let Some(pe) = prev_exit {
                    println!(
                        "[probe] {label}:     joint gap (entry - prev exit) = ({:.6}, {:.6}) at prev v={:.6} (cos v = {:.6})",
                        entry.x - pe.x,
                        entry.y - pe.y,
                        pe.y,
                        pe.y.cos()
                    );
                }
                if first_entry.is_none() {
                    first_entry = Some(entry);
                }
                prev_exit = Some(exit);
                println!(
                    "[probe] {label}:   he {h:?} edge {:?} plus={plus} start={:?}@{:?} cache pcurve={:?} params=({t0},{t1}) entry={:?} exit={:?} | desc {desc}",
                    hd.edge,
                    hd.start,
                    point(body, hd.start),
                    cache.pcurve(),
                    cache.pcurve().eval(entry_t),
                    cache.pcurve().eval(exit_t)
                );
            }
            None => println!(
                "[probe] {label}:   he {h:?} edge {:?} plus={plus} start={:?}@{:?} NO CACHE | desc {desc}",
                hd.edge,
                hd.start,
                point(body, hd.start)
            ),
        }
    }
    if let (Some(a), Some(b)) = (first_entry, prev_exit) {
        println!(
            "[probe] {label}:     closure gap (first entry - last exit) = ({:.6}, {:.6})",
            a.x - b.x,
            a.y - b.y
        );
    }
}

fn evidence_for(cavity: &Body<f64>) -> VoidEvidence {
    VoidEvidence {
        shells: cavity
            .shells()
            .map(|(k, _)| {
                (
                    k,
                    VoidContainment::Carried {
                        sign: geom_core::Sign::Positive,
                    },
                )
            })
            .collect(),
    }
}

fn plane_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    body.edges()
        .filter(|(_, e)| {
            let c = body
                .get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .unwrap();
            matches!(c.description(), geom_brep::EdgeDescription::Chart(c)
                if matches!(body.get_surface(c.surface), Some(Surface::Plane { .. })))
        })
        .map(|(k, _)| k)
        .collect()
}

/// **Drum, stage by stage.** The door's cavity is tier-3 valid; its
/// `revert()` is not — and not only by the expected `NegativeVolume`.
#[test]
fn drum_reverted_cavity_fails_recertification_on_the_plane_chart_circle() {
    let t = 0.05;
    let body = collinear_cap_drum();
    let cavity = door_cavity(&body, t);
    println!(
        "[probe] drum: cavity graft-style recert failures = {}",
        recertify_like_the_graft("drum cavity", &cavity).len()
    );
    let reverted = cavity.revert().expect("revert");
    let v = topo::validate_geometric(&reverted, tol());
    println!("[probe] drum: reverted tier 3 = {v:?}");
    let failures = recertify_like_the_graft("drum reverted", &reverted);
    for (ek, _) in &failures {
        describe_edge("drum cavity  ", &cavity, *ek);
        describe_edge("drum reverted", &reverted, *ek);
    }
    let on_plane = plane_edges(&reverted);
    let failing: Vec<EdgeKey> = failures.iter().map(|(k, _)| *k).collect();
    println!("[probe] drum: Chart-on-plane edges = {on_plane:?}; failing = {failing:?}");
    for ek in &on_plane {
        let e = reverted.get_edge(*ek).unwrap();
        let c = reverted
            .get_curve_geom(e.curve)
            .and_then(|g| g.certified())
            .unwrap();
        let geom_brep::EdgeDescription::Chart(cc) = c.description() else {
            unreachable!()
        };
        println!(
            "[probe] drum: plane edge {ek:?} carrier kind {} image {:?}",
            match c.carrier() {
                geom::Curve3::Circle { .. } => "circle",
                geom::Curve3::Line { .. } => "line",
                _ => "other",
            },
            cc.pcurve
        );
    }
    // The same insertion `shell` runs, by hand: refused at the graft's
    // re-certification, before any pcurve pass.
    let mut out = body.clone();
    let solids: Vec<_> = body.solids().map(|(k, _)| k).collect();
    let err = topo::insert_voids(&mut out, &solids, cavity, &evidence_for(&reverted), tol())
        .expect_err("measured: insert_voids refuses");
    println!("[probe] drum: insert_voids = {err}");
    assert!(
        matches!(err, topo::VoidInsertError::Recertify(_)),
        "got {err:?}"
    );
    // Measured: exactly the two latitude half-circles (the ONLY plane
    // chart images with a non-zero v channel) fail; the radial lines on
    // the u_ref axis are fixed by the mirror and pass.
    assert!(
        !failing.is_empty() && failing.iter().all(|k| on_plane.contains(k)),
        "the failing edges are Chart images on the reverted PLANE"
    );
    for k in &failing {
        let e = reverted.get_edge(*k).unwrap();
        let c = reverted
            .get_curve_geom(e.curve)
            .and_then(|g| g.certified())
            .unwrap();
        assert!(
            matches!(c.carrier(), geom::Curve3::Circle { .. }),
            "{k:?} is a latitude circle"
        );
    }
    assert_eq!(
        failing.len(),
        2,
        "the two half-circles of the latitude ring"
    );
}

/// **Sphere, stage by stage.** Revert, graft, the assembled body's
/// tier 3, and the loop the discontinuity is on.
#[test]
fn sphere_reverted_cavity_and_the_grafted_loop() {
    let t = 0.05;
    let body = two_arc_sphere();
    let cavity = door_cavity(&body, t);
    println!(
        "[probe] sphere: cavity graft-style recert failures = {}",
        recertify_like_the_graft("sphere cavity", &cavity).len()
    );
    let reverted = cavity.revert().expect("revert");
    let v = topo::validate_geometric(&reverted, tol());
    println!("[probe] sphere: reverted tier 3 = {v:?}");
    if let Err(errors) = &v {
        for f in errors {
            let s = format!("{f:?}");
            if let Some(i) = s.find("LoopDiscontinuity") {
                let key_txt = &s[i..];
                let he = reverted
                    .half_edges()
                    .map(|(k, _)| k)
                    .find(|k| key_txt.contains(&format!("{k:?}")))
                    .expect("the finding names a live half-edge");
                println!(
                    "[probe] sphere: reverted finding {s}; keys are the cavity's own, so the same loop on both:"
                );
                describe_loop("sphere cavity   (finding's loop)", &cavity, he);
                describe_loop("sphere reverted (finding's loop)", &reverted, he);
            }
        }
    }
    assert!(
        matches!(&v, Err(errors) if errors.iter().any(|f| format!("{f:?}").contains("LoopDiscontinuity"))),
        "measured: revert() alone breaks the stored pcurve loop, got {v:?}"
    );
    let failures = recertify_like_the_graft("sphere reverted", &reverted);
    assert!(
        failures.is_empty(),
        "measured: every sphere edge re-certifies on the reverted body"
    );
    println!(
        "[probe] sphere: reverted graft-style recert failures = {}",
        failures.len()
    );
    for (k, e) in reverted.edges() {
        let c = reverted
            .get_curve_geom(e.curve)
            .and_then(|g| g.certified())
            .unwrap();
        if matches!(c.description(), geom_brep::EdgeDescription::Chart(c) if !c.seam) {
            describe_edge("sphere cavity  ", &cavity, k);
            describe_edge("sphere reverted", &reverted, k);
        }
    }
    let mut out = body.clone();
    let solids: Vec<_> = body.solids().map(|(k, _)| k).collect();
    let evidence = evidence_for(&cavity);
    let inserted = topo::insert_voids(&mut out, &solids, cavity.clone(), &evidence, tol())
        .expect("measured: the sphere's graft is taken");
    let v = topo::validate_geometric(&out, tol());
    println!("[probe] sphere: grafted body tier 3 = {v:?}");
    let Err(errors) = v else {
        panic!("expected the grafted body to fail tier 3");
    };
    let mut seen = false;
    for f in &errors {
        let s = format!("{f:?}");
        if let Some(i) = s.find("LoopDiscontinuity") {
            seen = true;
            println!("[probe] sphere: finding {s}");
            // Recover the half-edge key by matching the debug form.
            let key_txt = &s[i..];
            let he = out
                .half_edges()
                .map(|(k, _)| k)
                .find(|k| key_txt.contains(&format!("{k:?}")))
                .expect("the finding names a live half-edge");
            describe_loop("sphere grafted", &out, he);
            // The same loop on the cavity and on its revert, via the graft map.
            let src_he = cavity
                .edges()
                .find_map(|(ek, e)| {
                    let dk = inserted.edge(ek)?;
                    let d = out.get_edge(dk)?;
                    // The graft's he_plus is the REVERTED he_plus, i.e. the cavity's he_minus.
                    if d.he_plus == he {
                        Some(e.he_minus)
                    } else if d.he_minus == he {
                        Some(e.he_plus)
                    } else {
                        None
                    }
                })
                .expect("the grafted half-edge has a cavity twin");
            describe_loop("sphere cavity  ", &cavity, src_he);
            describe_loop("sphere reverted", &reverted, src_he);
        }
    }
    assert!(
        seen,
        "measured: a LoopDiscontinuity on the grafted body, got {errors:?}"
    );
}

/// **Drum, the fix hypothesis isolated.** On the reverted body the
/// latitude circle's carrier and endpoints are right; only the plane
/// chart IMAGE is stale (the reverted plane's `v_ref = normal × u_ref`
/// flipped under it). Re-deriving the image from the carrier against
/// the reverted plane certifies; carrying it verbatim does not.
#[test]
fn drum_reverted_plane_circle_certifies_once_its_image_is_rederived() {
    let body = collinear_cap_drum();
    let cavity = door_cavity(&body, 0.05);
    let reverted = cavity.revert().expect("revert");
    let band = geom_core::Band::linear(tol()).expect("band");
    let mut circles = 0;
    for (ek, e) in reverted.edges() {
        let curve = reverted
            .get_curve_geom(e.curve)
            .and_then(|g| g.certified())
            .unwrap();
        let geom_brep::EdgeDescription::Chart(c) = curve.description() else {
            continue;
        };
        if !matches!(reverted.get_surface(c.surface), Some(Surface::Plane { .. }))
            || !matches!(curve.carrier(), geom::Curve3::Circle { .. })
        {
            continue;
        }
        circles += 1;
        let start_v = reverted.get_half_edge(e.he_plus).unwrap().start;
        let end_v = reverted.half_edge_end(e.he_plus).unwrap();
        let spec_with = |image: Option<geom_brep::Pcurve<f64>>| geom_brep::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::Chart {
                surface: c.surface,
                image,
                seam: c.seam,
                declared: None,
            },
            carrier: curve.carrier().clone(),
            param_start: curve.params().0,
            param_end: curve.params().1,
        };
        let certify = |spec| {
            geom_brep::EdgeCurve::certify(
                spec,
                point(&reverted, start_v),
                point(&reverted, end_v),
                |sk| reverted.get_surface(sk).cloned(),
                band,
            )
        };
        let verbatim = certify(spec_with(Some(c.pcurve.clone())));
        let rederived = certify(spec_with(None));
        println!(
            "[probe] drum {ek:?}: verbatim image -> {:?}; re-derived image -> {:?}",
            verbatim.as_ref().err(),
            rederived.as_ref().map(|c| c.description().clone())
        );
        assert!(verbatim.is_err(), "measured: the verbatim image refuses");
        let re = rederived.expect("measured: the re-derived image certifies on the reverted plane");
        let geom_brep::EdgeDescription::Chart(rc) = re.description() else {
            panic!("a chart image")
        };
        // The re-derived image is the stored one mirrored in v.
        for i in 0..9u32 {
            let t = curve.params().0 + (curve.params().1 - curve.params().0) * f64::from(i) / 8.0;
            let (a, b) = (c.pcurve.eval(t), rc.pcurve.eval(t));
            assert!(
                (a.x - b.x).abs() <= 1e-12 && (a.y + b.y).abs() <= 1e-12,
                "{ek:?} sample {i}: stored {a:?} vs re-derived {b:?}"
            );
        }
    }
    assert_eq!(circles, 2, "the latitude ring's two half-circles");
}

/// **Sphere, the fix hypothesis isolated.** The grafted body's only
/// tier-3 finding is the stale stored pcurve rows `revert` left
/// key-for-key and the graft copied verbatim; a final `mint_pcurves`
/// pass — which the boolean and the revolve run after their
/// `insert_void` and `shell` does not — re-derives them and the body
/// is tier-3 valid.
#[test]
fn sphere_grafted_body_is_tier_3_valid_after_a_pcurve_re_mint() {
    let body = two_arc_sphere();
    let cavity = door_cavity(&body, 0.05);
    let mut out = body.clone();
    let solids: Vec<_> = body.solids().map(|(k, _)| k).collect();
    let evidence = evidence_for(&cavity);
    topo::insert_voids(&mut out, &solids, cavity, &evidence, tol()).expect("the graft is taken");
    let before = topo::validate_geometric(&out, tol());
    println!("[probe] sphere: before re-mint tier 3 = {before:?}");
    assert!(before.is_err(), "measured: stale rows");
    topo::mint_pcurves(&mut out, tol()).expect("measured: the re-mint takes the grafted body");
    let after = topo::validate_geometric(&out, tol());
    println!("[probe] sphere: after re-mint tier 3 = {after:?}");
    assert_eq!(after, Ok(()), "measured: the re-mint restores tier 3");
    let props = topo::mass_properties(&out, tol()).expect("props");
    let want = 4.0 / 3.0 * PI * (1.0 - 0.95f64.powi(3));
    println!(
        "[probe] sphere: shell volume {} (pad {}), want {want}",
        props.volume, props.volume_pad
    );
    assert!((props.volume - want).abs() <= 1e-9 + props.volume_pad);
}
