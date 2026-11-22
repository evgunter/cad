---
id: a-faces-loop-or-closed-trace-cannot-become-a-profile-loop
kind: issue
title: A picked edge loop, or a face's whole closed trace, cannot become a profile loop: an elbow off a hollow cylinder's end is re-typed by hand
status: parked
opened: 2026-11-20
priority: P1
cost: M
blocked_on: [a-profile-on-a-face-holds-that-faces-outline-or-a-hole, a-click-on-an-edge-of-the-sketched-face-writes-its-loop]
---


Asked by Ev in chat (2026-11-20), and banded P1 at Ev's request: "is it
possible to use an edge selected in the viewer as a profile that can
then be revolved or something? like if i have a hollow cylinder, can i
make an elbow by clicking on the outer and inner loops, putting them
into a profile on the cylinder end face, and revolving it?"

## What exists

- **New profile on a face** opens a profile on the latched face's chart
  (`Recording::draw_on_face`; `crates/viewer/README.md`, "New profile on
  a face").
- **Traces** (`crates/editor-core/src/trace.rs`; DESIGN "Constructing a
  coincidence"). A profile drawn on a frame reads a face's carrier cut
  by the frame's plane. Three document forms read one, each lowering to
  a step of the profile's own vocabulary: a straight piece running to
  `ProgramTarget::OnTrace`, an arc about a centre running to `OnTrace`,
  and `ProgramStep::AlongTrace` (a heading). In the viewer, a second
  latched face makes the profile's one chain run onto that face's trace
  and along it.

## What is missing

Nothing takes a picked edge, a picked loop, or a face's whole closed
trace and makes it a loop of a profile. For the elbow, the outer and
inner walls of the hollow cylinder, cut by the end face's plane, are
exactly the two circles wanted, and the end face's own two loops are
those circles again. Today the person opens a profile on the end face
and types two circles (centre and radii, or the variables the cylinder
was built from), then revolves.

## The design question (`design`)

What a profile loop that comes from existing geometry is: what the
person picks (edges, a loop, a face, a face's trace), what the document
stores (a read that re-resolves when the body changes, as a trace does,
never a copy of numbers), how it lowers to the profile's vocabulary so
the replay sees an ordinary program, what it refuses (an open chain of
edges, a non-planar loop, an edge with no closed form in the plane), and
how it sits beside the three trace forms rather than as a second way to
say what they say (`[one-way]`).

## Not checked

- Whether the three trace forms can already spell a whole circle (an arc
  about a centre from the trace back to itself), which would make part
  of this sugar over what exists.
- Whether the hand-typed elbow builds: an annulus on the end face
  revolved about an axis in that plane. Read, not run.

## Weighed (2026-11-21)

Two designers, three rounds (fork log row 297). They crossed twice on
whether a picked loop is the face's outline as it becomes or the edges
picked, and met on this:

- **A profile drawn on a face may hold that face's loops**:
  `Boundary { Outer | Through(edge) }`, on the face the profile is drawn
  on and no other. It stores the face's name (the chart already holds
  it) and, for a hole, one edge name. Each evaluation walks the loop on
  the body as it is and lowers it to `line`, `arc_to(Center)` and
  `circle`, so the replay, validation and the scalar tiers see an
  ordinary program. A keyway cut later is in the loop; a filleted rim
  moves it; a lost face name or hole edge refuses typed.
- **The precedent is the face read, not the selector.** A shell's open
  face and a profile's own chart store a face name and read its
  boundary as it is. `names/select.rs`'s freeze is for a set an
  operation acts on cell by cell (a fillet gaining edges).
- **Not a sweep-level input.** A revolve's axis and an extrude's depth
  are written in a chart on the face, and a profile on the face is that
  chart.
- **Corrections to this row.** The three trace forms cannot spell a
  whole circle: the arc form's centre is written, `AlongTrace` is a
  heading, and a chain may not close on its own carrier. An "open chain
  of edges" is not a refusal under this answer, and a non-planar loop
  cannot be written. `Node::HollowTube` with an arc window is already a
  hollow elbow, from four typed numbers.

What they did not settle is the second read, for the cases a face's
loop does not cover (part of an outline; a section where no edge
exists): a piece or closed loop along a wall's **trace**, which ignores
edits to the cap, or a run along picked **edges**, frozen as a fillet's
selection is. Either is a further row.

First check for a spec: the lowered joints. Where two loop edges meet
tangentially the lowering must declare the joint from the body's
smooth-edge fact and not re-decide it; smoothness is read per edge
(`geom-brep/src/certify.rs`, `geom_brep::dihedral`, `topo::edge_wedge`),
and nothing yet maps a loop vertex to the edge between its two walls.
A filleted rim's wall is tangent to the cap, which the trace reading
refuses as `Grazing`; the boundary loop reads the edge's own curve.

## The hand-typed elbow, measured (2026-11-21)

Run on hosted CI (PR 7280, run 44889924284). A pipe of radii 1 and 0.6,
the same two circles drawn on its end cap, revolved a quarter turn
about a line in the cap's plane 3 from the axis:

- The revolve builds for one heading of the axis and refuses for the
  other as "across the revolve axis", though the axis is clear of the
  profile: `work/lathe/a-revolve-refuses-a-profile-on-one-side-of-its-axis-as-across-it.md`.
- Where the revolve builds, the union of the pipe and the elbow
  refuses: `work/heldrecords/a-pipe-does-not-union-with-an-elbow-revolved-off-its-own-cap.md`
  (P0).

So the elbow is not only an authoring gap: a picked loop makes the
elbow a body, and joining it to its pipe waits on the union.

## Moved (2026-11-22)

From `work/profile/` to `work/faceloop/` by `git mv`, id kept, with the
cut below: PROFILE stood at 35/30 and this build is not the lattice's
(`work/faceloop/program.md`).

## Cut (2026-11-22)

Ruled by Ev on PR 7284 (2026-11-21: "this sounds good!"). Parked on the
two units that make it true; the plan is `work/faceloop/plan.md`.

1. **U1** `a-faces-loop-reads-each-joint-off-the-body` (M, open): the
   joint at each vertex of a face's loop, read off the edge between its
   two walls, at every lane. Inert; it settles the spec's first check
   above.
2. **U2** `a-profile-on-a-face-holds-that-faces-outline-or-a-hole` (H,
   parked on U1): the form, its lowering at each lane, its typed
   refusals and the names of the walls it sweeps.
3. **U3** `a-click-on-an-edge-of-the-sketched-face-writes-its-loop` (M,
   on VSEAM, parked on U2): the click.

Off the slate: the Python spelling (PYDOOR,
`a-faces-loop-has-no-python-spelling`, P2), and the second read
(PROFILE, `part-of-an-outline-or-a-section-where-no-edge-exists-has-no-profile-form`,
`design`).

**Where the elbow stands after each.** After U1, nothing a person sees.
After U2, by the Rust API: the profile holding both loops evaluates and
follows the pipe's radii; a revolve builds for one heading of its axis
and refuses for the other (`work/lathe/a-revolve-refuses-a-profile-on-one-side-of-its-axis-as-across-it.md`);
the union with the pipe refuses
(`work/heldrecords/a-pipe-does-not-union-with-an-elbow-revolved-off-its-own-cap.md`,
P0). After U3, the same by two clicks and Revolve. The elbow joins its
pipe only when the HELDRECORDS row closes.
