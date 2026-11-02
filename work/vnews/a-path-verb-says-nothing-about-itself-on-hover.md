---
id: a-path-verb-says-nothing-about-itself-on-hover
kind: issue
title: A path verb in the profile editor says nothing on hover: no one-sentence meaning, no line of which incoming states it accepts
status: open
opened: 2026-11-02
priority: P1
cost: M
---

Reported by Ev (in chat, 2026-11-02): "hovering over each path verb
should pop up a 1-sentence explanation of what it is, plus another
line which should have a consistent shared format which says which
kinds of incoming states it's compatible with". Filed by the VSEAM
orchestrator.

Where it lives: the profile editor's step list and the add-profile
path chain draw each verb's name and fields (`widgets::path_step_fields`,
`crates/viewer/src/pane/profile.rs`, `pane/create.rs`).

What a fix needs:
- **A one-sentence meaning per verb.** One home for it, shared by the
  editor and anything else that names a verb, such as the Python
  docstrings if they say the same thing. It should not be a literal at
  each call site.
- **A second line in ONE shared format** listing the incoming states
  the verb accepts. It should be DERIVED from the verb lattice's own
  transition data (the states `profile`'s lattice checks:
  Point / Directed / directed point / pending director, as
  `docs/PATHS-DESIGN.md` §3's table spells them), not written per verb,
  so it cannot drift from what the lattice actually refuses.
- A row that holds every verb in the vocabulary to having both lines,
  so a new verb without them goes red.

Neighbour: `work/profile/tangent-arc-to-needs-an-explicit-tangent-where-line-takes-the-directed-point`
is one place where the incoming states are currently surprising, and
this hover line would have made it visible.

P1: a GUI defect Ev reported. VNEWS's ground (the words a reader is
shown), with the lattice data coming from `crates/profile`.
