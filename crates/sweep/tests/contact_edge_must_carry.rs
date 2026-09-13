//! **A blend's contact edge takes its description from the must-carry
//! rule.**
//!
//! `geom_brep::must_carry_over_edge` is the one home of the rule that
//! decides what description a definitely-smooth join carries: the jet
//! certificate's lane gate, the certification schedule's interior
//! stations, a three-way typed answer. A blend's contact edge — the
//! band's tangent contact with its support, and the corner ball's with
//! its band — is such a join, and `attach_contact` routes its
//! intrinsic arm through the rule: jet-determinate stores
//! `TangentIntersection`, under-determined stores the conventional
//! chart image, in-band refuses typed at the blend door.
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
//! convexity, `κ_support → 1/r_band` — a convex band on a convex
//! cylinder (a rod with a flat, `R → r_band`), or a concave band in a
//! concave cylinder (a bore with a flat).
//!
//! # Why the in-band verdict is reachable through the door at all
//!
//! The battery's radius headroom on a cylinder support of radius `R`
//! is `(1 − r/R)·r`, and the second-order margin on the difference
//! branch is `(1 − r/R)·r/2` — exactly half of it. Against one band
//! `(ε, K·ε)` the rule is in band while the headroom is definite
//! exactly when the margin lies in `(K·ε/2, K·ε)`, a window of one
//! octave; below it the headroom gate refuses first, so the
//! under-determined verdict (margin under ε) is not reachable on this
//! branch through the door. The rows derive the osculation from the
//! RESOLVED band, as `must_carry_rule.rs` does, because CI gates three
//! ε rows and a margin in band at one is definite at the others.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fmt::Write as _;

use geom_brep::{
    CERT_SAMPLES, EdgeDescription, MustCarryVerdict, SurfaceKind, edge_extent,
    must_carry_over_edge, sample_param, tangent_certificate_lane, tangent_second_order,
};
use geom_core::{Affine3, Band, Point2, Sign, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::Revolution;
use sweep::blend::{BlendError, BlendRefusal, fillet_edges};
use sweep::test_support::{
    ROD_FILLET, ball_poled_z, bored_block_of_arcs, boss_of_arcs, circle_arcs_at_z, cube,
    disc_of_arcs, dome, lantern, one_edge_rim_at, pocket_of_arcs, realized, rim_arcs_at,
    rod_creases, rod_with_flat, sphere_zone, waisted,
};
use sweep::{Extrusion, extrude};
use topo::boolean::BooleanOp;
use topo::query;
use topo::query::SurfaceKindSet;
use topo::{Body, EdgeKey};

fn tol() -> Tol {
    Tol::witness()
}

/// The run's resolved linear band — the one the blend door classifies
/// against, so a row's derived osculation is the one the kernel reads.
fn band() -> Band {
    Band::linear(tol()).expect("the run's linear band")
}

// ---------------------------------------------------------------
// Reading a body's contact edges through the rule.
// ---------------------------------------------------------------

/// One contact edge's reading: the pair, the lane, every interior
/// station's margin and verdict, and the rule's own answer.
struct ContactReading {
    edge: EdgeKey,
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
}

/// Every edge storing `TangentIntersection`, re-read through the rule
/// and, station by station, through the metered predicate.
fn contact_readings(body: &Body<f64>) -> Vec<ContactReading> {
    let band = band();
    let mut out = Vec::new();
    for (edge, e) in body.edges() {
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
                let margin = reading.jet.kappa_rel.abs() * reading.arm * reading.arm * 0.5;
                let tag = match reading.verdict {
                    Ok(Sign::Positive) => "+",
                    Ok(Sign::Zero) => "0",
                    Ok(Sign::Negative) => "-",
                    Err(_) => "band",
                };
                (margin, tag)
            })
            .collect();
        let verdict = must_carry_over_edge(s1, s2, carrier, t0, t1, extent, band);
        out.push(ContactReading {
            edge,
            kinds: (SurfaceKind::of(s1), SurfaceKind::of(s2)),
            in_lane,
            stations,
            verdict,
        });
    }
    out
}

/// One fixture's readings summarised per support pair: the count, how
/// many are in lane, the station tags seen, the margin range and the
/// verdicts seen.
fn summary(name: &str, readings: &[ContactReading]) -> String {
    let mut s = format!("== {name}: {} contact edges ==\n", readings.len());
    let mut pairs: Vec<(SurfaceKind, SurfaceKind)> = readings.iter().map(|r| r.kinds).collect();
    pairs.sort_by_key(|p| (p.0.name(), p.1.name()));
    pairs.dedup();
    for pair in pairs {
        let rows: Vec<&ContactReading> = readings.iter().filter(|r| r.kinds == pair).collect();
        let lane = rows.iter().filter(|r| r.in_lane).count();
        let mut tags: Vec<String> = rows
            .iter()
            .map(|r| r.stations.iter().map(|s| s.1).collect::<Vec<_>>().join(""))
            .collect();
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

/// One fixture's readings as a table, one line per edge.
fn table(name: &str, readings: &[ContactReading]) -> String {
    let mut s = format!("== {name}: {} contact edges ==\n", readings.len());
    for r in readings {
        let tags: Vec<&str> = r.stations.iter().map(|s| s.1).collect();
        let _ = writeln!(
            s,
            "  {:?} {}-{} lane={} stations={} min={:.3e} verdict={:?}",
            r.edge,
            r.kinds.0.name(),
            r.kinds.1.name(),
            r.in_lane,
            tags.join(""),
            r.min_margin(),
            r.verdict
        );
    }
    s
}

// ---------------------------------------------------------------
// The near-osculating family.
// ---------------------------------------------------------------

/// **A rod with a flat**, parametrised: a cylinder of radius `big_r`
/// about `z` over `z ∈ [0, len]`, minus a box whose face at `x = flat`
/// planes the flat — `rod_with_flat`'s construction at any radius,
/// through the public boolean door, which keeps the cylinder's stored
/// radius exactly `big_r` (a D-profile through the extrude door
/// reconstructs the radius from a chord that collapses as the flat
/// nears tangency).
fn rod_with_flat_at(big_r: f64, flat: f64, len: f64) -> Result<Body<f64>, String> {
    let disc = profile::circle(Point2::new(0.0, 0.0), big_r, tol()).expect("a disc");
    let rod = Profile::new(SketchPlane::xy(), vec![disc.into()])
        .validate(tol())
        .expect("the rod's profile validates");
    let rod = extrude(&rod, Extrusion::Distance(len), tol())
        .expect("the rod extrudes")
        .body;
    let square = ProfileLoop::new(
        [
            (flat, -2.0 * big_r),
            (2.0 * big_r, -2.0 * big_r),
            (2.0 * big_r, 2.0 * big_r),
            (flat, 2.0 * big_r),
        ]
        .into_iter()
        .map(|(x, y)| ProfileVertex::new(Point2::new(x, y), 0.0))
        .collect(),
    );
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, -0.5)));
    let cutter = Profile::new(plane, vec![square])
        .validate(tol())
        .expect("the cutter's profile validates");
    let cutter = extrude(&cutter, Extrusion::Distance(len + 1.0), tol())
        .expect("the cutter extrudes")
        .body;
    Ok(topo::subtract(&rod, &cutter, tol())
        .map_err(|e| format!("the mill refuses: {e:?}"))?
        .body()
        .expect("a body remains")
        .body
        .clone())
}

/// **A block with a D-shaped bore**: the block `[−1, 1]² × [0, len]`
/// with a D-shaped hole of radius `big_r` and flat at `x = flat`
/// through it along `z`, so the bore's cylinder wall and its flat wall
/// meet at a CONCAVE crease along the ruling — the material-adding
/// twin of the rod's crease, on the same difference branch. Through the
/// extrude door as one profile with a ring (the boolean door refuses a
/// D-shaped through-cut with `JoinDesync`), so the cylinder's stored
/// radius is RECONSTRUCTED from the D's chord and bulge — a chord that
/// collapses as the flat nears tangency, which bounds how close to
/// osculation this fixture can be built honestly.
fn block_with_d_bore(big_r: f64, flat: f64, len: f64) -> Result<Body<f64>, String> {
    let block = ProfileLoop::new(
        [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(Point2::new(x, y), 0.0))
            .collect(),
    );
    let y = (big_r * big_r - flat * flat).sqrt();
    let theta = 2.0 * (core::f64::consts::PI - y.atan2(flat));
    let bulge = (theta / 4.0).tan();
    let hole = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(flat, y), bulge),
        ProfileVertex::new(Point2::new(flat, -y), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![block, hole])
        .validate(tol())
        .map_err(|e| format!("the profile refuses: {e:?}"))?;
    Ok(extrude(&profile, Extrusion::Distance(len), tol())
        .map_err(|e| format!("the extrude refuses: {e:?}"))?
        .body)
}

/// The crease whose band is asked for: the one straight cylinder–plane
/// edge on the `+y` side (the ball rests at `(0, ±(R − r))`; the
/// request is one crease so the two bands cannot collide on the flat).
fn upper_crease(body: &Body<f64>) -> EdgeKey {
    let creases: Vec<EdgeKey> = rod_creases(body)
        .into_iter()
        .filter(|&k| {
            let e = body.get_edge(k).unwrap();
            let v = body.get_half_edge(e.he_plus).unwrap().start;
            let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            p.y > 0.0
        })
        .collect();
    assert_eq!(creases.len(), 1, "one crease on the +y side: {creases:?}");
    creases[0]
}

/// The door's outcome on one member of the family, as prose.
fn door_outcome(result: &Result<sweep::blend::Filleted<f64>, BlendRefusal>) -> String {
    match result {
        Ok(_) => "admitted".to_owned(),
        Err(BlendRefusal {
            error: BlendError::Escalated { site, source },
            ..
        }) => format!(
            "Escalated at {site}: predicate={:?} margin={:?} band=({:e}, {:e})",
            source.predicate,
            source.margin,
            source.band.zero(),
            source.band.escalate()
        ),
        Err(BlendRefusal { error, .. }) => format!("refused: {error}"),
    }
}

/// Walks one family — a body builder at `(R, flat, L)` — over the
/// ratios given, filleting the upper crease at `r_band`, and returns
/// the table: the door's outcome, and where admitted the contact
/// edges' readings.
fn family_table(
    name: &str,
    build: impl Fn(f64, f64, f64) -> Result<Body<f64>, String>,
    r_band: f64,
    ratios: &[f64],
) -> String {
    let mut s = format!("=== {name}, r_band = {r_band} ===\n");
    let b = band();
    let _ = writeln!(s, "band = ({:e}, {:e})", b.zero(), b.escalate());
    for &ratio in ratios {
        let big_r = r_band * ratio;
        let predicted = (1.0 / r_band - 1.0 / big_r).abs() * r_band * r_band * 0.5;
        let headroom = (1.0 - r_band / big_r) * r_band;
        let _ = writeln!(
            s,
            "-- R/r = {ratio}: predicted margin {predicted:.4e}, headroom {headroom:.4e}"
        );
        let body = match build(big_r, r_band, 1.0) {
            Ok(body) => body,
            Err(why) => {
                let _ = writeln!(s, "   fixture: {why}");
                continue;
            }
        };
        let crease = upper_crease(&body);
        let result = fillet_edges(&body, &[crease], r_band, tol());
        let _ = writeln!(s, "   door: {}", door_outcome(&result));
        if let Ok(out) = result {
            s.push_str(&table("  contact edges", &contact_readings(&out.body)));
        }
    }
    s
}

// ---------------------------------------------------------------
// Phase 1: the measurement.
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

/// The corpus table: every contact edge of every carving fixture the
/// bit-dump corpus holds, read through the rule.
#[test]
fn phase1_corpus_table() {
    let mut s = String::new();
    let mut total = 0;
    for (name, body) in corpus() {
        let readings = contact_readings(&body);
        total += readings.len();
        s.push_str(&summary(&name, &readings));
    }
    let _ = writeln!(s, "TOTAL contact edges: {total}");
    println!("{s}");
}

/// The near-osculating family at the spec's six ratios, both branches.
#[test]
fn phase1_family_table() {
    let ratios = [2.0, 1.5, 1.1, 1.01, 1.0 + 1e-4, 1.0 + 1e-6];
    let s = family_table(
        "convex band on a convex cylinder (rod with a flat)",
        rod_with_flat_at,
        0.1,
        &ratios,
    );
    println!("{s}");
    let s = family_table(
        "concave band in a concave cylinder (block with a D bore)",
        block_with_d_bore,
        0.1,
        &ratios,
    );
    println!("{s}");
}

/// Exploratory: where the door's admission boundary lies on the ratio.
#[test]
fn phase1_ladder() {
    let s = family_table(
        "convex rod ladder",
        rod_with_flat_at,
        0.1,
        &[1.4, 1.35, 1.3, 1.25, 1.2],
    );
    println!("{s}");
}

/// Exploratory: the rod at an admitted ratio, scaled until the
/// second-order margin sits in the octave `(Kε/2, Kε)`.
#[test]
fn phase1_scaled() {
    let b = band();
    for ratio in [2.0, 1.5, 1.4] {
        for frac in [0.55, 0.75, 0.95] {
            let target = frac * b.escalate();
            let r = 2.0 * target / (1.0 - 1.0 / ratio);
            let s = family_table(
                &format!("scaled rod, target margin {frac}·Kε = {target:e}"),
                |big_r, flat, _| rod_with_flat_at(big_r, flat, 10.0 * flat),
                r,
                &[ratio],
            );
            println!("{s}");
        }
    }
}
