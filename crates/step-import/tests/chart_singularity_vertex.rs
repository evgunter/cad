//! **A chart singularity in a face is a vertex of it, through the import
//! door.** Each fixture states a sphere pole or a cone apex in a face's
//! closure without making it a pole-form vertex — inside the face, inside
//! an edge, or at the tip of a seam that is its only edge — and import
//! re-mints it (`NormalizationKind::ChartSingularityVertex`), reported
//! with where the file put it. The body that comes out passes tier 3,
//! measures its closed form, meshes, and ends every singular point it
//! reaches in exactly two edges.
//!
//! Fixtures: `fixtures/rim-only-cap/gen_rim_only_cap.py` (the rim-only,
//! seamed, spurred, tilted and cone caps), `fixtures/halfcap/` (a meridian arc
//! across the pole), `fixtures/seam-region/` (a side-cut ball split at
//! its equator or along a meridian), and `fixtures/occ/gen_occ.py` (Open
//! CASCADE's own statements of a cap and a side-cut ball).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::Surface;
use geom_core::{Point3, Tol};
use step_import::{
    ImportOptions, NormalizationKind, SingularityFound, StepImport, StepImportError, import_step,
};

fn fixture(path: &str) -> String {
    let p = format!("{}/tests/fixtures/{path}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("reading {p}: {e}"))
}

/// A sphere cap of height `h` on radius `r`.
fn cap(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}

/// One row: the fixture, where the file put each singular point import
/// made a vertex (in report order), the body's face/edge/vertex census,
/// and its closed-form volume.
struct Row {
    path: &'static str,
    found: &'static [SingularityFound],
    census: (usize, usize, usize),
    volume: f64,
}

fn rows() -> Vec<Row> {
    use SingularityFound::{InsideEdge, Interior, ValenceOneVertex};
    let r = 0.010_f64;
    let rim_cap = cap(r, r * (1.0 - 0.5_f64.sin()));
    // The ball less the cap x > 8 mm.
    let side_cut = 4.0 * PI * r * r * r / 3.0 - cap(r, 0.002);
    let cone_r = 0.005_f64;
    let cone = PI * cone_r * cone_r * (cone_r / 0.5_f64.tan()) / 3.0;
    vec![
        Row {
            path: "rim-only-cap/rimonly1.step",
            found: &[Interior],
            census: (3, 4, 3),
            volume: rim_cap,
        },
        Row {
            path: "rim-only-cap/rimonly2.step",
            found: &[Interior],
            census: (3, 4, 3),
            volume: rim_cap,
        },
        Row {
            path: "rim-only-cap/seamed.step",
            found: &[ValenceOneVertex],
            census: (3, 4, 3),
            volume: rim_cap,
        },
        // The spur does not take the face out: the pole is still inside
        // it, and the meridian along the spur is joined to it.
        Row {
            path: "rim-only-cap/spur.step",
            found: &[Interior],
            census: (3, 4, 3),
            volume: rim_cap,
        },
        Row {
            path: "rim-only-cap/tilted.step",
            found: &[Interior],
            census: (3, 4, 3),
            volume: cap(r, 0.004),
        },
        Row {
            path: "rim-only-cap/conecap.step",
            found: &[Interior],
            census: (3, 4, 3),
            volume: cone,
        },
        Row {
            path: "halfcap/halfcap.step",
            found: &[InsideEdge],
            census: (3, 4, 3),
            volume: rim_cap / 2.0,
        },
        Row {
            path: "halfcap/halfcap_nosplit.step",
            found: &[InsideEdge],
            census: (3, 4, 3),
            volume: rim_cap / 2.0,
        },
        Row {
            path: "seam-region/side_cut_equator_split.step",
            found: &[Interior, Interior],
            census: (5, 10, 7),
            volume: side_cut,
        },
        Row {
            path: "seam-region/side_cut_meridian_split.step",
            found: &[Interior, InsideEdge],
            census: (5, 10, 7),
            volume: side_cut,
        },
        Row {
            path: "occ/cap.step",
            found: &[ValenceOneVertex],
            census: (3, 4, 3),
            volume: rim_cap,
        },
        Row {
            path: "occ/side_cut.step",
            found: &[ValenceOneVertex],
            census: (3, 5, 4),
            volume: side_cut,
        },
    ]
}

/// The singular points of every sphere and cone face of `body`.
fn singular_points(body: &topo::Body<f64>) -> Vec<Point3<f64>> {
    let mut out = Vec::new();
    for (_, f) in body.faces() {
        match *body.get_surface(f.surface).unwrap() {
            Surface::Sphere {
                center,
                radius,
                axis,
                ..
            } => out.extend([center + axis * radius, center - axis * radius]),
            Surface::Cone { apex, .. } => out.push(apex),
            _ => {}
        }
    }
    out
}

#[test]
fn every_chart_singularity_a_file_puts_in_a_face_imports_as_a_pole_form_vertex() {
    let tol = Tol::witness();
    for row in rows() {
        let name = row.path;
        let imported = import_step(&fixture(name), &ImportOptions::default(), tol)
            .unwrap_or_else(|e| panic!("{name} imports: {e:?}"));
        let StepImport::Solid {
            body,
            normalizations,
            ..
        } = imported
        else {
            panic!("{name} imports as a solid");
        };
        let found: Vec<SingularityFound> = normalizations
            .iter()
            .filter_map(|n| match n.kind {
                NormalizationKind::ChartSingularityVertex { found } => Some(found),
                _ => None,
            })
            .collect();
        assert_eq!(found, row.found, "{name}: the reported dispositions");
        // Every record's deltas sum to the body's own counts change, so
        // the census pair is a mapping and not decoration.
        let b = body.body();
        let census = (
            b.faces().count(),
            b.edges().count(),
            b.vertex_points().count(),
        );
        assert_eq!(census, row.census, "{name}: the body's census");
        assert_eq!(topo::validate_geometric(b, tol), Ok(()), "{name}: tier 3");
        let volume = topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished("the measured body", b.clone(), tol),
            tol,
        )
        .volume;
        assert!(
            (volume - row.volume).abs() <= 1e-9 * row.volume,
            "{name}: volume {volume} vs the closed form {}",
            row.volume
        );
        assert_eq!(
            mesh::tessellate(b, 1e-4, tol).map(|_| ()),
            Ok(()),
            "{name}: meshes"
        );
        // The rule itself: every singular point the body reaches is a
        // vertex with exactly two edge ends at it.
        let points: Vec<_> = b.vertex_points().collect();
        for s in singular_points(b) {
            let Some((v, _)) = points.iter().find(|(_, p)| (*p - s).norm() <= 1e-9) else {
                continue;
            };
            let ends = b
                .edges()
                .flat_map(|(_, e)| [e.he_plus, e.he_minus])
                .filter(|h| b.get_half_edge(*h).unwrap().start == *v)
                .count();
            assert_eq!(
                ends, 2,
                "{name}: the singular point {s:?} ends {ends} edges"
            );
        }
    }
}

/// The census pair is the mapping: each record's kernel census less its
/// file census, summed with the other records', is the change from the
/// file's counts to the body's. Read on the two files whose re-mint
/// splits an edge another face shares.
#[test]
fn a_singularity_record_maps_the_census_it_changes() {
    let tol = Tol::witness();
    for (name, file) in [
        ("rim-only-cap/rimonly1.step", (2, 1, 1)),
        ("seam-region/side_cut_equator_split.step", (3, 3, 2)),
    ] {
        let Ok(StepImport::Solid {
            body,
            normalizations,
            ..
        }) = import_step(&fixture(name), &ImportOptions::default(), tol)
        else {
            panic!("{name} imports as a solid");
        };
        let mut sum = file;
        for n in &normalizations {
            sum.0 = sum.0 + n.kernel_census.faces - n.file_census.faces;
            sum.1 = sum.1 + n.kernel_census.edges - n.file_census.edges;
            sum.2 = sum.2 + n.kernel_census.vertices - n.file_census.vertices;
        }
        let b = body.body();
        assert_eq!(
            sum,
            (
                b.faces().count(),
                b.edges().count(),
                b.vertex_points().count()
            ),
            "{name}: the file's census plus the records' deltas is the body's"
        );
    }
}

/// A seam ending at the pole of a face whose sense bit puts its material
/// on the far side of the loop is an inside-out face: import refuses it
/// before any body exists, rather than re-mint it right-side-out.
#[test]
fn a_seam_at_the_pole_of_an_inverted_face_refuses_typed() {
    let text = fixture("rim-only-cap/seamed.step");
    let face = "#43 = ADVANCED_FACE('',(#42),#37,.T.);";
    assert!(text.contains(face), "the seamed cap's sphere face");
    let flipped = text.replace(face, "#43 = ADVANCED_FACE('',(#42),#37,.F.);");
    match import_step(&flipped, &ImportOptions::default(), Tol::witness()) {
        Err(StepImportError::Topology { id: 43, what }) => {
            assert!(what.contains("ORIENTATION-INVERTED"), "{what}");
        }
        other => panic!("the inverted seamed cap: {other:?}"),
    }
}
