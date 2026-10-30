//! Review probe for PR 6081: `Doc::unused` over the gallery documents.

use pncad::geom_core::Tol;

#[test]
fn probe_gallery_unused_count() {
    use demo_tour::*;
    let tol = Tol::witness();
    let docs = [
        ("bracket", bracket::gallery_document(tol)),
        ("checks", checks::gallery_document(tol)),
        ("ring", ring::gallery_document(tol)),
        ("diefillet", diefillet::gallery_document(tol)),
        ("heatsink", heatsink::gallery_document(tol)),
        ("teapot", teapot::gallery_document(tol)),
        ("impeller", impeller::gallery_document(tol)),
        ("plate", plate::gallery_document(tol)),
        ("chain", chain::gallery_document(tol)),
    ];
    let mut total = 0;
    for (name, doc) in &docs {
        let u = doc.unused();
        let kinds: Vec<_> = u
            .operations
            .iter()
            .map(|id| doc.node(*id).map(std::mem::discriminant))
            .collect();
        eprintln!("{name}: ops={kinds:?} vars={:?}", u.vars);
        total += u.operations.len() + u.vars.len();
    }
    eprintln!("gallery: {total} findings over {} documents", docs.len());
}

/// The checks narration's exact counts, under the new default.
#[test]
fn probe_checks_narration_still_holds() {
    demo_tour::checks::narration(Tol::witness());
}
