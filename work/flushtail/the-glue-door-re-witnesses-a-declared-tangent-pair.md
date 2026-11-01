---
id: the-glue-door-re-witnesses-a-declared-tangent-pair
kind: issue
title: The glue door re-runs the tangency witness on a pair already declared Tangent
status: closed
opened: 2026-10-10
priority: P3
cost: E
closed: 2026-10-26
---


The glue door (`decided_declarations`,
`crates/topo/src/boolean/glue.rs:40`) reads every box-overlapping
cross pair that is not already declared, and sends a `Distinct` pair
of a ruled or rim-sharing kind to the tangency witness
(`tangency`, `glue.rs:94`), which runs the whole
`verify_tangency_declaration` lane on it.

On the G1 chain fixture of
`crates/sweep/tests/mate7a_r1_probes.rs`
(`p2_the_g1_chain_price_is_the_measured_195_rows`), where one face pair
is declared and the lane runs once for it on the declared path, the
door's reads put four `contact_tangent_on_1` rows in the log and the
first-order screen at 5 × `CERT_SAMPLES` stations, where it was one
× before INTENT stage 4 PR E. The price went from 69 rows to 195.

Two questions for whoever picks this up:

- Whether the declared pair is among the four reads. The skip
  (`declared(fa, fb)`, `glue.rs:56`) compares `(a, b)` as the
  declaration stores it, so a declaration keyed the other way round
  would be read again.
- Whether the witness lane needs a cheaper screen before it runs (a
  cylinder pair whose axes are not parallel cannot be tangent along a
  ruling).

The price is a cost, not a wrong answer: every outcome in the sweep
suite is the declared one.

## Closed (2026-10-26, tracker sweep)

Superseded: there is no declared pair left for the door to re-witness.
The boolean declare surface retired with INTENT stage 4 E and F
(`docs/DESIGN.md`, State of the build). On main the door's input is
`Glue::of(inputs)` (`crates/topo/src/boolean/ops.rs:708`), whose `pairs`
are empty whenever the operation decides its own verdicts
(`crates/topo/src/boolean/mod.rs:1171-1181`, `Verdicts::Decided`), and
the door returns at once when they are given
(`crates/topo/src/boolean/glue.rs:54`, `Verdicts::Given`, a union fold
step). So the `declared(fa, fb)` skip at `glue.rs:56-60` never fires on
a real operation, and the key-order question has no subject. The
fixture's price pin (`p2_the_g1_chain_price_is_the_measured_195_rows`)
is no longer in `crates/sweep/tests/mate7a_r1_probes.rs`. The second
question (a cheaper screen before the witness lane, such as non-parallel
cylinder axes) is a cost idea with no measured price on main; file it
fresh if a profile shows the door's tangency reads cost.

