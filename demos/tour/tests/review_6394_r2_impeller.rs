//! Review probe (PR 6394, lane placing-dual-6394-r2, claim 8): the
//! impeller's evaluated geometry, dumped so the frozen head and main can
//! be compared numerically: every evaluated body's volume and its vertex
//! points, in full precision. Writes to `$REVIEW_6394_DUMP` when set.

use pncad::document::{
    BooleanValue, CancelToken, EvalOptions, Evaluation, ValuePayload, evaluate,
};
use pncad::geom_core::Tol;

#[test]
fn impeller_geometry_dump() {
    let tol = Tol::witness();
    let doc = demo_tour::impeller::gallery_document(tol);
    let ev: Evaluation<f64> = evaluate(&doc, None, &CancelToken::new(), &EvalOptions::default(), tol);
    let mut out = String::new();
    for id in doc.ids() {
        let Some(value) = ev.value(id) else { continue };
        let ValuePayload::Boolean(BooleanValue::Body { body, .. }) = &value.payload else {
            continue;
        };
        let body = body.body();
        let vol = pncad::topo::mass_properties(
            &demo_tour::booleans::at_rest(body, tol),
            tol,
            demo_tour::printed(),
        )
        .map(|m| m.enclosure.volume);
        let mut pts: Vec<[f64; 3]> = body.vertex_points().map(|(_, p)| [p.x, p.y, p.z]).collect();
        pts.sort_by(|a, b| a.partial_cmp(b).unwrap());
        out.push_str(&format!("node {id:?} volume {vol:?} vertices {}\n", pts.len()));
        for p in pts {
            out.push_str(&format!("  {:?}\n", p));
        }
    }
    assert!(!out.is_empty(), "the impeller evaluated no body");
    if let Ok(path) = std::env::var("REVIEW_6394_DUMP") {
        std::fs::write(path, &out).unwrap();
    }
}
