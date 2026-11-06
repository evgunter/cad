---
id: arc-carrier-has-three-spellings-under-a-comment-saying-one
kind: issue
title: seg.rs says arc_carrier is THE one spelling of the closed form; path.rs and lift.rs each carry another, with different association and abs placement
status: closed
opened: 2026-09-12
refs: [2409]
priority: P1
cost: C
closed: 2026-11-06
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

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to PATHS (opened at this exit as S-BOOL's successor for the profile lattice) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

## Closed (2026-11-06, ROUND orchestrator)

Fixed by PATHS 5b before this row was picked up. `465f43a450` (each
construction stores the arc it builds) dropped `path.rs`'s copy and
`65ef55b2f4` (one chord lowering in geom-core) dropped `seg::arc_carrier`:
the one chord lowering is now `geom_core::Arc2::from_chord`, read by
`profile::lower_arc` and `sweep`'s swept lowering, and `lift.rs` carries
no bulge closed form. The three neighbours the row named were checked
the same way: `perp`/`n_hat` is written several times (`seg::perp`,
`sugar::left_normal`, inline in `path.rs`) but each is an exact negation,
so they cannot differ in the last ulp; the sagitta's two forms
(`seg::sagitta`, `Margin::sagitta`) play different roles; and
`4·atan(x)` appears per construction on different inputs, not as one
closed form copied. None is the drift this row was about.
