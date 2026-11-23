---
id: the-charge-gate-finds-charging-methods-by-their-name-prefix
kind: issue
title: The bare-rounding gate finds Rounded's charging methods by their with_ prefix, so a charging method by another name goes uncounted
status: open
opened: 2026-11-22
priority: P3
cost: E
---


Found by the delta review of PR 7221 (`oneround/rounded-pairs`).

`scripts/gates/bare-rounding-allowlist.sh` reads `Rounded`'s charging
methods from `geom_core::running`. It takes them to be the nested
`pub fn` items whose name starts with `with_`, and counts each call of
one (`.with_charge(` today) as a site. A method that adds a charge
under another name is not found. The reviewer planted
`pub fn plus_charge(self, c: T) -> Self` in `running`'s impl, with a
call outside geom-core, and the gate stayed green at the listed counts.
The convention is unwritten: nothing in `running` or the README says a
charging method must be named `with_…`.

Two fixes are possible. One is to say the convention where it binds
(`running`'s module docs and the README's backstop sentence) and have
the gate red on a nested `pub fn` of `Rounded` that returns `Self` and
adds to `error` under any other name. The other is to read charging
methods by what they do rather than by name: a `Rounded` method whose
body writes `error` from an argument.

A second note from the same review. N4's pin
(`implicit::tests::a_conics_extremes_are_its_forms_within_its_rounding`)
catches the dropped noise only through an `assert_eq` on the formula's
own spelling (`(h.c0 - swing) - h.noise`), not through its
sampled-residual containment. So it pins the spelling, not the bound. A pin that holds the extremes against an exact
evaluation of the rounded form at a pose where the noise decides a
containment would be the stronger row.

P3: the one charging method today is found, and the mutant caught.
