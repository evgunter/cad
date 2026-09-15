---
id: fillet-inband-recourse-drops-the-tolerance-lever
kind: issue
title: Three fillet recourse sentences omit the tolerance lever where it is real, and two gates have none
status: open
opened: 2026-09-13
---


## Finding

`PathError::Escalated`'s fillet arm (`crates/profile/src/path.rs`) now
renders, for an in-band `fillet_*` verdict, the site and the gate's own
sentence from `validate::fillet_recourse_for` — and nothing else. Three
of the six do not name the tolerance lever where it is real, and two
gates have no such lever at all:

| sentence (`crates/profile/src/validate.rs`) | names the tolerance lever? | is the tolerance a lever at its gate? |
|---|---|---|
| `FILLET_TURN_INBAND_RECOURSE` | yes — "(or lower the tolerance)" | yes (linear band) |
| `FILLET_OFFSET_LEVER_RECOURSE` | yes — "or lower the tolerance" | yes (linear band) |
| `FILLET_NO_CORNER_RECOURSE` | **no** | **only for three of its four gates** |
| `FILLET_ENCLOSING_RECOURSE` | **no** | yes (linear band) |
| `FILLET_FIT_RECOURSE` | **no** | **NO — exact-order band** |
| `FILLET_LEG_EXTENT_RECOURSE` | **no** | yes (linear band) |

**The tolerance is a lever at a gate classified against the run's LINEAR
band, and at no other.** `fillet_leg_fit` and `fillet_leg_reach` classify
against the exact-order band `Band::new(f64::from_bits(1),
f64::from_bits(2))` (`crates/profile/src/sugar.rs`'s
`line_fillet_trims` and `arc_fillet_corner`), which is fixed at two f64
bit patterns and does not move with ε at all. Adding "or lower the
tolerance" to `FILLET_FIT_RECOURSE` — and to `FILLET_NO_CORNER_RECOURSE`
on the `fillet_leg_reach` arm it shares — would render a lever that does
nothing at those sites, which is the same defect this file is about,
pointed the other way.

So the repair is narrower than "add the clause to four sentences": it is
a lever at `fillet_enclosing_carrier`, `fillet_corner_arm` and the three
offset clearances, and not at the two exact-band gates. At the linear-band
sites the tolerance IS real — the margin is inside `(ε, K·ε)` and a
tighter ε makes it definite — and every other in-band arm in the same
`match` names one: the continuation arm ("widen the input tolerance
(K·ε)"), the seam-arrival arm, the leg-length arm, and BLEND-10's
stored-form sentence ("and so does lowering the tolerance"). The shared
`COINCIDENCE_RECOURSE` those sites used to carry named it too, so what
the caller reads there lost a true lever when the tailored sentence
replaced the shared one (BLEND-12).

**Corrected 2026-09-13** (BLEND-12 fix pass, R2 MINOR-3): the original
text of this file asserted "at an in-band site the tolerance is always a
real lever", which is false at the two exact-band gates. Two of its four
listed sentences have also since changed: `FILLET_TURN_INBAND_RECOURSE`
and `FILLET_OFFSET_LEVER_RECOURSE` were rewritten in the fix pass and
both now name the tolerance, so the row count is four → three.

## Why it was not fixed there

The six sentences are shared, by D4 ¶1's addendum, between each gate's
in-band escalation and its DEFINITE sibling refusal — at the definite
site the tolerance is not a lever, so the clause cannot simply be added
to the constant. The repair is therefore one of:

1. the arm adds the band clause itself, after the shared sentence — and
   it can do so correctly, because the arm has the `Indeterminate`'s own
   `band` in hand and can tell an exact-order band from the run's linear
   one, which the constants cannot;
2. the affected constants gain a conditional clause and the definite arms
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
