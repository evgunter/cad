---
id: viewer-readme-recourse-count-does-not-say-what-it-counts
kind: issue
title: The README's 'recourse text is composed six ways across five modules' does not say whether it counts ways or sites
status: closed
opened: 2026-09-21
priority: P4
cost: E
closed: 2026-09-21
branch: vdoc/readme-counts
---


## Finding

Found by AUTH-1's style reviewer (AUTHOR, PR 2955, 2026-09-21),
confidence `unsure` — and the uncertainty IS the finding.

`crates/viewer/README.md` says recourse text *"is composed six ways
across five modules"*. AUTH-1 added `FaceFrameFault`'s `Display`
(`crates/viewer/src/session/refuse.rs`), a new site that composes
recourse text. The reviewer could not tell from the sentence whether
that makes seven, or whether it is one more instance of a way already
counted — because the sentence does not say what a "way" is.

**A measurement whose unit is unstated cannot be maintained**, and
nothing here could go red either way: a number over ways survives the
new site, a number over sites does not, and the reader cannot tell
which claim they are holding. `docs/prompts/reviewer-style-lane.md`
Q6's rule for a claim resting on a measurement applies — a mechanical
guard, a scheduled register that re-measures it, or a written reason
it can have neither, at the claim site.

## Disposition

Filed on VDOC because it is a claim the tree makes about itself, which
is this program's charter. A taker decides what the sentence counts,
says so in it, and re-derives the number under that definition — the
number moving is not a cost (`memories/output-stability-as-justification.md`).

Sibling rows of the same shape, to read first rather than re-derive:
`work/vdoc/viewer-readme-multi-field-write-sweep-count-does-not-reproduce.md`,
`work/vdoc/viewer-readme-driver-count-says-two-over-a-roster-of-eleven.md`,
`work/vdoc/the-citation-receipts-summary-numbers-are-not-re-derivable.md`.
Whether the four are one sweep is the taker's call.

## Closed — it counts SHAPES, and the shapes are now named (#vdoc/readme-counts)

**The unit is shapes, not sites**, and that is the reading the section
is about rather than the one that keeps the digit. Two things settle it.
The sentence's job is the claim it hangs off — *the rule does not say
the wording family has one shape* — and a count of SITES cannot support
that claim, since six sites could share one shape. And the commit that
wrote the sentence says so in its own words: `aba7ee8ef3`
(2026-09-04)'s `work/view/log.md` entry reads *"The wording family
still has six shapes across five modules"*.

**Saying "shapes" is only half the repair.** A shape has no syntactic
marker and no grep produces the population, so *six shapes* stated
alone is exactly the defect this row names one level in. The
enumeration rule is therefore **the list itself**, written into the
section: six shapes, five modules of this crate, each named with its
sites, and *a new site is a seventh member only when its shape is not
one of these*. That is the question AUTH-1's reviewer could not answer
and now can — `FaceFrameFault`'s `Display` is the second shape
(composed in the vocabulary's own `Display`) and its `NO_FACE_PICKED`
the third (a named `&'static str` with one home), so it adds no member.

**The population moved even though the digit did not**, and the section
says so. `AtRestBadge` used to be named BESIDE the six as a separate
fact; it is a shape like the other five — already stringified into the
value and stored — and is counted as the sixth. So *six* now means
something different from the *six* it replaces, which is the thing a
bare digit could never have told anyone.

The six, derived on the merged tree at `f45df59dc5`: the composer
functions on `Refusal` (`session::refuse`); composition in a
vocabulary's own `Display` arm (`Refusal`, `FaceFrameFault`); a named
`&'static str` with one home spent by more than one door
(`refuse::NO_FACE_PICKED`, `platform::NO_CHOOSER_BACKEND`,
`editor_core::edit::UNDECLARED_PARAM_RECOURSE`); a literal at the
chrome site (`pane::create`, `pane::profile`); a reason accumulated as
chrome-local data (`pane::create`'s `blocked`); and a refusal
stringified into the value (`session::AtRestBadge::Refused`).
