//! The demo-document gallery: every document-authored scene, saved as
//! a `.pncad` a person can open in the viewer.
//!
//! # What this mode is for
//!
//! The GUI's acceptance shape is "open the application and load one of
//! the existing demo documents through a file dialog". That needs the
//! documents to exist as files. This mode writes them, from the SAME
//! authoring functions the tour renders — never a second authoring,
//! which would make the gallery evidence about itself rather than
//! about the scenes (`memories/demo-purpose.md`).
//!
//! # Which scenes are here, and which are not
//!
//! The document-authored scenes are the ones that build a `Doc` and
//! evaluate it: **checks, ring, diefillet, heatsink**, plus
//! **assembly**, whose documents are a workspace of several files and
//! are written by that scene's own store. The rest of the tour drives
//! the kernel API directly and has no document to save; they join the
//! gallery as they are re-authored, which is per-scene library work
//! and independent of the GUI.
//!
//! Everything here goes through the public doors: author, then
//! `pncad::document::save`, whose own validation is what decides
//! whether a scene's document is writable at all.

use std::path::{Path, PathBuf};

use pncad::document::{
    CancelToken, CheckId, ChecksConfig, EvalOptions, ProfileDoc, evaluate, run_checks, save,
};
use pncad::geom_core::Tol;

/// Write the gallery into `dir` (default `gallery/`).
pub fn run(dir: Option<String>, tol: Tol) {
    let dir = PathBuf::from(dir.unwrap_or_else(|| "gallery".to_owned()));
    std::fs::create_dir_all(&dir).expect("create the gallery directory");

    println!("demo-document gallery → {}", dir.display());
    let mut written = 0usize;
    for (name, doc) in [
        ("checks", crate::checks::gallery_document(tol)),
        ("ring", crate::ring::gallery_document(tol)),
        ("diefillet", crate::diefillet::gallery_document(tol)),
        ("heatsink", crate::heatsink::gallery_document(tol)),
    ] {
        write_one(&dir, name, &doc, tol);
        written += 1;
    }

    // The assembly scene's documents are a WORKSPACE — several files
    // that reference one another by content pin — so they are written
    // by that scene's own store rather than by the loop above. Running
    // its stops is what populates it, exactly as the ordinary tour run
    // does.
    let work = dir.join("assembly");
    let stops = crate::assembly::stops(&work, tol);
    let assembly_files = std::fs::read_dir(&work)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|e| e.path().extension().is_some_and(|x| x == "pncad"))
                .count()
        })
        .unwrap_or(0);
    println!(
        "   assembly/: {assembly_files} document(s) from {} stop(s)",
        stops.len()
    );
    println!(
        "gallery complete: {} standalone document(s) + {assembly_files} assembly document(s)",
        written
    );
    println!("   open one with: cargo run -p viewer --features app  (Open… in the toolbar)");
}

/// Save one document, loudly.
///
/// `save` validates before it writes — a document it refuses is a
/// finding about that scene, not something to skip past, so this
/// panics rather than logging and continuing.
fn write_one(dir: &Path, name: &str, doc: &ProfileDoc, tol: Tol) {
    // An empty edit log: the gallery ships the STATE, and a viewer
    // that opens it starts its own history from this snapshot.
    let text = save(doc, &[], tol)
        .unwrap_or_else(|error| panic!("the {name} document does not save: {error:?}"));
    let path = dir.join(format!("{name}.pncad"));
    std::fs::write(&path, &text)
        .unwrap_or_else(|error| panic!("cannot write {}: {error}", path.display()));
    println!(
        "   {name}.pncad — {} node(s), {} product root(s), {} byte(s){}",
        doc.len(),
        doc.roots().len(),
        text.len(),
        advisory(doc, tol)
    );
}

/// What the advisory registry says about the document being written,
/// as a phrase for the line above.
///
/// **Why the exporter says it at all.** A gallery document's whole job
/// is to be opened in the viewer, and the viewer runs this same
/// registry on every landing — so a scene whose document reports a
/// finding ships a picture that is wrong in a way only the badge
/// explains. The die shipped exactly that for as long as this exporter
/// has existed: two product roots, one sitting on the other, the pips
/// filled in and the outer faces z-fighting (#1162 diagnosed it; its
/// separation resident is what reports it). Writing the count here
/// means the next one is noticed when it is WRITTEN rather than when
/// someone opens the file and wonders what they are looking at.
///
/// A count, never a verdict: the registry reports, it does not gate
/// (`editor_core::checks`), and the one scene that legitimately
/// reports today — the heatsink, whose fins are unioned into its base
/// in this demo's own `solidify()` and never in the recipe — is a
/// scene-authoring gap, not a reason to refuse to write its file.
fn advisory(doc: &ProfileDoc, tol: Tol) -> String {
    let evaluation = evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default(), tol);
    match run_checks(doc, &evaluation, &ChecksConfig::default(), tol) {
        Err(error) => format!(" — the check registry refused: {error}"),
        Ok(report) if report.findings.is_empty() => String::new(),
        Ok(report) => {
            let separation = report
                .findings
                .iter()
                .filter(|finding| finding.check == CheckId::Separation)
                .count();
            format!(
                " — {} finding(s), {separation} of them separation",
                report.findings.len()
            )
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use pncad::topo::mass_properties;

    /// What one gallery scene is expected to PRODUCE.
    struct Shape {
        name: &'static str,
        doc: ProfileDoc,
        /// Product roots. One, unless the scene has a reason.
        roots: usize,
        /// Separation findings, and why any of them are there.
        separation: usize,
        why: &'static str,
    }

    /// **A gallery document must denote what its scene means, or say
    /// why not.**
    ///
    /// The gallery's whole purpose is to be opened in the viewer, and
    /// the viewer draws the PRODUCT: the gather of every root. A scene
    /// that authors a body for narration authors a DAG sink, the root
    /// set is exactly the sink set (`editor_core::roots`), and a sink
    /// nobody meant as a product is a second body in the picture.
    ///
    /// That is the bug this row exists for. `diefillet` shipped with
    /// two roots — the blank and the composed die, the blank being the
    /// die's own outer shape with no pips cut — so the file drew one
    /// die-shaped thing with its pips filled in, its faces z-fighting,
    /// and twice the material (115 faces, V = 1.918146). It looked
    /// almost right, which is why it survived: every local battery
    /// passes, because each root's body is individually perfect.
    ///
    /// The row is a TABLE rather than a blanket "no findings", because
    /// one scene legitimately reports and hiding that would be the
    /// same silence in a different place. Changing any number here is
    /// a claim about what a scene produces, so it should cost a
    /// sentence in `why`.
    #[test]
    fn each_gallery_document_denotes_its_scene_or_says_why_not() {
        let tol = Tol::witness();
        let shapes = [
            Shape {
                name: "checks",
                doc: crate::checks::gallery_document(tol),
                roots: 1,
                separation: 0,
                why: "one root; its connectedness finding is the scene's own subject                       and is not a separation one",
            },
            Shape {
                name: "ring",
                doc: crate::ring::gallery_document(tol),
                roots: 1,
                separation: 0,
                why: "one revolve, one root",
            },
            Shape {
                name: "diefillet",
                doc: crate::diefillet::gallery_document(tol),
                roots: 1,
                separation: 0,
                why: "the composed die alone — the blank is a narration body and                       `gallery_document` deletes it, which is what this row guards",
            },
            Shape {
                name: "heatsink",
                doc: crate::heatsink::gallery_document(tol),
                roots: 1,
                separation: 0,
                why: "ONE root since the PlacedUnion migration (#1344), and this row is                       where that shows. It used to read two roots and five separation                       findings, and said so as a KNOWN defect: the base and the fin                       pattern genuinely interpenetrated, because the union making them                       one solid lived in this demo's `solidify()` and never in the                       recipe. The fix that row named — author the union — is exactly                       what landed: `PlacedUnion(fin, Linear) -> Boolean(Union, base,                       group)`, so the document has one root and nothing in it                       interpenetrates",
            },
        ];

        for shape in &shapes {
            let evaluation = evaluate::<f64>(
                &shape.doc,
                None,
                &CancelToken::new(),
                &EvalOptions::default(),
                tol,
            );
            assert_eq!(
                shape.doc.roots().len(),
                shape.roots,
                "{}: product roots ({})",
                shape.name,
                shape.why
            );
            let report = run_checks(&shape.doc, &evaluation, &ChecksConfig::default(), tol)
                .unwrap_or_else(|error| panic!("{}: the registry refused: {error}", shape.name));
            let separation = report
                .findings
                .iter()
                .filter(|finding| finding.check == CheckId::Separation)
                .count();
            assert_eq!(
                separation, shape.separation,
                "{}: separation findings ({}) — {report}",
                shape.name, shape.why
            );
        }
    }

    /// The die, by the numbers its own scene already knows.
    ///
    /// `diefillet::stops` asserts the composed BODY is 26 + 21·3 faces
    /// and tier-3 valid. This asserts the same thing one level up, of
    /// the document's PRODUCT — which is the thing the viewer draws
    /// and the thing that was wrong. The volume is the composed die's,
    /// not twice it.
    #[test]
    fn the_die_document_produces_one_die() {
        let tol = Tol::witness();
        let doc = crate::diefillet::gallery_document(tol);
        let evaluation = evaluate::<f64>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        let product =
            pncad::document::product(&doc, &evaluation, tol).expect("the die document gathers");
        assert_eq!(
            product.faces().count(),
            26 + 21 * 3,
            "the product is the composed die's own face count, not it plus a blank"
        );
        let volume = mass_properties(&product, tol)
            .expect("the product has mass properties")
            .volume;
        // MEASURED, not a closed form: the scene closes the BLANK's
        // volume in `diefillet::blank_volume` (core + slabs + quarter
        // cylinders + octants), and the composed die — 21 spherical
        // caps out, 21 torus bands in — has no such form written
        // anywhere, so this is a pin on the value the gather answers.
        // What makes it worth pinning is the defect's signature: with
        // the blank still a root the product answered 1.918146, which
        // is this number DOUBLED, because the same material was
        // gathered twice.
        let want = 0.952_914_984_014_647_f64;
        assert!(
            (volume - want).abs() < 1.0e-9 * want,
            "the product's volume is the composed die's: {volume} vs {want}"
        );
    }
}
