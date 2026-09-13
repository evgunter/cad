---
id: fillet-inband-recourse-drops-the-tolerance-lever
kind: issue
title: Four fillet recourse sentences do not name the tolerance lever at their in-band site
status: open
opened: 2026-09-13
---


## Finding

`PathError::Escalated`'s fillet arm (`crates/profile/src/path.rs`) now
renders, for an in-band `fillet_*` verdict, the site and the gate's own
sentence from `validate::fillet_recourse_for` — and nothing else. Four of
the six sentences do not name the tolerance lever:

| sentence (`crates/profile/src/validate.rs`) | names "lower the tolerance"? |
|---|---|
| `FILLET_TURN_INBAND_RECOURSE` | yes — "(or lower the tolerance)" |
| `FILLET_OFFSET_LEVER_RECOURSE` | yes — "(or lower the tolerance)" |
| `FILLET_NO_CORNER_RECOURSE` | **no** |
| `FILLET_FIT_RECOURSE` | **no** |
| `FILLET_ENCLOSING_RECOURSE` | **no** |
| `FILLET_LEG_EXTENT_RECOURSE` | **no** |

At an IN-BAND site the tolerance is always a real lever: the margin is
inside `(ε, K·ε)` and a tighter ε makes it definite. Every other in-band
arm in the same `match` names one — the continuation arm ("widen the
input tolerance (K·ε)"), the seam-arrival arm, the leg-length arm, and
BLEND-10's stored-form sentence ("and so does lowering the tolerance").
The shared `COINCIDENCE_RECOURSE` these four used to carry named it too,
so what the caller reads at these four sites lost a true lever when the
tailored sentence replaced the shared one (BLEND-12).

## Why it was not fixed there

The six sentences are shared, by D4 ¶1's addendum, between each gate's
in-band escalation and its DEFINITE sibling refusal — at the definite
site the tolerance is not a lever, so the clause cannot simply be added
to the constant. The repair is therefore one of:

1. the arm adds the band clause itself, after the shared sentence (the
   in-band site knows it is in band; the definite site never reaches this
   arm);
2. the four constants gain a conditional clause and the definite arms
   stop sharing them — which is the D4 ¶1 addendum being re-opened, not a
   wording fix.

BLEND-12's spec put the sentences' wording out of scope ("the nine
predicates' bands and lever arms" and the shared recourse's wording) and
said the six constants stay where they live, so the choice is owed here.

## Evidence

- The arm: `crates/profile/src/path.rs`, `PathError::Escalated`'s
  `Display`, the `fillet_recourse_for` branch.
- The rendered text, from
  `crates/profile/tests/fillet_recourse_followability.rs`'s
  `the_offset_clearance_recourse_reaches_the_caller_and_reduces`:
  `resolving the fillet at this corner, 'fillet_offset_line_circle' could
  not be classified: <payload>. use a smaller radius, or move the legs so
  a circle of that radius can sit in the corner` — no tolerance lever.
- The contrast: `crates/profile/src/validate.rs`'s
  `FILLET_STORED_FORM_INBAND_RECOURSE`, whose tail is "and so does
  lowering the tolerance".
