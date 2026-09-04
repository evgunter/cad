---
id: reviewer-pair-rebuilds-two-trees-two-rules
kind: issue
title: Reviewer-pair rebuilds: two test-support trees now state opposite rules
status: closed
opened: 2026-09-03
closed: 2026-09-04
---


Raised by TCOST-10's style review. Two shared-helper trees in this repo
now state opposite rules about the same class of duplicate, and both
state theirs as the general one. Ev's ruling is asked for; nothing is
blocked on it.

## The two readings

**`crates/geom-brep/tests/shared/mod.rs`** (TCOST-7/8) ends its
not-absorbed list with: the reviewer-pair rebuilds "are the class this
tree must never absorb even when the text is identical", naming
`control`/`tmer`, `wall_domain`, `width`/`point_width` and `cone_dist`
across five reviewer/unit pairs. Its stated ground: "two such
derivations sharing one routine would be one derivation wearing two
names."

**`crates/sweep/tests/common/cavity.rs`** (TCOST-10) absorbs exactly
that class: the vented cavity and its builders are now shared by the
unit suite `blend3_concave_chamfer`/`blend4_concave_fillet` and by
`blend3_r2_probes`, `review_blend3_r1_probes` and
`review_blend4_r2_probes`, which are its review probes. Two of those
files' headers previously advertised the fixture as "re-authored here
… rather than imported from the unit's own suite"; this PR deletes that
claim.

## The distinction TCOST-10 claims, stated fairly

A reviewer-pair rebuild is protected because it is a SECOND OPINION.
That holds when the rebuilt thing is a DERIVATION — a seeding, a
quadrature, an evaluator, a closed form — because the pair then hold
two independently-reached answers and a merge silently leaves one.
`geom-brep`'s protected list is entirely of that kind.

A FIXTURE BUILDER is not a derivation. It carries no answer, so two
copies are not two opinions. What TCOST-10 argues they are instead is
two experiments: `blend3_r2_probes`'s P2 exists to audit the shipped
vented cavity geometrically, and if it builds its own likeness it
audits a body no other row measures — the audit becomes decorative in
exactly the way the audited claim would not notice. The independence
worth keeping there is the AUDIT (concavity read from the supports'
outward normals rather than from the kernel's classifier), which stays
with the probe, and the probes' own closed forms, which
`crates/sweep/tests/common/oracles.rs` protects under a rule of its own.

## The case against, stated fairly

1. `memories/review-and-dependency-policy.md` says a promoted reviewer
   suite's independence "is worth keeping where it pulls its weight",
   and does not carve out fixtures. A drift in the shared builder now
   reddens every suite the same way at once, which is the failure mode
   an independent rebuild exists to catch — the shared body cannot
   disagree with itself.
2. "Two experiments, only one measured" is a claim about THESE rows.
   A probe that re-authors the fixture and gets a different body has
   found something; under the merged builder it cannot.
3. Two trees stating opposite general rules is itself the defect,
   whichever rule wins: a third tree's author will read one of them.

## What a ruling would settle

Whether the protected class is "anything a reviewer rebuilt" or "any
DERIVATION a reviewer rebuilt", and therefore which of the two headers
gets rewritten. Either answer is cheap to apply: TCOST-10's merge is
mechanical to undo, and `geom-brep`'s bullet is one sentence.

## Ruling (Ev, 2026-09-04)

Neither reading. The policy memory was overstated: reviewer tests are
ordinary tests — a reviewer writes useful rows, the useful ones join
the permanent suite, and nothing about them is special afterwards. A
copy stays apart when a row's claim needs its own derivation, which is
a general test-design question, not a question of who wrote the row;
a fixture builder carries no derivation, so sweep's merge stands. The
"fixture drift" argument for keeping a probe's own builder is a
nuisance change detector, not a feature: the unit suites' own pinned
volumes and oracles already red on a changed body. Applied: the memory
rewritten (`memories/review-and-dependency-policy.md`, the index line,
`orchestration-model.md`), geom-brep's "never absorb" bullet replaced
by the reason-at-the-copy rule, sweep's "opposite line" paragraph
replaced by the ruling.
