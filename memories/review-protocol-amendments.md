---
name: Review protocol amendments (2026-08-18)
description: Disclosed non-improvement deviations need a concretely scheduled followup; reviews weight style/structure more heavily; A/B scores are not comparable across this boundary
type: convention
---

# Review protocol amendments, 2026-08-18

Ratified by Evan on the `docs/SMELL-SCAN-2026-08.md` process findings (§C).
Three changes, all aimed at defects the current protocol structurally cannot
see. **Each names the §C observation that motivated it**, so the reasoning
survives without re-reading the scan.

## A1. A disclosed deviation that is not an improvement needs a scheduled followup

**The problem (§C2, §C7).** Disclosure currently functions as immunity. The
A/B rubric's headline column is "silent devs", so writing a shortcut into the
PR body scores as a *positive* — PR #364 scored *"0 silent (5 deviations
reported)"*, and one of those five was the constant `DocumentId` that makes two
Python-authored documents un-coexistable in a workspace. Both blinded reviewers
read the disclosure and filed nothing. There is no counter-metric asking
whether a disclosed deviation was **acceptable**, only whether it was
disclosed.

**Explicitly NOT the fix:** penalising disclosure. Disclosure is working and
must keep being rewarded — the honesty in these PR bodies is what made the
whole smell-scan archaeology possible at all.

**The rule.** A deviation falls into one of two kinds, and the PR body must say
which:

- **An improvement** — the deviation is better than the spec's letter. Nothing
  further owed. (Most deviations are these.)
- **Anything else** — a shortcut, a narrowing, a placeholder, a fence artifact,
  a "can move there later", a "kept for now". These need a **concretely
  scheduled followup** before the PR merges: an issue number, or a named unit
  in a plan. *"Recorded as a pickup"*, *"deferred"*, and a comment at the
  constructor are **not** schedules.

**Why "concretely".** §C3: the repo has exactly one self-enforcing register —
`docs/guide/north-star-audit.md`, whose test fails as doors land. Everything
else is prose. The `DocumentId` deferral went into `LIB-LOG.md`'s residual
register, **which had closed the day before the PR merged**. A deferral
recorded only in prose is not deferred, it is forgotten on a schedule.

**Precedent that this works:** the `decide_flagged` census (issue #214) — a
machine-checked count with a per-family retirement plan, which has moved
exactly once, downward.

## A2. Reviews weight style and structure more heavily

**The problem (§C1, §C9).** Reviewers here are exceptionally strong at
soundness and structurally blind. The same reviews that ran 8000-matrix SVD
differentials, re-derived a meters conversion by hand, and found a certificate
excluding true 2π by ~1111 widths produced **zero** findings on: a mode switch
on `is_empty()`, a two-ε signature, a file holding four engines, three parallel
CDT pipelines, a second surface enum, or a body-wide accessor in the wrong
crate. Structural findings appear only as *side-effects of bug hunts*.

The mechanism is that the protocol is **claims-driven**: reviewers falsify the
claims they are handed, and they do it well. A code-free module, an accumulated
449-line header, and a duplicate type name across a façade **assert nothing**,
so nothing points a reviewer at them.

**The rule.** Review briefs carry structural questions alongside the claims to
falsify. **The brief text is `docs/REVIEW-STYLE-BRIEF.md`** — dispatchers paste
its §2 (the stance) and §3 (the questions) verbatim; §1 and §4 are dispatcher
context. The eight questions, in short:

- **Does this duplicate something?** §C11 is the cheapest mechanism this whole
  exercise produced: every duplication found is **self-declared in prose at the
  copy site** — *"verbatim"*, *"re-derived"*, *"ported from"*, *"mirror of"*,
  *"one dimension down"* — and nothing in CI, review, or the log ever reads
  that prose. A grep for those words over `crates/*/src` surfaces most of it in
  seconds.
- **Does the acceptance row go red when the guarantee degrades**, or only when
  it is violated at a chosen fixture? §C8: an exhaustiveness row named
  `..._refuses_typed_even_though_branches_were_found` has a premise that
  *excludes* the failing mode; an enclosure row asserting `pad > 0` plus
  containment is **monotone in the wrong direction** and gets easier as the
  enclosure degrades.
- **Did this PR invalidate a premise something else cites?** §C10, §C14: an
  invariant established by a bugfix must be swept across sibling
  implementations *in that same PR*, and a pin guards the invariant only as it
  was reachable when written.
- **Is the comment true, and is it attached to the right item?** Prose defects
  are currently caught only when executably refutable.

**Calibration note.** The doc-honesty axis already exists and is used, but has
almost no dynamic range: of 106 scored rows, **96 are 4 or 5, ten involve a 3,
and none is ever 1 or 2**. A rating with no lower tail is measuring "no doc
claim in this diff was executably refuted", not "the documentation is honest".

## A3. A/B scores are not comparable across this boundary

**Recorded in `docs/MODEL-AB-LOG.md` at the amendment row.** A2 changes what
reviewers are asked to look for and what the rubric rewards, so review-quality
figures (findings counts, MAJOR/MINOR mix, the docs column) collected **before
2026-08-18 are not directly comparable** with those after. Any cross-milestone
readout that spans the boundary must say so.

## What produced these

`docs/SMELL-SCAN-2026-08.md` §C, from twenty structural scans plus eleven
steelman and seven postmortem passes over the merged history. The postmortems
are the evidence base: each finding carries how it landed and whether a
reviewer flagged it.
