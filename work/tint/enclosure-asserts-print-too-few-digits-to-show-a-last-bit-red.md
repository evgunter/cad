---
id: enclosure-asserts-print-too-few-digits-to-show-a-last-bit-red
kind: issue
title: geom-brep enclosure/chord-ceiling asserts print at .15e/.6e/.0e, so a red decided by the last bits prints as a pass
status: open
opened: 2026-09-18
---


## Finding

Found by the TESS lane that made `crates/mesh`'s domination asserts legible
(`tess/domination-assert-messages`), sweeping for the same shape outside its
fence. The shape: an assert whose condition is a **bare** ordering comparison
between a certified quantity and a sampled/oracle truth — so it can go red by
one ULP — and whose message prints the two sides at fewer than 17 significant
digits. When it fires at the last bits the message shows the two sides EQUAL,
and the reader has to reproduce by hand to learn anything. That is what cost
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

## What the sweep could not see

The pattern was: `assert!`/`debug_assert!` whose first argument contains an
ordering operator, classified by whether the message carries a `{…:.N}` /
`{…:.Ne}` spec with N < 17. Outside `crates/mesh` only that low-precision
class was read (48 hits workspace-wide, of which the rows above are the ones
that are bare bound-vs-truth comparisons; the rest are `rel < 1e-12`-style
tolerance checks where 16 digits resolve the tolerance). **Not read outside
`crates/mesh`:** the 457 message-less ordering asserts and the 2129 with a
message and no precision spec — an unlabelled pair of lossless tuples is in
that second class and the pattern cannot tell it from a labelled one. Also
invisible everywhere: `assert_eq!`-family rows, `if … { panic!(…) }`,
comparisons through `partial_cmp`/`total_cmp`/a helper's return value, and
messages assembled by `format!` into a variable first.
