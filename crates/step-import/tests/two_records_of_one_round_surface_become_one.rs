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
    format!("{}#9000={record} ;\n{}", &text[..data_end], &text[data_end..])
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
        assert_eq!(shape(&imported), base, "radius {radius}: the file's own body");
        let joins = joins(&imported);
        assert_eq!(joins.len(), 1, "radius {radius}: one join: {joins:?}");
        let (kept, joined, moved_by) = joins[0];
        assert_eq!((kept, joined), (94, 9000), "radius {radius}: the earlier record kept");
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
        Ok(imported) => assert!(joins(&imported).is_empty(), "no join: {:?}", joins(&imported)),
        Err(e) => assert!(
            !e.render().contains("step_one_record"),
            "not refused on the one-record reading: {}",
            e.render()
        ),
    }
}
