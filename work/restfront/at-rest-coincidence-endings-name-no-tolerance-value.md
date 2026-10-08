---
id: at-rest-coincidence-endings-name-no-tolerance-value
kind: issue
title: validate's too_close ends in the levers alone where it holds the margin that would value the tolerance arm
status: open
opened: 2026-10-01
---


(PROPS' recourse-grammar lane, from
`work/props/coincidence-recourse-says-lower-where-d4-says-tighten`.)

## What

`topo::validate`'s `too_close(margin: Option<&MarginDiag>)` returns a
`&'static str`, so it cannot carry a number. PROPS' unit retired the
unvalued "lower the tolerance" arm from `geom_core::COINCIDENCE_RECOURSE`
and the two hand-spellings here followed it, which leaves the at-rest
reading naming the two LEVERS and no tolerance at all:

- `Recourse: declare the coincidence, or move the geometry`
- `Recourse: check the inputs that built this body, then declare the
  coincidence, or move the geometry`

That is honest — it offers no lever that does not exist — but it is less
than D4 ¶1 (i) allows. `Reading::AtRest` is a reading that MAY name a
tolerance (only `Reading::Adopt` may not), the arm is band-decided, and
the escalation that reached this classifier carries the margin and the
band that would value it. The one home for the valued sentence is
`geom_core::Indeterminate::ending(levers)`, which `geom_brep::recourse`'s
table and every escalation `Display` now compose.

## What would close it

Give `too_close` the escalation rather than its margin alone, and return
`Cow<'static, str>` from `Indeterminate::ending(COINCIDENCE_RECOURSE)` —
the same move `classify_mass_props`' props arm now makes through
`PropsCheck::ending`. Its callers inside `validate.rs` hold the
`Indeterminate`; the signature is what stands in the way.

## Why it is filed here

`crates/topo/src/validate.rs` is `restfront`'s by territory. PROPS' unit
edited `classify_mass_props`, `unnamed` and `too_close` in that file —
the three its own rows named — and announced the seam on this log; this
is the residue it did not take.
