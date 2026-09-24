//! **A blend contact edge's description, re-read independently.**
//!
//! The rule `geom_brep::must_carry_over_edge` decides what description
//! a definitely-smooth join carries, and a blend's contact edge is
//! such a join. These rows re-derive the closed form
//! `|1/r_band ∓ κ_support|·r_band²/2` from the fixtures' own radii,
//! reach the in-band verdict at radii derived here, and read the
//! rendered recourse's LEVER against the direction the margin
//! actually moves on each branch.
//!
//! On a plane support the margin is `r/2`, monotone increasing in the
//! blend radius. On the DIFFERENCE branch — a band curving its
//! support's own way, curvature radius `R` — it is `(1 − r/R)·r/2`,
//! which peaks at `r = R/2` and decreases for every larger radius; the
//! ball also stops fitting at `r = (R + flat)/2`. So "enlarge the
//! radius" is the lever on one branch and not on the other, and a
//! sentence naming one direction is true at some of the sites the
//! predicate fires and false at others (README A3-2).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fmt::Write as _;

use geom_brep::{
    CERT_SAMPLES, EdgeDescription, MustCarryVerdict, SurfaceKind, edge_extent,
    must_carry_over_edge, sample_param, tangent_certificate_lane, tangent_second_order,
};
use geom_core::{Band, Margin, MarginDiag, Tol};
use sweep::blend::{
    BlendError, BlendRefusal, BlendSite, FILLET3_CONTACT_RECOURSE, Filleted, fillet_edges,
};
use sweep::test_support::{
    ROD_FILLET, cube, dome, one_edge_rim_at, rod_creases, rod_upper_crease, rod_with_flat,
    rod_with_flat_at,
};
use topo::Body;
use topo::query;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).expect("the run's linear band")
}

/// The second-order separation of a band of radius `r` against a
/// support whose transverse curvature radius is `big_r`, the two
/// curving the same way — re-derived here rather than imported.
fn difference(r: f64, big_r: f64) -> f64 {
    (1.0 / r - 1.0 / big_r).abs() * r * r * 0.5
}

/// One contact edge as the rule sees it: its support pair, whether the
/// certificate's lane admits it, the rule's verdict and the weakest
/// interior station's margin.
struct Contact {
    kinds: (SurfaceKind, SurfaceKind),
    in_lane: bool,
    verdict: MustCarryVerdict,
    min_margin: f64,
}

/// Every edge storing the intrinsic tangency, re-read through the rule
/// with the extent and band the surgery hands it.
fn contacts(body: &Body<f64>) -> Vec<Contact> {
    let band = band();
    let mut out = Vec::new();
    for (_, e) in body.edges() {
        let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
            continue;
        };
        let EdgeDescription::TangentIntersection { s1, s2, .. } = *c.description() else {
            continue;
        };
        let (s1, s2) = (
            body.get_surface(s1).expect("a described edge's surface"),
            body.get_surface(s2).expect("a described edge's surface"),
        );
        let carrier = c.carrier();
        let (t0, t1) = c.params();
        let chord = carrier.eval(t0).distance(carrier.eval(t1));
        let extent = edge_extent(carrier, t0, t1, chord);
        let min_margin = (1..CERT_SAMPLES - 1)
            .map(|i| {
                let t = sample_param(t0, t1, i);
                let reading =
                    tangent_second_order(s1, s2, carrier.eval(t), carrier.deriv(t), extent, band);
                Margin::sagitta(reading.jet.kappa_rel.abs(), reading.arm).value()
            })
            .fold(f64::INFINITY, f64::min);
        out.push(Contact {
            kinds: (SurfaceKind::of(s1), SurfaceKind::of(s2)),
            in_lane: tangent_certificate_lane(carrier, s1, s2),
            verdict: must_carry_over_edge(s1, s2, carrier, t0, t1, extent, band),
            min_margin,
        });
    }
    out
}

/// The weakest margin over the contact edges of one support pair.
fn min_of(rows: &[Contact], pair: (SurfaceKind, SurfaceKind)) -> f64 {
    rows.iter()
        .filter(|r| r.kinds == pair)
        .map(|r| r.min_margin)
        .fold(f64::INFINITY, f64::min)
}

/// The in-band escalation the rule raises, or a panic naming what came
/// instead. Returns `(the deciding station's margin, the rendered
/// refusal)`.
fn in_band(result: Result<Filleted<f64>, BlendRefusal>, what: &str) -> (f64, String) {
    match result {
        Ok(_) => panic!("{what}: built where the rule reads in band"),
        Err(BlendRefusal { error, .. }) => {
            let BlendError::Escalated {
                site: BlendSite::Link { .. },
                source,
            } = &error
            else {
                panic!("{what}: not the rule's in-band escalation: {error}");
            };
            assert_eq!(source.predicate, Some("tangent_second_order"), "{what}");
            let MarginDiag::Value(m) = source.margin else {
                panic!("{what}: the margin is not a value: {source:?}");
            };
            let b = band();
            assert!(
                m > b.zero() && m < b.escalate(),
                "{what}: the margin {m:e} is strictly inside the band"
            );
            let shown = error.to_string();
            assert!(
                shown.contains(FILLET3_CONTACT_RECOURSE),
                "{what}: the contact recourse is rendered.\n  got: {shown}"
            );
            (m, shown)
        }
    }
}

/// How one fillet attempt came out, in one line.
fn outcome(result: &Result<Filleted<f64>, BlendRefusal>) -> String {
    match result {
        Ok(out) => {
            let rows = contacts(&out.body);
            format!(
                "BUILT ({} contact edges, min margin {:.4e}, verdicts {})",
                rows.len(),
                rows.iter()
                    .map(|r| r.min_margin)
                    .fold(f64::INFINITY, f64::min),
                if rows
                    .iter()
                    .all(|r| r.verdict == MustCarryVerdict::JetDeterminate)
                {
                    "all JetDeterminate"
                } else {
                    "MIXED"
                }
            )
        }
        Err(BlendRefusal { error, .. }) => match error {
            BlendError::Escalated { source, .. } => format!(
                "ESCALATED {:?} margin {:?}",
                source.predicate, source.margin
            ),
            other => format!("REFUSED {}", refusal_kind(other)),
        },
    }
}

/// The refusal's variant name, without its prose.
fn refusal_kind(e: &BlendError) -> String {
    format!("{e:?}")
        .split_once(' ')
        .map_or_else(|| format!("{e:?}"), |(head, _)| head.to_owned())
        .trim_end_matches('{')
        .trim()
        .to_owned()
}

// ---------------------------------------------------------------
// C1: the census, re-taken on the die and the rod.
// ---------------------------------------------------------------

/// **The die's and the rod's contact edges, re-read independently.**
/// Every one is in the certificate's lane and jet-determinate, and
/// every weakest margin is the closed form written here rather than
/// imported: `r/2` on a plane support and on the corner ball (whose
/// band has zero transverse curvature against the ball's `1/r`), and
/// `(1/r − 1/R)·r²/2` on the rod's cylinder.
#[test]
fn r2_the_die_and_the_rod_census_retaken() {
    let die = cube(1.0, tol());
    let out = fillet_edges(&die, &query::all_edges(&die), 0.15, tol()).expect("the die carves");
    let rows = contacts(&out.body);
    assert_eq!(rows.len(), 48, "24 trimlines and 24 corner arcs");
    assert!(rows.iter().all(|r| r.in_lane));
    assert!(
        rows.iter()
            .all(|r| r.verdict == MustCarryVerdict::JetDeterminate)
    );
    for pair in [
        (SurfaceKind::Plane, SurfaceKind::Cylinder),
        (SurfaceKind::Cylinder, SurfaceKind::Sphere),
    ] {
        assert!(
            (min_of(&rows, pair) - 0.15 / 2.0).abs() < 1e-12,
            "{pair:?} reads r/2, got {:e}",
            min_of(&rows, pair)
        );
    }

    let body = rod_with_flat(tol());
    let out = fillet_edges(&body, &rod_creases(&body), ROD_FILLET, tol()).expect("the rod carves");
    let rows = contacts(&out.body);
    assert_eq!(rows.len(), 4, "the rod's four trimlines");
    assert!(
        rows.iter()
            .all(|r| r.in_lane && r.verdict == MustCarryVerdict::JetDeterminate)
    );
    let plane = min_of(&rows, (SurfaceKind::Plane, SurfaceKind::Cylinder));
    let cyl = min_of(&rows, (SurfaceKind::Cylinder, SurfaceKind::Cylinder));
    assert!(
        (plane - ROD_FILLET / 2.0).abs() < 1e-12,
        "the flat reads r/2"
    );
    assert!(
        (cyl - difference(ROD_FILLET, 0.5)).abs() < 1e-12,
        "the rod's cylinder reads the difference branch, got {cyl:e}"
    );
}

// ---------------------------------------------------------------
// C2: the in-band verdict, at radii derived here.
// ---------------------------------------------------------------

/// **The in-band verdict through the public door, at radii of this
/// file's own derivation.** On a plane support the margin is `r/2` and
/// the radius headroom is `r`, so the rule is in band with the
/// headroom definite exactly for `r ∈ (K·ε, 2·K·ε)`; on the difference
/// branch at ratio `R/r` both are `(1 − r/R)·r` and half of it. The
/// escalated margin is the closed form, not merely "inside the band".
///
/// The closed form is compared RELATIVELY and loosely: these radii are
/// `K·ε` against fixtures a metre across, so the band's own arithmetic
/// carries the ratio of the two scales into the reading (at `ε = 1e-12`
/// the die's margin departs from `r/2` in the sixth significant
/// figure). What the row pins is the identity, not the last bits.
#[test]
fn r2_the_in_band_verdict_at_radii_derived_here() {
    let b = band();
    let die = cube(1.0, tol());
    for mult in [1.2, 1.9] {
        let r = mult * b.escalate();
        let (m, _) = in_band(
            fillet_edges(&die, &query::all_edges(&die), r, tol()),
            &format!("the die at r = {mult}·Kε"),
        );
        assert!(
            (m - r / 2.0).abs() <= 1e-4 * r,
            "the die's escalated margin is r/2: {m:e} against {:e}",
            r / 2.0
        );
    }

    // The difference branch at a ratio and a fraction this file picks:
    // `r = 2·frac·K·ε/(1 − r/R)` inverts the closed form.
    let (ratio, frac) = (1.75, 0.7);
    let r = 2.0 * frac * b.escalate() / (1.0 - 1.0 / ratio);
    let body = rod_with_flat_at(ratio * r, r, 10.0 * r, 2.0 * ratio * r, tol())
        .expect("the family member mills");
    let crease = rod_upper_crease(&body);
    let (m, shown) = in_band(
        fillet_edges(&body, &[crease], r, tol()),
        "the rod at R/r = 1.75, margin 0.7·Kε",
    );
    assert!(
        (m - difference(r, ratio * r)).abs() <= 1e-4 * m,
        "the escalated margin is the difference branch: {m:e} against {:e}",
        difference(r, ratio * r)
    );
    println!("rod R/r = 1.75 refusal:\n  {shown}");
}

// ---------------------------------------------------------------
// C3: is the recourse's lever true at the site it fires?
// ---------------------------------------------------------------

/// **The contact recourse's direction, executed on the difference
/// branch.** A rod of radius `R = 1.5·r₀` with its flat at `r₀`
/// refuses in band at `r₀`. On this body the margin `(1 − r/R)·r/2` is
/// already past its peak (`r₀ = 2R/3 > R/2`), and the ball stops
/// fitting at `r = (R + flat)/2 = 1.25·r₀`, so EVERY radius the body
/// admits above `r₀` reads a SMALLER margin than `r₀` does and none
/// of them builds, while every smaller radius reads a LARGER one —
/// which is what the rendered sentence says of a site past the peak.
/// The peak itself (`R/8 = 0.84·Kε` here) is still in band, so the
/// clause that builds is the feature one: the same rod and blend
/// scaled by two builds jet-determinate.
///
/// The row asserts the direction, not the refusals' kinds: a sentence
/// pointing the other way would be one a caller cannot follow.
#[test]
fn r2_the_recourse_names_the_peak_and_the_smaller_radius_past_it() {
    let b = band();
    let (ratio, frac) = (1.5, 0.75);
    let r0 = 2.0 * frac * b.escalate() / (1.0 - 1.0 / ratio);
    let big_r = ratio * r0;
    let body = rod_with_flat_at(big_r, r0, 10.0 * r0, 2.0 * big_r, tol()).expect("the rod mills");
    let crease = rod_upper_crease(&body);
    let (m0, shown) = in_band(
        fillet_edges(&body, &[crease], r0, tol()),
        "the rod at R/r = 1.5, margin 0.75·Kε",
    );
    assert!(
        shown.contains(
            "smaller on one curving the band's own way, where the margin is past its peak"
        ),
        "the rendered sentence names the peak and the direction past it: {shown}"
    );
    // Each branch is scoped to the support that makes it true: the SUM
    // branch (a support curving away from the band) grows with the
    // radius and has no peak, and a slim corner arc is levered DOWN
    // under the tolerance, where it builds — it is never "raised".
    assert!(
        shown.contains("larger on a plane support or one curving away from the band")
            && shown.contains("slim corner arc, which then builds conventionally")
            && !shown.contains("on a curved one"),
        "every clause is true at the support it names: {shown}"
    );

    let mut table =
        format!("the same rod (R = {big_r:e}, flat = {r0:e}), refused at r₀ with margin {m0:e}\n");
    for mult in [1.05, 1.1, 1.2, 1.24] {
        let r = mult * r0;
        let predicted = difference(r, big_r);
        assert!(
            predicted < m0,
            "enlarging to {mult}·r₀ lowers the closed form: {predicted:e} against {m0:e}"
        );
        let result = fillet_edges(&body, &[crease], r, tol());
        let _ = writeln!(
            table,
            "  enlarge to {mult}·r₀ (closed form {predicted:.4e}): {}",
            outcome(&result)
        );
        assert!(
            result.is_err(),
            "no radius above r₀ on this rod escapes the band: {mult}·r₀ built"
        );
    }
    for mult in [0.95, 0.8, 0.6] {
        let r = mult * r0;
        assert!(
            difference(r, big_r) > m0,
            "reducing to {mult}·r₀ raises the closed form: {:e} against {m0:e}",
            difference(r, big_r)
        );
        let result = fillet_edges(&body, &[crease], r, tol());
        let _ = writeln!(
            table,
            "  reduce to {mult}·r₀ (closed form {:.4e}): {}",
            difference(r, big_r),
            outcome(&result)
        );
    }
    // The second clause: the whole feature twice as large.
    let scaled = rod_with_flat_at(2.0 * big_r, 2.0 * r0, 20.0 * r0, 4.0 * big_r, tol())
        .expect("the scaled rod mills");
    let crease = rod_upper_crease(&scaled);
    let out = fillet_edges(&scaled, &[crease], 2.0 * r0, tol())
        .unwrap_or_else(|e| panic!("blending a larger feature is followable: {e}"));
    let rows = contacts(&out.body);
    assert!(
        !rows.is_empty()
            && rows
                .iter()
                .all(|r| r.verdict == MustCarryVerdict::JetDeterminate),
        "the doubled rod is jet-determinate on every contact edge"
    );
    let _ = writeln!(
        table,
        "  blend a larger feature (×2): {}",
        outcome(&Ok(out))
    );
    println!("{table}");
}

// ---------------------------------------------------------------
// C2: a second support geometry, near-osculating without a scale trick.
// ---------------------------------------------------------------

/// **A sphere support, taken toward osculation at ordinary scale.**
/// The dome's equator rim is filleted against a sphere of radius 1 and
/// a plane; the sphere side is the difference branch, so the closed
/// form `(1/r − 1)·r²/2` collapses as `r → 1`. Every member the door
/// admits reads jet-determinate and the closed form; the rest refuse
/// definitely. So this family does not reach the rule either, and the
/// row says where the door's boundary is rather than asserting it
/// cannot be crossed.
#[test]
fn r2_a_sphere_support_toward_osculation_never_reaches_the_rule() {
    let body = dome(1.0, tol());
    let rim = one_edge_rim_at(&body, 1.0, 0.0);
    let mut table = String::from("dome (sphere support, R = 1), equator rim:\n");
    for r in [0.1, 0.25, 0.4, 0.5, 0.6, 0.75, 0.9, 0.95, 0.99] {
        let result = fillet_edges(&body, &[rim], r, tol());
        let _ = writeln!(
            table,
            "  r = {r}: closed form {:.4e} — {}",
            difference(r, 1.0),
            outcome(&result)
        );
        match &result {
            Ok(out) => assert!(
                contacts(&out.body)
                    .iter()
                    .all(|c| c.verdict == MustCarryVerdict::JetDeterminate),
                "r = {r} built, so the rule must have demanded the intrinsic description"
            ),
            Err(BlendRefusal { error, .. }) => assert!(
                !matches!(
                    error,
                    BlendError::Escalated {
                        source,
                        ..
                    } if source.predicate == Some("tangent_second_order")
                ),
                "r = {r} reached the rule in band at ordinary scale: {error}"
            ),
        }
    }
    println!("{table}");
}

// ---------------------------------------------------------------
// C4: the rule's K cost, counted per contact edge.
// ---------------------------------------------------------------

/// **The rule is spent once per contact edge, beside the
/// certificate's.** Filleting ONE crease of the D-rod yields two
/// contact edges and filleting both yields four, and the
/// `tangent_second_order` sample count is `2 × (CERT_SAMPLES − 2)` per
/// contact edge either way — the rule's interior stations plus the
/// certification door's re-ask. Counting at two edge counts is what
/// separates "per contact edge" from "per carve".
#[cfg(feature = "probe")]
#[test]
fn r2_the_rules_stations_scale_with_the_contact_edge_count() {
    use geom_core::k_stats::{self, Probe};
    use sweep::test_support::rod_d_profile_at;
    let interior = usize::try_from(CERT_SAMPLES - 2).expect("a small count");
    let body = rod_d_profile_at::<Probe>(tol());
    let creases = rod_creases(&body);
    assert_eq!(creases.len(), 2, "the D-rod's two creases");
    let mut seen = Vec::new();
    for request in [&creases[..1], &creases[..]] {
        k_stats::start_recording();
        let out = fillet_edges(&body, request, Probe(ROD_FILLET), tol())
            .unwrap_or_else(|e| panic!("{} crease(s) carve: {e}", request.len()));
        let spent = k_stats::take_samples()
            .iter()
            .filter(|s| s.predicate == "tangent_second_order")
            .count();
        let contact = out
            .body
            .edges()
            .filter(|(_, e)| {
                matches!(
                    out.body
                        .get_curve_geom(e.curve)
                        .and_then(|g| g.certified())
                        .map(|c| c.description()),
                    Some(EdgeDescription::TangentIntersection { .. })
                )
            })
            .count();
        assert_eq!(
            contact,
            2 * request.len(),
            "two contact edges per requested crease"
        );
        assert_eq!(
            spent,
            contact * 2 * interior,
            "{} crease(s): the rule's stations once per contact edge, beside the certificate's",
            request.len()
        );
        seen.push((request.len(), contact, spent));
    }
    println!("D-rod tangent_second_order samples (creases, contact edges, samples): {seen:?}");
    assert_eq!(seen[1].2, 2 * seen[0].2, "the count is per contact edge");
}

// ---------------------------------------------------------------
// C5: nothing else in the blend mints the intrinsic tangency.
// ---------------------------------------------------------------

/// **The corner ball's arcs take the same path as the trimlines.** A
/// die's contact edges are 24 straight trimlines against plane and
/// cylinder supports and 24 corner arcs against the ball, and every
/// one of them carries the intrinsic tangency the rule demanded — so
/// the corner arcs are not minted on a structural flag beside the
/// rule. The pair census is the evidence: an arm that minted its own
/// would still show as a `TangentIntersection`, but it would not read
/// `JetDeterminate` through the rule with the corner ball's own closed
/// form `r/2`.
#[test]
fn r2_the_corner_balls_arcs_carry_the_rules_verdict() {
    let die = cube(1.0, tol());
    let out = fillet_edges(&die, &query::all_edges(&die), 0.15, tol()).expect("the die carves");
    let rows = contacts(&out.body);
    let arcs: Vec<&Contact> = rows
        .iter()
        .filter(|r| r.kinds == (SurfaceKind::Cylinder, SurfaceKind::Sphere))
        .collect();
    assert_eq!(
        arcs.len(),
        24,
        "the die's twelve corner balls, two arcs each"
    );
    assert!(
        arcs.iter()
            .all(|a| a.in_lane && a.verdict == MustCarryVerdict::JetDeterminate),
        "every corner arc is in lane and jet-determinate"
    );
    assert!(
        arcs.iter().all(|a| (a.min_margin - 0.075).abs() < 1e-12),
        "the corner ball's closed form is r/2"
    );
    // Every intrinsic tangency in the carved die is a contact edge the
    // rule answered for: the reader above walks the same set.
    assert_eq!(rows.len(), 48);
}
