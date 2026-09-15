scalar-rate-r1 probe artifacts — RATE-PAIR (PR 2657), frozen head 711236057.
Probe branch only; NOT for merge.

Merge base: 6dd5983ba71794cd7dca23a127810645843f281f

r1_rate_e2e.rs / r1_rate_e2e_mergebase.rs
    The end-to-end program, as `crates/sweep/examples/r1_rate_e2e.rs`.
    Builds a five-section lofted body with NURBS chart walls through the
    public door, then:
      validate_geometric           -> the pcurve certify path (param_rate,
                                      trim_containment, the iso lane)
      pcurves::validate_pcurves    -> loop continuity (v_meter/metered_sup)
      chart_boundary per face      -> the closure-height door
      patch_regularity + offset_normal_floor per NURBS wall
                                   -> PatchRegularity's SupSpeed fields,
                                      speed_lever, thinness
      chart_stretch_sup per chart
      mass_properties
    Every number printed as f64 bits. The `_mergebase` copy differs only in
    dropping the `.get()`s that do not exist before the change.
    e2e-head.out and e2e-mergebase.out are byte-identical
    (sha256 5f4b141d4afcb9932dd5e3e6cf2bfd258ec7e7bbd37ebdf4d7bc00ac0f506e67).

mutation-A-ulp.log
    SupSpeed::to_meters mutated to `span * self.0 * (1.0 + f64::EPSILON)`.
    Exactly one row reds: rate_pair_doors::sup_to_meters_is_the_bare_product.

mutation-BCD-sanitize-reassoc-reorder.log
    Three mutations at once:
      B  InfSpeed::new sanitises poison (`if rate.is_poison() { zero }`)
         -> a_poisoned_or_collapsed_rate_passes_straight_through reds,
            and so do inf_to_meters/inf_to_param on the NaN rate.
      C  SupSpeed::to_param reassociated to `meters * (1/s)`
         -> sup_to_param_is_the_bare_quotient reds.
      D  InfSpeed::to_meters operand order swapped (`self.0 * span`)
         -> nothing reds, correctly: IEEE multiply is commutative, so a
            commuted product IS bit-identical. Not a gap.

compile-fail-codes.log
    The three compile_fail doctest snippets compiled as one example at the
    pinned toolchain (rustc 1.97.0), to read the codes off rustc rather
    than trust the annotations: E0308, E0308, E0369 (and E0369 for `<`).

twins-and-doctests.log
    The three twins compile and run; `cargo test -p geom-core --doc` is
    14 passed / 0 failed.

narration-recipe.sh
    The tour narration recipe from work/scalar/rate-pair-in-geom-core.md,
    executed. Three runs — head twice (different outdirs) and the merge
    base once — all give 728 lines and one sha256,
    e963b2e499342a4786c413752e7f7ec6aa3e947470b58933eb21d8cf048af485,
    with the outdir rewritten to the literal string "OUTDIR".
    The committed hash is 8522886427381545d312aafd1ff6a1a6757ea4d6500c47278036c2d847ab6712;
    the recipe does not say which constant the outdir is rewritten to, so
    a second party cannot reproduce it. Twenty-one candidate constants
    were tried; none matched.

Tour file-listing digest, reproduced independently at BOTH commits:
    1766 files, per-file listing digest
    87be4dd9df4cc3af9bd44593a6b981608c8e73721c746413a00322ff61e4a892
    at the merge base AND at the head. The bit-identity claim holds.
