# SHELL — shell, offset and transform (plan)

Live state is `work/shell/log.md`'s tail and the item files beside
this plan, never this file. Branch prefix **`shell/`** (unit branches
`shell/<unit>-<slug>`); away-channel tag `(SHELL orchestrator)`.
Sibling tracks sharing this ground by design: CLEAR (the clearance
engine), OFFSET (the offset lane's carriers), SHELF (the follow-on).

## Charter

Finish the shell and offset verbs to the semantics Ev ruled, on the
everyday shapes a user authors: every operand the kernel can build
shells or refuses typed, and every refusal says what to do next.

## Territory

`crates/topo/src/{shell,replace_face,transform,offset_together,offset_axial}.rs`,
`crates/geom-brep/src/{offset,offset_meters}.rs`,
`crates/sweep/tests/verbs_shell*.rs`, `crates/editor-core/src/clearance.rs`
— shared with the sibling tracks above. Not this program's:
`offset_fit.rs`, `crates/verbs` (its shell `VerbRecord` arm must agree
with `ShellNaming`), `editor-core`'s recipe doors.

## Unit order

The landed units are in `log.md`: SHELL-1, 2 and 5–10 earlier; the
2026-10-06 cut's seven units (PRs 4111, 4112, 4115, 4117, 4163, 4151,
4191) on 2026-10-06/07. SHELL-3 and SHELL-4 are CLEAR's. The slate
after that cut, as units:

8. **Tilted planar walls** — `shell-clearance-gate-skips-planar-pairs-tilted-off-antiparallel`
   (P1, M): the planar gate reads only antiparallel pairs, so two
   walls meeting at an angle across less than `2t` shell silently.
   A silent wrong body; first in line.
9. **The lofted oblique corner** — `shell-of-a-lofted-body-meets-the-oblique-corner-on-a-slanted-spline-seam`
   (P1, H): decided 2026-10-08 (the item's `## Decided`): the
   per-chart door derives edges by section and corners by crossing,
   carrying ISO's `nurbs-iso-derive-line-rim-arm-refuses-an-interior-row`
   first. Lofts then stop at the first wall:
   `a-fitted-wall-has-no-section-with-a-moved-cap` is decided
   2026-10-08 (a fitted face's section is its fit's; P2, after unit 9),
   and behind it the wall–wall seams
   (`a-wall-seam-between-two-fits-has-no-section`, P2;
   `two-fits-sharing-a-smooth-seam-disagree-by-their-certificates`, P3).
   The one-door merge `offset-doors-are-one-door-with-a-held-distance`
   (P3, H) follows.
10. **The face door is at rest** — `replace-face-offset-answers-for-the-complement-of-an-inside-out-body`
    (P2, M) carrying `shell-operand-shape-arms-behind-the-at-rest-gate`
    (P3, E): decided 2026-10-08 (the item's `## Decided`): the
    doors stay construction steps, the premise is corrected and
    pinned; the arms the `shell` gate made unreachable go.
11. **The sealed arm's two refusals** — `shell-of-a-cone-tip-refuses-at-the-nappe-decision`
    and `shell-of-a-tangent-dome-refuses-at-the-axial-corner` (P2,
    M each). Unit 7's cone tip, tangent dome and the lift's cone arm
    all wait here.
12. **A band between two boundaries** — `shell-open-band-wrapping-between-two-boundaries`
    (P2, M), unit 7's residue.

The P3 re-anchor rows (`reanchor-does-not-extend-…`,
`reanchor-reads-a-closed-spline-carrier-…`,
`replace-faces-offset-drops-rows-…`,
`nurbs-lane-absence-has-three-spellings-…`) and the per-chart door's
`per-chart-door-transports-a-torus-rim-…` follow; the two unpriced
rows are priced before they are dispatched.

then, as units:

13. **A fitted wall's section is its fit's** — `a-fitted-wall-has-no-section-with-a-moved-cap`
    (P2, H): decided 2026-10-08 (the item's `## Decided`). C5 routes
    `(Plane, Approx)` over `approx.fit()`, the edge stores the `Approx`
    key, and nothing is composed into its bound. Lofts then stop at the
    wall–wall seams. Dual review.
14. **The per-chart door's inverted body** — `the-per-chart-door-adopts-an-inverted-body`
    (P2, M): unit 10's `## Decided` already rules that the doors are
    construction steps, finished only through `AtRestBody::validate`.
    What is owed is the check that every caller adopting the door's
    result passes that gate, and pins showing the frustum and tube
    moves refuse there. Single review.

After 13, re-measure and price `a-wall-seam-between-two-fits-has-no-section`
(P2, H) before dispatching it.

then, as units:

15. **A moved fit's corners** — `a-moved-fitted-faces-corners-have-no-root-on-a-derived-spline-section`
    (P2, H): seed the plane's root on a derived spline section, or root
    the fit along a held edge, so a fitted face bounded by planes moves.
    Lands after 13 (both in `replace_face.rs`). Dual review.
16. **The tilted read's three gaps** — `tilted-read-accepts-a-zero-touch-on-any-vertex-sharing-pair`
    (E), `tilted-read-skips-edge-adjacent-pairs-that-cross-away-from-their-edge`
    (M) and `tilted-read-takes-a-spline-or-spiric-edge-as-its-carrier-ball`
    (M), all P3 and all in `moved_walls_cross` (`shell.rs`). The skipped
    edge-adjacent pair is a possible silent crossing, so the unit is
    M-tier: rule-1 draw byte 178 (mod 3 = 1), sequential.

The wall seam re-measured (its `## Measured`, 2026-10-09): the loft's
seams refuse at the iso-row guard and at `Approx × Nurbs`, behind a fit
budget the default ε misses. It needs a designer pair before it is
priced, after 15.

Units 8 and 10 run in parallel (different files); unit 9's measure
runs beside them.

## Adjacent, not taken

- `work/seat/shell-doors-take-tolerance-beside-tol` — SEAT's.
- WALKS' `producer-closing-mint-is-a-convention-with-thirteen-copies`
  — the cross-producer half of the laundering class.

## Exit shape

The verbs' README states the ruled semantics and every item above has
landed; the walk convention applies.
