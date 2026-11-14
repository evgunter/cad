//! **The blend reads a requested edge's wedge through the one fold.**
//!
//! The edge's wedge, its class at every station, each corner's sign, a
//! tangency's pairing and end, is read once, by `geom_brep`'s fold
//! through `topo::edge_wedge_as`, and the blend composes none of it
//! itself. The token methods a composition would call
//! (`Corner::convexity`, `Tangency::pairing`) and `normals_alignment`
//! are crate-private to `geom_brep`, so the compiler keeps those out;
//! this row pins the rest. The blend's production code calls the door
//! once, spells no other wedge read (a class, a gradient or normal
//! alignment, a jet, a material curvature, a walk or a station minted by
//! hand), and reads its faces' outward normals at exactly the three
//! sites that read the ARM's geometry (`battery::outward` and the
//! link's two normals for `classify_arm`), none of which decides a sign.
//! The `#[cfg(test)]` modules are blanked first: a row there naming a
//! read is not a second spelling.

#![allow(clippy::unwrap_used, clippy::expect_used)]

/// The code-and-literals view with every `#[cfg(test)] mod … { … }`
/// blanked.
fn production(text: &str) -> String {
    use test_utils::source::{balanced_end, code_and_literals, code_only, skip_ws, word_at};
    let code = code_only(text);
    let mut kept = code_and_literals(text).into_bytes();
    assert_eq!(code.len(), kept.len(), "the two views align byte for byte");
    let mut from = 0;
    while let Some(at) = code[from..].find("#[cfg(test)]").map(|i| i + from) {
        let mut item = skip_ws(&code, at + "#[cfg(test)]".len());
        while code[item..].starts_with("#[") {
            let attr_end = balanced_end(&code, item + 1).expect("an attribute closes");
            item = skip_ws(&code, attr_end + 1);
        }
        from = item;
        if !word_at(&code, item, "mod") {
            continue;
        }
        let open = item + code[item..].find('{').expect("a test module has a body");
        let close = balanced_end(&code, open).expect("a test module closes");
        kept[at..=close].fill(b' ');
        from = close;
    }
    String::from_utf8(kept).expect("only ASCII spans are blanked")
}

#[test]
fn the_blend_composes_no_wedge_of_its_own() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/blend");
    let wedge_reads = [
        "classify_dihedral",
        "classify_seam(",
        "classify_material_pairing",
        "gradient_alignment(",
        "normals_alignment(",
        "tangent_jet(",
        "tangent_second_order(",
        "material_kappa_rel(",
        "implicit_gradient(",
        "folded_lever_arm(",
        "edge_wedge_over",
        "WedgeWalk::",
        "Station::on(",
        "DIHEDRAL_SIGN",
        "\"dihedral_sign\"",
    ];
    let mut door = 0;
    let mut normals = 0;
    let files = test_utils::source::rust_sources(&dir);
    assert!(files.len() > 5, "the blend's sources are found: {files:?}");
    for file in files {
        let source = production(&std::fs::read_to_string(&file).unwrap());
        for read in wedge_reads {
            assert!(
                !source.contains(read),
                "`{}` spells `{read}`: the blend reads an edge's wedge through \
                 `topo::edge_wedge_as` alone",
                file.display()
            );
        }
        door += source.matches("topo::edge_wedge_as(").count();
        door += source.matches("topo::edge_wedge(").count();
        normals += source.matches("implicit_outward_normal(").count();
    }
    assert_eq!(door, 1, "the blend asks the edge's wedge door once");
    assert_eq!(
        normals, 3,
        "the blend reads outward normals only at its arm's three sites"
    );
}
