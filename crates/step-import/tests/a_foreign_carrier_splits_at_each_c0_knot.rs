//! **The import splits a foreign face or edge at each interior C⁰
//! knot** (`docs/DESIGN.md` [topology]: every face chart and edge
//! carrier is C¹ by its knot structure strictly inside its cell, and
//! import splits at its door).
//!
//! The files are `fixtures/c0-knot/generate.py`'s: a 2 × 1 prism whose
//! top is a B-spline wall with a crease at the ridge `x = 1`, a double
//! knot at degree 2, stated with the knot along `u` (`ridge_u`) and
//! along `v` (`ridge_v`); its front and back edges are the matching
//! iso curves, double knot included. On the merge base both import and
//! the mesher refuses them (the face at its crease, the edges at their
//! kink). `seam_edge` states the knot on one edge carrier alone, where
//! nothing else meets it; `half_cone` is a rational wall whose CIRCLE
//! rims carry no knot. `c_prism` and `half_ridge` lie under a smooth
//! paraboloid stated with a double knot: a C-shaped trim the knot line
//! crosses four times, and a face whose boundary runs along the curved
//! knot line.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use mesh::tessellate;
use mesh::validate::check_mesh;
use step_import::{FaceCensus, ImportOptions, NormalizationKind, StepImport, import_step};
use topo::CurveGeom;

fn fixture(name: &str) -> String {
    let p = format!(
        "{}/tests/fixtures/c0-knot/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(p).unwrap()
}

/// Asserts every spline face of `body` (the file's wall and the pieces
/// its split made) keeps the wall's stated `same_sense`.
fn walls_keep_sense(body: &topo::Body<f64>, sense: bool, name: &str) {
    for (fk, _) in body.faces() {
        if let Some(geom::Surface::Nurbs(_)) = body.face_surface(fk) {
            assert_eq!(
                body.face(fk).map(|f| f.sense()),
                Some(sense),
                "{name}: a wall piece keeps the wall's sense"
            );
        }
    }
}

/// Whether a carrier's knots hold an interior run of multiplicity at
/// least the degree.
fn creased(knots: &geom_core::spline::Knots<f64>) -> bool {
    knots.structure().crease_runs().next().is_some()
}

/// The ridge prism, stated either way round, imports split: the top is
/// two sub-patches with the iso curve at the ridge between them, each
/// edge on the ridge is two sub-arcs, every split is reported, and the
/// body is valid at rest, meshes watertight and holds the file's
/// volume.
#[test]
fn a_creased_wall_and_its_creased_edges_import_split_at_the_knot() {
    let tol = Tol::witness();
    for (name, sense) in [("ridge_u.step", true), ("ridge_v.step", false)] {
        let imported = import_step(&fixture(name), &ImportOptions::default(), tol)
            .unwrap_or_else(|e| panic!("{name} imports: {e_at}", e_at = e.render()));
        let StepImport::Solid {
            body,
            normalizations,
            ..
        } = imported
        else {
            panic!("{name} is a solid");
        };
        let normalizations = crate::common::structural(normalizations);
        let b = &*body;
        assert_eq!(
            (b.faces().count(), b.edges().count(), b.vertices().count()),
            (7, 15, 10),
            "{name}: the file's 6 faces, 12 edges and 8 vertices, with the top split in two \
             by one new edge and each ridge edge split by one new vertex"
        );

        // No cell carries a crease: every spline chart and carrier is
        // C¹ by its knot structure.
        let mut walls = 0;
        for (fk, _) in b.faces() {
            if let Some(geom::Surface::Nurbs(s)) = b.face_surface(fk) {
                walls += 1;
                assert!(
                    !creased(s.knot_values_u()) && !creased(s.knot_values_v()),
                    "{name}: a wall chart still carries a C⁰ knot"
                );
            }
        }
        assert_eq!(walls, 2, "{name}: the top is two sub-patches");
        walls_keep_sense(b, sense, name);
        let mut spline_edges = 0;
        for (_, c) in b.curves() {
            if let CurveGeom::Certified(c) = c
                && let geom::Curve3::Nurbs(n) = c.carrier()
            {
                spline_edges += 1;
                assert!(
                    !creased(n.knot_values()),
                    "{name}: an edge carrier still carries a C⁰ knot"
                );
            }
        }
        assert_eq!(
            spline_edges, 5,
            "{name}: four ridge sub-arcs and the minted iso edge between the sub-patches"
        );

        // Every split is reported, in the census shape the records sum.
        let edge = |n: &&step_import::StructureNormalization| {
            n.kind == NormalizationKind::EdgeSplitAtC0Knot
        };
        let split_edges: Vec<_> = normalizations.iter().filter(edge).collect();
        assert_eq!(split_edges.len(), 2, "{name}: one record per ridge edge");
        for n in &split_edges {
            assert_eq!(
                (n.file_census, n.kernel_census),
                (
                    FaceCensus {
                        faces: 0,
                        edges: 1,
                        vertices: 0
                    },
                    FaceCensus {
                        faces: 0,
                        edges: 2,
                        vertices: 1
                    }
                ),
                "{name}: an edge become two sub-arcs and the vertex between them"
            );
        }
        let faces: Vec<_> = normalizations
            .iter()
            .filter(|n| n.kind == NormalizationKind::FaceSplitAtC0Knot)
            .collect();
        assert_eq!(faces.len(), 1, "{name}: one record for the top");
        assert_eq!(
            (faces[0].file_census, faces[0].kernel_census),
            (
                FaceCensus {
                    faces: 1,
                    edges: 6,
                    vertices: 6
                },
                FaceCensus {
                    faces: 2,
                    edges: 7,
                    vertices: 6
                }
            ),
            "{name}: the top become two faces sharing one new edge"
        );
        assert!(
            normalizations.iter().all(|n| n.face == faces[0].face),
            "{name}: every record names the top face, the least face each ridge edge bounds"
        );
        let delta = |f: fn(&FaceCensus) -> usize| -> usize {
            normalizations
                .iter()
                .map(|n| f(&n.kernel_census) - f(&n.file_census))
                .sum()
        };
        assert_eq!(
            (
                delta(|c| c.faces),
                delta(|c| c.edges),
                delta(|c| c.vertices)
            ),
            (1, 3, 2),
            "{name}: the records' deltas are the body's totals over the file's 6/12/8"
        );

        // Valid at rest, meshable, and the file's own volume: two
        // quadratic halves over [0, 1] each with mean height 1.3.
        topo::validate_geometric(b, tol).unwrap_or_else(|e| panic!("{name}: tier 3: {e:?}"));
        let m = tessellate(b, 1e-2, tol).unwrap_or_else(|e| panic!("{name} meshes: {e:?}"));
        check_mesh(&m).unwrap_or_else(|e| panic!("{name}: the mesh is watertight: {e:?}"));
        let v = body
            .measure(topo::Settle::Resolution(quantity::Length::from_meters(
                1e-6,
            )))
            .unwrap_or_else(|e| panic!("{name} measures: {e:?}"))
            .enclosure;
        assert!(
            (v.volume - 2.6).abs() <= v.volume_pad.max(1e-9),
            "{name}: volume {} ± {:e}, the prism's 2.6",
            v.volume,
            v.volume_pad
        );
    }
}

/// A knot on an edge carrier alone, where the curve is smooth: the edge
/// is two sub-arcs meeting at a vertex no other edge meets, and the
/// import's closing join keeps it, because the joined carrier would
/// carry the knot again (Ev's PR 5913 ruling: the vertex is the one the
/// knot structure puts there).
#[test]
fn a_knot_vertex_nothing_else_meets_survives_the_closing_join() {
    let tol = Tol::witness();
    let imported = import_step(&fixture("seam_edge.step"), &ImportOptions::default(), tol)
        .unwrap_or_else(|e| panic!("the lone knot imports: {e_at}", e_at = e.render()));
    let StepImport::Solid {
        body,
        normalizations,
        ..
    } = imported
    else {
        panic!("seam_edge is a solid");
    };
    let normalizations = crate::common::structural(normalizations);
    let b = &*body;
    assert_eq!(
        (b.faces().count(), b.edges().count(), b.vertices().count()),
        (6, 13, 9),
        "the file's 6 faces, 12 edges and 8 vertices, with the front edge two sub-arcs"
    );
    let kinds: Vec<_> = normalizations.iter().map(|n| n.kind).collect();
    assert_eq!(
        kinds,
        [NormalizationKind::EdgeSplitAtC0Knot],
        "one edge split and no join taking it back"
    );
    let knot_vertex = b
        .points()
        .map(|(_, p)| *p)
        .filter(|p| p.x == 1.0 && p.y == 0.0 && p.z == 1.25)
        .count();
    assert_eq!(
        knot_vertex, 1,
        "a vertex at the front edge's knot point (1, 0, 1.25)"
    );
    topo::validate_geometric(b, tol).unwrap_or_else(|e| panic!("tier 3: {e:?}"));
    let m = tessellate(b, 1e-2, tol).unwrap_or_else(|e| panic!("seam_edge meshes: {e:?}"));
    check_mesh(&m).unwrap_or_else(|e| panic!("the mesh is watertight: {e:?}"));
}

/// A rational wall smooth across its double knot, bounded by CIRCLE
/// rims that carry no knot: the split places its vertices on the rims
/// at the knot's iso line, and the two quarter-circle sub-patches
/// import, mesh and measure as the file's half frustum.
#[test]
fn a_rational_wall_splits_where_its_rims_carry_no_knot() {
    let tol = Tol::witness();
    let imported = import_step(&fixture("half_cone.step"), &ImportOptions::default(), tol)
        .unwrap_or_else(|e| panic!("the half cone imports: {e_at}", e_at = e.render()));
    let StepImport::Solid {
        body,
        normalizations,
        ..
    } = imported
    else {
        panic!("the half cone is a solid");
    };
    let normalizations = crate::common::structural(normalizations);
    let b = &*body;
    assert_eq!(
        (b.faces().count(), b.edges().count(), b.vertices().count()),
        (5, 9, 6),
        "the file's 4 faces, 6 edges and 4 vertices, with the wall split in two by one new \
         edge whose ends split each rim"
    );
    assert_eq!(normalizations.len(), 1, "one record: {normalizations:?}");
    assert_eq!(normalizations[0].kind, NormalizationKind::FaceSplitAtC0Knot);
    assert_eq!(
        (
            normalizations[0].file_census,
            normalizations[0].kernel_census
        ),
        (
            FaceCensus {
                faces: 1,
                edges: 4,
                vertices: 4
            },
            FaceCensus {
                faces: 2,
                edges: 7,
                vertices: 6
            }
        ),
        "the wall become two faces, its two rims four arcs, and one new edge between them"
    );
    walls_keep_sense(b, true, "half_cone");
    let rim_points: Vec<_> = b
        .points()
        .map(|(_, p)| *p)
        .filter(|p| p.y.abs() < 1e-12)
        .collect();
    assert_eq!(
        rim_points.len(),
        2,
        "a vertex on each rim at the knot's meridian y = 0: {rim_points:?}"
    );
    for (fk, _) in b.faces() {
        if let Some(geom::Surface::Nurbs(s)) = b.face_surface(fk) {
            assert!(
                !creased(s.knot_values_u()) && !creased(s.knot_values_v()),
                "a wall chart still carries a C⁰ knot"
            );
        }
    }
    topo::validate_geometric(b, tol).unwrap_or_else(|e| panic!("tier 3: {e:?}"));
    let m = tessellate(b, 1e-2, tol).unwrap_or_else(|e| panic!("the half cone meshes: {e:?}"));
    check_mesh(&m).unwrap_or_else(|e| panic!("the mesh is watertight: {e:?}"));
    // Half a frustum of height 1 and radii 1 and 1/2: π/6 · (1 + 1/2 + 1/4).
    let frustum = core::f64::consts::PI / 6.0 * 1.75;
    let v = body
        .measure(topo::Settle::Resolution(quantity::Length::from_meters(
            1e-6,
        )))
        .unwrap_or_else(|e| panic!("the half cone measures: {e:?}"))
        .enclosure;
    assert!(
        (v.volume - frustum).abs() <= v.volume_pad,
        "volume {} ± {:e}, the half frustum's {frustum}",
        v.volume,
        v.volume_pad
    );
}

/// `∫∫ (1 + x²/8 + y²/4) dx dy` over `[x0, x1] × [y0, y1]`: the volume
/// under `generate.py`'s paraboloid top.
fn under_top((x0, x1): (f64, f64), (y0, y1): (f64, f64)) -> f64 {
    (x1 - x0) * (y1 - y0)
        + (x1.powi(3) - x0.powi(3)) / 24.0 * (y1 - y0)
        + (y1.powi(3) - y0.powi(3)) / 12.0 * (x1 - x0)
}

/// Imports `name`, holds it to the shared tier gate (valid at rest,
/// the reading `tier_gate::every_corpus_import_passes_the_shared_gate`
/// applies), its census, its wall pieces' `sense` and a watertight
/// mesh, and measures its volume against `volume`. Answers the import's
/// normalizations.
fn gated(
    name: &str,
    sense: bool,
    census: (usize, usize, usize),
    volume: f64,
) -> Vec<step_import::StructureNormalization> {
    let tol = Tol::witness();
    let imported = import_step(&fixture(name), &ImportOptions::default(), tol)
        .unwrap_or_else(|e| panic!("{name} imports: {e_at}", e_at = e.render()));
    let StepImport::Solid {
        body,
        normalizations,
        ..
    } = imported
    else {
        panic!("{name} is a solid");
    };
    let b = &*body;
    assert_eq!(
        topo::validate_geometric(b, tol),
        Ok(()),
        "{name}: the shipped body passes the shared gate"
    );
    assert_eq!(
        (b.faces().count(), b.edges().count(), b.vertices().count()),
        census,
        "{name}: census (faces, edges, vertices)"
    );
    walls_keep_sense(b, sense, name);
    let m = tessellate(b, 1e-2, tol).unwrap_or_else(|e| panic!("{name} meshes: {e:?}"));
    check_mesh(&m).unwrap_or_else(|e| panic!("{name}: the mesh is watertight: {e:?}"));
    let v = body
        .measure(topo::Settle::Resolution(quantity::Length::from_meters(
            1e-6,
        )))
        .unwrap_or_else(|e| panic!("{name} measures: {e:?}"))
        .enclosure;
    assert!(
        (v.volume - volume).abs() <= v.volume_pad.max(1e-9),
        "{name}: volume {} ± {:e}, the file's {volume}",
        v.volume,
        v.volume_pad
    );
    normalizations
}

/// A C-shaped plan under a top whose chart holds a double knot at
/// `x = 1` where its locus is smooth, and whose top edges carry no
/// knot: the knot line crosses the trim four times, twice inside edges
/// away from the chart's boundary rows. The split places a vertex at
/// each crossing and parts the top into the C's back and its two arms,
/// with the two runs of the line inside the face as edges between them.
#[test]
fn a_trim_crossing_the_knot_line_four_times_imports_in_three_pieces() {
    let normalizations = gated(
        "c_prism.step",
        true,
        // The file's 10 faces, 24 edges and 16 vertices; four top edges
        // split at a crossing each, two runs of the line minted, and the
        // top three faces.
        (12, 30, 20),
        under_top((0.0, 2.0), (0.0, 1.0)) - under_top((0.5, 2.0), (0.25, 0.75)),
    );
    let faces: Vec<_> = normalizations
        .iter()
        .filter(|n| n.kind == NormalizationKind::FaceSplitAtC0Knot)
        .collect();
    assert_eq!(
        faces.len(),
        1,
        "one record, for the top: {normalizations:?}"
    );
    assert_eq!(
        (faces[0].file_census, faces[0].kernel_census),
        (
            FaceCensus {
                faces: 1,
                edges: 8,
                vertices: 8
            },
            FaceCensus {
                faces: 3,
                edges: 14,
                vertices: 12
            }
        ),
        "the top become three faces, its eight edges twelve and two runs of the line"
    );
}

/// The same chart trimmed to `x ≤ 1`: the face's right edge is the
/// chart's own iso curve at the knot, a parabola, so no chord holds it
/// on the line and the selection keeps the line. The split certifies the
/// edge along it by its iso pcurve and leaves the face whole, the crease
/// on its boundary.
#[test]
fn a_face_bounded_by_a_curved_knot_line_imports_whole() {
    let normalizations = gated(
        "half_ridge.step",
        true,
        (6, 12, 8),
        under_top((0.0, 1.0), (0.0, 1.0)),
    );
    assert!(
        normalizations
            .iter()
            .all(|n| n.kind != NormalizationKind::FaceSplitAtC0Knot),
        "no face split: {normalizations:?}"
    );
}
