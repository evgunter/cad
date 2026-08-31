//! **BOOL-1 R1 review probes** (blinded reviewer R1, PR #1378, frozen
//! head `3f14f3c4`, issue 1152). Independent consumer rows attacking
//! the unit's claims from OUTSIDE its own tests.
//!
//! 1. **Mechanism** — the three `below` section-boundary edges are
//!    named directly and their carried descriptions cross-checked
//!    against their OWN adjacency, rather than through tier 3's
//!    verdict. Prints the census so a run under the reverted arm
//!    shows the stale citations verbatim.
//! 2. **Carrier immobility** — the restated edges' carrier endpoints
//!    and parameter interval are printed as raw bits, so a run with
//!    the fix and a run with the arm reverted can be diffed for the
//!    1-ULP rebuild-vs-restate class.
//! 3. **Band sensitivity** — the same body with the notch floor
//!    displaced off the section plane by a ladder of deltas, split at
//!    the plane. The outcome table is printed; it is a band probe, so
//!    it is run at more than one `CAD_TOLERANCE_EPS`.
//! 4. **End to end** — profile → extrude → face-coplanar split →
//!    tier 3 → volume conservation → watertight tessellation, on both
//!    products, through the public API only.
//! 5. **Declared authority** — a declared-tangent profile through the
//!    same coplanar split, checking the authority census across the
//!    restatement.
//! 6. **Tangent curved wall** — the site comment's unreachability
//!    argument, attacked with a cylindrical wall tangent to the
//!    section plane.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, SurfaceKey};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn validated(loops: Vec<ProfileLoop<f64>>) -> profile::ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .expect("a valid probe profile")
}

fn extruded(loops: Vec<ProfileLoop<f64>>, h: f64) -> Body<f64> {
    extrude(&validated(loops), Extrusion::Distance(h), Tol::witness())
        .expect("the probe profile extrudes")
        .body
}

/// The Fig. 14.2 notched block of issue 1152's reproduction, with the
/// notch floor displaced off `y = 1` by `dy` (0.0 is the committed
/// fixture).
fn notched(dy: f64) -> ProfileLoop<f64> {
    ProfileLoop::polygon(
        [
            (0.0, 0.0),
            (8.0, 0.0),
            (8.0, 2.0),
            (7.0, 1.0 + dy),
            (6.0, 1.0 + dy),
            (5.0, 2.0),
            (4.0, 1.0 + dy),
            (3.0, 2.0),
            (0.0, 2.0),
        ]
        .map(|(x, y)| p2(x, y)),
    )
}

fn plane_y1() -> topo::SplitPlane<f64> {
    topo::SplitPlane {
        origin: Point3::new(0.0, 1.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
    }
}

/// The two face surfaces an edge actually lies between, in this body.
fn adjacent_surfaces(body: &Body<f64>, e: EdgeKey) -> (SurfaceKey, SurfaceKey) {
    let ed = body.get_edge(e).expect("edge");
    let face_of = |he| {
        let l = body.get_half_edge(he).expect("he").parent_loop;
        let f = body.get_loop(l).expect("loop").face;
        body.get_face(f).expect("face").surface
    };
    (face_of(ed.he_plus), face_of(ed.he_minus))
}

/// Every edge's description rendered as (class, cited surfaces).
fn description_census(body: &Body<f64>) -> Vec<(EdgeKey, String, Vec<SurfaceKey>)> {
    body.edges()
        .filter_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            let (class, cited) = match *c.description() {
                geom_brep::EdgeDescription::Intersection { s1, s2, .. } => {
                    ("Intersection", vec![s1, s2])
                }
                geom_brep::EdgeDescription::TangentIntersection { s1, s2, .. } => {
                    ("TangentIntersection", vec![s1, s2])
                }
                geom_brep::EdgeDescription::Chart(ref ch) => ("Chart", vec![ch.surface]),
                geom_brep::EdgeDescription::Scaffold(_) => ("Scaffold", vec![]),
            };
            Some((k, class.to_string(), cited))
        })
        .collect()
}

/// Every edge whose citation names a surface that is not one of its
/// own two adjacent faces' — tier 3's `DescriptionNotAdjacent`
/// condition, recomputed here so the row does not depend on the
/// validator under review.
fn non_adjacent_citations(body: &Body<f64>) -> Vec<EdgeKey> {
    description_census(body)
        .into_iter()
        .filter(|(k, class, cited)| {
            if class == "Chart" || class == "Scaffold" {
                // A chart names ONE surface; a scaffold names none.
                let (a, b) = adjacent_surfaces(body, *k);
                return cited.iter().any(|s| *s != a && *s != b);
            }
            let (a, b) = adjacent_surfaces(body, *k);
            !(cited.contains(&a) && cited.contains(&b))
        })
        .map(|(k, _, _)| k)
        .collect()
}

/// PROBE 1 + 2: the reproduction's `below` product, with every
/// section-boundary edge's description checked against its own
/// adjacency, and the restated edges' carrier bits printed.
///
/// Under the shipped fix this is green. Under the smooth arm reverted
/// to `=> {}` it goes red naming the stale citations, and the printed
/// carrier bits are what the two runs are diffed on.
#[test]
fn coplanar_below_citations_are_all_adjacent() {
    let body = extruded(vec![notched(0.0)], 1.0);
    let result = topo::split(&body, &plane_y1(), Tol::witness()).expect("the coplanar split runs");
    for (name, part) in [("above", &result.above), ("below", &result.below)] {
        let b = part.body().expect("side has material");
        println!("--- {name} ---");
        for (k, class, cited) in description_census(b) {
            let (sa, sb) = adjacent_surfaces(b, k);
            let c = b
                .get_curve_geom(b.get_edge(k).unwrap().curve)
                .unwrap()
                .certified()
                .unwrap();
            let (t0, t1) = c.params();
            let carrier = match c.carrier() {
                geom::Curve3::Line { origin, dir } => format!(
                    "Line o({:016x},{:016x},{:016x}) d({:016x},{:016x},{:016x})",
                    origin.x.to_bits(),
                    origin.y.to_bits(),
                    origin.z.to_bits(),
                    dir.x.to_bits(),
                    dir.y.to_bits(),
                    dir.z.to_bits()
                ),
                other => format!("{other:?}"),
            };
            println!(
                "  {k:?} {class} cites {cited:?} adjacent ({sa:?},{sb:?}) \
                 params ({:016x},{:016x}) declared={} carrier {carrier}",
                t0.to_bits(),
                t1.to_bits(),
                c.authority().is_declared()
            );
        }
        let bad = non_adjacent_citations(b);
        assert!(
            bad.is_empty(),
            "{name}: citations naming a non-adjacent surface: {bad:?}"
        );
    }
}

/// PROBE 3: band sensitivity. The same body with the notch floor
/// displaced off the section plane by a ladder of deltas — the regime
/// the fix's new `set_edge_curve` call has to survive, since before
/// the fix the smooth arm never certified anything.
///
/// Prints an outcome table rather than asserting a shape: the point is
/// to compare the table across `CAD_TOLERANCE_EPS` rows.
#[test]
fn near_coplanar_notch_floor_outcome_table() {
    println!("eps={:?}", std::env::var("CAD_TOLERANCE_EPS").ok());
    for dy in [
        0.0, 1e-15, 1e-13, 1e-11, 1e-9, 2e-7, 5e-7, 1e-6, 2e-6, 5e-6, 1e-5, -1e-15, -1e-11, -2e-7,
        -5e-7, -1e-6, -2e-6, -5e-6, -1e-5,
    ] {
        let body = extruded(vec![notched(dy)], 1.0);
        let outcome = match topo::split(&body, &plane_y1(), Tol::witness()) {
            Err(e) => format!("split refused: {e:?}"),
            Ok(r) => {
                let mut parts = Vec::new();
                for (name, part) in [("above", &r.above), ("below", &r.below)] {
                    match part.body() {
                        None => parts.push(format!("{name}: no material")),
                        Some(b) => {
                            let v = topo::validate_geometric(b, Tol::witness());
                            let bad = non_adjacent_citations(b);
                            parts.push(format!(
                                "{name}: tier3={} stale={bad:?}",
                                if v.is_ok() { "ok" } else { "ERR" }
                            ));
                        }
                    }
                }
                parts.join(" | ")
            }
        };
        println!("dy={dy:e}: {outcome}");
    }
}

/// PROBE 4: the real end-to-end path — profile, extrude, the
/// face-coplanar split, tier 3, volume conservation, and a watertight
/// tessellation of BOTH products. Public API only.
#[test]
fn coplanar_split_e2e_volume_and_watertight() {
    let body = extruded(vec![notched(0.0)], 1.0);
    let v0 = topo::mass_properties(&body, Tol::witness())
        .expect("operand mass properties")
        .volume;
    let result = topo::split(&body, &plane_y1(), Tol::witness()).expect("the coplanar split runs");
    let mut total = 0.0;
    for (name, part) in [("above", &result.above), ("below", &result.below)] {
        let b = part.body().expect("side has material");
        assert_eq!(
            topo::validate_geometric(b, Tol::witness()),
            Ok(()),
            "{name} at tier 3"
        );
        total += topo::mass_properties(b, Tol::witness())
            .unwrap_or_else(|e| panic!("{name} mass properties: {e:?}"))
            .volume;
        let m = mesh::tessellate(b, 5e-3, Tol::witness())
            .unwrap_or_else(|e| panic!("{name} tessellates: {e:?}"));
        mesh::validate::check_mesh(&m).unwrap_or_else(|e| panic!("{name} watertight: {e:?}"));
    }
    assert!(
        (total - v0).abs() <= 1e-12 * v0,
        "volume conserved: {total} vs {v0}"
    );
}

/// PROBE 5: a DECLARED locus through the coplanar restatement. The
/// notch floor's own wall is a declared-tangent arc joint's neighbour,
/// so the extrude hands the split a body carrying declared authority;
/// the fix carries `declared_by` onto a chart in a DIFFERENT surface,
/// which this row measures rather than argues about.
#[test]
fn declared_authority_across_the_coplanar_restatement() {
    let body = extruded(vec![notched(0.0)], 1.0);
    let before: usize = body
        .edges()
        .filter_map(|(_, e)| body.get_curve_geom(e.curve)?.certified())
        .filter(|c| c.authority().is_declared())
        .count();
    let result = topo::split(&body, &plane_y1(), Tol::witness()).expect("the coplanar split runs");
    let mut after = 0;
    for part in [&result.above, &result.below] {
        let b = part.body().expect("side has material");
        after += b
            .edges()
            .filter_map(|(_, e)| b.get_curve_geom(e.curve)?.certified())
            .filter(|c| c.authority().is_declared())
            .count();
    }
    println!("declared before={before} after={after}");
}

/// PROBE 6: the site comment's unreachability argument for the
/// second-order ladder — "every smooth pair here is a flush plane
/// pair". A cylindrical wall TANGENT to the section plane is the
/// counterexample shape; this row records what the verb actually does
/// with one rather than assuming the gate catches it.
#[test]
fn cylindrical_wall_tangent_to_the_section_plane() {
    // A disc of radius 1 centred at (4, 2): its cylindrical wall is
    // tangent to y = 1 along the ruling at x = 4.
    let disc = ProfileLoop::new(vec![
        ProfileVertex::new(p2(3.0, 2.0), 1.0),
        ProfileVertex::new(p2(5.0, 2.0), 1.0),
    ]);
    let body = extruded(vec![disc], 1.0);
    let outcome = match topo::split(&body, &plane_y1(), Tol::witness()) {
        Err(e) => format!("refused: {e:?}"),
        Ok(r) => {
            let mut parts = Vec::new();
            for (name, part) in [("above", &r.above), ("below", &r.below)] {
                match part.body() {
                    None => parts.push(format!("{name}: no material")),
                    Some(b) => parts.push(format!(
                        "{name}: tier3={:?} stale={:?}",
                        topo::validate_geometric(b, Tol::witness()).is_ok(),
                        non_adjacent_citations(b)
                    )),
                }
            }
            parts.join(" | ")
        }
    };
    println!("tangent cylinder at the section plane: {outcome}");

    // And the two-sided version: the same disc raised so the plane
    // genuinely cuts it, with the tangency removed — the control.
    let disc2 = ProfileLoop::new(vec![
        ProfileVertex::new(p2(3.0, 1.5), 1.0),
        ProfileVertex::new(p2(5.0, 1.5), 1.0),
    ]);
    let body2 = extruded(vec![disc2], 1.0);
    println!(
        "control (transverse cylinder): {:?}",
        topo::split(&body2, &plane_y1(), Tol::witness())
            .map(|r| r
                .below
                .body()
                .map(|b| topo::validate_geometric(b, Tol::witness()).is_ok()))
            .map_err(|e| format!("{e:?}"))
    );
}
