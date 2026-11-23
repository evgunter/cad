---
id: smooth
kind: program
title: SMOOTH — carrying on from a leg end is written .tangent(): the End arc mode, the inherited direction, and tangent_arc_to and continue_to retired
status: ready
opened: 2026-11-22
group: kernel/sketch
prefix: smooth/
tag: (SMOOTH orchestrator)
paths: [crates/profile/src/path.rs, crates/profile/src/path/*, crates/profile/src/lift.rs, crates/profile/src/fillet_recourse.rs, docs/PATHS-DESIGN.md]
keep_out: [opened 2026-11-22 by PROFILE's cut of Ev's PR 7251 ruling per work/README.md Track size - PROFILE measured 35 against 30 before this build and the build is one story so it is its own track, shares crates/profile and docs/PATHS-DESIGN.md with PROFILE and SELFREF by design - run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled, docs/PATHS-DESIGN.md is a RATIFIED design page - the ruled text came in with PR 7251 and a re-word forced by a unit here lands with it, the Python door (crates/pncad-py and crates/pncad) is PYDOOR's and BIND's - every unit here that changes a Python spelling announces on their logs before landing, the persist schema (crates/editor-core/src/program.rs ProgramStep and ProgramArcData) is contended ground - announce on the away channel before landing - DOCTAIL and SELFREF own program.rs, the trace targets (crates/editor-core/src/trace.rs) are TRACE's and SELFREF's - a line_to whose trace target supplies extent only stays legal after an authored director, the viewer (crates/viewer) is CHROME's DOORS' and VGEOM's - the editor reads the transition table so it follows the vocabulary but its step menus and its provisional closer change and they are told]
---

**Carrying on smoothly from a leg end is always written `.tangent()`.**
Ev ruled on PR 7251 (2026-11-21: "i think this is good. End as drafted
is fine"). The ruled text is `docs/PATHS-DESIGN.md` as PR 7251 leaves
it: §2's lattice (`Directed` in two flavours, authored and inherited),
§2 Binders (`.tangent()`), §2 Legs (heading-taking and self-aiming
legs; the `End{p}` arc mode), §2 Fillet (`arc_fillet(End { p }, r)`,
`Radius` at every Directed tip), §2 Closure (the straight-run seam is
`.tangent().line_to(Start)`), §3's table rows, and §4 items 1, 2 and 4.

What the build does, in short:

- `Directed` gains its two flavours by type, and `line_to` gains a
  CHECKED row after an inherited direction (`continue_to`'s kernel).
- `arc_to` gains the `End{p}` mode, the arc to `p` leaving along the
  bound angle (`tangent_arc_to`'s kernel), and the fused verbs take
  `End` and `Radius` at every Directed tip.
- `tangent_arc_to`, `continue_to`, and the bare leg-end rows of `line`,
  `fillet`, `fillet_arc` and the fused `Radius` incoming retire, with
  the corpus, the persisted schema, the Python door, the lift and the
  recourse texts.

Opened by PROFILE's cut of the ruling (`work/README.md`, Track size):
PROFILE weighed 35 against its 30 before this build, and the build is one
story with its own order, so it is its own track rather than PROFILE's
tail. The parent row and the fused-arc row it closes moved here by
`git mv` with their ids kept.

Nobody holds it. It opens `ready`: its first two units are dispatchable
and independent.

Slate and order: `work/smooth/plan.md`; narrative in `work/smooth/log.md`.

## Ratified ground (cited, not re-litigated)

- `docs/PATHS-DESIGN.md` as PR 7251 leaves it, and everything PROFILE's
  list cites (`work/profile/program.md`): the sixth-round ruling (every
  zero-turn joint is a declared tangent joint; carriers are never
  compared), §2c's directed point, the seam token's matrix (§6's
  revised PQ4), BOOL-11's banded no-lever target check.
- The persist module's rule (`crates/editor-core/src/persist/mod.rs`):
  no schema version and no migration; a format change re-blesses the
  checked-in corpus.
