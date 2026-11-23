---
id: tangent-arc-to-needs-an-explicit-tangent-where-line-takes-the-directed-point
kind: issue
title: tangent_arc_to needs an explicit .tangent() after a leg, while line(len) takes the leg's directed point
status: parked
opened: 2026-11-02
priority: P1
cost: M
blocked_on: [continue-to-and-the-bare-leg-end-rows-retire]
---

Reported by Ev (in chat, 2026-11-02): "`tangent_arc_to` appears to
require an explicit `tangent` in advance, whereas `line` assumes
tangency. these should be made consistent though i'm not sure how best
to do that yet." Filed by the VSEAM orchestrator.

## What the vocabulary says

`docs/PATHS-DESIGN.md` §3, the transition table:
- `line(len)` — "Directed → Point; directed point → directed point".
  Off a directed point (a leg's arrival), the leg "departs along the
  point's own intrinsic tangent".
- `tangent_arc_to(p)` — "Directed → Point" only. After a leg, the
  author writes `.tangent()` first to bind the director; the corpus
  spells it `.tangent().tangent_arc_to(p)` (§2, §2a item 6, and the
  worked examples).

So two continuations that both mean "carry on smoothly from here" take
different incoming states. One reads the arrival's tangent implicitly,
and the other needs it bound explicitly.

## What deciding it needs (`design`)

Pick which reading is right for the family and apply it uniformly:
- both read a directed point implicitly;
- both need the director bound;
- or the difference is principled and wants stating.

Weigh it against the §2 departure rules, the junction checks
(`JunctionTangent`), `continue_to` and the arc analogs of `line(len)`
(`ArcLen`/`Sweep`, "tangent-departing, endpoint DERIVED"), and against
`[one-way]`. It goes to the designer pair before any lane builds it.
PATHS-DESIGN is ratified (#124), so the answer is an `[ev]` PR. P1: two
spellings of one continuation that should behave uniformly.

## The question as weighed (2026-11-20)

Two designers weighed it and read the tree and the PR threads. What they
agree on:

- **The split is an accident, and it is seven rows wide.** In the Rust
  table (`transition_table!`, `crates/profile/src/path/program.rs`), at a
  bare leg end `line(len)`, `fillet`, `fillet_arc`, `continue_to` and
  `arc_fillet(Radius)` take the arrival tangent with nothing written.
  `tangent_arc_to`, `arc_to(Sweep/ArcLen)` and
  `arc_fillet(Sweep/ArcLen)` refuse until `.tangent()` is written.
  `arc_fillet(Radius)` refuses after `.tangent()`, and `continue_to` is
  ill-typed there.
- **`line(len)` and `.tangent().line(len)` are two spellings of one act**,
  both in live use, and so are the two fillet spellings. The lift writes
  `.tangent()` then `line(len)` while the `JunctionTangent` recourse
  names the bare form.
- **The doors disagree.** The typed Python door has no `line` and no
  `continue_to` at a leg end. Python's `Chain` has no `tangent` at all.
  The document layer and the viewer read the Rust table.
- **Nothing decided it.** The leg-end `line(len)` row was built on
  2026-09-01 to spell a same-carrier run that `.tangent()` then refused;
  the sixth-round ruling of the next day retired that distinction, and
  neither spelling was removed. No PR thread holds a word of Ev's on
  whether a smooth departure is written.
- **`docs/PATHS-DESIGN.md` contradicts itself here.** §2 says legs depart
  Directed only, against §3's leg-end rows. §3's `line` row says
  "nothing is declared", against §4 item 4.

The question that remains is one: **must carrying on smoothly from a leg
end be a written word?**

- **No word.** A heading-taking leg departs along the tip's heading: the
  written director if there is one, otherwise the arrival tangent.
  `.tangent()` is deleted; `continue_to` stays, at a leg end only.
- **A word.** Leaving a leg end, the author says what the joint is: a
  director for a corner, `.tangent()` to carry on, `.cusp()` to reverse.
  After `.tangent()` a heading-taking leg is supplied the direction and
  `line_to(p)` is checked against it, which replaces `continue_to`.

Either way: one spelling per act, the heading-taking arc modes
(`Radius`, `Sweep`, `ArcLen`) legal at every tip that has a heading, the
Python door brought into line, and the contradictions above repaired.
The two states build the same loops.

## Ruled, and cut into units (2026-11-22)

Ev ruled on PR 7251 (2026-11-21): a smooth departure from a leg end is
always written `.tangent()`. `tangent_arc_to` gives way to the `arc_to`
mode `End{p}`, and `continue_to` to `.tangent().line_to(p)`, the checked
straight leg. The bare leg-end rows of `line`, `fillet`, `fillet_arc` and
`arc_fillet(Radius)` go. The ruled text is `docs/PATHS-DESIGN.md` as that
PR leaves it, so the `design` flag is off.

The build is the SMOOTH program (`work/smooth/plan.md`), and this row
moved there by `git mv` from `work/profile/`. Its five units are, in
order:

1. `a-tangent-tip-is-its-own-state-and-line-to-is-checked-from-it`, and
   `arc-to-gains-the-end-mode` beside it;
2. `the-fused-verbs-take-end-and-radius-at-every-directed-tip` and
   `tangent-arc-to-retires-into-arc-to-end`;
3. `continue-to-and-the-bare-leg-end-rows-retire`.

The row is parked on the last of these and closes in its PR.
