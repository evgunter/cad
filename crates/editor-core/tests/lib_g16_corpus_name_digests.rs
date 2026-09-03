//! **Per-document NAME-TABLE digests over the whole corpus registry,
//! pinned** — adopted from the LIB-G16 R2 review probe, which printed
//! them for a cross-tree comparison and so expired with it
//! (`memories/test-suite-cost.md`).
//!
//! What earns it a permanent seat is the hole it fills. Two corpus-wide
//! goldens already exist and neither is this one: `m10_p_fence` pins
//! every body POINT's bits and says nothing about names, and
//! `m4_pr3_names_ci` pins a name digest for the die FIXTURE only. So
//! until now no committed number covered "the registry's name tables",
//! which is exactly the surface an emitter change moves — LIB-G16
//! re-shaped `emit_fillet` onto the shared tie deferral and had to
//! measure that claim by hand, against a checkout of main, because
//! nothing in the tree would have caught it.
//!
//! It is a golden in the ordinary sense: when one of these moves the
//! question is whether the NEW names are right, never how to restore
//! the old number. A document added to or removed from the registry
//! moves its own row and no other, which is what makes a diff here
//! readable.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

/// FNV-1a 64 over the tables' deterministic Debug encoding — copied
/// from `m4_pr3_names_ci.rs::digest` so the number is comparable.
fn digest(ev: &editor_core::Evaluation<f64>) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |s: &str| {
        for b in s.bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x1000_0000_01b3);
        }
    };
    for id in &ev.order {
        feed(&format!("#{id:?}"));
        if let Some(v) = ev.value(*id) {
            for (n, e) in v.name_table.iter() {
                feed(&format!("{n:?}={e:?};"));
            }
        }
    }
    h
}

/// The pinned digest per registered document, in registry order.
///
/// `die_composed_tour` is the row this file was built to be able to
/// take. The measurement that motivated the gate — "do two documents
/// with the same recipe have the same name table" — had to be made by
/// hand against the demo tour's composed die, because that document
/// lived only in `demos/tour` and no registered document was its
/// equal. LIB-CORPUS-DIE registered it (as committed bytes the tour
/// regenerates — `corpus::die_composed_tour`), so the hand
/// measurement is now this number: forty-two rim arcs and twelve box
/// edges named through twenty pairwise unions, in one pin.
///
/// Two more rows are worth a reader's second look, and neither is a bug.
///
/// `die` is `0xd77331053f8a87fe` — the same number
/// `m4_pr3_names_ci::DIE_TABLE_DIGEST` carries, because it is the same
/// digest of the same tables. The two pins agreeing is a cross-check,
/// not a duplication: that one covers the die FIXTURE through its own
/// bump rows, this one covers the registry.
///
/// `measured_web` (M10-2) was ADDED to this table when its document
/// joined the registry, and the add is the header's "one row moves"
/// claim being MEASURED rather than restated: the re-cut printed
/// nineteen numbers identical to the ones already pinned here and one
/// new row. A measurement sink denotes no body and mints no name of
/// its own, so it moves neither the geometry fence nor any other
/// document's names.
///
/// `die_composed` and `die_composed_tour` are the only registered
/// documents that carve a CLOSED chain, so they are the only two whose
/// tables carry the rim-phase roles at all (four band trimlines and
/// eighty-four respectively; every other row's tables have none). A
/// change to the rim vocabulary therefore moves exactly these two
/// numbers, and a change that moves a third is not about rims.
///
/// That census is not prose to be trusted: it is computed by
/// `blend5_r1_probes::the_recorded_band_trim_counts_are_executable`
/// (and again by `blend5_r2_probes`'s), which walks the same registry
/// and fails if any of the three numbers drifts. Read the claim here,
/// believe it there.
///
/// `die_fillet` and `die_chamfer` are IDENTICAL, and that is what
/// RECIPE-DOORS D3 says should happen. The two documents are the same
/// three-node recipe with the blend swapped, so the blend mints under
/// the same node id (2) and — D3 having ruled the role vocabulary
/// shared — composes the same `RoleSeg`s off the same upstream names.
/// The names are therefore equal, which costs nothing: a `StableName`
/// is scoped to its own document, and WITHIN either document every
/// name is still unique. What distinguishes a chamfer's blend from a
/// fillet's is the minting node, and here the two nodes live in
/// different documents. `emit_fillet.rs`'s tie probe asserts the
/// within-one-document case, where the two nodes differ and the names
/// must be disjoint.
const PINNED: &[(&str, u64)] = &[
    ("die", 0xd773_3105_3f8a_87fe),
    ("corner_table", 0xb673_87e5_9fec_7a49),
    ("heat_sink", 0xe6c9_bc2c_6126_7d8b),
    ("crossing_slots", 0x676e_0a08_79be_357c),
    ("nested_islands_105", 0xf01b_971b_ffbd_9c19),
    ("nested_islands_106_depth1", 0x1aa6_20bb_6917_8f6d),
    ("nested_islands_106_depth2", 0xcda5_e910_7e81_9968),
    ("declared_tangency", 0xb6e1_4ab8_871e_2a8b),
    // Moved by the in-plane revolve axis, and the ONLY row that did.
    // `kitchen_sink` shared one `Datum::Axis` between a circular
    // pattern and a revolve; those are two node kinds now — a pattern
    // turns a body about a world line, a revolve turns a sketch about a
    // line in its own plane — so the document authors both and every
    // node after the new one is renumbered.
    //
    // The three die documents mint their axis in the same edit as
    // before, one position later (after the frame it names), and a
    // datum mints no names, so the swap moves no id that any name
    // holds: their rows are byte-identical. That is what this
    // per-document instrument is for.
    ("kitchen_sink", 0x4c53_ed70_a7fc_43e8),
    ("cut_cylinder", 0xc700_93ca_419f_9d81),
    ("measured_web", 0x50b4_88ab_2fe5_8e7d),
    ("boss_union", 0x1d0f_6334_988b_11bd),
    ("die_fillet", 0xeeaa_6c7d_9ac0_6d44),
    ("die_chamfer", 0xeeaa_6c7d_9ac0_6d44),
    ("die_pips", 0xdbdf_dd4f_1d10_5da9),
    ("heat_sink_fins", 0x78b6_1cf3_87e1_40c2),
    ("die_tool", 0x1826_addb_ac81_96b4),
    ("loft_prism", 0xb00f_c86f_2ce8_fcf6),
    ("die_composed", 0x9f65_6fe9_b3d7_6184),
    ("die_composed_tour", 0xfcbc_3092_7d57_5aea),
    ("plate_param", 0xdf84_033a_2c18_1acc),
    ("kiss_carry", 0x8008_375b_ab91_3274),
    // LIB-TUBE. Both tables are minted by `name_revolve` — the
    // tube doors return `Revolved<T>` and the emitter reads only
    // its maps — so these two rows are the revolve role vocabulary
    // over a body no revolve node built. Their arrival moved no
    // other row, which is the property this table exists to make
    // readable.
    ("tube_ring", 0x1293_7fbf_295c_f16c),
    ("hollow_tube_elbow", 0x98e4_97a0_679c_33ad),
];

#[test]
fn every_corpus_documents_name_tables_are_golden() {
    let got: Vec<(String, u64)> = corpus::documents()
        .iter()
        .map(|d| {
            let ev = corpus::eval::<f64>(&d.doc);
            (d.name.to_owned(), digest(&ev))
        })
        .collect();
    let want: Vec<(String, u64)> = PINNED.iter().map(|(n, h)| ((*n).to_owned(), *h)).collect();
    let render = |v: &[(String, u64)]| {
        v.iter()
            .map(|(n, h)| format!("    (\"{n}\", 0x{h:016x}),"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_eq!(
        got,
        want,
        "the corpus's name tables moved. Decide whether the NEW names \
         are right; if they are, this is the fresh table:\n{}",
        render(&got)
    );
}
