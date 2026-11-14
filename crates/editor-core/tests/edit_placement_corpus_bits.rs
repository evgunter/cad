//! **Every corpus placement places its body by the same bits** — the
//! value channel of each `Node::Place` in the registry, pinned per node.
//!
//! The digest is the fixture's value channel (`placed_body_digest`):
//! the placed bodies' arena counts and every stored point where the
//! copy stands, composed into its space through `in_space` (a copy is
//! its source's body under its frame), so a motion that moved one bit
//! of one coordinate moves the word. A golden in the
//! ordinary sense: when one moves, the question is whether the new
//! motion is right, never how to restore the old number.

use crate::corpus;
use crate::fixture::value_channel::placed_body_digest;
use corpus::{eval, failures};
use editor_core::Node;

/// `(document, node's mint ordinal, digest)` for every placement the
/// registry holds, in registry then document order, the same at the
/// default, `1e-6` and `1e-12` ε rows. The digest feeds the node id, so
/// a re-minted id moves the word with no geometry moving; so does a
/// point landing in another arena slot.
///
/// A copy is its source's body under its frame, so each word digests
/// the copy composed into its space through `in_space`, and moves with
/// any bit of any coordinate a motion, a rebuilt source or a re-minted
/// id moves; a row is added with every placement the registry gains.
/// The PR that moves a row says which copies moved, by how many points
/// and why.
const PINNED: &[(&str, u32, u64)] = &[
    ("die", 161, 0xb63e_a126_3087_5adc),
    ("die", 198, 0xcf9f_c440_58b8_cf52),
    ("die", 235, 0x86c6_a529_61e8_d6d9),
    ("die", 272, 0x2df5_f668_aa68_1d64),
    ("die", 309, 0x4816_dfb3_bd98_21b5),
    ("die", 346, 0x6be3_d58c_82ee_7126),
    ("die", 397, 0xbc04_ffcc_494d_1d34),
    ("die", 448, 0x4c8c_4d05_ec74_7988),
    ("die", 492, 0x7005_3aa0_25e3_9efe),
    ("die", 536, 0xd0c3_46c0_2fd3_ea2d),
    ("die", 580, 0x1a42_4bcc_962a_4de7),
    ("die", 624, 0x1c18_5abc_8407_111f),
    ("die", 668, 0x99a4_0e22_d093_e20f),
    ("die", 719, 0xa43a_051e_70c9_c4f4),
    ("die", 770, 0xe6a5_d26e_2424_7008),
    ("die", 821, 0xaa44_10cc_3551_b436),
    ("die", 858, 0xa0f5_5008_b505_07e8),
    ("die", 895, 0x3b1b_90e7_3e12_b212),
    ("die", 932, 0xc99f_8860_d03d_0fa7),
    ("die", 969, 0x4d60_06bc_28f6_ab0a),
    ("die", 1013, 0xb64b_36a0_bd2e_c721),
    ("die", 1025, 0xd13f_5d56_4ab5_f6c0),
    ("corner_table", 51, 0xb8c4_b9f5_f950_7400),
    ("corner_table", 90, 0xd1a8_c731_fb31_f4e3),
    ("corner_table", 129, 0xdde2_3c83_2d62_55dc),
    ("corner_table", 168, 0x80a6_e6fb_e2fc_7b15),
    ("corner_table", 183, 0x7e62_0440_51b3_be90),
    ("heat_sink", 49, 0x663e_adb9_dd98_810a),
    ("heat_sink", 66, 0xafb2_0e34_9ce3_e063),
    ("heat_sink", 81, 0x1dbb_6e19_6b83_d3ae),
    ("crossing_slots", 51, 0x893d_da80_f551_53f1),
    ("crossing_slots", 90, 0x5c09_9283_0431_c363),
    ("crossing_slots", 105, 0x81e3_ff99_5309_412b),
    ("nested_islands_105", 70, 0xcdf6_bfd4_9a8e_8363),
    ("nested_islands_105", 90, 0xd30f_bedb_f66e_5c4c),
    ("nested_islands_105", 129, 0x05c8_0f15_d677_12fe),
    ("nested_islands_105", 144, 0x2dc4_b4f5_8d54_ea0b),
    ("nested_islands_106_depth1", 70, 0xcdf6_bfd4_9a8e_8363),
    ("nested_islands_106_depth1", 90, 0xd30f_bedb_f66e_5c4c),
    ("nested_islands_106_depth1", 129, 0x9407_12f0_b439_aa57),
    ("nested_islands_106_depth1", 144, 0x9410_504f_0fcc_2832),
    ("nested_islands_106_depth2", 70, 0xcdf6_bfd4_9a8e_8363),
    ("nested_islands_106_depth2", 90, 0xd30f_bedb_f66e_5c4c),
    ("nested_islands_106_depth2", 129, 0x05c8_0f15_d677_12fe),
    ("nested_islands_106_depth2", 168, 0x2566_a632_57a3_ab78),
    ("nested_islands_106_depth2", 183, 0x50ca_ab3b_1392_c101),
    ("declared_tangency", 67, 0x5d48_8b82_d0f4_6dd5),
    ("declared_tangency", 82, 0x3f94_e6ef_f5ef_5394),
    ("kitchen_sink", 65, 0xbaa5_85fa_ff58_f3e7),
    ("kitchen_sink", 90, 0x4da4_9727_788b_c7d9),
    ("kitchen_sink", 105, 0xb3ac_fd1a_39be_6a98),
    ("kitchen_sink", 126, 0x959e_969d_d6ce_d1a0),
    ("kitchen_sink", 171, 0xc787_7bf5_fa19_17f9),
    ("kitchen_sink", 186, 0x4790_2cad_eabb_13bb),
    ("kitchen_sink", 203, 0x77bf_cc33_c746_6f89),
    ("kitchen_sink", 220, 0x2422_5544_5591_c336),
    ("kitchen_sink", 237, 0x392d_ba99_0d9e_30e8),
    ("cut_cylinder", 46, 0xcca5_bbcf_71e7_1ba1),
    ("cut_cylinder", 61, 0x72b3_f745_d255_c01b),
    ("measured_web", 47, 0x5b66_b559_a071_2879),
    ("measured_web", 67, 0x3584_2878_7aa0_e2ed),
    ("boss_union", 43, 0x31c7_169b_091e_4970),
    ("boss_union", 58, 0x9893_3189_ada5_b953),
    ("die_fillet", 31, 0xa20b_770f_7459_b422),
    ("die_chamfer", 31, 0x051e_ace3_46f7_7503),
    ("die_pips", 81, 0xbd6c_e45a_dd40_621a),
    ("die_pips", 95, 0xc870_b641_1503_3e6e),
    ("heat_sink_fins", 30, 0x3f4b_a4ad_51b3_88e3),
    ("heat_sink_fins", 47, 0xa5a5_21b9_acc7_4e38),
    ("die_tool", 53, 0x5ad4_000d_402a_5a51),
    ("die_tool", 65, 0x03b7_a364_c0f6_bb34),
    ("die_tool", 80, 0x769a_fadd_bb07_a21b),
    ("die_tool", 95, 0x1da7_8c43_66b7_277f),
    ("die_tool", 110, 0xfa47_bed0_c04f_1a6e),
    ("die_tool", 125, 0x14d4_3488_b6bf_7277),
    ("die_tool", 140, 0x0a46_1b68_c097_f8a4),
    ("die_tool", 159, 0xe165_9b44_2420_ff15),
    ("face_sketch", 46, 0x9187_24b8_3abb_47bf),
    ("face_sketch", 75, 0xbaf9_2d61_ad9a_c836),
    ("part_select", 42, 0xc3fc_7496_331d_125a),
    ("part_select", 57, 0x02d1_bd37_35f9_904f),
    ("part_select", 73, 0xddba_f94e_5d07_d2cc),
    ("symmetric_plate", 39, 0xfa92_edb7_4a6c_fedc),
    ("loft_prism", 62, 0xa738_0ec1_516e_dd23),
    ("die_composed", 81, 0xbd6c_e45a_dd40_621a),
    ("die_composed", 99, 0x6f76_3181_a5c9_a678),
    ("die_composed_tour", 99, 0xfefc_b165_b59b_d6d3),
    ("die_composed_tour", 175, 0x1416_cd6a_3af1_4cc8),
    ("die_composed_tour", 251, 0x94ad_8faa_6637_bdc2),
    ("die_composed_tour", 327, 0x4d93_ef48_7b0d_8d4b),
    ("die_composed_tour", 403, 0xe2d1_1a5c_2359_7ef6),
    ("die_composed_tour", 479, 0x1a3a_8f9d_cae1_24ac),
    ("die_composed_tour", 555, 0x6c51_efe7_e602_b119),
    ("die_composed_tour", 631, 0xc043_825c_8a3f_30a4),
    ("die_composed_tour", 707, 0x775d_c560_6b12_29f3),
    ("die_composed_tour", 783, 0x81e7_f722_f790_a947),
    ("die_composed_tour", 859, 0x90da_d65c_fea4_79ae),
    ("die_composed_tour", 935, 0x5e83_654d_0c02_6e0a),
    ("die_composed_tour", 1011, 0x1c44_e6e3_ea46_037c),
    ("die_composed_tour", 1087, 0x98f1_4872_9137_f151),
    ("die_composed_tour", 1163, 0x2523_2a49_efa1_0fd8),
    ("die_composed_tour", 1239, 0xb591_c12a_43a8_fc1b),
    ("die_composed_tour", 1315, 0xcf39_5ba2_a9d5_fe83),
    ("die_composed_tour", 1391, 0x680b_97a7_6a70_f325),
    ("die_composed_tour", 1467, 0x4909_b150_66c7_7162),
    ("die_composed_tour", 1543, 0xf203_d0ea_65b7_2508),
    ("die_composed_tour", 1619, 0x8807_f417_6a36_17b3),
    ("die_composed_tour", 1643, 0x076a_dcef_568b_604c),
    ("plate_param", 58, 0x9bda_c831_1b5c_5224),
    ("plate_param", 73, 0x7c6c_0940_5a69_c25f),
    ("kiss_carry", 51, 0x3603_1edd_8e45_2895),
    ("kiss_carry", 90, 0x485a_6f72_f77b_f073),
    ("kiss_carry", 105, 0xe69e_1d09_f41d_73f4),
    ("tube_ring", 13, 0xe368_bae2_d2d3_a790),
    ("tube_arc", 15, 0x9eaf_a4ac_8644_0422),
    ("hollow_tube_elbow", 16, 0x8816_821c_a6ac_d21b),
    ("hollow_tube_ring", 14, 0x8b21_7b2f_6154_76e6),
    ("reshaped_rod", 56, 0x8d82_79a5_f7a0_a723),
    ("pin_in_bore", 58, 0x4e30_de7f_b1cc_68d2),
    ("pin_in_bore", 73, 0xe5c1_caa8_c2a3_9d1a),
    ("pin_in_bore", 149, 0x8b74_72bd_9abd_f5df),
    ("pin_in_bore", 166, 0x266a_7cd4_fee3_4525),
    ("axis_projection", 39, 0x4089_991a_0b71_178c),
    ("axis_projection", 53, 0xaf7f_69e4_7038_20de),
    ("blocks_on_one_floor", 46, 0x14e5_f356_936d_20f2),
    ("blocks_on_one_floor", 92, 0xaf86_9fe1_3ba5_d266),
    ("blocks_on_one_floor", 121, 0xb153_7df8_0365_55c7),
    ("blocks_on_one_floor", 136, 0xcdf4_8032_dbfb_4b79),
    ("revolves_on_one_axis", 52, 0x68dd_9580_280b_9b70),
    ("revolves_on_one_axis", 69, 0x9add_a604_796f_1eb8),
];

#[test]
fn every_corpus_placement_places_its_body_by_the_pinned_bits() {
    let mut got: Vec<(&'static str, u32, u64)> = Vec::new();
    for doc in corpus::documents() {
        let placements: Vec<_> = doc
            .doc
            .ids()
            .iter()
            .copied()
            .filter(|id| matches!(doc.doc.node(*id), Some(Node::Place { .. })))
            .collect();
        if placements.is_empty() {
            continue;
        }
        let ev = eval::<f64>(&doc.doc);
        let bad = failures(&ev);
        assert!(bad.is_empty(), "{} failed to evaluate: {bad:?}", doc.name);
        for id in ev
            .order
            .iter()
            .copied()
            .filter(|id| placements.contains(id))
        {
            let digest = placed_body_digest(&ev, id);
            assert!(
                digest.is_some(),
                "{} node {id}: a placement's value is a body or a family",
                doc.name
            );
            let (word, fed) = digest.unwrap_or_default();
            assert!(fed > 0, "{} node {}: nothing was digested", doc.name, id);
            got.push((doc.name, id.0.ordinal(), word));
        }
    }
    for (name, id, word) in &got {
        println!("    (\"{name}\", {id}, {word:#018x}),");
    }
    assert_eq!(
        got.as_slice(),
        PINNED,
        "a corpus placement's placed body moved"
    );
}
