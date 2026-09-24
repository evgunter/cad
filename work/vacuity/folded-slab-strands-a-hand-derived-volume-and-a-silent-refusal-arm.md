---
id: folded-slab-strands-a-hand-derived-volume-and-a-silent-refusal-arm
kind: issue
title: review_s12_adv's log probe restates the slab's volume by hand and swallows a refusal, so no fixture change can red it
status: open
opened: 2026-09-20
priority: P4
cost: E
---


## Finding

- **Where**: `crates/sweep/tests/review_s12_adv.rs`'s
  `probe_horizontal_log_halfburied_is_exact_or_typed`.
- **Importance**: medium — the row is a review probe kept as a
  permanent guard, and no change to either operand can make it fail
- **Confidence**: sure; measured by planted mutation on
  `dup/one-line-fixture-wrappers`, 2026-09-20
- **Raised by**: that lane, re-running its plants after folding the
  row's inline slab onto the shared fixture

Two defects that compound, and the second is why the first is
invisible.

**1. The `Err` arm prints and asserts nothing.** The row's shape is

```rust
match boolean_op_with(op, &slab, &log, …) {
    Err(e) => println!("log {op:?}: typed refusal: {e:?}"),
    Ok(out) => { …assert!((v - expect).abs() < 1e-9, …)… }
}
```

Its doc says *"Exact or typed; silence is the MAJOR"* — so a typed
refusal IS an accepted outcome by design. The cost is that **every
route into the `Err` arm is a pass**, and any perturbation that makes
the door refuse converts a would-be failure into a green print. This
is the same shape as `corpus-result-node-loops-skip-silently`, at one
row.

**2. The oracle restates the fixture's dimensions by hand.**
`let v_a = 16.0;` is the slab's volume, written as a literal. Until
2026-09-20 the slab was built four lines above it as
`brick((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), …)`, so the literal and its
premise were adjacent; the `dup/one-line-fixture-wrappers` fold moved
the construction to `common::operands::slab` and left the literal
behind. **The fold created the separation and this row records it** —
a comment at the site now says the constant is the fixture's volume
and has to move with it, which is a marker, not a fix.

**Measured.** Planting `operands::slab()` at 4 x 4 x **1.1** and at
4 x 4 x **3.0** reds neither of this row's three operations; the
sibling rows in `offd2_r1_probes` that share the fixture red at one or
other thickness. At 3.0 the log is fully buried, `v_a` is wrong by 32,
and the row is still green — which is defect 1 masking defect 2.

**What a fix has to decide**, and it is a coverage decision rather than
a cleanup: whether the `Err` arm should record WHICH refusal it
accepts (so an unexpected refusal reds), and whether `v_a` should be
derived from the fixture or stay an independent closed form. Reading
it back with the row's own `vol` helper would make the plant live and
would cost the oracle its independence from the kernel, which is
exactly the trade `common/oracles.rs`'s own rule governs.

## Why it sits here and not on S-DUP's slate

A row that cannot go red is a coverage defect, S-TINT's charter, and
explicitly not S-DUP's (`work/dup/plan.md`). The fold that surfaced it
is S-DUP's; the finding is not. `scripts/work.py territory` puts
`crates/sweep/tests/` on S-TCOST's and S-TINT's ground, and this is
the S-TINT half.
