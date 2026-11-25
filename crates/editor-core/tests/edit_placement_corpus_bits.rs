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
    // Re-cut, both halves: PR 7299 places the x = -0.5 seam's split vertex at cross's root, z one ulp up onto the rounded crossing.
    ("cut_cylinder", 46, 0x6b46_72c6_e1f0_2a46),
    ("cut_cylinder", 61, 0xdd0d_1457_d05d_a9dc),
    ("measured_web", 47, 0x5b66_b559_a071_2879),
    ("measured_web", 67, 0x3584_2878_7aa0_e2ed),
    ("boss_union", 43, 0x31c7_169b_091e_4970),
    ("boss_union", 58, 0x9893_3189_ada5_b953),
    ("die_fillet", 31, 0xa20b_770f_7459_b422),
    ("die_chamfer", 31, 0x051e_ace3_46f7_7503),
    ("die_pips", 81, 0xcd79_7756_b3d4_04ba),
    ("die_pips", 95, 0x65b6_e2c5_5a09_b30c),
    ("heat_sink_fins", 30, 0x3f4b_a4ad_51b3_88e3),
    ("heat_sink_fins", 47, 0xa5a5_21b9_acc7_4e38),
    ("die_tool", 53, 0x75cd_1cc0_b536_9df1),
    ("die_tool", 65, 0xb52d_b171_a5b7_3ad4),
    ("die_tool", 80, 0x93f3_1714_e691_487b),
    ("die_tool", 95, 0xebe3_c8cd_b2b8_825f),
    ("die_tool", 110, 0x60bd_f35a_90dc_d20e),
    ("die_tool", 125, 0x967b_47d9_d37c_e057),
    ("die_tool", 140, 0x5bce_fd2f_b54a_14c4),
    ("die_tool", 159, 0xd63a_d49e_fbeb_6e3c),
    ("face_sketch", 46, 0x9187_24b8_3abb_47bf),
    ("face_sketch", 75, 0xbaf9_2d61_ad9a_c836),
    ("part_select", 42, 0xc3fc_7496_331d_125a),
    ("part_select", 57, 0x02d1_bd37_35f9_904f),
    ("part_select", 73, 0xddba_f94e_5d07_d2cc),
    ("symmetric_plate", 39, 0xfa92_edb7_4a6c_fedc),
    ("loft_prism", 62, 0xa738_0ec1_516e_dd23),
    ("die_composed", 81, 0xcd79_7756_b3d4_04ba),
    ("die_composed", 99, 0x9ef5_c7fa_421a_af6e),
    ("die_composed_tour", 99, 0xe842_8826_89fb_4533),
    ("die_composed_tour", 175, 0xeedc_db22_a1dc_4e68),
    ("die_composed_tour", 251, 0x6351_94c8_e8fa_41e2),
    ("die_composed_tour", 327, 0x9bc2_d4d0_4476_58ab),
    ("die_composed_tour", 403, 0x624b_670b_139d_a916),
    ("die_composed_tour", 479, 0x1257_7fd4_f63a_ab4c),
    ("die_composed_tour", 555, 0xc8a1_26b4_2e69_7e79),
    ("die_composed_tour", 631, 0xda97_9251_a47c_3dc4),
    ("die_composed_tour", 707, 0x3c24_2ebb_b462_4153),
    ("die_composed_tour", 783, 0x1e93_31c4_fdf3_37a7),
    ("die_composed_tour", 859, 0x9064_891a_2cae_de4e),
    ("die_composed_tour", 935, 0x977c_8803_4382_322a),
    ("die_composed_tour", 1011, 0x35ee_570d_2896_5c9c),
    ("die_composed_tour", 1087, 0x66d7_7220_3e52_fe31),
    ("die_composed_tour", 1163, 0x5387_ad44_3476_1578),
    ("die_composed_tour", 1239, 0x10af_e1ed_cc1f_9cfb),
    ("die_composed_tour", 1315, 0x8180_ad8d_b64e_efe3),
    ("die_composed_tour", 1391, 0x8cab_d92b_d789_b485),
    ("die_composed_tour", 1467, 0x42bf_fc8b_df0a_b682),
    ("die_composed_tour", 1543, 0x3f93_05e0_c9ab_4ea8),
    ("die_composed_tour", 1619, 0x7743_2ec3_717f_6b13),
    ("die_composed_tour", 1643, 0xcae8_867e_31ed_530c),
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
    ("pin_in_bore", 73, 0xf1dc_6b7a_5b3a_6f62),
    ("pin_in_bore", 149, 0x8b74_72bd_9abd_f5df),
    ("pin_in_bore", 166, 0x206d_20bf_0c85_ccf9),
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
