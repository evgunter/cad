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
    ("die", 198, 0x7189_2a93_af43_5b55),
    ("die", 235, 0x83f9_9e34_38f6_374f),
    ("die", 272, 0x766e_f7e7_dcb9_469f),
    ("die", 309, 0x1f67_180c_f7cf_84ef),
    ("die", 346, 0xbe6d_a10c_3101_e4dd),
    ("die", 397, 0x2c66_94a1_e0ea_2393),
    ("die", 448, 0xf3c1_f6f1_895d_e053),
    ("die", 492, 0x4598_4b15_5d37_2217),
    ("die", 536, 0xef3a_5795_0edb_7b4d),
    ("die", 580, 0x6c76_e63b_77d9_fa5a),
    ("die", 624, 0x9661_fc41_53eb_eb1a),
    ("die", 668, 0x0f5a_ac83_edf8_b959),
    ("die", 719, 0xe73a_a8cc_c2d8_dc49),
    ("die", 770, 0xe7d7_7bdb_e7b1_4afc),
    ("die", 821, 0xaa22_cc5f_ae9a_970d),
    ("die", 858, 0xdd7f_9582_0b06_6f35),
    ("die", 895, 0x4657_57bc_6792_5755),
    ("die", 932, 0x01fa_9a5a_fffc_f5b5),
    ("die", 969, 0xf01a_5531_6248_06a4),
    ("die", 1013, 0x38fe_ed39_b644_25e3),
    ("die", 1025, 0x787c_f949_0c5b_654f),
    ("corner_table", 51, 0xb8c4_b9f5_f950_7400),
    ("corner_table", 90, 0xd1a8_c731_fb31_f4e3),
    ("corner_table", 129, 0xdde2_3c83_2d62_55dc),
    ("corner_table", 168, 0x80a6_e6fb_e2fc_7b15),
    ("corner_table", 183, 0x7e62_0440_51b3_be90),
    ("heat_sink", 49, 0x663e_adb9_dd98_810a),
    ("heat_sink", 66, 0x11ae_9d3b_5ff9_0926),
    ("heat_sink", 81, 0x644b_8bd7_658e_a54d),
    ("crossing_slots", 51, 0x893d_da80_f551_53f1),
    ("crossing_slots", 90, 0x4535_b1c0_f809_55bf),
    ("crossing_slots", 105, 0x3fe0_6e90_a7e8_b527),
    ("nested_islands_105", 70, 0xcdf6_bfd4_9a8e_8363),
    ("nested_islands_105", 90, 0x26ff_3032_af08_f3c1),
    ("nested_islands_105", 129, 0x6051_f26e_a1de_745c),
    ("nested_islands_105", 144, 0x9f88_0f47_1c03_b4a4),
    ("nested_islands_106_depth1", 70, 0xcdf6_bfd4_9a8e_8363),
    ("nested_islands_106_depth1", 90, 0x26ff_3032_af08_f3c1),
    ("nested_islands_106_depth1", 129, 0x599c_a57b_b1c6_85ca),
    ("nested_islands_106_depth1", 144, 0x823a_b965_2c64_519b),
    ("nested_islands_106_depth2", 70, 0xcdf6_bfd4_9a8e_8363),
    ("nested_islands_106_depth2", 90, 0x26ff_3032_af08_f3c1),
    ("nested_islands_106_depth2", 129, 0x6051_f26e_a1de_745c),
    ("nested_islands_106_depth2", 168, 0x17f1_d379_5b94_3875),
    ("nested_islands_106_depth2", 183, 0x3f9c_51d6_24fe_13b5),
    ("declared_tangency", 67, 0x5d48_8b82_d0f4_6dd5),
    ("declared_tangency", 82, 0x3f94_e6ef_f5ef_5394),
    ("kitchen_sink", 65, 0xbaa5_85fa_ff58_f3e7),
    ("kitchen_sink", 90, 0x4da4_9727_788b_c7d9),
    ("kitchen_sink", 105, 0xb3ac_fd1a_39be_6a98),
    ("kitchen_sink", 126, 0x959e_969d_d6ce_d1a0),
    ("kitchen_sink", 171, 0x3d18_4073_5b16_5695),
    ("kitchen_sink", 186, 0x2cae_c1d0_9cb3_95aa),
    ("kitchen_sink", 203, 0xc0b8_721a_86eb_23ba),
    ("kitchen_sink", 220, 0x770b_ff5d_321d_f5ca),
    ("kitchen_sink", 237, 0x19e8_34ea_eb2b_4578),
    ("cut_cylinder", 46, 0x6b46_72c6_e1f0_2a46),
    ("cut_cylinder", 61, 0xdd0d_1457_d05d_a9dc),
    ("measured_web", 47, 0x5b66_b559_a071_2879),
    ("measured_web", 67, 0x3584_2878_7aa0_e2ed),
    ("boss_union", 43, 0x31c7_169b_091e_4970),
    ("boss_union", 58, 0x9893_3189_ada5_b953),
    ("die_fillet", 31, 0xa20b_770f_7459_b422),
    ("die_chamfer", 31, 0x051e_ace3_46f7_7503),
    ("die_pips", 81, 0xbd6c_e45a_dd40_621a),
    ("die_pips", 95, 0xc2c4_ae4d_f879_3939),
    ("heat_sink_fins", 30, 0x3f4b_a4ad_51b3_88e3),
    ("heat_sink_fins", 47, 0xa5a5_21b9_acc7_4e38),
    ("die_tool", 53, 0x5ad4_000d_402a_5a51),
    ("die_tool", 65, 0x03b7_a364_c0f6_bb34),
    ("die_tool", 80, 0x769a_fadd_bb07_a21b),
    ("die_tool", 95, 0x1da7_8c43_66b7_277f),
    ("die_tool", 110, 0xfa47_bed0_c04f_1a6e),
    ("die_tool", 125, 0x14d4_3488_b6bf_7277),
    ("die_tool", 140, 0x0a46_1b68_c097_f8a4),
    ("die_tool", 159, 0x3d40_d19f_3a0b_2d87),
    ("face_sketch", 46, 0x9187_24b8_3abb_47bf),
    ("face_sketch", 75, 0xbaf9_2d61_ad9a_c836),
    ("part_select", 42, 0xc3fc_7496_331d_125a),
    ("part_select", 57, 0x02d1_bd37_35f9_904f),
    ("part_select", 73, 0xddba_f94e_5d07_d2cc),
    ("symmetric_plate", 39, 0xfa92_edb7_4a6c_fedc),
    ("loft_prism", 62, 0xa738_0ec1_516e_dd23),
    ("die_composed", 81, 0xbd6c_e45a_dd40_621a),
    ("die_composed", 99, 0xa378_3143_4dc2_f92a),
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
    ("die_composed_tour", 1643, 0x0264_2a24_8eab_e77d),
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
