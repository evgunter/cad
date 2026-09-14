---
id: register-equals-witness-limits-citation-names-no-file
kind: issue
title: Real::register_equal's doc cites geom-core/tests/m10_9_witness_limits_interval.rs, a file that has never existed
status: closed
closed: 2026-09-14
opened: 2026-09-14
---


**Found by SYM-6's read of the door** (`docs/SYM-6-SPEC.md` item 5 asked
for `m10_9_witness_limits_interval` to be re-read and its numbers
re-taken; there is nothing to read).

`Real::register_equal`'s contract paragraph in
`crates/geom-core/src/real.rs` ends:

> The rows that establish this are
> `geom-core/tests/m10_9_witness_limits_interval.rs`.

No such file exists, and none ever has:
`git log --all --diff-filter=A -- '*witness_limits*'` is empty, and the
only two mentions of the name anywhere in the tree are that line and
the SYM-6 spec quoting it. So the sentence that carries the door's
sharpest admission — *the door cannot tell an identity from a
coincidence* — points its evidence at nothing, and a reader who goes
looking finds an empty `ls`.

**The rows it means exist under other names.** The claim's evidence is
`crates/geom-core/tests/m10_9_r1_sym_probes.rs` — `x² ≡ x` recorded over
`[0.9, 1.1]` (`r1_a_coincidence_at_one_point_of_the_box_registers_and_decides_zero`)
and the 0.1%-wrong rim recorded at `Interval` and refused at `f64`
(`r1_a_geometric_lie_the_f64_witness_refuses_is_recorded_at_interval`) —
together with `m10_9_r2_sym_probes.rs`'s wide-box row
(`r2_the_interval_witness_lets_a_geometric_lie_through_over_a_wide_box`).

**CLOSED by SYM-6's fix pass (PR #2604).** Both reviewers raised it
independently (R1 S8, R2), the orchestrator amended the announced seam to
cover the line, and the citation now names the three rows above — the same
paragraph was being re-taken for the ε spelling, so the repair landed with
it rather than waiting for PROPS.

**Why it is worth a row rather than a shrug.** The discipline's citation
rule is *cite by name; line numbers rot* — a name is supposed to be the
durable half. A name that resolves to nothing is worse than a stale line
number, because it reads as a receipt and is not one, and nothing in the
gates checks that a cited path exists.
