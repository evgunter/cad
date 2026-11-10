---
id: main-red-two-offered-tolerances-fail-just-below-their-margin
kind: issue
title: main red: offer_rows every_offered_tolerance_passes_just_below_it quotes two false offers (an arc root just outside its span, roots a hair apart)
status: open
opened: 2026-11-10
priority: P2
refs: [6705]
---


Found by `onecopy/held-and-pierce` (PR 6705) while running the slow set
in its touched files. It is red on main at 53ed936f (checked in a clean
worktree), and on 6705's branch with and without its changes, so it is
not 6705's.

`topo boolean::refusal_routes::offer_rows::every_offered_tolerance_passes_just_below_it`
(slow set; `crates/topo/src/boolean/offer_rows.rs:1989`) fails at the
default ε:

```
false offers:
arc_root_just_outside_its_span: quotes margin Some(5.500000048223606e-9), computed -5.5e-9: whether a crossing lands strictly inside its edge is undecided … tighten the tolerance below …
roots_a_hair_apart: quotes margin Some(5.499881129594528e-9), computed -5.5e-9: which of two crossings on an edge comes first is undecided … tighten the tolerance below …
```

Both rows quote a positive margin of about 5.5e-9 where the fixture's
computed margin is −5.5e-9. A margin read with the wrong sign, or an
absolute value taken before the quote, would show exactly this.
Nightly runs the slow set, so the nightly is red on it too. Bisect from
the last green nightly.
