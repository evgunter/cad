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
against `dev_samples` (`tools/tess-lint/README.md` `CC3`).

**The discriminant the kernel names does not reach the gate.** It is
`dev_samples == 0`, and that qualifies *"whenever the meter was armed
for deviation"* — the sweep CI gates on is `--sizing-only`, where
`dev_samples` is `0` on every row. On the gating sweep the two
meanings are indistinguishable from the CSV as it stands.

**Latent, not live.** No sized row of the committed
`docs/tess-budget-data/tess-budget-baseline.csv` carries
`worst_cert = 0` today.

## Why it is not METER unit 3's fix

Closing it needs a column the CSV does not have — the producer's
`tessellate` result, or `mesh::budget`'s own separation of the two
states — so the fix starts at `tools/tess-meter` and `crates/mesh`,
outside `D203`'s fence. `D203` classified it and left it: by the
README's `CC5` test the refusal is owed in the harness voice (a face
that certified nothing is not a tessellation that got better), but
there is nothing in the row to refuse it WITH.

## Refs

Found by `D203`'s sweep (METER unit 3), which fixed the one instance
that was closable from the consumer's side (`cap_bands` / `snap_bands`
against `bands`).
