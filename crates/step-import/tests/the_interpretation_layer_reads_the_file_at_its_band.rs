//! **The interpretation layer reads the file at its band** (`[import]`,
//! D7): what the file's statements are evidence of is read through the
//! kernel's readings at the file's band, every placement is built by
//! `build_axes`, two records of one surface become one, and what the
//! interpretation moves or refuses names the file entity and a distance.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{
    ImportOptions, InterpretationRefusal, NormalizationKind, StepImport, StepImportError,
    import_step,
};

use crate::common::freecad_fixture;

fn import(text: &str) -> Result<StepImport, StepImportError> {
    import_step(text, &ImportOptions::default(), Tol::witness())
}

/// `text` with `old` replaced by `new`, which must occur.
fn with(text: &str, old: &str, new: &str) -> String {
    assert!(text.contains(old), "the fixture states {old}");
    text.replace(old, new)
}

/// A surface placement's reference stated leaning toward its axis: the
/// frame is built by `build_axes` (the cylinder imports on a unit axis
/// and a perpendicular reference), and the move is reported keyed by the
/// `AXIS2_PLACEMENT_3D`. Its axis stated twice unit is a ratio vector,
/// whose normalizing is its meaning: no report.
#[test]
fn a_surface_placement_stated_off_its_frame_is_built_and_reported() {
    let reports = |text: &str| -> (StepImport, Vec<(u64, f64)>) {
        let imported = import(text).unwrap_or_else(|e| panic!("imports: {}", e.render()));
        let moved = imported
            .normalizations()
            .iter()
            .filter_map(|n| match n.kind {
                NormalizationKind::PlacementReMinted {
                    transform,
                    moved_by,
                } => Some((transform, moved_by)),
                _ => None,
            })
            .collect();
        (imported, moved)
    };
    let leaning = with(
        &freecad_fixture("cylinder"),
        "#49 = DIRECTION('',(1.,0.,-0.));",
        "#49 = DIRECTION('',(1.,0.,0.5));",
    );
    let (imported, moved) = reports(&leaning);
    let [(46, moved_by)] = moved.as_slice() else {
        panic!("one report, keyed by the cylinder's placement #46: {moved:?}");
    };
    // The move is measured from the frame of the two directions
    // normalized, not from the stated vectors: the reference normalized
    // is (c, 0, s), s its projection on the axis, and the built one
    // (1, 0, 0), so a box corner p moves by ((1−c)·pₓ, (1−c)·p_y, −s·pₓ),
    // farthest at |pᵢ| = E: E·√(s² + 2(1−c)²).
    let (c, s) = (1.0 / 1.25_f64.sqrt(), 0.5 / 1.25_f64.sqrt());
    let want = geom_core::tolerance::SESSION_BOX_EXTENT * (s * s + 2.0 * (1.0 - c).powi(2)).sqrt();
    assert!(
        (moved_by - want).abs() <= 1e-12 * want,
        "the move from the normalized frame: {moved_by:e} vs {want:e}"
    );
    let StepImport::Solid { body, .. } = &imported else {
        panic!("a solid")
    };
    let (axis, u_ref) = body
        .surfaces()
        .find_map(|(_, s)| match s {
            geom::Surface::Cylinder { frame, .. } => Some((frame.axis(), frame.u_ref())),
            _ => None,
        })
        .expect("the cylinder");
    assert!(
        (axis.norm() - 1.0).abs() < 1e-15 && axis.dot(u_ref).abs() < 1e-15,
        "the built frame is orthonormal: {axis:?} {u_ref:?}"
    );
    let stretched = with(
        &freecad_fixture("cylinder"),
        "#48 = DIRECTION('',(0.,0.,1.));",
        "#48 = DIRECTION('',(0.,0.,2.));",
    );
    assert_eq!(
        reports(&stretched).1,
        vec![],
        "a ratio vector's length moves nothing"
    );
    assert_eq!(
        reports(&freecad_fixture("cylinder")).1,
        vec![],
        "a unit placement moves nothing"
    );
}

/// A negative radius is a root no construction takes: refused at
/// interpretation, naming the surface record and the value, before any
/// edge certification reads it.
#[test]
fn a_negative_surface_radius_refuses_as_an_invalid_root() {
    let text = with(
        &freecad_fixture("cylinder"),
        "#45 = CYLINDRICAL_SURFACE('',#46,0.5);",
        "#45 = CYLINDRICAL_SURFACE('',#46,-0.5);",
    );
    match import(&text) {
        Err(
            ref e @ StepImportError::Interpretation {
                id,
                refusal: InterpretationRefusal::InvalidRoot { value, .. },
                ..
            },
        ) => {
            assert_eq!(id, 45, "the surface record is named");
            assert!(value < 0.0, "the file's value: {value}");
            assert!(
                e.render().contains("re-export the file from a valid model"),
                "{}",
                e.render()
            );
        }
        other => panic!("an invalid root, got {other:?}"),
    }
}

/// A `VERTEX_LOOP`'s point 0.1 mm off its sphere misses past the file's
/// band: an evidence miss naming the vertex and the ε_in that would read
/// the two as one.
#[test]
fn a_vertex_loop_point_off_its_sphere_is_an_evidence_miss() {
    let text = with(
        &freecad_fixture("sphere"),
        "#21 = CARTESIAN_POINT('',(6.123233995737E-17,-1.499759782662E-32,-1.));",
        "#21 = CARTESIAN_POINT('',(6.123233995737E-17,-1.499759782662E-32,-1.1));",
    );
    match import(&text) {
        Err(
            ref e @ StepImportError::Interpretation {
                id,
                refusal: InterpretationRefusal::EvidenceMiss { miss, .. },
                ..
            },
        ) => {
            assert_eq!(id, 20, "the VERTEX_POINT is named");
            assert!((miss - 1e-4).abs() < 1e-12, "a 0.1 mm miss: {miss}");
            assert!(e.render().contains("import with ε_in ≥ "), "{}", e.render());
        }
        other => panic!("an evidence miss, got {other:?}"),
    }
}

/// A boolean union's top and bottom are each tiled into three `PLANE`
/// records of one locus: read at the faces, the two later records of
/// each become one with the earliest, kept, and the body wears one
/// surface per flat side.
#[test]
fn two_records_of_one_plane_become_one() {
    let imported = import(&freecad_fixture("fuse_boxes")).unwrap();
    let joins: Vec<(u64, u64, f64)> = imported
        .normalizations()
        .iter()
        .filter_map(|n| match n.kind {
            NormalizationKind::OneRecord {
                kept,
                joined,
                moved_by,
            } => Some((kept, joined, moved_by)),
            _ => None,
        })
        .collect();
    assert_eq!(
        joins.len(),
        4,
        "the top's and the bottom's two later records each: {joins:?}"
    );
    for &(kept, joined, moved_by) in &joins {
        assert!(kept < joined, "the earlier record stays: {kept} {joined}");
        assert!(
            moved_by <= imported.eps_in().max(Tol::witness().eps()),
            "the faces agree within the band's zero: {moved_by}"
        );
    }
    let StepImport::Solid { body, .. } = &imported else {
        panic!("a solid")
    };
    let mut planes = std::collections::BTreeSet::new();
    for (_, face) in body.faces() {
        if matches!(
            body.get_surface(face.surface),
            Some(geom::Surface::Plane { .. })
        ) {
            planes.insert(face.surface);
        }
    }
    // Two overlapping unit boxes, offset by half a unit in x and y: an
    // eight-sided outline, a top and a bottom, each one surface (the
    // file states fourteen faces on fourteen records).
    assert_eq!(planes.len(), 10, "one surface per flat side");
}

/// The band's zero the FreeCAD fixtures are read at: their declared
/// 1e-7 mm, floored at the run's ε.
fn zero() -> f64 {
    1e-10_f64.max(Tol::witness().eps())
}

/// A `VERTEX_LOOP`'s point three band zeros off its sphere: the reading
/// passes only where the two agree, so in band it is a miss whose one
/// lever is an ε_in at least the miss, never a finer one.
#[test]
fn a_vertex_loop_point_in_the_band_is_a_miss_with_no_finer_lever() {
    let z_mm = -(1.0 + 3.0 * zero() * 1e3);
    let text = with(
        &freecad_fixture("sphere"),
        "#21 = CARTESIAN_POINT('',(6.123233995737E-17,-1.499759782662E-32,-1.));",
        &format!("#21 = CARTESIAN_POINT('',(6.123233995737E-17,-1.499759782662E-32,{z_mm:e}));"),
    );
    match import(&text) {
        Err(
            ref e @ StepImportError::Interpretation {
                id,
                refusal: InterpretationRefusal::MissInBand { miss, .. },
                ..
            },
        ) => {
            assert_eq!(id, 20, "the VERTEX_POINT is named");
            assert!(
                (miss - 3.0 * zero()).abs() <= 0.01 * zero(),
                "the miss: {miss}"
            );
            let text = e.render();
            assert!(text.contains("import with ε_in ≥ "), "{text}");
            assert!(
                !text.contains("below") && !text.contains("tighten"),
                "a decision that passes only at Zero offers no finer lever: {text}"
            );
        }
        other => panic!("a miss in band, got {other:?}"),
    }
}

/// An ellipse semi-axis of 5e-4 mm is a positive length the band of a
/// file read at ε_in = 1 µm cannot tell from zero: refused as that, with
/// the ε_in that reads it, not as an invalid root. At the file's own
/// ε_in the same record is a length.
#[test]
fn a_semi_axis_the_band_cannot_tell_from_zero_is_no_invalid_root() {
    let text = with(
        &freecad_fixture("cylinder"),
        "#24 = CIRCLE('',#25,0.5);",
        "#24 = ELLIPSE('',#25,0.5,5.E-04);",
    );
    let options = ImportOptions { eps_in: Some(1e-6) };
    match import_step(&text, &options, Tol::witness()) {
        Err(
            ref e @ StepImportError::Interpretation {
                id,
                refusal: InterpretationRefusal::LengthUnread { length, .. },
                ..
            },
        ) => {
            assert_eq!(id, 24, "the ELLIPSE is named");
            assert!(
                (length - 5e-7).abs() < 1e-15,
                "the length in metres: {length}"
            );
            assert!(
                e.render().contains("import with ε_in below"),
                "{}",
                e.render()
            );
        }
        other => panic!("a length the band cannot tell from zero, got {other:?}"),
    }
    // At the file's own ε_in, where the band it is read at escalates
    // below the semi-axis, it is a length.
    if Tol::witness().k() * zero() < 5e-7
        && let Err(StepImportError::Interpretation { id: 24, .. }) = import(&text)
    {
        panic!("at the file's own ε_in the semi-axis is a length");
    }
}

/// **An ambiguity's advice decides it.** FTC11 read at ε_in = 1 µm puts a
/// rim vertex 1.6 µm from its seam azimuth, in band. Its ending names
/// every lever the distance needs: an ε_in below the distance over K,
/// and the run's tolerance below it too where the run's ε is not.
/// Following the ε_in lever imports where the run's ε already lies below.
#[test]
fn ftc11s_ambiguity_names_what_decides_it_and_following_it_imports() {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/band/ftc11_uref_off.stp"
    ))
    .unwrap();
    let at = |eps_in: f64| {
        let options = ImportOptions {
            eps_in: Some(eps_in),
        };
        import_step(&text, &options, Tol::witness())
    };
    let e = match at(1e-6) {
        Err(e @ StepImportError::Interpretation { .. }) => e,
        other => panic!("FTC11 at ε_in 1e-6 refuses in band, got {other:?}"),
    };
    let StepImportError::Interpretation {
        refusal: InterpretationRefusal::Ambiguous { gap, k, .. },
        ..
    } = e
    else {
        panic!("an ambiguity: {e:?}")
    };
    let below = gap / k;
    let rendered = e.render();
    assert!(rendered.contains("import with ε_in below"), "{rendered}");
    let eps = Tol::witness().eps();
    assert_eq!(
        rendered.contains("tighten the tolerance below"),
        eps >= below,
        "the run's tolerance is named exactly where it does not already decide ({eps} vs \
         {below}): {rendered}"
    );
    if eps < below {
        match at(below / 2.0) {
            Ok(StepImport::Solid { .. }) => {}
            other => panic!("following the advice imports, got {:?}", other.map(|_| ())),
        }
    }
}

/// **A placement pair's one ledger entry bounds all its moves.** The
/// two-body file's first placement `#15` stated 0.01 mm off the root's
/// origin, its reference leaning 1e-12 toward its axis, read at
/// ε_in = 0.02 mm: the pair reads as the identity (a snap of 1e-5 m) and
/// the leaning frame is built (a move of about 1e-9 m at the box). The
/// one entry for `#194` is their sum, not the frame's alone.
#[test]
fn a_placement_pairs_entry_bounds_its_snap_and_its_frames() {
    let text = with(
        &with(
            &freecad_fixture("twobody_importexport"),
            "#16 = CARTESIAN_POINT('',(0.,0.,0.));",
            "#16 = CARTESIAN_POINT('',(1.E-02,0.,0.));",
        ),
        "#18 = DIRECTION('',(1.,0.,0.));",
        "#18 = DIRECTION('',(1.,0.,1.E-12));",
    );
    let options = ImportOptions { eps_in: Some(2e-5) };
    let imported = import_step(&text, &options, Tol::witness())
        .unwrap_or_else(|e| panic!("imports: {}", e.render()));
    let entries: Vec<f64> = imported
        .normalizations()
        .iter()
        .filter_map(|n| match n.kind {
            NormalizationKind::PlacementReMinted {
                transform: 194,
                moved_by,
            } => Some(moved_by),
            _ => None,
        })
        .collect();
    let [moved_by] = entries.as_slice() else {
        panic!("one entry for the pair #194: {entries:?}")
    };
    assert!(
        (1e-5..1.001e-5).contains(moved_by) && *moved_by > 1e-5,
        "the snap and the frame's build, summed: {moved_by:e}"
    );
}

/// An ε_in whose band `K·ε_in` is no finite length makes no band the
/// file can be read at: refused as that before any statement is read,
/// naming no entity, and naming the value as what it is (here the
/// caller's override, not the file's declaration), in scientific
/// notation.
#[test]
fn an_eps_in_that_makes_no_band_refuses_naming_whose_it_is() {
    let options = ImportOptions {
        eps_in: Some(1e308),
    };
    match import_step(&freecad_fixture("box"), &options, Tol::witness()) {
        Err(ref e @ StepImportError::UnbandedUncertainty { eps_in, overridden }) => {
            assert_eq!(eps_in, 1e308);
            assert!(overridden, "the override, not the declaration");
            let text = e.render();
            assert!(
                text.contains("the ε_in override (the coincidence distance ε_in = 1e308 m)"),
                "{text}"
            );
            assert!(
                !text.contains("declared"),
                "the file declared no such value: {text}"
            );
            assert!(!text.contains("entity #"), "no entity: {text}");
        }
        other => panic!("an unbanded uncertainty, got {other:?}"),
    }
}

/// A declared ε_in of zero is read at the run's ε like any other value
/// below it, not as an exact comparison: fuse_boxes declaring `0.`, with
/// one of its top's records `#223` raised 1e-13 m (below every run's ε),
/// still joins all four later records.
#[test]
fn a_declared_eps_in_of_zero_reads_at_the_runs_floor() {
    let text = with(
        &with(
            &freecad_fixture("fuse_boxes"),
            "LENGTH_MEASURE(1.E-07)",
            "LENGTH_MEASURE(0.)",
        ),
        "#223 = CARTESIAN_POINT('',(0.,0.,1.));",
        "#223 = CARTESIAN_POINT('',(0.,0.,1.0000000001));",
    );
    let imported = import(&text).unwrap_or_else(|e| panic!("imports: {}", e.render()));
    assert_eq!(imported.eps_in(), 0.0, "the declaration stands as stated");
    let joins = imported
        .normalizations()
        .iter()
        .filter(|n| matches!(n.kind, NormalizationKind::OneRecord { .. }))
        .count();
    assert_eq!(joins, 4, "the raised record joins at the run's ε");
}

/// A declared ε_in below zero is no coincidence distance: refused where
/// the declaration is read, as an invalid root naming the record, the
/// value and its unit.
#[test]
fn a_negative_declared_eps_in_refuses_as_an_invalid_root() {
    let text = with(
        &freecad_fixture("fuse_boxes"),
        "LENGTH_MEASURE(1.E-07)",
        "LENGTH_MEASURE(-1.E-07)",
    );
    match import(&text) {
        Err(
            ref e @ StepImportError::Interpretation {
                id,
                refusal: InterpretationRefusal::InvalidRoot { value, unit, .. },
                ..
            },
        ) => {
            assert_eq!(id, 397, "the UNCERTAINTY_MEASURE_WITH_UNIT is named");
            assert!((value + 1e-10).abs() < 1e-24, "in metres: {value:e}");
            assert_eq!(unit, "m");
            assert!(
                e.render().contains("declared uncertainty"),
                "{}",
                e.render()
            );
        }
        other => panic!("an invalid root, got {other:?}"),
    }
}

/// A torus's major radius that is not positive, or not past its minor
/// (a spindle or horn torus, outside D3's ring convention), is a root no
/// construction takes: refused at the `TOROIDAL_SURFACE` record, naming
/// which, before any edge's adoption reads the surface.
#[test]
fn a_torus_off_the_ring_convention_refuses_at_its_record() {
    for (major, names, value) in [
        ("-1.", "major radius, which", -1e-3),
        ("0.", "major radius, which", 0.0),
        ("0.1", "less its minor radius", -1.5e-4),
        ("0.25", "less its minor radius", 0.0),
    ] {
        let text = with(
            &freecad_fixture("torus"),
            "#38 = TOROIDAL_SURFACE('',#39,1.,0.25);",
            &format!("#38 = TOROIDAL_SURFACE('',#39,{major},0.25);"),
        );
        match import(&text) {
            Err(
                ref e @ StepImportError::Interpretation {
                    id,
                    refusal: InterpretationRefusal::InvalidRoot { value: got, .. },
                    ..
                },
            ) => {
                assert_eq!(id, 38, "major {major}: the surface record is named");
                assert!(
                    (got - value).abs() < 1e-15,
                    "major {major}: the value in metres: {got:e}"
                );
                assert!(e.render().contains(names), "major {major}: {}", e.render());
            }
            other => panic!("major {major}: an invalid root, got {other:?}"),
        }
    }
}

/// A band fixture (`tests/fixtures/band/`), read as stated.
fn band_fixture(name: &str) -> String {
    let p = format!("{}/tests/fixtures/band/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(p).unwrap()
}

/// **Rim coaxiality is a lane choice** (`[verdicts.consumers]`): band_a's
/// lower rim circle (radius 10 mm, read at its declared 0.01 mm) tilted
/// by δ about its +y point moves its ±x points by 10·δ mm. Tilted 1e-12
/// (a 1e-14 m tilt, below every run's ε, so the edge certifies too) it
/// reads Zero and takes the band re-mint, which a coaxiality read exactly
/// would refuse; tilted 3e-3
/// (3e-5 m, in band) it takes the general lane, which is not written, so
/// it refuses with the closed form's pose as the lever and offers no
/// tolerance.
#[test]
fn a_rim_tilted_within_the_band_takes_the_re_mint_and_one_in_band_refuses() {
    let tilted = |dx: &str| {
        with(
            &band_fixture("band_a.stp"),
            "#113=DIRECTION('',(0.0,0.0,1.0));",
            &format!("#113=DIRECTION('',({dx},0.0,1.0));"),
        )
    };
    let zero = import(&tilted("1.E-12")).unwrap_or_else(|e| panic!("imports: {}", e.render()));
    let StepImport::Solid { body, .. } = &zero else {
        panic!("a solid")
    };
    assert_eq!(
        crate::common::arena_census(body),
        (1, 1, 2, 4, 2),
        "the re-minted band, as the coaxial fixture imports"
    );
    match import(&tilted("3.E-03")) {
        Err(ref e @ StepImportError::Topology { id, .. }) => {
            let text = e.render();
            assert_eq!(id, 136, "the torus face is named: {text}");
            assert!(
                text.contains("pose each rim as a circle centred on the surface's axis"),
                "{text}"
            );
            assert!(!text.contains("ε_in"), "no tolerance offer: {text}");
        }
        other => panic!("the lane's refusal, got {:?}", other.map(|_| ())),
    }
}

/// The `OneRecord` joins of an import, as `(kept, joined)`.
fn joins_of(text: &str) -> Vec<(u64, u64)> {
    import(text)
        .unwrap_or_else(|e| panic!("imports: {}", e.render()))
        .normalizations()
        .iter()
        .filter_map(|n| match n.kind {
            NormalizationKind::OneRecord { kept, joined, .. } => Some((kept, joined)),
            _ => None,
        })
        .collect()
}

/// **The kept record is the one earliest in the DATA section**, not the
/// one with the least id: fuse_boxes with one joined record's `PLANE`
/// line moved to the head of DATA keeps that record, now earliest
/// though its id is higher, and joins the other into it.
#[test]
fn the_kept_record_is_the_earliest_in_data_not_the_least_id() {
    let text = freecad_fixture("fuse_boxes");
    let (kept, joined) = joins_of(&text)[0];
    assert!(kept < joined, "as stated, DATA order and id order agree");
    let line = text
        .lines()
        .find(|l| l.starts_with(&format!("#{joined} = PLANE(")))
        .expect("the joined record's line");
    let moved =
        with(&text, &format!("{line}\n"), "").replacen("DATA;\n", &format!("DATA;\n{line}\n"), 1);
    let after = joins_of(&moved);
    assert!(
        after.contains(&(joined, kept)),
        "#{joined}, now first in DATA, is kept and #{kept} joins it: {after:?}"
    );
    assert!(
        after.iter().all(|&(_, j)| j != joined),
        "#{joined} is joined into nothing: {after:?}"
    );
}
