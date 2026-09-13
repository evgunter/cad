---
id: tess-lint-zero-certificate-two-meanings
kind: issue
title: tess-lint admits worst_cert = 0 as a reading, and the kernel says it has two meanings
status: open
opened: 2026-09-08
---


## What

`tess-lint`'s per-column admissions table states ONE meaning for a
zero certificate and the producer's own type states two.

`tools/tess-lint/src/lib.rs:504-506` — `Admissible::Certificate`:
*"finite and non-negative (zero is a face whose triangles are
exact)"*. `crates/mesh/src/budget.rs:159-167` — `worst_cert`:
*"**`0.0` can mean two things**: a genuinely tight face, or one that
refused before the emit pass ever ran (a failed insert, an empty
realised constraint, a self-touching trim loop) and certified nothing
— the caller's own `tessellate` result is what separates them, and
`dev_samples` is `0` in the second case whenever the meter was armed
for deviation."*

So the lint reads a face that certified NOTHING as a face whose
triangles are exact, which is the loudest reading the column can
carry, arriving as a benign zero. It is the same shape as `worst_dev`'s
two `NaN`s, which this crate already refuses at the parse boundary
against `dev_samples` (`tools/README.md` `CC3`).

**The discriminant the kernel names reaches the gate on ONE of the two
files it reads, and that is the correction to this item's first
draft.** `.github/workflows/ci.yml:4349-4355` runs the lint over two
CSVs through the same `tess_lint::parse`: the fresh sweep, cut with
`--sizing-only`, where `dev_samples` is `0` on every row and the
column says nothing; and the committed baseline
`docs/tess-budget-data/tess-budget-baseline.csv`, a full sweep whose
64 sized rows carry `dev_samples` between 2464 and 201096. So the
discriminant is live on the baseline side, and "there is nothing in
the row to refuse it with" was true of half the gate's input.

**Latent, not live.** No sized row of the committed baseline carries
`worst_cert = 0` today.

## Why the check is still not written

Not the fence — see below. Three things, in order of how much they
cost:

1. **The per-row check refuses a reading on the fresh half of the same
   gate.** `worst_cert == 0 && dev_samples == 0` is the refusal
   `mesh::budget`'s sentence licenses, and on a `--sizing-only` sweep
   `dev_samples` is `0` on every row — so the first genuinely tight
   face reds the gate as harness breakage. `tools/README.md`'s `CC5`
   is explicit that an admission never refuses what the instrument
   exists to measure.
2. **Narrowing it to the armed file makes it a JOIN, not an
   admission.** "This sweep was armed" is a property of the FILE (some
   row has `dev_samples > 0`), not of a row, and `CC5`'s second bullet
   sends a cross-row relation to the same test rather than to the
   per-column table.
3. **Even armed, the separation is the producer's to make.**
   `Deviation::NotResampled` is an admitted per-row state, so
   `dev_samples == 0` on an armed sweep means "certified nothing" only
   if an armed sweep cannot carry a legitimately unresampled sized
   row. That is a question about `mesh::budget` and the deviation
   pass, not about the CSV, and answering it by inference from a
   committed baseline is the shape `CC4` exists to refuse.

What closes it is one column: the producer separating "certified
nothing" from "certified zero" instead of spelling both `0.0`.

## Fence

**Corrected.** `tools/tess-meter` is inside METER's `paths`, not
outside them — the earlier draft said the fix "starts at
`tools/tess-meter` and `crates/mesh`, outside `D203`'s fence", and
only the second half of that is true. The row-shaping half (a column,
or an empty `worst_cert` for a face that certified nothing) is
METER's. The state to be separated is `mesh::budget::FaceMeasure`'s,
which is S-MESH's ground, and the tour's per-body `tessellate` result
(`demos/tour/src/tessbudget.rs`) is neither. So this is a METER unit
blocked on a kernel-side change, not an out-of-fence item.

## Refs

Found by `D203`'s sweep (METER unit 3), which fixed the one instance
that was closable from the consumer's side (`cap_bands` / `snap_bands`
against `bands`); premise and fence corrected by unit 3's fix pass,
which also closed `Extent`'s.

## Moved to INSTR (2026-09-08)

Moved from `work/meter/` to `work/instr/` by `git mv` when METER's exit
walk opened the successor (`docs/METER-EXIT-WALK.md` §4, ratified by Ev on
2026-09-08 at PR #2212). Id, header and body unchanged; the directory is
the claim. This row is one of the twenty on INSTR's opening slate.
