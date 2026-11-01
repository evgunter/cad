---
id: guided-lift-refuses-a-nominal-degeneracy-it-never-reads
kind: issue
title: Under ProfileLift::Guided the profile is placed by frame_plane_lane and never reads the nominal, yet a nominal direction degeneracy still refuses it; a DERIVED frame in the same position builds
status: closed
opened: 2026-09-12
refs: [2435]
priority: P0
cost: H
closed: 2026-10-23
---


## Finding

Measured by PR 2435's delta review (NOTE 3), by probing rather than
reading: row 7's document run with `profile_lift: ProfileLift::Guided`
gives `Failed(DegenerateDirection { role: "datum frame x axis" })`.

Under Guided the value is placed by `frame_plane_lane`
(`crates/editor-core/src/eval/wire.rs:1495`), and the pre-pass plane is
the one the code itself says *"no decision reads"*
(`crates/editor-core/src/eval/anchor.rs:122-127`). A **derived** frame
in that exact position uses `SketchPlane::xy()` and builds.

So the refusal's reach is **lift-independent while the need for the
nominal is not** — which is narrower than the rule PR 2435 states for
itself, that a nominal refusal is *"raised at the reader that needed
it"*. Under Guided, the reader did not need it.

## Why this is a row and not a bug report

**Refusing may well be the right call.** An authored frame whose nominal
axes are degenerate is arguably broken however the profile on it is
lifted, and answering "fine under Guided, refused under Pinned" is its
own kind of surprise. The finding is that **nothing tests either way**
and nothing states which is intended — the asymmetry is a consequence of
where the refusal is raised, not a decision anyone made.

Confidence from the review: `likely` that the asymmetry is real (it is
measured), `unsure` that it should change.

## What a taker owes

A decision, stated at the site, plus a row that pins it. Either:

- **refusing is right** — then say why at `profile_plane_fdisambiguation`'s
  `Unreadable` arm, and pin the Guided case so the reach is deliberate;
  or
- **refusing under Guided is wrong** — then the `Unreadable` arm is
  consulted only on the lift that reads the nominal, and a Guided
  profile on a nominally-degenerate authored frame builds like the
  derived one does.

Read PR 2435's report first: it establishes that a nominal refusal has
somewhere to live (`FramePlacement::Unreadable`) precisely so it can be
raised at a reader rather than at the frame, and this row is about which
readers those are.

## Beside it

`work/wire/the-is-this-a-frame-door-was-deleted-and-its-classification-dispersed.md`
carries the delta's finding that the three `WrongOperand { expected:
"datum frame" }` sites have now **stopped being one door** — two test
the payload and one tests `placement`. A taker of either row is touching
the same three sites.

## Parked on the D10 hold (2026-10-17, WIRE triage)

Read against the tree at `fd218851f9`. `profile_plane_f64` still runs under every lift and raises `FrameDirection` on `FramePlacement::Unreadable`. Stage 3 D makes a profile 2-D and retires `Node::Datum`, so the authored frame this refusal reads goes; probably moot. Parked on `transform-retires-into-a-placement`, the INTENT row whose landing reaches this ground, and moved to WIREHOLD by WIRE's priority-seam cut, keeping its id.

Signed (WIRE orchestrator).

## Closed (2026-10-23)

Moot in D, as the 10-17 triage expected. `ProfileLift`,
`profile_plane_f64`, `FramePlacement`, `frame_plane_lane` and the
`"datum frame x axis"` role no longer exist anywhere under `crates/`, and
`Node::Datum` retired in `5bae0fff20`. A profile is 2-D now:
`wire_profile` builds every value on `profile::SketchPlane::xy()`
(`crates/editor-core/src/eval/wire.rs:1586-1592`). No lift reads an
authored frame's nominal, and no authored frame is left for a nominal
refusal to read, so the Guided/Pinned asymmetry cannot occur.
