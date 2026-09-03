//! BLEND-2 (#935) review probes — unique-signal attacks on PR #1268's
//! load-bearing claims, driven as an OUTSIDE CONSUMER through the
//! `pncad` facade (no `sweep` internals, no `test_support` selectors).
//!
//! Review-lane only; not part of the PR under review. The fixture is a
//! new one on purpose — a four-rim annular vase whose sharing chains
//! one wall further than anything the PR rows use.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::authoring::{p2, validated};
use pncad::geom::{Curve3, Surface};
use pncad::geom_brep::SurfaceKind;
use pncad::geom_core::{Point2, Tol, Vec2};
use pncad::prelude::{ArcSweep, BlendError, Center, Open, ProfileLoop, SketchPlane, Start};
use pncad::prelude::{fillet_edges, mass_properties, query, subtract, validate_geometric};
use pncad::sweep::{Revolution, RevolveAxis, revolve};
use pncad::topo::{Body, EdgeKey};

fn tol() -> Tol {
    Tol::witness()
}

/// A bored vase: base annulus, lower cylinder, conical waist, upper
/// cylinder, top annulus, bore. FOUR closed latitude rims, and every
/// consecutive pair shares the wall between them — a sharing chain one
/// link longer than the PR's lantern triple.
fn vase() -> Body<f64> {
    let t = tol();
    let meridian: ProfileLoop<f64> = Open
        .at(Point2::new(0.25, 0.0))
        .line_to(Point2::new(0.8, 0.0), t)
        .expect("base annulus")
        .line_to(Point2::new(1.0, 0.6), t)
        .expect("the flaring cone")
        .line_to(Point2::new(1.0, 1.2), t)
        .expect("the belly cylinder")
        .line_to(Point2::new(0.7, 1.8), t)
        .expect("the tapering cone")
        .line_to(Point2::new(0.25, 1.8), t)
        .expect("top annulus")
        .line_to(Start, t)
        .expect("the bore closes")
        .into();
    let profile =
        validated(SketchPlane::xy(), vec![meridian], t).expect("the vase's meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        t,
    )
    .expect("the meridian fully revolves")
    .body
}

/// A ball of radius `rad` centred at `(x, y, 0)`, authored as a
/// pole-touching revolve (its equator is two half-walls).
fn ball(x: f64, y: f64, rad: f64) -> Body<f64> {
    let t = tol();
    let meridian: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, -rad))
        .arc_to(
            Center {
                c: Point2::new(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Point2::new(0.0, rad),
            },
            t,
        )
        .expect("the half circle")
        .line_to(Start, t)
        .expect("the axis closes it")
        .into();
    let profile = validated(SketchPlane::xy(), vec![meridian], t).expect("ball meridian validates");
    let b = revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        t,
    )
    .expect("the ball revolves")
    .body;
    pncad::prelude::transform_rigid(
        &b,
        &pncad::geom_core::Affine3::translation(pncad::geom_core::Vec3::new(x, y, 0.0)),
        t,
    )
    .expect("the ball moves")
}

/// Every CLOSED edge of `body` whose carrier circle sits at latitude
/// `y` — selection by description: the kernel query seat materializes
/// the candidates; the closedness and the station/radius are this
/// probe's own reads (a numeric description no kind predicate
/// answers, the circle kind subsumed by the match).
fn rim_at(body: &Body<f64>, y: f64, rad: f64) -> Vec<EdgeKey> {
    query::all_edges(body)
        .into_iter()
        .filter(|&k| {
            let Some(e) = body.get_edge(k) else {
                return false;
            };
            let closed = body
                .get_half_edge(e.he_plus)
                .map(|h| Some(h.start) == body.half_edge_end(e.he_plus));
            if closed != Some(true) {
                return false;
            }
            let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
                return false;
            };
            matches!(*c.carrier(), Curve3::Circle { center, radius, .. }
                if (center.y - y).abs() < 1e-9 && (radius - rad).abs() < 1e-9)
        })
        .collect()
}

fn volume(body: &Body<f64>) -> f64 {
    let p = mass_properties(body, tol()).expect("mass properties");
    assert_eq!(p.volume_pad, 0.0, "closed-form faces only");
    p.volume
}

/// The multiset of a body's face shapes, said as sortable text — what
/// "the same closed-form faces under different arena keys" means if it
/// is true.
fn face_shapes(body: &Body<f64>) -> Vec<String> {
    let mut out: Vec<String> = body
        .faces()
        .map(|(_, f)| match body.get_surface(f.surface) {
            Some(Surface::Torus {
                major_radius,
                minor_radius,
                ..
            }) => format!("torus {major_radius:.17e} {minor_radius:.17e}"),
            Some(Surface::Sphere { radius, .. }) => format!("sphere {radius:.17e}"),
            Some(Surface::Cylinder { radius, .. }) => format!("cyl {radius:.17e}"),
            Some(Surface::Cone { half_angle, .. }) => format!("cone {half_angle:.17e}"),
            Some(Surface::Plane { .. }) => "plane".to_string(),
            other => format!("{:?}", other.map(SurfaceKind::of)),
        })
        .collect();
    out.sort();
    out
}

const ROLL: f64 = 0.05;

/// **P1 — the e2e: four chained shared-wall rims in ONE call, through
/// the public facade.** The PR's longest chain is three; this is four,
/// on a fixture with no relation to the zone, the lantern or the bud.
/// One call must build every band, stay tier-3 valid, and land on the
/// same volume as the sequential composition — checked in TWO
/// sequential orders (bottom-up and top-down), since the refresh runs
/// against a differently-carved body in each.
#[test]
fn p1_four_chained_rims_carve_in_one_call_through_the_facade() {
    let src = vase();
    let ys = [(0.0, 0.8), (0.6, 1.0), (1.2, 1.0), (1.8, 0.7)];
    let mut all: Vec<EdgeKey> = Vec::new();
    for (y, rad) in ys {
        let r = rim_at(&src, y, rad);
        assert_eq!(r.len(), 1, "one closed rim at y = {y}, got {}", r.len());
        all.extend(r);
    }
    let one = fillet_edges(&src, &all, ROLL, tol())
        .unwrap_or_else(|e| panic!("four chained rims in one call, got {e:?}"));
    assert_eq!(one.band_faces.len(), 4, "one band per rim");
    validate_geometric(&one.body, tol()).unwrap_or_else(|e| panic!("tier 3, got {e:?}"));

    let sequential = |order: &[(f64, f64)]| -> f64 {
        let mut b = vase();
        for &(y, rad) in order {
            let r = rim_at(&b, y, rad);
            assert_eq!(r.len(), 1, "one rim at y = {y} before its carve");
            b = fillet_edges(&b, &r, ROLL, tol())
                .unwrap_or_else(|e| panic!("the y = {y} rim fillets sequentially, got {e:?}"))
                .body;
        }
        validate_geometric(&b, tol()).expect("sequential tier 3");
        volume(&b)
    };
    let up = sequential(&ys);
    let mut down = ys;
    down.reverse();
    let down = sequential(&down);
    let v1 = volume(&one.body);
    assert!(v1 == up, "one call == bottom-up: {v1:.17e} vs {up:.17e}");
    assert!(v1 == down, "one call == top-down: {v1:.17e} vs {down:.17e}");
}

/// **P2 — the "same closed forms, different arena keys" mechanism the
/// bud demo's one-ulp pin rests on, tested directly.** If the mechanism
/// claim is true, the one-call and sequential bodies carry the same
/// multiset of face shapes and the same entity census; only the keys
/// (hence `mass_properties`' summation order) differ. Red would mean
/// the ulp has some other source.
#[test]
fn p2_one_call_and_sequential_carry_the_same_face_shapes() {
    let src = vase();
    let ys = [(0.0, 0.8), (0.6, 1.0), (1.2, 1.0), (1.8, 0.7)];
    let all: Vec<EdgeKey> = ys.iter().flat_map(|&(y, r)| rim_at(&src, y, r)).collect();
    let one = fillet_edges(&src, &all, ROLL, tol()).expect("one call");
    let mut b = vase();
    for &(y, rad) in &ys {
        b = fillet_edges(&b, &rim_at(&b, y, rad), ROLL, tol())
            .expect("sequential")
            .body;
    }
    assert_eq!(
        face_shapes(&one.body),
        face_shapes(&b),
        "the same closed-form faces either way"
    );
    let census = |body: &Body<f64>| {
        (
            body.vertices().count(),
            body.edges().count(),
            body.faces().count(),
        )
    };
    assert_eq!(census(&one.body), census(&b), "the same census either way");
}

/// **P3 — what the refusal a consumer reads at the request's boundary
/// actually says** — FLIPPED in the fix pass from the absence pin the
/// review filed (the reviewer measured that no refusal named the
/// sequential recourse; the fix made a CROSS-CHAIN clearance refusal
/// name and mean it — `FILLET3_CLEARANCE_SPLIT_RECOURSE`, executed
/// followably in `blend_tworims`). On THIS fixture the first refusal
/// up the radius scan is the spine fold, which is not splittable and
/// must NOT name the split — one call and the sequential composition
/// refuse identically, and the probe pins that agreement; any
/// cross-chain clearance refusal met on the way must carry the split
/// sentence.
#[test]
fn p3_the_boundary_refusal_names_the_split_exactly_when_it_is_splittable() {
    let src = vase();
    let ys = [(0.0, 0.8), (0.6, 1.0), (1.2, 1.0), (1.8, 0.7)];
    let all: Vec<EdgeKey> = ys.iter().flat_map(|&(y, r)| rim_at(&src, y, r)).collect();
    // Walk up until one call refuses.
    let mut found = None;
    let mut r = 0.05f64;
    while r < 0.45 {
        if let Err(e) = fillet_edges(&src, &all, r, tol()) {
            found = Some((r, e));
            break;
        }
        r += 0.005;
    }
    let (r, err) = found.expect("some radius refuses the one-call request");
    println!("   [blend2-r1] one call refuses at r = {r}: {err}");
    // The same radius, one rim at a time.
    let mut b = vase();
    let mut sequential_ok = true;
    for &(y, rad) in &ys {
        match fillet_edges(&b, &rim_at(&b, y, rad), r, tol()) {
            Ok(out) => b = out.body,
            Err(e) => {
                println!("   [blend2-r1] sequential also refuses at y = {y}: {e}");
                sequential_ok = false;
                break;
            }
        }
    }
    // MEASURED: the vase's binding boundary is the SPINE FOLD, a
    // per-rim fact no request split changes — both paths refuse.
    assert!(
        !sequential_ok,
        "the vase's first one-call refusal (r = {r}) is per-rim; sequential must \
         refuse it too, and a build here means the boundary moved — re-measure"
    );
    match &err.error {
        BlendError::FaceClearanceUncertified { cross_chain, .. } => {
            let text = format!("{err}");
            assert_eq!(
                *cross_chain,
                text.to_lowercase().contains("sequential"),
                "a clearance refusal names the split exactly when it is cross-chain: {text}"
            );
        }
        other => {
            let text = format!("{other}");
            assert!(
                !text.to_lowercase().contains("sequential"),
                "a non-splittable refusal must not borrow the split recourse: {text}"
            );
        }
    }
}

/// **P4 — the mixed (ladder + annulus) arm's unreachability, measured
/// independently.** The PR claims the only public construction of a
/// plane face carrying both a pip ring and a revolution-wall cycle is a
/// boolean of a ball against a revolve, and that the operand gate
/// refuses it first. This probe drives exactly that on the vase's top
/// annulus and records the refusal kind — so the fence's premise is
/// pinned rather than asserted.
#[test]
fn p4_no_public_door_builds_a_mixed_ladder_and_annulus_support() {
    let out = subtract(&vase(), &ball(0.45, 1.8, 0.08), tol());
    match out {
        Ok(_) => panic!(
            "a ball subtracted into a revolve's cap BUILT — the mixed ladder+annulus \
             support is reachable and the gate's mixed arm needs a row"
        ),
        Err(e) => println!("   [blend2-r1] ball into a revolve cap refuses: {e:?}"),
    }
}

/// A narrow-belly variant of the vase: the cylinder between the two
/// middle rims is only 0.12 m tall, so two bands on it set back into
/// each other well before either band is geometrically impossible.
fn pinched_vase() -> Body<f64> {
    let t = tol();
    let meridian: ProfileLoop<f64> = Open
        .at(Point2::new(0.25, 0.0))
        .line_to(Point2::new(1.0, 0.0), t)
        .expect("base annulus")
        .line_to(Point2::new(1.0, 1.0), t)
        .expect("the spool wall")
        .line_to(Point2::new(0.25, 1.0), t)
        .expect("top annulus")
        .line_to(Start, t)
        .expect("the bore closes")
        .into();
    let profile = validated(SketchPlane::xy(), vec![meridian], t).expect("pinched meridian");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        t,
    )
    .expect("the pinched meridian revolves")
    .body
}

/// **P5 — the spool measurement that RECHARACTERIZED the zone's
/// r = 0.749 gap.** On this plane×cylinder spool the screen's
/// straight-line gap IS the wall's own meridian, so one call and the
/// sequential composition refuse at the IDENTICAL radius (margin
/// −2.22e-16 at r ≈ 0.5): there is NO conservative gap here. The
/// zone's gap is therefore the clearance screen's pre-existing
/// chord-vs-arc direction-conservatism surfacing on a SPHERE wall — a
/// property of the fixture's geometry, not of one-call metering.
/// FLIPPED in the fix pass from the review's absence pin: the
/// cross-chain clearance refusal met here now names the split
/// recourse, and this row pins the presence (the followable execution
/// lives in `blend_tworims`, where the split really builds).
#[test]
fn p5_the_spool_refuses_identically_both_ways_and_names_the_split() {
    let src = pinched_vase();
    let pair = [(0.0, 1.0), (1.0, 1.0)];
    let both: Vec<EdgeKey> = pair.iter().flat_map(|&(y, r)| rim_at(&src, y, r)).collect();
    assert_eq!(both.len(), 2, "the two belly rims");
    let mut gap = None;
    let mut first_one: Option<(f64, String)> = None;
    let mut first_seq: Option<f64> = None;
    let mut r = 0.30f64;
    while r < 0.60 {
        let one = fillet_edges(&src, &both, r, tol());
        let mut b = pinched_vase();
        let mut seq_ok = true;
        for &(y, rad) in &pair {
            match fillet_edges(&b, &rim_at(&b, y, rad), r, tol()) {
                Ok(out) => b = out.body,
                Err(_) => {
                    seq_ok = false;
                    break;
                }
            }
        }
        if one.is_err() && first_one.is_none() {
            first_one = Some((r, format!("{}", one.as_ref().expect_err("err"))));
        }
        if !seq_ok && first_seq.is_none() {
            first_seq = Some(r);
        }
        if one.is_err() && seq_ok {
            gap = Some((r, one.expect_err("just checked")));
            break;
        }
        r += 0.001;
    }
    assert!(
        gap.is_none(),
        "the spool grew a one-call/sequential gap at {gap:?} — the screen's exactness \
         on parallel opposed features moved; re-measure the recharacterization"
    );
    let (one_r, one_text) = first_one.expect("the scan reaches the spool's boundary");
    let seq_r = first_seq.expect("sequential reaches the same boundary");
    assert!(
        one_r == seq_r,
        "one call and sequential refuse at the identical radius on the spool: \
         {one_r} vs {seq_r}"
    );
    assert!(
        one_text.to_lowercase().contains("sequential"),
        "the spool's cross-chain clearance refusal names the split recourse: {one_text}"
    );
    println!("   [blend2-r1] spool boundary identical both ways at r = {one_r}: {one_text}");
}
