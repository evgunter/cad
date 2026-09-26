---
id: tess-budget-baseline-names-predate-piece-naming
kind: issue
title: the tess-budget baseline's name column still spells rims and walls by index, not by the minted piece steps #3223 introduced
status: open
opened: 2026-09-25
---


## Finding

`docs/tess-budget-data/tess-budget-baseline.csv`'s `name` column was
cut before #3223 ("profile pieces are named by minted step ids") landed,
and nothing re-cut it after. A full `scripts/tess_budget_sweep.sh` on
the merged tree (measured on PR #3245's branch, 2026-09-25) re-spells
285 rows' `name` cells, e.g. `diefillet/diefillet,2`'s
`{"Lateral":{"loop_index":0;"segment":0}}` becomes the piece-step
spelling. Every other column, on every row, is byte-identical
(compared with the `name` column cut out).

The gate did not notice. `tools/tess-lint` joins rows on scene, face and
`chart` (`tools/tess-lint/src/lib.rs`, the join docs), not on `name`,
so a stale `name` column passes. A reader who takes the column as the
face's current identity reads a spelling no evaluation mints any more.

## What would close it

Re-cut the baseline on main, the same way the script's header says,
and update `tools/tess-lint/tests/cut_line_pin.rs`'s transcription of
the cut line along with it. Or decide that the `name` column is
informational, and say so where the column is documented.
