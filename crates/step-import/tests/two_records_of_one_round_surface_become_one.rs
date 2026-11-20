//! **Two records of one round surface become one** (`step_import`'s
//! `one_record`, `[topo.one-record]`): a cylinder or a cone stated in two
//! records whose datums read one locus at the file's band takes the
//! earlier record, reported; a datum off by a distance in the band
//! refuses as the file's ambiguity; one definitely apart stays its own.
//!
//! The fixture is `sg1-c5-214`, whose 40 mm cylinder `#94` and cone
//! `#151` each carry two faces on one record, read at its declared
//! ε_in = 0.005 mm. Each case states one face's surface again as a second
//! record at the end of the DATA section.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{
    ImportOptions, InterpretationRefusal, NormalizationKind, StepImport, StepImportError,
    import_step,
};

const SG1: &str = "tests/fixtures/wild/stepcode/sg1-c5-214.stp";

fn sg1() -> String {
    std::fs::read_to_string(format!("{}/{SG1}", env!("CARGO_MANIFEST_DIR"))).unwrap()
}

/// `sg1` with face `face`'s surface `surface` restated as record `#9000`,
/// `record` its text: the face's reference moved to it.
fn restated(face: &str, surface: &str, record: &str) -> String {
    let text = sg1();
    let old = format!("{face}=ADVANCED_FACE('PartBody',(#");
    let at = text.find(&old).expect("the face");
    let end = at + text[at..].find(';').unwrap();
    let line = &text[at..end];
    let moved = line.replace(&format!(",{surface},"), ",#9000,");
    assert_ne!(line, moved, "the face states {surface}");
    let text = text.replacen(line, &moved, 1);
    let data_end = text.rfind("ENDSEC;").unwrap();
    format!(
        "{}#9000={record} ;\n{}",
        &text[..data_end],
        &text[data_end..]
    )
}

fn import(text: &str) -> Result<StepImport, StepImportError> {
    import_step(text, &ImportOptions::default(), Tol::witness())
}

/// The one-record joins an import reports, with each move.
fn joins(imported: &StepImport) -> Vec<(u64, u64, f64)> {
    imported
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
        .collect()
}

/// The shipped body's (faces, edges, vertices) and its distinct surfaces.
fn shape(imported: &StepImport) -> (usize, usize, usize, usize) {
    let StepImport::Solid { body, .. } = imported else {
        panic!("a solid")
    };
    let surfaces: std::collections::BTreeSet<_> = body.faces().map(|(_, f)| f.surface).collect();
    (
        body.faces().count(),
        body.edges().count(),
        body.vertices().count(),
        surfaces.len(),
    )
}

const CYLINDER_40: &str = "CYLINDRICAL_SURFACE('generated cylinder',#93,40.)";

/// A second record of the 40 mm cylinder, exact or off by 2 µm in its
/// radius (inside the band's zero, 5 µm), joins the earlier record: the
/// body is the file's own, on as many surfaces, and the join is reported
/// with its move.
#[test]
fn a_cylinder_restated_within_the_zero_joins_the_earlier_record() {
    let base = shape(&import(&sg1()).unwrap());
    for (radius, moved) in [("40.", 0.0), ("40.002", 2e-6)] {
        let record = CYLINDER_40.replace("40.)", &format!("{radius})"));
        let imported = import(&restated("#146", "#94", &record))
            .unwrap_or_else(|e| panic!("radius {radius}: imports: {}", e.render()));
        assert_eq!(
            shape(&imported),
            base,
            "radius {radius}: the file's own body"
        );
        let joins = joins(&imported);
        assert_eq!(joins.len(), 1, "radius {radius}: one join: {joins:?}");
        let (kept, joined, moved_by) = joins[0];
        assert_eq!(
            (kept, joined),
            (94, 9000),
            "radius {radius}: the earlier record kept"
        );
        assert!(
            (moved_by - moved).abs() < 1e-9,
            "radius {radius}: the move is the radius's ({moved_by})"
        );
    }
}

/// The cone restated joins likewise.
#[test]
fn a_cone_restated_joins_the_earlier_record() {
    let imported = import(&restated(
        "#189",
        "#151",
        "CONICAL_SURFACE('Cone',#150,20.,0.278299659005)",
    ))
    .unwrap_or_else(|e| panic!("imports: {}", e.render()));
    assert_eq!(shape(&imported), shape(&import(&sg1()).unwrap()));
    let joins: Vec<_> = joins(&imported).iter().map(|j| (j.0, j.1)).collect();
    assert_eq!(joins, [(151, 9000)]);
}

/// A radius 20 µm off is in the band (5 µm, 50 µm): the file does not
/// say whether the two records are one, so the import refuses on the
/// one-record reading, naming the later record.
#[test]
fn a_cylinder_restated_in_the_band_refuses_as_the_files_ambiguity() {
    let record = CYLINDER_40.replace("40.)", "40.02)");
    match import(&restated("#146", "#94", &record)) {
        Err(StepImportError::Interpretation {
            id,
            refusal: InterpretationRefusal::Ambiguous { reading, gap, .. },
            ..
        }) => {
            assert_eq!((id, reading), (9000, "step_one_record"));
            assert!((gap - 2e-5).abs() < 1e-9, "the gap is the radius's: {gap}");
        }
        other => panic!("an ambiguity on the one-record reading: {other:?}"),
    }
}

/// A radius 1 mm off reads apart: the two records stay two, and nothing
/// joins them (whatever the build then makes of the two surfaces).
#[test]
fn a_cylinder_restated_apart_is_never_joined() {
    let record = CYLINDER_40.replace("40.)", "41.)");
    match import(&restated("#146", "#94", &record)) {
        Ok(imported) => assert!(
            joins(&imported).is_empty(),
            "no join: {:?}",
            joins(&imported)
        ),
        Err(e) => assert!(
            !e.render().contains("step_one_record"),
            "not refused on the one-record reading: {}",
            e.render()
        ),
    }
}

/// The 40 mm cylinder's faces' axial middle, in the file's millimetres:
/// the middle of the `z` its points on the cylinder span.
fn faces_middle_z() -> f64 {
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for line in sg1().lines().filter(|l| l.contains("=CARTESIAN_POINT(")) {
        let inner = &line[line.rfind('(').unwrap() + 1..line.find(')').unwrap()];
        let xyz: Vec<f64> = inner
            .split(',')
            .filter_map(|c| c.trim().parse().ok())
            .collect();
        if xyz.len() == 3 && (xyz[0].hypot(xyz[1]) - 40.0).abs() < 1e-6 {
            lo = lo.min(xyz[2]);
            hi = hi.max(xyz[2]);
        }
    }
    assert!(lo < hi, "the cylinder's faces have points");
    0.5 * (lo + hi)
}

/// `sg1` with face `#146` on a 40 mm cylinder record `#9000` placed at
/// `origin` along `dir` (millimetres; `dir` need not be unit).
fn restated_on_axis(origin: [f64; 3], dir: [f64; 3]) -> String {
    let text = restated("#146", "#94", "CYLINDRICAL_SURFACE('',#9001,40.)");
    let data_end = text.rfind("ENDSEC;").unwrap();
    let p = |v: [f64; 3]| format!("({:?},{:?},{:?})", v[0], v[1], v[2]);
    format!(
        "{}#9001=AXIS2_PLACEMENT_3D('',#9002,#9003,#9004) ;\n#9002=CARTESIAN_POINT('',{}) ;\n\
         #9003=DIRECTION('',{}) ;\n#9004=DIRECTION('',(1.,0.,0.)) ;\n{}",
        &text[..data_end],
        p(origin),
        p(dir),
        &text[data_end..]
    )
}

/// **Decided where the faces are, never at the stored axis point.** Each
/// restatement places its axis point 1 m along the axis from the faces:
///
/// - the axis through a point 1 m along the old axis and 30 µm off it at
///   the faces: in the band (5 µm, 50 µm) where the faces are, whatever it
///   reads at its stored point, so the file's ambiguity; 54 µm off, apart;
/// - the axis point 30 µm off the old axis 1 m along it, the two axes
///   meeting at the faces: one locus there, joined;
/// - the axis tilted 3e-4 about a point 1 m along the old axis: 300 µm
///   apart at the faces. Apart is never joined and never the file's
///   ambiguity.
#[test]
fn two_axes_are_read_apart_where_the_faces_are() {
    let z = faces_middle_z();
    let far = z + 1000.0;
    for off in [0.03] {
        match import(&restated_on_axis([0.0, 0.0, far], [-off, 0.0, 1000.0])) {
            Err(StepImportError::Interpretation {
                id,
                refusal: InterpretationRefusal::Ambiguous { reading, gap, .. },
                ..
            }) => {
                assert_eq!((id, reading), (9000, "step_one_record"), "{off} mm off");
                assert!(
                    (gap - off * 1e-3).abs() < 0.2 * off * 1e-3,
                    "{off} mm off at the faces: {gap:e}"
                );
            }
            other => panic!("{off} mm off at the faces: the file's ambiguity, got {other:?}"),
        }
    }
    let imported = import(&restated_on_axis([0.03, 0.0, far], [0.03, 0.0, 1000.0]))
        .unwrap_or_else(|e| panic!("meeting at the faces: imports: {}", e.render()));
    let joined: Vec<_> = joins(&imported).iter().map(|j| (j.0, j.1)).collect();
    assert_eq!(joined, [(94, 9000)], "meeting at the faces: one locus");
    for (what, origin, dir) in [
        (
            "54 µm off at the faces",
            [0.0, 0.0, far],
            [-0.054, 0.0, 1000.0],
        ),
        ("tilted 3e-4", [0.0, 0.0, far], [0.3, 0.0, 1000.0]),
    ] {
        match import(&restated_on_axis(origin, dir)) {
            Ok(imported) => assert!(joins(&imported).is_empty(), "{what}: never joined"),
            Err(e) => assert!(
                !e.render().contains("step_one_record"),
                "{what}: apart, not the one-record reading: {}",
                e.render()
            ),
        }
    }
}

/// **A cone's tilt is levered at its faces' reach from its apex.** The
/// cone `#151` restated on its own placement point turned 2e-5 rad:
/// its apex, some 70 mm away, moves 1.4 µm and its faces, within about
/// 0.1 m of it, by the tilt over that reach, inside the band's zero
/// (5 µm), so the two records are one.
#[test]
fn a_cone_turned_by_a_small_angle_joins_at_its_faces_reach() {
    let text = restated(
        "#189",
        "#151",
        "CONICAL_SURFACE('Cone',#9001,20.,0.278299659005)",
    );
    let data_end = text.rfind("ENDSEC;").unwrap();
    let text = format!(
        "{}#9001=AXIS2_PLACEMENT_3D('',#9002,#9003,#9004) ;\n\
         #9002=CARTESIAN_POINT('',(0.,0.,20.)) ;\n\
         #9003=DIRECTION('',(2.E-05,0.,-1.)) ;\n\
         #9004=DIRECTION('',(-0.479425538604,0.87758256189,0.)) ;\n{}",
        &text[..data_end],
        &text[data_end..]
    );
    let imported = import(&text).unwrap_or_else(|e| panic!("imports: {}", e.render()));
    let joined = joins(&imported);
    assert_eq!(joined.len(), 1, "one join: {joined:?}");
    let (kept, record, moved_by) = joined[0];
    assert_eq!((kept, record), (151, 9000));
    assert!(
        moved_by > 1e-6 && moved_by < 5e-6,
        "the apex's move and the tilt over the faces' reach: {moved_by:e}"
    );
}
