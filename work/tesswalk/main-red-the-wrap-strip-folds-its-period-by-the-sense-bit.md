---
id: main-red-the-wrap-strip-folds-its-period-by-the-sense-bit
kind: issue
title: Main red: topo's face_normal guard reads mesh/walk.rs's wrap strip folding TAU by the sense bit by hand
status: open
opened: 2026-11-11
priority: P1
cost: E
---


## Finding

Filed by PCERT's orchestrator. Its PR 6836 lane met this red on main, and the
orchestrator reproduced it on `origin/main` at 57551229c8.

`topo`'s slow-set guard `face_normal::tests::no_source_file_folds_the_bit_by_hand`
(`crates/topo/src/face_normal.rs:1400`) fails. It finds one hit that is on neither
`sanctioned` nor `pinned`: `("crates/mesh/src/walk.rs", "TAU")`.

The hit is the wrap strip's period, `crates/mesh/src/walk.rs:964`:

    period: if rising == sense { -TAU } else { TAU },

That line came in with PR 6823 (`tesswalk/one-face-ball`, commits ef278b7752
and a420ca2a7c). The guard is in the slow set, so PR CI did not run it.

## Repair

Do one of these:
- Fold the strip's direction through the sense door the guard names.
- If the period is a genuinely signed scalar negated at its point of use (D6),
  add `("crates/mesh/src/walk.rs", "TAU")` to the guard's `sanctioned` list,
  with a one-line note saying what it negates.

Reproduce with:

    cargo test -p topo --lib face_normal::tests::no_source_file_folds_the_bit_by_hand

## Pinned meanwhile (GREEN, 2026-11-14)

Still red on main `b363f5fa13`, from nightly 43371638371. Bisected on main's first parent: the `-TAU } else { TAU }` line is absent at `eb319b9df5` and present at PR 6823's merge `38d647a540` (`tesswalk/one-face-ball`), confirming the row. GREEN (`green/censuses`) put `("crates/mesh/src/walk.rs", "TAU")` on the guard's `pinned` list, beside `trimmed.rs`'s, citing this row, so the suite is green and the repair reds the guard until the entry is struck. The choice between the sense door and `sanctioned` stays this row's; taking either strikes the pin.
