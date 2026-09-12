---
id: arc-carrier-has-three-spellings-under-a-comment-saying-one
kind: issue
title: seg.rs says arc_carrier is THE one spelling of the closed form; path.rs and lift.rs each carry another, with different association and abs placement
status: open
opened: 2026-09-12
refs: [2409]
---


## Finding

From the full review of WIRE's PR 2409 (S1, confidence `sure` on the
three spellings, `likely` that they can differ). Filed here by the WIRE
orchestrator because `crates/profile/*` is S-BOOL's glob.

`crates/profile/src/seg.rs:141-148` says `arc_carrier` is *"the ONE
spelling of it."* The crate has **three**:

| site | shape |
| --- | --- |
| `seg.rs:149` | `ChordFrame`, `len = a.distance(b)`, `mid = a.lerp(b, 0.5)`, `radius = (…).abs()` |
| `path.rs:2203` | `l = chord.norm_squared().sqrt()`, `mid = a + chord*half`, `/(4*bulge.abs())` |
| `lift.rs:650` | f64-only, `Option`-returning, the same formula a third time |

Different **association** and different **`abs` placement**, so they
need not agree in the last ulp. This is Q2's sharpest shape: the comment
asserting singularity is the only thing tying them together, and the
code compiles either way.

## Scope — what this does NOT touch

PR 2409's bit-identity claim is **unaffected** and was verified: it is
correctly scoped to `seg::arc_carrier`, and the review confirmed
`ValidatedSegment::lift` and `build_seg` call the same `pub(crate) fn`
at `seg.rs:149` on scalars carried verbatim, with nothing rounding
between. Nothing here is a defect in that PR; the sentence at
`seg.rs:141-148` was already false when it was written.

## The class, and where to look next

The reviewer names three more places the same question should be asked
inside this crate, and a taker should not stop at `arc_carrier`:

- `perp` / `n_hat`
- the sagitta
- `theta = 4·atan(b)`

The instrument is the **data**, not the prose: the same constant, the
same literal ladder, the same closed form written twice with no sentence
admitting it. A prose sweep finds only the disclosed copies — and here
the disclosure says the opposite of the truth, so prose would have
actively misdirected it.

## What a taker owes

One home, or a corrected sentence at `seg.rs:141-148` saying how many
spellings there are and why each exists. A difference in the last ulp
between two of them is a real possibility that nothing currently pins,
so a row asserting agreement across the three (or measuring where they
diverge) is worth more than the consolidation on its own.
