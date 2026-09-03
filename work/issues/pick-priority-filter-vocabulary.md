---
id: pick-priority-filter-vocabulary
kind: issue
title: Pick-priority has no filter vocabulary - a tool states which entity kinds it takes, and only the mate tool's case is served
status: open
opened: 2026-08-31
github: 1379
refs: [1407]
---

## From GitHub issue 1379

Opened 2026-08-31; 1 comment.

**Corrected 2026-08-31 after review measured the original claim and found it understated. The mate-tool instance is now FIXED in the GAUTH-2 PR; what stays open is the vocabulary.**

GAUTH-2 gave edges pick priority: a cursor within `viewer::pick::EDGE_PICK_RADIUS_PX` (6 physical pixels) of a drawn edge of the body under the cursor selects the EDGE rather than the face behind it. The rule lives in `PickIndex::hovered_for`, the one place hovering and clicking both read, deliberately so they cannot disagree.

### What the original text of this issue got wrong

It described the consequence as an aiming annoyance — "mating a narrow face now needs a click aimed away from its own edges". Measured on the shipped fixtures, that is not what happens. Faces become **entirely unreachable**, not merely awkward:

- a shelf face of the assembly bench measured **13 pickable pixels out of 723** in its own screen footprint at default framing, and **0** one zoom step in;
- a hole's cylindrical wall lost **100%** of its footprint one zoom step out — and that is exactly the face a coaxial mate wants to pick.

The rule scales with the picture, the face does not: any face whose narrow dimension projects to under ~12 px is fully inside its own edges' catch band, and every face reaches that width at some zoom. So the defect was a shipped tool losing its input, not a user aiming more carefully.

### What was done

The minimal narrowing, adjudicated from two independent reviews that converged on the same shape: a kind filter threaded as a parameter through the one door that decides priority (`PickKinds` → `hovered_for` / `op_under`), with the mate tool asking faces-only while it is open. Default behaviour is unchanged — a bare cursor still gets edge-beats-face — and a tool cannot end up on a different rule, because it narrows the one rule rather than re-deciding it. Pinned by a headless row driving both filters at one cursor inside the radius.

### What remains open (this issue)

`PickKinds` is two answers to one question, not a vocabulary. `docs/GUI-DESIGN.md` GQ7 still owns:

- **which filters are offered where** — per-tool, per-pane, or a user-visible filter control, and whether a filter is tool state or session state;
- **how a tool states what it wants**, once there are more than two kinds. GAUTH-5's blend tool is the second data point and wants the opposite filter (edges only); a vertex-pick unit would be the third, at which point an enum of hand-written combinations stops paying;
- **what the picture owes the user about an active filter.** Today a faces-only pick is silent: the hover simply stops marking edges, and nothing says why.

Deferred to sketcher/tree design with GQ7's other clauses, per that section's own note.

## Comments

**2026-08-31** — comment:

**Second data point for the filter-vocabulary question** (GAUTH-5, PR #1407).

`PickKinds` gained a third variant, `EdgesOnly`, for the blend tool — the mate
tool's case mirrored rather than a new kind of question.

The measurement this time is the same shape as the mate tool's and points the
other way. The mate tool needed `FacesOnly` because whole faces sit inside
`EDGE_PICK_RADIUS_PX` of their own boundary, so edges winning globally made
those faces unpickable. The blend tool consumes edges and nothing else, so
unfiltered a cursor a few pixels off an edge answers the FACE behind it: the
selection changes, the tool takes nothing, and the click did nothing the user
can see. `EdgesOnly` makes that miss a miss — the edge test still runs, and a
cursor no edge wins answers `None` exactly as a cursor over the background does.

It is threaded through the same one door (`PickIndex::hovered_for`) and reached
the same way (`ToolKind::pick_kinds`, the exhaustive per-tool match), so no tool
gets a different priority rule by re-implementing it. `PickKinds` now has one
private predicate per axis (`edges()`, `faces()`), each an exhaustive match, and
`hovered_for` reads them in that order: edge if admitted and near, else face if
admitted, else nothing.

What this does and does not settle:

- It does NOT turn the enum into a filter vocabulary. It is still three answers
  to one question — one per shape of tool the GUI has actually shipped — and
  each is a whole priority rule rather than a membership set. A tool wanting
  "faces and vertices but not edges", or a per-tool cursor hint, still has
  nowhere to say so.
- It does suggest the eventual shape, if a third asymmetric case ever turns up:
  the two predicates above are already a per-kind admission set in disguise, and
  a `kinds: EnumSet<EntityKind>` with the priority order fixed at the door would
  subsume all three variants without changing a single call site's meaning. Not
  taken now — two tools do not justify the vocabulary, and GQ7 has not ruled on
  where filters are offered.

The row that pins the new arm is
`edge_pick::an_edges_only_pick_answers_nothing_where_an_unfiltered_one_answers_the_face`
(on the rim both rules answer the same edge; four radii inside it the bare
cursor takes the face and the filtered one takes nothing, hover included), and
`combine_ops::each_tool_narrows_the_cursor_to_what_it_can_use` is the
per-tool sweep, now an exhaustive match so a further tool must state its side
before the row compiles.


---
_Generated by [Claude Code](https://claude.ai/code)_

## Home

Viewer/GUI ground: GAUTH's closing entry names this issue as its residue and `docs/GUI-DESIGN.md` GQ7 owns the open question, but both GAUTH and GUI are closed programs, so it lands in `work/issues/`.
