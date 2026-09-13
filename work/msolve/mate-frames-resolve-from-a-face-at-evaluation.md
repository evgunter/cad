---
id: mate-frames-resolve-from-a-face-at-evaluation
kind: issue
title: a MateFrame arm that names a face and resolves at evaluation, and A11's inputs sentence
status: open
opened: 2026-09-09
---

Handed over by LIB under Ev's ruling (F) on
`work/lib/no-door-mints-mate-frame-from-face.md` (`[ev]` PR #2256,
2026-09-09): the design half is this program's, in `mate.rs` and
`mate/solve.rs`; LIB's façade and Python half follows the arm.

## The ruling, and why

Mates today are plain numbers: `crates/editor-core/src/mate.rs`
carries no `Expr` and no `ParamName` — `MateFrame` is three
`[f64; 3]`, `PlanarRest.offset` is `f64`, the clocking rider
`Option<f64>`. A document parameter change in a part stales every
mate on it silently until the at-rest gate refutes it. Ev's question
was whether the solve can avoid reading geometry forever; measured, it
cannot even read a parameter, so the answer is no, and the design
moves now rather than later.

## The shape ruled

- `MateFrame` gains an arm: the authored vectors as today, or
  `FromFace { face: StableName, reference: Option<[f64; 3]> }`,
  resolved at EVALUATION through the exact `face_pose` readback
  (`crates/topo/src/readback.rs`: plane/cylinder/cone/sphere/torus off
  their surface parameters, no tolerance; a NURBS face refuses typed,
  `NoCanonicalFrame`, and keeps taking authored vectors). `reference`
  is used when the face carries no `u_ref` and refused, typed, when
  neither does (`placement` refuses a reference parallel to the axis).
- `solve_document` runs over RESOLVED frames — it takes, or runs, the
  mated parts' evaluations. The solve itself is unchanged: structural,
  decided predicates, no geometry inspection inside it.
- Nothing is stored twice: the face name is the state, the frame is
  derived. The drift `no-door-mints-mate-frame-from-face` was filed
  about disappears rather than being reported.

## What it revises

A11's wording in `docs/DESIGN.md` and the `mate.rs` module docs
("nothing here reads geometry: the frames are authored data") — the
ALGORITHM claim (coset intersection over decided predicates, no
numeric fitting) stays; the INPUTS sentence becomes "the document plus
its parts' evaluations". A DESIGN.md revision is discussed with Ev
before it is ratified; Ev has agreed to the direction on the PR, and
the wording is this program's to draft.

## Not built

(A) — a door that freezes a face's frame into authored numbers — is
not built separately: freezing is materializing the arm.
