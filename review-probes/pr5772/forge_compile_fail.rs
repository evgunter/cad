//! Review probe (PR 5772): every way a sibling crate might hand a
//! boolean step a glue no judgement decided. NOT in the harness: each
//! item below must be a compile error. Checked by temporarily adding
//! `mod forge_compile_fail;` to a test harness and running
//! `cargo check -p verbs --tests`.
#![allow(dead_code, unused)]

fn forge(v: &topo::GlueVerdict<f64>, f: topo::FaceKey) {
    // 1. the type by its root re-export
    let _p: Option<topo::GluedPair<f64>> = None;
    // 2. the type through the public `boolean` module
    let _q: Option<topo::boolean::GluedPair<f64>> = None;
    // 3. the mint
    let _g = v.given_at(topo::Operand::A, f);
    // 4. the input field
    let _i = topo::BooleanInputs {
        given: None::<Vec<u8>>,
        ..topo::BooleanInputs::default()
    };
    // 5. the crate-private pipeline door
    let _d = topo::boolean::boolean_op_given::<f64>;
    // 6. a verdict built by hand
    let _v = topo::GlueVerdict::<f64> { a: f, b: f, class: todo!(), margin: todo!(), line: None };
    // 7. the restated reader
    let _r = topo::boolean::GluedPair::<f64>::restated_by;
}
