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
    ("die", 160, 0x0b0e_9ff2_2465_e66d),
    ("die", 196, 0x36ae_6d4e_acb1_34a2),
    ("die", 232, 0xade0_1fc7_1fec_c8c4),
    ("die", 268, 0x83e3_1b93_7b9e_4b17),
    ("die", 304, 0xdf09_1afa_d563_2aaf),
    ("die", 340, 0x0510_8b91_59df_8f6a),
    ("die", 390, 0xd4ab_9db0_6535_6fa0),
    ("die", 440, 0xba48_9e86_13e4_0e0f),
    ("die", 483, 0x2854_99e4_05e3_39a4),
    ("die", 526, 0x2539_bc65_ce72_a157),
    ("die", 569, 0x05ff_7f68_ee8b_3212),
    ("die", 612, 0x5a8c_b8c0_a3db_21a1),
    ("die", 655, 0x69f1_1c5f_7f55_a79d),
    ("die", 705, 0x52c9_88df_ff79_1cf3),
    ("die", 755, 0x7d57_f2f1_20d0_79b8),
    ("die", 805, 0x6448_eeb9_04c7_37bd),
    ("die", 841, 0xc1b3_a36f_b5f3_6d40),
    ("die", 877, 0xb72a_c554_5504_a131),
    ("die", 913, 0x4544_7f76_a6e2_fae5),
    ("die", 949, 0x34b4_67d2_e03b_efed),
    ("die", 992, 0x3512_5a68_1f43_d0bc),
    ("die", 1003, 0x25a7_6c82_81a1_b1f9),
    ("corner_table", 50, 0xb3e0_5892_034a_8894),
    ("corner_table", 88, 0xab90_afdd_20f8_888a),
    ("corner_table", 126, 0x4583_d0c1_f816_ef5f),
    ("corner_table", 164, 0x09b9_4c71_7384_b795),
    ("corner_table", 178, 0xe601_a4b2_16e2_54c4),
    ("heat_sink", 48, 0xd1ab_a4f3_417b_c486),
    ("heat_sink", 64, 0x41f5_a137_b222_8a90),
    ("heat_sink", 78, 0x6dd2_9e74_7294_1dc4),
    ("crossing_slots", 50, 0xc1dd_3b0e_9159_b3a2),
    ("crossing_slots", 88, 0x09f9_ba1c_9f87_1dc9),
    ("crossing_slots", 102, 0xf97d_e0f5_7717_0fca),
    ("nested_islands_105", 69, 0x6304_f06e_5bb7_56d5),
    ("nested_islands_105", 88, 0xeed8_a5d0_1e75_8619),
    ("nested_islands_105", 126, 0x771b_7d77_9f23_5ef2),
    ("nested_islands_105", 140, 0x71ce_e521_0c7d_c40c),
    ("nested_islands_106_depth1", 69, 0x6304_f06e_5bb7_56d5),
    ("nested_islands_106_depth1", 88, 0xeed8_a5d0_1e75_8619),
    ("nested_islands_106_depth1", 126, 0x47a8_f7cb_35d5_aaf8),
    ("nested_islands_106_depth1", 140, 0x82a9_2357_de76_74f6),
    ("nested_islands_106_depth2", 69, 0x6304_f06e_5bb7_56d5),
    ("nested_islands_106_depth2", 88, 0xeed8_a5d0_1e75_8619),
    ("nested_islands_106_depth2", 126, 0x771b_7d77_9f23_5ef2),
    ("nested_islands_106_depth2", 164, 0x2be4_09a8_4fc4_c076),
    ("nested_islands_106_depth2", 178, 0xcf74_d9a8_e2cd_de5a),
    ("declared_tangency", 66, 0xaffe_8409_f108_694e),
    ("declared_tangency", 80, 0xe69c_31c4_a41b_5039),
    ("kitchen_sink", 64, 0x685f_ccab_23b8_deed),
    ("kitchen_sink", 88, 0x536f_61f8_dfa6_c306),
    ("kitchen_sink", 102, 0x78a9_276c_5c8e_6ab4),
    ("kitchen_sink", 122, 0x606d_220d_5c11_5230),
    ("kitchen_sink", 166, 0xbb2c_6d95_3374_dd09),
    ("kitchen_sink", 180, 0xf402_f78f_1304_c0e5),
    ("kitchen_sink", 196, 0xf95f_35e8_501d_3d31),
    ("kitchen_sink", 212, 0xe3ce_b287_b9ef_25c3),
    ("kitchen_sink", 228, 0x0e5c_bd38_a6e3_0385),
    ("cut_cylinder", 45, 0xd8d0_024a_fc3a_0827),
    ("cut_cylinder", 59, 0x2750_3802_b18d_dfb2),
    ("measured_web", 46, 0x4c51_d7b6_0b61_4057),
    ("measured_web", 65, 0xf81a_799d_a371_89fd),
    ("boss_union", 42, 0x6e54_20d9_163c_5bc9),
    ("boss_union", 56, 0x080e_a9b8_6ae9_5fdf),
    ("die_fillet", 30, 0x42f7_21f4_cb1c_e71f),
    ("die_chamfer", 30, 0xc860_4c07_e6cd_efa2),
    ("die_pips", 80, 0xff6f_0374_12ee_5476),
    ("die_pips", 93, 0x58a4_65bd_e703_bc05),
    ("heat_sink_fins", 29, 0xc099_0744_aeef_9442),
    ("heat_sink_fins", 45, 0x236d_c67b_8788_568a),
    ("die_tool", 52, 0x403a_d616_dad1_9725),
    ("die_tool", 63, 0x981a_8ce6_6142_78c9),
    ("die_tool", 77, 0x9133_33b5_4267_cb45),
    ("die_tool", 91, 0x2dac_62ca_bdbd_3691),
    ("die_tool", 105, 0x4de5_5aa6_9250_bf66),
    ("die_tool", 119, 0x27e9_d611_17e3_5490),
    ("die_tool", 133, 0xb589_2ee9_e0c5_d1de),
    ("die_tool", 151, 0xc684_df12_b766_f87a),
    ("face_sketch", 45, 0x5479_522d_afb0_3509),
    ("face_sketch", 73, 0xb405_e06c_7eae_44c7),
    ("part_select", 41, 0xd60d_8b70_8d01_f7bc),
    ("part_select", 55, 0x2505_ed2e_1b5d_a75a),
    ("part_select", 70, 0xe082_7c13_5b16_ed69),
    ("symmetric_plate", 38, 0xe478_fc21_5f55_c43e),
    ("loft_prism", 58, 0x5d81_80d3_48ae_1f2d),
    ("die_composed", 80, 0xff6f_0374_12ee_5476),
    ("die_composed", 97, 0x2640_176a_3e90_41ba),
    ("die_composed_tour", 98, 0xa0a7_4608_b707_23db),
    ("die_composed_tour", 173, 0x2ca1_72ee_1cef_1b30),
    ("die_composed_tour", 248, 0x6690_2aa9_3fc4_c70b),
    ("die_composed_tour", 323, 0x3f20_92e4_7051_df17),
    ("die_composed_tour", 398, 0xe9a9_2532_a216_393c),
    ("die_composed_tour", 473, 0x14a2_a712_9245_b74c),
    ("die_composed_tour", 548, 0x93ba_7440_4def_c365),
    ("die_composed_tour", 623, 0x408f_a8a7_3798_ef07),
    ("die_composed_tour", 698, 0x3d2c_bcd4_0c7f_0f66),
    ("die_composed_tour", 773, 0x2358_428c_3743_9ee7),
    ("die_composed_tour", 848, 0x67d1_dedd_5008_54e8),
    ("die_composed_tour", 923, 0x56db_65c9_1efa_9327),
    ("die_composed_tour", 998, 0x27c5_b89b_d5db_4eff),
    ("die_composed_tour", 1073, 0x3af0_42de_c767_2506),
    ("die_composed_tour", 1148, 0x5ca7_a7aa_d7a3_f3f1),
    ("die_composed_tour", 1223, 0x91dc_e511_69a3_f75a),
    ("die_composed_tour", 1298, 0xc1af_7d17_be5b_fec8),
    ("die_composed_tour", 1373, 0x7dcf_4392_8e3b_9b63),
    ("die_composed_tour", 1448, 0x0f43_8d0c_1168_ff00),
    ("die_composed_tour", 1523, 0xe6e3_14c7_e5f1_b012),
    ("die_composed_tour", 1598, 0xa2bc_4381_9236_825f),
    ("die_composed_tour", 1621, 0xf9df_e49c_1f8f_b48f),
    ("plate_param", 57, 0xabb2_f540_f547_e688),
    ("plate_param", 71, 0xfa3a_93f3_6456_2949),
    ("kiss_carry", 50, 0x88fe_841c_789d_0440),
    ("kiss_carry", 88, 0x9477_a514_b0d5_087c),
    ("kiss_carry", 102, 0xbe14_d499_184f_60f1),
    ("tube_ring", 12, 0xd785_cea9_c3ee_e2fa),
    ("tube_arc", 14, 0x1ac2_adc4_a57c_1e39),
    ("hollow_tube_elbow", 15, 0x2dd1_86b2_9afd_0375),
    ("hollow_tube_ring", 13, 0xfb79_1da5_4bf8_80f3),
    ("reshaped_rod", 55, 0xde1c_e559_b047_222b),
    ("pin_in_bore", 57, 0x0835_f363_3ab6_8043),
    ("pin_in_bore", 71, 0xf4b5_32e9_193c_23a3),
    ("pin_in_bore", 146, 0x4926_a22b_9b5b_751e),
    ("pin_in_bore", 162, 0x6873_1af4_d165_17d0),
    ("axis_projection", 38, 0x241b_bf9f_c0f1_3619),
    ("axis_projection", 51, 0x8728_75e0_0186_5dee),
    ("blocks_on_one_floor", 45, 0x9730_a6b9_70d4_a786),
    ("blocks_on_one_floor", 90, 0x1d45_bd18_0497_6860),
    ("blocks_on_one_floor", 118, 0xa91b_63f9_045a_db2e),
    ("blocks_on_one_floor", 132, 0x3f92_7f0d_ae15_d0d3),
    ("revolves_on_one_axis", 51, 0xb707_a714_e841_557b),
    ("revolves_on_one_axis", 67, 0xa78c_57f0_20fe_03bb),
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
