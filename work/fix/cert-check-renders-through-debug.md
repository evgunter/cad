---
id: cert-check-renders-through-debug
kind: issue
title: CertifyError renders CertCheck through Debug at a user surface — the Display class in an eighth crate, on no program's ground
status: open
opened: 2026-09-11
refs: [verb-error-arity-renders-verbkind-through-debug]
---


From the `verb-error-arity-renders-verbkind-through-debug` lane's
sweep. `CertCheck` (`crates/geom-brep/src/certify.rs:89`) is a public
fieldless enum naming WHICH certification check was at fault. It has no
`Display`, and `CertifyError`'s `Display` renders it through `Debug` at
three arms:

- `crates/geom-brep/src/certify.rs:369` —
  `"certification: {check:?} residual at sample {sample} definitely
  exceeds the tolerance band …"`
- `crates/geom-brep/src/certify.rs:412` —
  `"certification: {check:?} (not a sampled check) escalated: {cause}"`
- `crates/geom-brep/src/certify.rs:420` —
  `"certification: {check:?} at sample {sample} escalated: {cause}"`

The rendering is word-shaped (`EndpointStart`, `Surface1Residual`,
`WitnessSurface2`), so it does not trip the prose gate and no census
sees it. What it is, is a type identifier standing in a sentence a user
reads: a variant renamed for an internal reason silently re-spells the
refusal, and the sentence's claim that the identifier IS the check's
name is made nowhere the type can see.

The parent class's decision (PR 2347 for `profile::path::Verb`, and
this item's sibling for `VerbKind`/`Arity`) is the shape: the
vocabulary says its own words once, in an impl or on the declaring
row, and the consumer forwards. Whether each check's word is its
identifier (`EndpointStart`) or the sentence a person would write
(`the start endpoint`) is the only decision, and it is worth making
deliberately here: unlike a verb name, these are not doors anyone
CALLS — they are internal check ids, and the sentence around them
(`{check:?} residual at sample {sample}`) already reads as a
diagnostic.

`crates/geom-brep/src/certify.rs` is in no open program's `paths`
(`props` owns `geom-brep/src/props/*`, `offset_fit.rs` and
`patch_bound.rs`; `trim` owns `pcurve_cache.rs`, `nurbs_iso.rs` and
`edge_nurbs.rs`; neither reaches this file), which is why this is
homed on FIX's slate by the same argument that homed the `VerbKind`
row here.

## The rest of the sweep is other programs' ground

The same sweep found 26 further sites of this exact shape, every one
of them inside a fence: `topo/src/boolean/mod.rs` (`Operand`,
`RadiusEvidence` — BOOL/CURVED), `topo/src/shell.rs` and
`topo/src/replace_face.rs` (`SurfaceKind` — SHELL),
`topo/src/splitting/finish.rs` (`PlaneSide` — BOOL/CURVED),
`geom-brep/src/offset.rs` (`SurfaceKind` — SHELL),
`geom-brep/src/pcurve_cache.rs` (`PcurveCheck` — TRIM),
`geom/src/curves/compose.rs` (`SeamSide` — PROPS),
`editor-core/src/persist/check.rs` (`TipState` — DOCM, and argued-KEEP
there: that sentence names the lattice COORDINATE deliberately, the
rule PR 2347 left intact). They are listed with their dispositions in
the sibling row's PR and are not scheduled by this item.
