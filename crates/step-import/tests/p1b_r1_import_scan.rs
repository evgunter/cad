//! **PCURVE P-1b R1 review probe** (blinded reviewer R1, ordinal 201):
//! every committed solid fixture is imported and its body scanned for
//! scaffold descriptions AT REST, independently of tier 3's own fence
//! — a scan hit here with a clean import is a fence hole (the import
//! gate runs the shared at-rest validation), and a scan hit that the
//! import gate refused would have failed `import_body` already.
//!
//! The adoption ladder's conventional rung now states a chart image
//! with the pushforward demoted to the authority record; this row also
//! counts the fixtures where that record is populated, so a future
//! change that silently stops declaring shows up as a census move.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use common::{SOLID_FIXTURES, import_body};

use geom_brep::EdgeDescription;
use topo::CurveGeom;

#[test]
fn no_committed_fixture_imports_with_a_scaffold_at_rest() {
    let mut declared_fixtures = 0usize;
    for name in SOLID_FIXTURES {
        let (body, _eps) = import_body(name);
        let scaffolds: Vec<_> = body
            .edges()
            .filter(|(_, e)| {
                matches!(
                    body.get_curve_geom(e.curve)
                        .and_then(CurveGeom::certified)
                        .map(topo::EdgeCurve::description),
                    Some(EdgeDescription::Scaffold(_))
                )
            })
            .map(|(k, _)| k)
            .collect();
        assert!(
            scaffolds.is_empty(),
            "{name}: imported body carries scaffold descriptions at rest: {scaffolds:?}"
        );
        if body.edges().any(|(_, e)| {
            body.get_curve_geom(e.curve)
                .and_then(CurveGeom::certified)
                .map(|c| c.authority().is_declared())
                .unwrap_or(false)
        }) {
            declared_fixtures += 1;
        }
    }
    // The adoption ladder's conventional rung declares; a corpus this
    // wide adopting NOTHING conventionally would mean the rung is dead.
    assert!(
        declared_fixtures > 0,
        "no fixture imported with a declared edge — the conventional rung never fired"
    );
}
