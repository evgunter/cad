---
id: enclosure-asserts-print-too-few-digits-to-show-a-last-bit-red
kind: issue
title: geom-brep enclosure and chord-ceiling asserts print fewer than 17 digits and do not name the failing end, so a last-bit red shows no usable margin
status: open
opened: 2026-09-18
---


## Finding

Found by the TESS lane that made `crates/mesh`'s domination asserts legible
(`tess/domination-assert-messages`), sweeping for the same shape outside its
fence. The shape: an assert whose condition is a **bare** ordering comparison
between a certified quantity and a sampled/oracle truth — so it can go red by
one ULP — and whose message prints the two sides at fewer than 17 significant
digits. What that hides depends on the spec: `{:.6e}` / `{:.0e}` print a
last-bit red as two EQUAL numbers; `{:.15e}` is 16 significant digits, which
shows a 2-ULP gap at magnitude ~1 as a one-digit difference in the last
place and shows a 1-ULP gap, or a 2-ULP gap at magnitude >= 5, not at all —
and in every case the reader cannot read off the margin, and is not told
which end failed, without reproducing by hand. That is what cost
`work/tess/nurbs-face-bound-unsound-on-a-random-rational.md` two weeks (its
"CORRECTION" section): a `{:.3e}` message on a 2-ULP red.

Hits, all in `crates/geom-brep/tests/`:

| site | condition | prints |
|---|---|---|
| `cert5_arm_and_cells.rs`, the C0-composite bracket row (the assert after `eprintln!("CERT5-CELLS c0-composite bracket …")`, `:213`) | `got.0 <= true_flux && true_flux <= got.1` — bare, both ends | `{:.15e}` (16 digits), and does not say WHICH end excluded the truth |
| `cert10r2_probes.rs`, the `probe2` chord-tolerance row (`:311`) | `worst <= delta_s` — bare | `{worst:.6e} > {delta_s:.0e}` |
| `cert5_r1_patch_probes.rs`, `FLUX ENCLOSURE EXCLUDES THE TRUTH` / `AREA ENCLOSURE EXCLUDES THE TRUTH` (`:81`, `:87`) | `lo - s <= oracle && oracle <= hi + s` — slackened by `sf`/`sa`, so weaker than the two above, but the slack is not printed and neither is the failing end | `{:.15e}` |

The repair is the one-line shape `crates/mesh` took: print every `f64` at
`{:.17e}` (or `{:e}`/`{}`, which are shortest-round-trip and lossless), label
the sides, and name the end/component that failed with its excess. `mesh`'s
spelling is `nurbs_cert::tests::Domination`; it is crate-private test support,
so a second crate wanting it is the argument for a home in
`crates/test-utils` beside `tightness::Sup` (whose `Display` is already
lossless and labelled, but is single-component and must reach `within`).
That home is asked for in
`work/tint/two-domination-helpers-with-opposite-operand-orders.md`.

## The rest of the sweep is its own row

These are the hits of the one class that was READ outside `crates/mesh`.
The unread remainder, and what the extraction cannot see at all, is
`work/tint/ordering-asserts-outside-mesh-unswept-for-illegible-domination-messages.md`
— closing this row does not discharge it.
