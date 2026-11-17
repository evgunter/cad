# Design fork `embed-sliver` — the problem, as handed to every designer

Read `docs/prompts/designer.md` in full before you start; it is binding.

**The row:** `work/embed/the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause.md` (P1, design).

**What needs deciding.** What the body-level injectivity certificate
(`crates/topo/src/injective.rs`, asked through
`topo::clearance::embedded`) does with the part of a face that lies
within the band (K·ε, in metres) of a line its boundary is placed on,
and what a finished body's embedding promise says about that part.

**Why it is a question.** The module docs of `injective.rs` list six
"ε concessions": places where a presentation drops points of a face.
One of them, the sliver (`SELF_INTERSECTION_SLIVER`; its readers are
`present_chart`, `present_line_corner` and `present_plane_corner`),
reads a presentation rectangle within the band of a shared line as
holding nothing off that line. The face is then
`Presentation::Vacuous` in that cell, and the departure arm certifies
a pair outright when either face is vacuous. `docs/DESIGN.md`
`[topology.tiers]`, in its finished-body paragraph, says: "A reading
inside the band refuses as ambiguous, never as Holds or Violated."
Nobody has said how the two stand together, or whether they do.

**Why it matters now.**
- The at-rest gate (`work/embed/the-at-rest-gate-certifies-embedding.md`,
  P0) is being built: every finished body will be asked this
  certificate, and a certified pair is a promise the body carries.
- A sibling row shows what a wrong `Vacuous` costs: a face read as
  holding nothing certifies any pair it is in, unread
  (`work/embed/a-half-sphere-reads-vacuous-in-its-own-window-…`).
- Bodies are known to hold only through the sliver: the tour's `chute`
  (PR 6894 measured that its annulus cells, which touch a line from
  its far side within rounding, stay undecided at depth 40 without it),
  and `present_line_corner`'s pins.
- The row holds that no new reader adopts the sliver until this is
  decided, and two engine lanes are in flight on new corner readings.

**Where to look.**
- `crates/topo/src/injective.rs`: the module docs ("The ε
  concessions"), the three readers, `vacuous`, `sheets`, `departure`.
- `docs/DESIGN.md`: `[topology.tiers]` (the finished-body paragraph),
  `[tolerance.recourse]`, and whatever else governs what a verdict
  inside the band may say.
- Provenance: the clause came from Ev's embedding ruling
  (`31091b6747`, PR 6301).
- The rows under `work/embed/` and the history in `work/clear/log.md`
  (2026-11-12 to 2026-11-14).

**What is not known.** Which bodies hold only through the sliver has
been measured for `chute` alone. A measurement lane is running now
(the sliver refused rather than conceded, over the injectivity suites
and the tour walk); its numbers will be on the row. If your answer
turns on a number that does not exist, say which.

A cold build of `topo` here takes 16 to 20 minutes and the box holds
one at a time, so do not build: read, and name any measurement you
need in `## For the orchestrator`.
