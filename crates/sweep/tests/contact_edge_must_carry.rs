//! **A blend's contact edge takes its description from the must-carry
//! rule.**
//!
//! `geom_brep::must_carry_over_edge` is the one home of the rule that
//! decides what description a definitely-smooth join carries: the jet
//! certificate's lane gate, the certification schedule's interior
//! stations, each gated first-order before it is read second-order,
//! and a typed verdict. A blend's contact edge — the band's tangent
//! contact with its support, and the corner ball's with its band — is
//! such a join, and `attach_contact` routes its intrinsic arm through
//! the rule: jet-determinate stores `TangentIntersection`,
//! under-determined stores the conventional chart image, in-band
//! refuses typed at the blend door, and a transverse station refuses
//! `BlendError::SurgeryInvariant` (the smooth premise refuted).
//!
//! # The closed form, and where it collapses
//!
//! Along a contact locus the band is tangent to its support, so the
//! relative transverse normal curvature is `|1/r_band ∓ κ_support|`
//! (the difference where the two curve the same way, the sum where
//! they oppose) and the folded lever arm is `r_band` whenever the
//! support's curvature radius and the edge's extent exceed it; the
//! rule's margin is therefore `|1/r_band ∓ κ_support| · r_band² / 2`.
//! On a plane support that is `r_band / 2`; on the corner ball's arcs
//! the band's transverse curvature is zero and the ball's is
//! `1/r_band`, so it is `r_band / 2` again. The form collapses on the
//! DIFFERENCE branch only: a band osculating a support of its own
//! convexity, `κ_support → 1/r_band`.
//!
//! # What the door admits, and so what these rows can reach
//!
//! The rule's arm is `min(curvature arm of either surface, the edge's
//! extent)`. Where the curvature arm is `r_band` — every trimline and
//! rim arc — the battery's radius headroom bounds the margin from
//! below: on a plane support they are `r` and `r/2`; on the difference
//! branch, `(1 − r/R)·r` and half of it; on the sum branch the margin
//! is at least `r/2`. So against one band `(ε, K·ε)` the in-band
//! verdict is reachable on those edges exactly while the margin lies
//! in `(K·ε/2, K·ε)` — a blend a few `K·ε` in radius — and the
//! under-determined verdict (margin under ε) needs a headroom under
//! `2ε`, which `fillet3_radius_headroom` refuses first WHEN `K ≥ 2`:
//! the octave is one because the band's own width is; at any run with
//! `K < 2` the same rod reaches the under-determined verdict through
//! the door (`CAD_AMBIGUITY_K`, any finite `K > 1` is legal). And the
//! corner ball's arcs take the same path with a DIFFERENT arm: on a
//! slim wedge the arc's extent `r·θ` is shorter than `r_band`, the
//! margin is `≈ θ²·r/2`, and both non-determinate verdicts are reached
//! at an ordinary radius with no scaling — measured in
//! `review_contact_edge_must_carry_r1_probes`, which is the pin of the
//! under-determined arm; the corpus table's `r/2` for the corner arcs
//! holds because the die's wedges are square.
//!
//! The near-osculating family itself (`R → r` on a cylinder of the
//! band's own convexity) never reaches the rule: a ball that nearly
//! fills its support forces the crease's dihedral toward zero, and at
//! `r = 0.1` the clearance screen refuses it (definite negative) at
//! `R/r = 1.15` while admitting `1.16`; at the scale where that screen
//! would read in band the MILL refuses the body first
//! (`review_contact_edge_must_carry_r1_probes`), and a sphere support
//! taken toward osculation at ordinary scale refuses definitely too
//! (`review_contact_edge_must_carry_r2_probes`). The rows derive their
//! radii from the RESOLVED band, as `must_carry_rule.rs` does, because
//! CI gates three ε rows and a margin in band at one is definite at
//! the others.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fmt::Write as _;

use geom_brep::{
    CERT_SAMPLES, EdgeDescription, MustCarryVerdict, SurfaceKind, edge_extent,
    must_carry_over_edge, sample_param, tangent_certificate_lane, tangent_second_order,
};
use geom_core::{Band, Margin, MarginDiag, Real, Sign, Tol, Vec3};
use sweep::Revolution;
use sweep::blend::{
    BlendError, BlendRefusal, BlendSite, FILLET3_CONTACT_RECOURSE, Filleted, fillet_edges,
};
use sweep::test_support::{
    ROD_FILLET, ball_poled_z, bored_block_of_arcs, boss_of_arcs, circle_arcs_at_z, cube,
    disc_of_arcs, dome, lantern, one_edge_rim_at, pocket_of_arcs, realized, rim_arcs_at,
    rod_creases, rod_upper_crease, rod_with_flat, rod_with_flat_at, sphere_zone, waisted,
};
use topo::boolean::BooleanOp;
use topo::query::{self, SurfaceKindSet};
use topo::{Body, EdgeKey};

fn tol() -> Tol {
    Tol::witness()
}

/// The run's resolved linear band — the one the blend door classifies
/// against, so a row's derived radius is the one the kernel reads.
fn band() -> Band {
    Band::linear(tol()).expect("the run's linear band")
}

/// The closed form on the difference branch: a band of radius `r` on a
/// support of curvature radius `big_r` curving the band's own way.
fn difference_branch(r: f64, big_r: f64) -> f64 {
    (1.0 / r - 1.0 / big_r).abs() * r * r * 0.5
}

// ---------------------------------------------------------------
// Reading a body's contact edges through the rule.
// ---------------------------------------------------------------

/// One contact edge's reading: the pair, the lane, every interior
/// station's margin and verdict, and the rule's own answer.
struct ContactReading {
    kinds: (SurfaceKind, SurfaceKind),
    in_lane: bool,
    /// `(margin, tag)` per interior station, `1..CERT_SAMPLES-1`.
    stations: Vec<(f64, &'static str)>,
    verdict: MustCarryVerdict,
}

impl ContactReading {
    fn min_margin(&self) -> f64 {
        self.stations
            .iter()
            .map(|s| s.0)
            .fold(f64::INFINITY, f64::min)
    }

    fn tags(&self) -> String {
        self.stations.iter().map(|s| s.1).collect()
    }
}

/// Every edge storing `TangentIntersection`, re-read through the rule
/// and, station by station, through the metered predicate — the same
/// extent (`edge_extent` over the stored window) and the same band the
/// surgery hands the rule. The station walk is DIAGNOSTIC (the rule
/// returns a verdict, not its readings); the rule's own verdict is
/// what the rows assert, and the walk's margin is the rule's spelling,
/// `Margin::sagitta` over the jet and the folded arm.
fn contact_readings(body: &Body<f64>) -> Vec<ContactReading> {
    let band = band();
    let mut out = Vec::new();
    for (_, e) in body.edges() {
        let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
            continue;
        };
        let EdgeDescription::TangentIntersection { s1, s2, .. } = *c.description() else {
            continue;
        };
        let s1 = body.get_surface(s1).expect("a described edge's surface");
        let s2 = body.get_surface(s2).expect("a described edge's surface");
        let carrier = c.carrier();
        let (t0, t1) = c.params();
        let chord = carrier.eval(t0).distance(carrier.eval(t1));
        let extent = edge_extent(carrier, t0, t1, chord);
        let in_lane = tangent_certificate_lane(carrier, s1, s2);
        let stations = (1..CERT_SAMPLES - 1)
            .map(|i| {
                let t = sample_param(t0, t1, i);
                let reading =
                    tangent_second_order(s1, s2, carrier.eval(t), carrier.deriv(t), extent, band);
                let margin = Margin::sagitta(reading.jet.kappa_rel.abs(), reading.arm).value();
                let tag = match reading.verdict {
                    Ok(Sign::Positive) => "+",
                    Ok(Sign::Zero) => "0",
                    Ok(Sign::Negative) => "-",
                    Err(_) => "?",
                };
                (margin, tag)
            })
            .collect();
        let verdict = must_carry_over_edge(s1, s2, carrier, t0, t1, extent, band);
        out.push(ContactReading {
            kinds: (SurfaceKind::of(s1), SurfaceKind::of(s2)),
            in_lane,
            stations,
            verdict,
        });
    }
    out
}

/// How many edges of a body store the intrinsic tangency, at any
/// scalar.
fn intrinsic_edges<T: Real>(body: &Body<T>) -> usize {
    body.edges()
        .filter(|(_, e)| {
            matches!(
                body.get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .map(|c| c.description()),
                Some(EdgeDescription::TangentIntersection { .. })
            )
        })
        .count()
}

/// One fixture's readings summarised per support pair: the count, how
/// many are in lane, the station tags seen, the margin range and the
/// verdicts seen — the table a reviewer reads with `--nocapture`.
fn summary(name: &str, readings: &[ContactReading]) -> String {
    let mut s = format!("== {name}: {} contact edges ==\n", readings.len());
    let mut pairs: Vec<(SurfaceKind, SurfaceKind)> = readings.iter().map(|r| r.kinds).collect();
    pairs.sort_by_key(|p| (p.0.name(), p.1.name()));
    pairs.dedup();
    for pair in pairs {
        let rows: Vec<&ContactReading> = readings.iter().filter(|r| r.kinds == pair).collect();
        let lane = rows.iter().filter(|r| r.in_lane).count();
        let mut tags: Vec<String> = rows.iter().map(|r| r.tags()).collect();
        tags.sort();
        tags.dedup();
        let lo = rows
            .iter()
            .map(|r| r.min_margin())
            .fold(f64::INFINITY, f64::min);
        let hi = rows
            .iter()
            .flat_map(|r| r.stations.iter().map(|s| s.0))
            .fold(f64::NEG_INFINITY, f64::max);
        let mut verdicts: Vec<String> = rows.iter().map(|r| format!("{:?}", r.verdict)).collect();
        verdicts.sort();
        verdicts.dedup();
        let _ = writeln!(
            s,
            "  {}-{}: n={} lane={lane} stations={} margin=[{lo:.4e}, {hi:.4e}] verdict={}",
            pair.0.name(),
            pair.1.name(),
            rows.len(),
            tags.join("|"),
            verdicts.join("|")
        );
    }
    s
}

// ---------------------------------------------------------------
// The corpus.
// ---------------------------------------------------------------

/// **The carving corpus**: every fillet the bit-dump corpus carves, as
/// `(name, the carved body)` — the die, the ruled band, the pip rims,
/// the convex closed rims, the concave closed rims and the two-arc
/// rims, at the radii `bitdump.rs` carves them at.
fn corpus() -> Vec<(String, Body<f64>)> {
    let tol = tol();
    let carve = |name: &str, body: &Body<f64>, edges: &[EdgeKey], r: f64| {
        let out = fillet_edges(body, edges, r, tol)
            .unwrap_or_else(|e| panic!("{name} carves on the corpus: {e}"));
        (name.to_owned(), out.body)
    };
    let mut rows = Vec::new();
    let body = cube(1.0, tol);
    rows.push(carve("die r=0.15", &body, &query::all_edges(&body), 0.15));
    let body = rod_with_flat(tol);
    rows.push(carve(
        "ruled band r=0.1",
        &body,
        &rod_creases(&body),
        ROD_FILLET,
    ));
    // The pipped die, as `bitdump::pipped_die` builds it.
    let cube0 = cube(1.0, tol);
    let box_keys: Vec<EdgeKey> = cube0.edges().map(|(k, _)| k).collect();
    let pip = ball_poled_z(0.09, Vec3::new(0.5, 0.5, 1.0 + (0.09 - 0.05)), tol);
    let pipped = realized(BooleanOp::Subtract, &cube0, &pip, tol);
    let mut all: Vec<EdgeKey> = box_keys
        .into_iter()
        .filter(|k| pipped.get_edge(*k).is_some())
        .collect();
    all.extend(plane_sphere_rim(&pipped));
    rows.push(carve("pip rims r=0.05", &pipped, &all, 0.05));
    let r = 0.05;
    let body = dome(1.0, tol);
    rows.push(carve(
        "dome equator",
        &body,
        &[one_edge_rim_at(&body, 1.0, 0.0)],
        r,
    ));
    let body = sphere_zone(0.5, Revolution::Full, tol);
    let mut pair = rim_arcs_at(&body, 3.75f64.sqrt(), -0.5);
    pair.extend(rim_arcs_at(&body, 3f64.sqrt(), 1.0));
    rows.push(carve("sphere zone rim pair", &body, &pair, r));
    let body = lantern(tol);
    for (name, rim_r, rim_y) in [
        ("lantern neck", 1.0, 0.0),
        ("lantern shoulder", 0.8, 0.6),
        ("lantern lip", 0.2, 1.2),
    ] {
        rows.push(carve(name, &body, &rim_arcs_at(&body, rim_r, rim_y), r));
    }
    let body = waisted(tol);
    for (name, rim_y) in [("waist base", 0.0), ("waist top", 1.0)] {
        rows.push(carve(name, &body, &rim_arcs_at(&body, 1.0, rim_y), r));
    }
    let mut body = sweep::test_support::boss(true, tol);
    body.merge_coplanar_faces(tol)
        .expect("the pole-split caps repair");
    rows.push(carve(
        "boss base rim",
        &body,
        &rim_arcs_at(&body, 1.0, 0.0),
        r,
    ));
    let body = waisted(tol);
    rows.push(carve(
        "waist annulus (concave)",
        &body,
        &rim_arcs_at(&body, 0.5, 0.5),
        r,
    ));
    let ball = ball_poled_z(0.3, Vec3::new(0.5, 0.5, 1.0 - (0.3 - 0.1)), tol);
    let boss = realized(BooleanOp::Union, &cube(1.0, tol), &ball, tol);
    rows.push(carve(
        "boss ladder (concave) r=0.02",
        &boss,
        &plane_sphere_rim(&boss),
        0.02,
    ));
    for (name, body, z) in [
        ("two-arc disc", disc_of_arcs(2, 0.5, 1.0, tol), 1.0),
        (
            "two-arc bore",
            bored_block_of_arcs(2, 2.0, 1.0, 0.5, tol),
            1.0,
        ),
        (
            "two-arc boss",
            boss_of_arcs(2, 2.0, 0.5, 1.0, 2.0, tol),
            2.0,
        ),
        ("two-arc pocket", pocket_of_arcs(2, 2.0, 0.5, 1.5, tol), 1.5),
    ] {
        rows.push(carve(
            &format!("{name} r=0.1"),
            &body,
            &circle_arcs_at_z(&body, z),
            0.1,
        ));
    }
    rows
}

/// Every plane–sphere edge of a body — a pip's or a boss's rim.
fn plane_sphere_rim(body: &Body<f64>) -> Vec<EdgeKey> {
    query::all_edges(body)
        .into_iter()
        .filter(|&k| {
            query::edge_adjacent_matches(
                body,
                k,
                SurfaceKindSet::just(SurfaceKind::Plane),
                SurfaceKindSet::just(SurfaceKind::Sphere),
            )
        })
        .collect()
}

/// The corpus's contact edges, as the bit-dump corpus's rows carve them.
/// The count is the corpus's own structure, pinned so a fixture that
/// silently stops carving cannot empty the census.
const CORPUS_CONTACT_EDGES: usize = 158;

// ---------------------------------------------------------------
// The near-osculating family: a rod with a flat, at any radius.
// ---------------------------------------------------------------

/// A rod of radius `ratio · r`, its flat at `r`, ten radii long (the
/// cutter two rod radii wide), filleted at `r` on its `+y` crease —
/// `test_support::rod_with_flat_at`, the corpus fixture's own
/// construction at any radius. The flat at `x = r` rests the ball at
/// `(0, R − r)` and keeps the band a quarter turn at every ratio; one
/// crease, so the two bands cannot collide on the flat.
fn rod_family(ratio: f64, r: f64) -> Result<Filleted<f64>, BlendRefusal> {
    let body = rod_with_flat_at(ratio * r, r, 10.0 * r, 2.0 * ratio * r, tol())
        .expect("the family member mills");
    let crease = rod_upper_crease(&body);
    fillet_edges(&body, &[crease], r, tol())
}

/// The radius at which the rod family at `ratio` reads a second-order
/// margin of `frac · K·ε` — the closed form inverted.
fn radius_for_margin(ratio: f64, frac: f64) -> f64 {
    2.0 * frac * band().escalate() / (1.0 - 1.0 / ratio)
}

/// The typed in-band refusal, or a panic naming what came instead.
fn in_band_refusal(result: Result<Filleted<f64>, BlendRefusal>, what: &str) -> BlendError {
    match result {
        Ok(out) => panic!(
            "{what}: built, storing {} intrinsic tangencies, where the rule reads in band",
            intrinsic_edges(&out.body)
        ),
        Err(BlendRefusal { error, .. }) => {
            let BlendError::Escalated {
                site: BlendSite::Link { .. },
                source,
            } = &error
            else {
                panic!("{what}: refused, but not as the rule's in-band escalation: {error}");
            };
            assert_eq!(
                source.predicate,
                Some("tangent_second_order"),
                "{what}: the escalation names the metered predicate"
            );
            let MarginDiag::Value(m) = source.margin else {
                panic!("{what}: the deciding station's margin is a value, got {source:?}");
            };
            let b = band();
            assert!(
                m > b.zero() && m < b.escalate(),
                "{what}: the escalated margin {m:e} lies strictly inside ({:e}, {:e})",
                b.zero(),
                b.escalate()
            );
            let shown = error.to_string();
            assert!(
                shown.contains(FILLET3_CONTACT_RECOURSE),
                "{what}: the refusal names the radius as the lever.\n  got: {shown}"
            );
            error
        }
    }
}

// ---------------------------------------------------------------
// The rows.
// ---------------------------------------------------------------

/// **The corpus census**: every contact edge the bit-dump corpus's
/// rows carve is in lane, reads `Positive` at every interior station,
/// and is `JetDeterminate` — so the rule stores what the structural
/// mint stored and no description in the tree moves. The die's and the
/// rod's minima are the closed form exactly: `r/2` on a plane and on
/// the corner ball, `(1/r − 1/R)·r²/2` on the rod's cylinder.
#[test]
fn every_contact_edge_on_the_corpus_is_jet_determinate() {
    let mut total = 0;
    let mut text = String::new();
    for (name, body) in corpus() {
        let readings = contact_readings(&body);
        assert_eq!(
            readings.len(),
            intrinsic_edges(&body),
            "{name}: every intrinsic tangency was read"
        );
        for r in &readings {
            assert!(
                r.in_lane,
                "{name}: a contact edge is in the certificate's lane"
            );
            assert_eq!(r.tags(), "+".repeat(7), "{name}: every station is Positive");
            assert_eq!(
                r.verdict,
                MustCarryVerdict::JetDeterminate,
                "{name}: the rule demands the intrinsic description"
            );
        }
        let min_of = |pair: (SurfaceKind, SurfaceKind)| {
            readings
                .iter()
                .filter(|r| r.kinds == pair)
                .map(|r| r.min_margin())
                .fold(f64::INFINITY, f64::min)
        };
        match name.as_str() {
            "die r=0.15" => {
                assert_eq!(
                    readings.len(),
                    48,
                    "the die's 24 trimlines and 24 corner arcs"
                );
                for pair in [
                    (SurfaceKind::Plane, SurfaceKind::Cylinder),
                    (SurfaceKind::Cylinder, SurfaceKind::Sphere),
                ] {
                    assert!(
                        (min_of(pair) - 0.15 / 2.0).abs() < 1e-12,
                        "the die reads r/2"
                    );
                }
            }
            "ruled band r=0.1" => {
                assert_eq!(readings.len(), 4, "the rod's four trimlines");
                let plane = min_of((SurfaceKind::Plane, SurfaceKind::Cylinder));
                let cylinder = min_of((SurfaceKind::Cylinder, SurfaceKind::Cylinder));
                assert!(
                    (plane - ROD_FILLET / 2.0).abs() < 1e-12,
                    "the flat reads r/2"
                );
                assert!(
                    (cylinder - difference_branch(ROD_FILLET, 0.5)).abs() < 1e-12,
                    "the rod's cylinder reads the difference branch: {cylinder}"
                );
            }
            _ => {}
        }
        total += readings.len();
        text.push_str(&summary(&name, &readings));
    }
    println!("{text}");
    assert_eq!(
        total, CORPUS_CONTACT_EDGES,
        "the corpus's contact-edge count"
    );
}

/// **The definite side of the rod family at ordinary scale**: the
/// ratios the door admits — down to `1.16`, the last one the clearance
/// screen passes at `r = 0.1` — store the intrinsic tangency on the
/// cylinder-side trimline, and its minimum margin is the closed form.
#[test]
fn the_rod_family_the_door_admits_stores_the_intrinsic_description() {
    for ratio in [2.0, 1.5, 1.2, 1.16] {
        let out = rod_family(ratio, 0.1).unwrap_or_else(|e| panic!("R/r = {ratio} carves: {e}"));
        let readings = contact_readings(&out.body);
        assert_eq!(
            readings.len(),
            2,
            "R/r = {ratio}: the flat's and the cylinder's trimline"
        );
        let cylinder = readings
            .iter()
            .find(|r| r.kinds == (SurfaceKind::Cylinder, SurfaceKind::Cylinder))
            .expect("the cylinder-side trimline");
        assert_eq!(cylinder.verdict, MustCarryVerdict::JetDeterminate);
        assert!(
            (cylinder.min_margin() - difference_branch(0.1, ratio * 0.1)).abs() < 1e-12,
            "R/r = {ratio}: the closed form, got {}",
            cylinder.min_margin()
        );
    }
}

/// **The near-osculating family never reaches the rule**: below the
/// clearance screen's boundary the battery refuses first, with a
/// DEFINITE negative — the band's setback on the cylinder (the arc
/// from the crease over to the foot) exceeds the straight-line gap
/// between the two creases, which collapses as the ball fills the rod.
/// So the closed form's collapse (`R → r`) is screened out at every ε
/// at this scale — from `1.15` down, `1.16` being admitted — and the
/// in-band rows below are blends small against the band, not
/// osculations. At the scale where the screen itself would read in
/// band the mill refuses first (`review_contact_edge_must_carry_r1_probes`).
#[test]
fn the_near_osculating_family_is_screened_by_the_battery_before_the_rule() {
    for ratio in [1.15, 1.1, 1.01] {
        match rod_family(ratio, 0.1) {
            Err(BlendRefusal {
                error: BlendError::FaceClearanceUncertified { margin, .. },
                ..
            }) => assert_eq!(
                margin.sign,
                Sign::Negative,
                "R/r = {ratio}: the screen decides, it does not escalate"
            ),
            other => panic!("R/r = {ratio}: expected the clearance screen, got {other:?}"),
        }
    }
}

/// **A contact edge in the band's octave refuses typed at the door**,
/// with the predicate, the deciding station's margin strictly inside
/// the band, the requested link as the site and the radius named as the lever
/// — on a plane support (the corpus's own die, at a radius whose
/// `r/2` sits in the band) and on the difference branch (the rod
/// family at two admitted ratios, at radii whose `(1 − r/R)·r/2` sits
/// at either end of the octave the headroom gate leaves open).
#[test]
fn a_contact_in_the_bands_octave_refuses_typed_with_the_predicate_and_the_lever() {
    let b = band();
    let die = cube(1.0, tol());
    let r = 1.5 * b.escalate();
    in_band_refusal(
        fillet_edges(&die, &query::all_edges(&die), r, tol()),
        "the die at r = 1.5·Kε",
    );
    for ratio in [2.0, 1.5] {
        for frac in [0.55, 0.95] {
            let r = radius_for_margin(ratio, frac);
            in_band_refusal(
                rod_family(ratio, r),
                &format!("the rod at R/r = {ratio}, margin {frac}·Kε"),
            );
        }
    }
}

/// **The contact recourse is followable at each site kind it names.**
/// Plane support: the die refused at `r = 1.5·Kε` builds at `3·Kε`
/// (the margin grows with the radius), every contact edge intrinsic.
/// Difference branch: the rod refused at `R/r = 2` sits AT the closed
/// form's peak, so no radius on that rod leaves the band, and "blend a
/// larger feature" is the clause that works — the rod and its blend
/// scaled by two build; the past-the-peak direction is measured in
/// `review_contact_edge_must_carry_r2_probes::r2_the_recourse_names_the_peak_and_the_smaller_radius_past_it`.
/// Slim corner arc: the wedge refused in band at `r` builds at `r/10`,
/// its corner arcs stored as conventional chart images — the smaller
/// radius leaves the join under-determined, as the sentence says.
#[test]
fn the_contact_recourse_is_followable_at_each_site_kind() {
    let b = band();
    let die = cube(1.0, tol());
    in_band_refusal(
        fillet_edges(&die, &query::all_edges(&die), 1.5 * b.escalate(), tol()),
        "the die at r = 1.5·Kε",
    );
    let out = fillet_edges(&die, &query::all_edges(&die), 3.0 * b.escalate(), tol())
        .unwrap_or_else(|e| panic!("the die at r = 3·Kε builds: {e}"));
    let readings = contact_readings(&out.body);
    assert_eq!(readings.len(), 48, "the die's contact edges, all intrinsic");
    assert!(
        readings
            .iter()
            .all(|r| r.verdict == MustCarryVerdict::JetDeterminate),
        "the die at r = 3·Kε is jet-determinate on every contact edge"
    );

    let r = radius_for_margin(2.0, 0.75);
    in_band_refusal(rod_family(2.0, r), "the rod at R/r = 2, margin 0.75·Kε");
    let out = rod_family(2.0, 2.0 * r)
        .unwrap_or_else(|e| panic!("the rod at twice the scale builds: {e}"));
    let readings = contact_readings(&out.body);
    assert_eq!(readings.len(), 2, "both trimlines intrinsic");
    assert!(
        readings
            .iter()
            .all(|r| r.verdict == MustCarryVerdict::JetDeterminate),
        "the scaled rod is jet-determinate on both trimlines"
    );

    // The slim corner arc, on the fixture its pin lives on.
    use crate::review_contact_edge_must_carry_r1_probes::{
        chart_contact_edges, skewed_cavity_edges,
    };
    let scale = b.zero() / 1e-9;
    let r = 8.7e-5 * scale;
    let theta = (1.5 * b.escalate() / r).sqrt();
    let (body, edges) = skewed_cavity_edges(theta, scale);
    in_band_refusal(
        fillet_edges(&body, &edges, r, tol()),
        "the slim wedge at θ²·r/2 = 0.75·Kε",
    );
    let out = fillet_edges(&body, &edges, 0.1 * r, tol())
        .unwrap_or_else(|e| panic!("the wedge at a tenth of the radius builds: {e}"));
    assert!(
        chart_contact_edges(&out.body) >= 4,
        "the four corner arcs are stored as chart images, got {}",
        chart_contact_edges(&out.body)
    );
}

/// **What the rule costs a contact edge on the K stream**: the
/// description pass spends `CERT_SAMPLES − 2` samples of
/// `tangent_second_order` per contact edge in the rule, and the
/// certification door then re-asks the same question at the same
/// stations for the stored `TangentIntersection` — the rod's four
/// trimlines spend `4 × (7 + 7)`. Without the rule the count is the
/// certificate's half alone.
#[cfg(feature = "probe")]
#[test]
fn each_contact_edge_spends_the_rules_stations_once_beside_the_certificates() {
    use geom_core::k_stats::{self, Probe};
    use sweep::test_support::rod_d_profile_at;
    let interior = usize::try_from(CERT_SAMPLES - 2).expect("a small count");
    let body = rod_d_profile_at::<Probe>(tol());
    let creases = rod_creases(&body);
    assert_eq!(creases.len(), 2, "the D-rod's two creases");
    k_stats::start_recording();
    let out = fillet_edges(&body, &creases, Probe(ROD_FILLET), tol())
        .unwrap_or_else(|e| panic!("the D-rod's creases carve: {e}"));
    let spent = k_stats::take_samples()
        .iter()
        .filter(|s| s.predicate == "tangent_second_order")
        .count();
    let contact = intrinsic_edges(&out.body);
    assert_eq!(
        contact, 4,
        "the rod's four trimlines store the intrinsic tangency"
    );
    assert_eq!(
        spent,
        contact * 2 * interior,
        "each contact edge spends the schedule's interior once in the rule and once in the \
         certificate"
    );
}
