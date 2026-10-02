// Reviewer probe (reach-dual3844-r1), appended locally to
// crates/sweep/tests/reach_continuation.rs (uses its private helpers).
// Q: does B ∖ A refuse FallbackExtentUnsupported when the declarations
// are keyed for (B, A) rather than (A, B)? And both operand orders of ∩.
#[test]
fn probe_r1_b_minus_a_with_correctly_keyed_declarations() {
    let a = plate(rounded(R), 0.0);
    let half = area(4.0) / 2.0;
    for (label, z0) in [("sunk", 0.25), ("flush top", 0.5), ("flush bottom", 0.0)] {
        let b = extruded(sketch_at(z0), vec![rounded(R)], 0.5, tol());
        let (rest, cont) = findings(&a, &b);
        let d_ab = with(&rest, &cont);
        let (rest2, cont2) = findings(&b, &a);
        let d_ba = with(&rest2, &cont2);
        let show = |o: Result<BooleanResult<f64>, BooleanError>| match o {
            Ok(BooleanResult::Body(bb)) => format!("body V={:.15} faces={}", volume(&bb.body), bb.body.faces().count()),
            Ok(BooleanResult::Empty) => "Empty".into(),
            Err(e) => format!("{e:?}").chars().take(100).collect(),
        };
        println!("{label}: oracle half={half:.15}");
        println!("  B∖A, (A,B)-keyed decls (the PR row): {}", show(topo::subtract_with(&b, &a, &d_ab, tol())));
        println!("  B∖A, (B,A)-keyed decls:              {}", show(topo::subtract_with(&b, &a, &d_ba, tol())));
        println!("  B∩A, (B,A)-keyed decls:              {}", show(topo::intersect_with(&b, &a, &d_ba, tol())));
        println!("  A∩B, (A,B)-keyed decls:              {}", show(topo::intersect_with(&a, &b, &d_ab, tol())));
        println!("  A∖B:                                 {}", show(topo::subtract_with(&a, &b, &d_ab, tol())));
        println!("  B∪A, (B,A)-keyed:                    {}", show(topo::union_with(&b, &a, &d_ba, tol())));
    }
}
