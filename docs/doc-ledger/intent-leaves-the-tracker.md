# INTENT leaves the tracker — 2026-10-30

INTENT built DESIGN.md D10: one way to say dependency, placement and
intent. Ev ratified D10 on PR #3990 from the ruling
`one-way-to-say-dependency-and-intent`. The program opened on 2026-10-03
and was split along its dependency seam on 2026-10-11, with COINCIDE,
REFS and INLANE as successors. It closed on 2026-10-30, when D10's
umbrella row `d10-one-way-to-say-intent-is-unbuilt` closed (#6103). Its
directory is recoverable at `a5a7a4b342` (`git show a5a7a4b342:work/intent/plan.md`).

## The build

| stage | what | closed by |
|---|---|---|
| 1 | variables; no dimensioned literal in a slot | stage 1 PRs, `docs/INTENT-STAGE2-SPEC.md` §0 |
| 2 | operations read and define variables; selections as definitions; the product a list; one dependency | `docs/INTENT-STAGE2-SPEC.md` |
| 3 | spaces, placements as mates into a target, the world a node, the per-operation computing frame, patterns as index variables | B #5232, C #5267, D #5506, F #5299, G #5954 |
| 4 | the coincidence door, its rungs and the `unproven-coincidence` lint; declared pairs and the undeclared refusals retired | `docs/INTENT-STAGE4-SPEC.md`; E and F (#4641) |
| 5 | assertions relating by `=`; the at-rest census as a check; quieting by a holding `Gap = 0`; interference as a finding | `docs/INTENT-STAGE5-SPEC.md` (built under ATREST) |
| 6 | coaxiality as one `Axis` read twice; a sketch continuing tangent to a face's trace, proven at the door and authored by the viewer | A #6023, B #6041, C #6043, D #6060, E #6071 |

## The exit walk and what came after it

The plan's exit criteria: no code path implements a clause D10
supersedes, and the companion pages state D10's design. The walk ran on
2026-10-29 (#5997, `d10-exit-walk-report` at the SHA above). It found 13
of 15 superseded mechanisms retired, one still implemented, a dead
refusal variant and three stale companion pages. Each was then closed:

- The glued pair re-read through the declared rung retired in
  CONTACTHOLD #6079.
- The dead `ContactRefusal::Undeclared` was removed in #6079.
- The stale companion pages were fixed in #6079 (the topo README's C4)
  and #6106 (ASSEMBLY A3, LIBRARY-DESIGN U6, and the companion table's
  ASSEMBLY row).

Two D10 sentences the walk did not read, because it checked superseded
mechanisms and not new requirements, were found unbuilt and built:

- "An unread body is a finding": Ev reworded it on `[ev]` #6075 as what
  the person made, inserted or named, and nothing uses. Built as the
  `unused` check in #6081.
- "An assertion at its site quiets it": Ev answered on `[ev]` #6086
  that assertions quiet every row they can say, flush included. Built in
  ATREST #6103.

With nothing of D10 left unbuilt, DESIGN's "State of the build" section, which tracked the refactor, is removed (Ev, PR 6109).

## Where the rows went

The non-D10 tail moved out on #6074 by `git mv`, ids kept: to TRACE (a
new program), MSOLVE, DOCTAIL, COINCIDE, HELDRECORDS, WIRE, GREEN, DOORS
and CHARTTAIL. The last two open rows closed before the sweep:
`g-moves-four-main-reds-messages` (#6107; its one defect is filed as
`sym/rule-h-loses-two-pad-decisions-to-a-coefficient-freeze`) and
`the-gallery-documents-carry-what-nothing-uses` (#6108).

What D10 leaves open is filed in live programs:
- `contacthold/a-fold-step-glues-a-pair-its-door-reads-in-band` (P1, design);
- `coincide/an-unproven-finding-names-the-variable-edit-that-would-construct-it` (Ev's GUI suggestion);
- `coincide/a-checks-refusal-discards-every-residents-findings`;
- `coincide/a-blend-joint-re-decides-its-operands-junction`;
- the two COINCIDE rung-one gaps (#6089).

Live rows across the tracker that cited an INTENT row in `refs`, or
named one as `parent`, now cite the PR that closed it.
