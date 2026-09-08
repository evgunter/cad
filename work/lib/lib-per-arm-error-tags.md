---
id: lib-per-arm-error-tags
kind: issue
title: per-arm Python error tags: one tag per op hides which arm refused
status: open
opened: 2026-09-03
needs_ev: false
---


Banked at LIB-TUBE (#1628), filed at its fix pass because a banked
finding with no item is a finding nobody can pick up.

`crates/pncad-py/src/tags.rs`'s `node_error_tag` gives every op ONE
tag. `revolve` covers all ten `RevolveError` arms; `tube` covers every
`TubeError` arm, including the three that only `hollow_tube` can
raise. So from Python a wall refusal and a frame refusal are the same
tag and differ only in prose, which is exactly the discrimination the
tag exists to spare a caller from parsing.

Not a LIB-TUBE defect and deliberately not fixed there: the tube
followed the convention every other op already sets, and changing the
convention for the one op whose unit happened to be written last would
leave the map inconsistent in a new way. The work is worth doing for
EVERY op at once — one pass over `node_error_tag`, deciding per family
whether the arms a caller can act on differently deserve their own
tags, with the census rows moving together.

Measured at LIB-TUBE: `NodeErrorKind::Tube` carries `Box<TubeError>`,
whose `Display` names the door on every arm, so the information is on
the wire today — it is only the TAG that collapses it.

## Question for Ev (2026-09-08, LIB orchestrator; `[ev]` PR)

This file, `pncad-py-seven-doors-lack-field-projection` and
`census-findings-cross-without-a-per-arm-tag` are one design question
asked three ways: **how a typed Python refusal projects WHICH arm
refused and WHAT the arm carried.** The bindings' stated rule is "the
payload is attributes, the message is prose", and today's curation
units (CUR5, CUR6) have been answering the arm half door by door under
one standing shape. Ev's ruling is asked once, so the remaining doors
become mechanical units instead of three more `[ev]` threads.

**The arm half (this file).** `node_error_tag` gives every op ONE word
(`revolve` for all ten `RevolveError` arms, `tube` for every
`TubeError` arm), so a wall refusal and a frame refusal differ only in
prose. Three shapes:

- **(A) A second word BESIDE the op tag** — `EvaluationError.kind`
  stays the op (`revolve`), and the kernel refusal's own arm arrives as
  a second attribute (say `inner_kind`), `None` on ops whose refusal
  has no arms. This is the shape CUR5 and CUR6 already shipped twice
  (`NodePickError.variant == "mesh_index"` beside `index_variant`;
  `StepImportError.variant == "recognition_ambiguous"` beside
  `promoted_kind`): the carrier's word stays put for callers branching
  on the op ladder, and the inner discriminant is matchable without a
  second class. One pass over every op family in `node_error_tag`,
  each inner enum matched exhaustively, the census rows moving
  together. Recommended.
- **(B) Forward the inner arm into `kind`** — `revolve_wall_too_thick`
  instead of `revolve`. Finer, but it deletes a shipped word a caller
  may branch on, and the two questions ("which op" / "which fault")
  collapse into one vocabulary a caller has to split by prefix.
- **(C) Leave it** — the information is on the wire in the prose
  (`Display` names the door on every arm) and callers parse it. That
  is exactly what tags exist to spare them.

Recommendation: **(A)**, as the standing rule for every refusal whose
carrier already projects a word.

## Ruled (2026-09-08, Ev on `[ev]` PR 2196; the choice left to the orchestrator)

Ev: "B works (though certainly with no prefix splitting — it'd be an
explicit correspondence somewhere), as does A if you think that's
really the most natural place to store the division into coarser
categories (which is plausible but I'm not sure)."

**Ruled (A), by the orchestrator under that licence**, for one reason
that is the shape's rather than a preference: the two words are two
different enums' discriminants, not one division stored twice. The
coarse word is the CARRIER's — which door refused, fixed by the node
kind before any payload is read — and the fine word is the kernel
refusal's own arm, which only exists once the carrier has said which
enum it holds. Projecting each where it lives is the LB17 carrier rule
and is what CUR5 and CUR6 already shipped twice (`variant` beside
`index_variant`, `variant` beside `promoted_kind`). B with an explicit
correspondence is A with the words swapped plus a table nobody
otherwise needs — and it moves every shipped `kind` value (`revolve`,
`tube`, …) that census, fixture and caller rows already branch on,
where A adds a word and moves none. Ev's reservation is recorded here
so the first unit under the rule states, at the door, why the second
attribute is the natural seat and not a convenience. **Standing rule:
every refusal whose carrier already projects a word gains its inner
arm as a second attribute (`inner_kind` or the enum's own name where
CUR5/CUR6 chose one), present on every op and `None` where the
refusal has no arms; each inner enum matched exhaustively, no
wildcard.** One mechanical unit over `node_error_tag`, no A/B row.
