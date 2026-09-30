//! namedepth-r2: saved bytes, canonical bytes and pins of every corpus
//! and committed document, and of crafted deep-name documents, written
//! to PROBE_DIR for a byte diff between main and the head.
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(unused)]

use crate::corpus;
use crate::fixture;

use editor_core::{
    EntityKind, EvalOptions, NameRef, Node, PatternKind, ProfileDoc, RecipeNodeId, RoleSeg,
    StableName, canonical_bytes, content_pin, load, save,
};
use fixture::{in_copy, insert, len, on_frame, square};
use geom_core::Tol;

fn dir() -> std::path::PathBuf {
    let d = std::path::PathBuf::from(std::env::var("PROBE_DIR").expect("PROBE_DIR"));
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn dump(label: &str, doc: &ProfileDoc, edits: &[editor_core::LoggedEdit<editor_core::ProfileProgram>]) {
    let tol = Tol::witness();
    let d = dir();
    let save_text = match save(doc, edits, tol) {
        Ok(t) => t,
        Err(e) => format!("ERR {e:?}"),
    };
    std::fs::write(d.join(format!("{label}.save")), &save_text).unwrap();
    let canon = match canonical_bytes(doc, tol) {
        Ok(b) => b,
        Err(e) => format!("ERR {e:?}").into_bytes(),
    };
    std::fs::write(d.join(format!("{label}.canon")), &canon).unwrap();
    let pin = match content_pin(doc, tol) {
        Ok(p) => format!("{p:?}"),
        Err(e) => format!("ERR {e:?}"),
    };
    std::fs::write(d.join(format!("{label}.pin")), pin).unwrap();
    // Load what was saved, and re-save.
    let back = match load(&save_text, tol) {
        Ok(l) => {
            let again = save(&l.snapshot, &l.edits, tol).map_err(|e| format!("{e:?}"));
            let same_doc = l.doc == *doc;
            format!("OK same_doc={same_doc} resave_equal={}", again.as_deref() == Ok(save_text.as_str()))
        }
        Err(e) => format!("ERR {e}"),
    };
    std::fs::write(d.join(format!("{label}.load")), back).unwrap();
}

fn big<R: Send + 'static>(f: impl FnOnce() -> R + Send + 'static) -> R {
    std::thread::Builder::new().stack_size(1 << 30).spawn(f).unwrap().join().unwrap()
}

#[test]
fn zz_r2_corpus_bytes() {
    big(|| {
        for c in corpus::documents() {
            dump(&format!("corpus-{}", c.name), &c.doc, &[]);
        }
        // Committed files: load, record, re-save.
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let files = [
            "crates/editor-core/tests/golden/golden.cad",
            "crates/editor-core/tests/corpus/die_tool.pncad",
            "crates/editor-core/tests/corpus/tour/die_composed_tour.pncad",
            "crates/pncad/tests/plate_param.pncad",
            "crates/viewer/tests/gallery_ring.pncad",
        ];
        let mut all: Vec<String> = files.iter().map(|s| s.to_string()).collect();
        for v in 1..=19 {
            all.push(format!("crates/editor-core/tests/bool13_goldens/v{v}_golden.cad"));
        }
        for f in all {
            let text = std::fs::read_to_string(root.join(&f)).unwrap();
            let label = f.replace('/', "_");
            match load(&text, Tol::witness()) {
                Ok(l) => {
                    std::fs::write(dir().join(format!("file-{label}.fileload")), "OK").unwrap();
                    dump(&format!("file-{label}"), &l.snapshot, &l.edits);
                    dump(&format!("file-{label}-cur"), &l.doc, &[]);
                }
                Err(e) => std::fs::write(dir().join(format!("file-{label}.fileload")), format!("ERR {e}")).unwrap(),
            }
        }
    });
}

/// A square extrude, then k patterns.
fn chain(label: &str, k: usize) -> (ProfileDoc, RecipeNodeId, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, profile) = on_frame(doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![square(0.0, 0.0, 1.0)]);
    let (mut doc, extrude) = insert(doc, Node::Extrude { profile, distance: len(1.0) });
    let mut patterns = Vec::new();
    let mut input = extrude;
    for _ in 0..k {
        let (next, pattern) = insert(
            doc,
            Node::Pattern {
                input,
                count: editor_core::Expr::count(1),
                kind: PatternKind::Linear {
                    direction: [fixture::scl(1.0), fixture::scl(0.0), fixture::scl(0.0)],
                    spacing: len(2.0),
                },
            },
        );
        doc = next;
        patterns.push(pattern);
        input = pattern;
    }
    (doc, extrude, patterns)
}

fn wrap(depth: usize, node: RecipeNodeId, bottom: StableName, seg: fn(NameRef) -> RoleSeg) -> StableName {
    (0..depth).fold(bottom, |n, _| StableName { kind: n.kind, node, path: vec![seg(NameRef::new(n))] })
}

/// Crafted deep-name documents: a fillet over a pattern chain's deep
/// edge, and fillets whose selection is a crafted InPart / FromA tower.
#[test]
fn zz_r2_deep_bytes() {
    big(|| {
        for k in [5usize, 20, 40, 100] {
            let (doc, extrude, patterns) = chain(&format!("deep-{k}"), k);
            let top = *patterns.last().unwrap();
            let rim = fixture::rim_edge(extrude, editor_core::CapEnd::End, fixture::piece(&doc, extrude, 0, 0));
            let deep = patterns.iter().fold(rim.clone(), |n, &p| in_copy(p, 0, n));
            let (d1, _) = insert(doc.clone(), Node::fillet(top, len(0.1), vec![deep.clone()]));
            dump(&format!("deep-pattern-{k}"), &d1, &[]);
            // A crafted FromA / InPart tower in a fillet over the extrude.
            let from_a = wrap(k, extrude, rim.clone(), RoleSeg::FromA);
            let in_part = wrap(k, extrude, rim.clone(), |r| RoleSeg::InPart { of: r });
            let merged = StableName { kind: EntityKind::Edge, node: extrude, path: vec![RoleSeg::Merged(vec![from_a.clone(), in_part.clone()])] };
            for (tag, n) in [("froma", from_a), ("inpart", in_part), ("merged", merged)] {
                let step = editor_core::DocEdit::InsertNode { node: Node::fillet(extrude, len(0.1), vec![n]) };
                match editor_core::apply(&doc, &step, Tol::witness(), &editor_core::RefusingReach) {
                    Ok(out) => dump(&format!("deep-{tag}-{k}"), &out.doc, &[]),
                    Err(e) => std::fs::write(dir().join(format!("deep-{tag}-{k}.refused")), format!("{e:?}")).unwrap(),
                }
            }
        }
    });
}
