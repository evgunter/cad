---
id: classify-dihedral-callers-do-not-all-establish-its-spline-premise
kind: issue
title: three of five classify_dihedral callers that can see an arbitrary face do not establish its no-spline-kind premise
status: open
opened: 2026-10-01
---


(PROPS' recourse-grammar lane, from the reach sweep
`work/flux/invalid-margin-recourse-cannot-tell-an-unimplemented-kind-from-bad-inputs.md`
asks for.)

## What

`geom_brep::implicit_gradient` and `curvature_lever_arm` return POISON
for `Surface::Nurbs | Surface::Approx` — neither kind has an implicit
form — so `geom_brep::classify_dihedral` cannot answer about a spline
face at all: it escalates with an invalid `MarginDiag` and a recourse
that tells the caller to check its inputs, which were fine.

The premise (neither surface is a spline kind) is the CALLER's to
establish, and the type does not enforce it. Two callers that can see an
arbitrary body face do establish it; three do not.

Establishes it:

- `topo::validate`'s tier-3 dihedral check — `nurbs_adjacent` from
  `Surface::spline_chart().is_some()` on both sides, with an O5 note
  recording why the gate inherits.
- `topo::boolean::rim_wedge` — the same `spline_chart()` gate.

Does not:

- `topo::census` (the smooth-site candidate walk, `material_wedge_side`);
- `topo::boolean::ops` (the wedge classification behind the Boolean's
  face-pair refusal);
- `topo::splitting::finish`.

`geom_brep::certify`'s `wedge_decided` call has no gate either, which is
`geom-brep`'s own and is noted rather than filed here.

A `Surface::Nurbs` face is reachable at rest: STEP import mints them
(D7), and `sweep::skin` fits them. The sweep did not build a body that
drives one of the three ungated calls — doing that is the first step on
this row — so the reach is argued from the kind inventory, not witnessed.

## Why it is filed here

`crates/topo/src/census.rs` is `contact`'s by territory and
`boolean/ops.rs` / `splitting/finish.rs` are `cleave`'s, `hone`'s and
`reach`'s; the finding is one fact about one premise across the three, so
it lands on the crate's own slate rather than being split three ways. A
lane taking it should announce the seam on those programs' logs.

## What would close it

Either each ungated caller answers the kind question before asking (as
tier 3 and `rim_wedge` do), with its own typed refusal for the kind, or
the premise moves into the predicate as a typed refusal — which is the
flux row above, and the choice there is a design one.
