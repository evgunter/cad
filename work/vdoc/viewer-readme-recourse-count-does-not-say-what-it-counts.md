---
id: viewer-readme-recourse-count-does-not-say-what-it-counts
kind: issue
title: The README's 'recourse text is composed six ways across five modules' does not say whether it counts ways or sites
status: open
opened: 2026-09-21
priority: P4
cost: E
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
