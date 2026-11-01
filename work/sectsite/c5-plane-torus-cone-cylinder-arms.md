---
id: c5-plane-torus-cone-cylinder-arms
kind: issue
title: C5 table - plane x torus and cone x cylinder section arms (blocking the Klein wall-pair debt)
status: open
opened: 2026-08-27
refs: [VERBS-C5ARMS, 1048]
priority: P1
cost: H
design: true
---

## From GitHub issue 1057

Opened 2026-08-27; 0 comments.

**Two analytic section arms the C5 table is missing, and the demo debt they block.** Raised by OFF-D PR-2 (#1048) — see that PR's Klein deviation row and banked finding 5, and the pinned row `verbs_shell::the_klein_wall_pair_waits_on_a_plane_torus_route`.

**`plane × torus`.** `geom_brep::intersect::route(Plane, Torus).implemented` is false, so `topo::replace_face_offset` refuses `NeighborPairUnroutable { Plane, Torus }` and `topo::shell` carries that up. The configuration actually needed is the easy one: **a plane CONTAINING the torus axis cuts it in two circles** — closed form, no marching, the same shape as the existing `plane_cylinder_section` / `plane_cone_section` doors. (The general tilted case is a quartic and is not what any current consumer needs.)

**`cone × cylinder`.** Same table, same shape of gap. Pinned by `verbs_offd::an_undescribable_neighbor_pair_refuses_typed`.

**What they block.** The Klein bottle's walls are all revolved, so every one of its hand-built `r ± t/2` wall pairs — the demo's own findings list calls this "paid once per wall" — waits on these two arms before a `shell` call can replace it. A partial revolve of a disc gives a torus wall and two planar meridian caps, so every rim is `plane × torus`; the flare adds `cone × cylinder`. Until both land, the debt cannot start retiring and the demo keeps spelling the wall thickness into two call sites per elbow.

The contract for the eventual re-authoring is recorded in #1048: **naturalness, not byte-identity**, per the demo rule. The pinned row states the comparison it will make when the arms land (topology exactly equal; stored radii within one ulp, since the two spellings reach the inner radius by different float routes; volume within 1e-12).

## Home

VERBS' charter names the C5 section arms; the unit VERBS-C5ARMS already carries this issue's two arms (PR-1 merged, PR-2 remaining).

**VERBS closed** (exit walk ratified, PR #1793); re-homed to
`work/issues/` awaiting an owner.

**Adopted by CURVED** at its opening for dispatch (2026-09-04, Ev's
in-chat direction): the plan's lane that carries this item is in
`work/curved/plan.md`.

**Both section arms delivered** (PR-1 #1577 plane×torus; PR-2 #1864
coaxial cone×cylinder, 2026-09-05). What this issue still blocks on is
not a section arm: the Klein wall-pair debt (rows 3/4/8) waits on the
spiric rim carrier — `docs/CURVED-SPIRIC-DESIGN.md`, ratified
2026-09-04, a CURVED unit to cut.

**2026-09-13 (CURVED-SPIRIC PR-1a).** The elbow's rim now mints as
`Curve3::Spiric` (the kernel half of rows 3/4/8's carrier debt), and
the measurement moved the elbow's wall to a door the spec did not
predict: `shell`/`shell_open` on the klein elbow refuse at the
EQUATOR SEAMS' re-author (`offset_axial_reauthor_plane` — a disc's
profile vertices revolve into `RevolvedPoint`-declared chart seams
whose corner the moved cap displaces off the sketch plane;
`torax_axial`, `verbs_shell`, `torax_interval`, `shell7_seam_corner`).
The sectioned torus VESSEL, whose band has no such seam, reaches tier
3 and stops at the props door (`spiric_rim`, the tour's `torusvessel`
wall 1). Two doors now stand between the elbow and its re-authoring:
the seam re-author (an orchestrator question raised by PR-1a) and the
props quadrature lane for a spiric-bounded face (the spiric unit's
PR-2, after PR-1b's pcurve variant and STEP spline).

**2026-09-30 (`curved/equator-seam`).** The equator seams re-author
onto the corners the moved caps turned about the axis; the elbow now
has its spiric rims' windows read forward. The sealed elbow reaches
check 7's props door, `VolumeUncomputable` at a spiric-bounded cap
(`Unimplemented`). The opened elbow stops at the rim stage's lift
(`work/shell/shell-open-lift-takes-the-per-chart-door-on-the-klein-elbow.md`).

## A second consumer: lily wall 1 (GERM, 2026-09-28)

With the circle × torus root lane landed, the lily's stem glue
(`crates/sweep/tests/germ_circle_torus.rs`
`the_lily_stem_glue_is_past_the_circle_torus_pairs`, and the tour's
wall 1) is past the crossing layer and refuses
`GermFrameUnsupported { a_kind: Plane, b_kind: Torus }` at
`boolean/join.rs` `pair_section_frame`: the germ pair is the stem's weld
cap against the arch's tube wall. The weld plane contains the arch
torus's axis, so this is the easy meridian-plane configuration above,
at the join's frame dispatch rather than at `route`.

## Moved to GERM at CURVED's close (2026-10-01)

Both section arms and the spiric carrier are delivered; the elbow's
seams re-author on the moved corners and its rim window reads forward
(PR 3626), so the sealed klein elbow hollows to check 7's props door.
What this row still carries: (1) the lily's stem glue at
`pair_section_frame`'s plane×torus germ frame, GERM's live consumer
above; (2) the Klein demo's wall-pair re-authoring (rows 3/4/8), which
waits on the props lane for a spiric-bounded face
(`work/props/spiric-bounded-face-area-is-unimplemented.md`) and on
SHELL's opened-arm lift (`work/shell/shell-open-lift-takes-the-per-chart-door-on-the-klein-elbow.md`).

**2026-10-10 (FLUX, `spiric-bounded-face-area-is-unimplemented`).** The
props lane for a spiric-bounded face landed (`geom_brep::props::spiric`):
the sealed klein elbow, the elbow opened at one cap, the hand-built
wall pair and the sectioned vessel all pass check 7 and measure against
their cut-revolve oracles. Of (2) above, neither blocker stands: the
props lane is in and SHELL's opened-arm lift closed, so rows 3/4/8's
re-authoring is the demo's own work now.

## An obligation for the join-frame arm: the corner bar's lens (REACH, 2026-10-01)

Since the pierce lane's sector-side charge is read at its peak (PR
#3627), every torus bar in `germ_torus_doors` and the corner bar in
`germ_interior_oval::the_corner_bar_never_comes_back_a_body` stops at
this same door, `GermFrameUnsupported { Torus × Plane }`. For the corner
bar that door is the ONLY thing between the op and a body: the section
certificate clears its lens (one component per end square with the
corners' events on it, W4), so nothing upstream guards the lens, and
tracing it is the crossing layer's business. The day the arm lands
here, that row goes red unless the lens is traced or given a real
guard; the arm's lane owns that row, not a relaxation of it.

## A tube on a torus latitude reaches this door (TANG, 2026-10-02)

Measured on branch `tang/abutting-rim`: a z-axis tube of radius
`2 + 1/√2` standing at `z = 1/√2` on a full torus `R = 2`, `r = 1` about
`z` (its rim on the torus's 45° latitude), unioned undeclared. Since that
branch the rim is an ON event at the crossing layer, and the union stops
here, `GermFrameUnsupported { Plane × Torus }`, in both member orders.

## U-T1, the plane × torus germ frame (GERM, 2026-10-11)

`docs/GERM-TORUS-PLANE-FRAME-SPEC.md`'s first unit, on
`germ/plane-torus-germ-frame`. `pair_section_frame_at` serves the
meridian circles, the spiric ovals and the concentric circles, each
germ on its own loop, and refuses every other transverse plane as
`GermSectionSpiric { loops }`. Every served pose of the spec's §0 now
stops at the germ-pair dispatch (F2, U-T2's door); TANG's tube stops at
the wall side's section (F3b); the donut's corner bar refuses at its end
square's null oval. The coaxial cylinder × sphere loop question (Q6)
measured not live.

## U-T2, the plane × torus join dispatch (GERM, 2026-10-11)

On `germ/plane-torus-join-dispatch` (PR 4743). `kind_arm` takes the torus
as a wall beside the sphere, cylinder and cone, so every served pose
and the lily (undeclared, both orders) pass F2 and stop at the chord
layer's section, now the typed `SplitJoinError::SectionUnarmed`: F3a
(the plane face divided) with the plane first, F3b (the torus face
divided) with the torus first. TANG's tube (cylinder × torus) takes the
edge-plane lane, not the germ dispatch, and stops at F3b in all six ops
as before; this unit does not move it. The torus face's ring closure
was measured unreachable through the public ops before F3 (266 runs)
and, read in-crate, a refusal (`RingIslandUnread { Torus }` on the
cylinder-only chart); it is now `ring_path`'s path parity, as the
sphere's and the cone's, reading meridian and parallel loop arcs. It
refuses a spiric one until U-T4, and the tube's top and bottom circles
(the parallels of radius `R`, whose plane touches the torus) until
U-T3.

## U-T3, the plane × torus chord for the circles (GERM, 2026-10-12)

On `germ/plane-torus-chord-circles` (PR 4847). The chord layer's torus arm takes an
arc of the meridian or concentric circle the chord's two sites lie on,
picked by the join's own loop split; every pose whose plane holds the
axis or stands square to it builds its true body in all six ops, on the
donut, the one-face torus and the seamed torus, and so does TANG's tube.
Spiric sections keep the typed refusal (U-T4's). The ring lane reads the
tube's top and bottom circles across the cylinder `ρ = R`. The lily now
stops past the join, at the interior-loop guard on its two tube walls
(`work/sect/the-lilys-stem-and-arch-walls-have-no-certificate-arm.md`).

## The corner bar's lens (Ev, PR 4895, 2026-10-12)

The torus spec's Q1 is answered: the fat corner bar's lens is traced
through the chord's existing arc rule (`arc_leaving`, `oriented_arc`),
with U-T4's row 3 as its pin and no guard in any layer. On main the
matcher already pairs B5's corners on one oval, and the same class
builds true bodies on the sphere, cylinder and cone; a rule refusing
vertex-only components would refuse up to 634 six-op runs that build
today. U-T4 is unblocked.

## U-T4, the plane × torus chord for the spiric ovals (GERM, 2026-10-12)

On `germ/plane-torus-chord-spiric` (PR 4927). The chord layer's torus arm takes an
arc of the spiric oval the chord's sites lie on, on the oval's own
parameter, through the arc rule every curved chord uses; the torus
face's ring lane reads oval arcs as it reads meridians, and the plane
face's winding sum takes a spiric term (a bounded quadrature). S1, M3
and P4 build their true bodies in all six ops on the donut, and on the
one-face and seamed tori where their rings allow. B1, B2, the box
between two ovals' planes and the fat corner bar's lens (Q1) build or
stop past the join at two doors filed on their owners
(`work/inside/point-in-solid-refuses-an-l-shaped-torus-face.md`,
`a-torus-face-with-a-spiric-ring-has-no-volume-lane`, closed by PR 5567); no op
returns a wrong body. An oval pinched near the inner equator refuses its
island's winding
(`work/germ/a-planar-island-on-a-pinched-oval-refuses-its-winding.md`).
What this row still carries is U-H1, held by D10.


## Moved to SECT (2026-10-25)

From GERM. Moved by `git mv` in the chat-approved refold (2026-10-25) that put the torus pairs with the section-first join they are built on. These are C5 table arms, and SECT's ratified design puts kind dispatch in C5 alone (`work/sect/plan.md`, 'The design'), so the section-first join reads them; the torus pair rows refuse at C5 until they exist.

## A third consumer: lily wall 8 (SPLICE, 2026-10-31)

Once a pierce of a spline face mints its ring
(`work/splice/a-pierce-of-a-spline-face-cannot-mint-its-ring-vertex`),
the tour's lily wall 8 (the long leaf unioned with the lower stem)
refuses at `boolean::join` `frame_refusal`, the same at 1e-9 and 1e-6:

`GermSectionSpiric { a_face: 3v1, a_kind: Torus, b_face: 2v1, b_kind: Plane, loops: AroundTube, section: RoutesToGeneralRung { pair: "plane×torus", .. } }`

The germ pair is the stem's torus wall (operand A) against the leaf's
planar lower cap (B), which the tube crosses in two chords. The cap's
plane is tilted against the torus's axis, so it cuts the general
spiric quartic this row's general rung holds. The wall pins this
payload in `demos/tour/src/lily.rs`.
