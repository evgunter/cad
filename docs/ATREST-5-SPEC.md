# ATREST-5 — check 9's nesting arm reaches the Disc class

**Binds one implementer lane.** Deleted at merge; `work/atrest/ATREST-5.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `atrest/5-check9-disc`. Row carried:
`work/atrest/check-9-nesting-is-line-bounded-only` — read it in full.

## The defect

Check 9's nesting half refuses `RingOutsideOuter` when a face's ring
lies outside that face's own outer loop, but its gate
(`validate::nesting_normal`) opens only for a `Plane` face whose outer
loop is in `boolean::contain::loop_shape`'s `Polygon` class. The
annular rim between two circles — every shelled vessel of revolution —
is the `Disc` class on both loops, so an inverted host/guest pick on
such a rim still certifies.

## Settled design

**D-A. The arm opens for a `Disc`-class outer loop and decides with
`boolean::contain::disc_side`** — one radial margin, one `decide` on
`bool_face_disc_radius`, exact for that class. `loop_shape` and
`LoopShape` are already `pub(crate)`; `disc_side` is private. Make it
`pub(crate)`, nothing more: its decide key, band and escalation stay as
they are. `crates/topo/src/boolean/contain.rs` is CONTACT's ground; the
ATREST orchestrator posted the ask on `work/contact/log.md` on
2026-09-21 and CONTACT has not objected — announce the seam in the PR
body.

**D-B. What is tested.** The existing arm tests ring VERTICES against
the outer region. For a polygon outer that is its stated premise; for a
disc outer, a ring that is itself a circle (or carries arcs) can have
every vertex inside while an arc bows outside. State the premise the
existing arm relies on (no loop crossing, checked elsewhere or assumed
— cite which), and if a Disc-class ring needs more than its vertices,
use the exact two-circle test (centre distance plus ring radius against
the outer radius, through one `decide`). Justify the choice at the site.

**D-C. Posture unchanged elsewhere.** `ArcParity` and `NoWalk` stay
silent — the arm refuses a body on an `Out`, so it stays strict about
its walk's domain; an escalation stays an escalation, never a guess.

## The residue gets its own file, at the moment you disclose it

The `ArcParity` and `NoWalk` classes wait on
`work/tang/arc-aware-point-in-loop` (#1076). File
`work/atrest/check-9-nesting-arc-parity-and-no-walk-wait-on-the-arc-aware-walk.md`
as `status: parked`, `blocked_on: [arc-aware-point-in-loop]`, P0, cost
H, carrying the two thirds' text from the carried row; the carried row
then closes with this unit. Cite the new row from check 9's banner and
from the not-yet-checked list.

## What you owe

The arm; rows that go red without it (an annular rim whose ring sits
outside the outer disc refuses naming the face; an ordinary vessel of
revolution still certifies — this program must not add to the
false-refusal direction); the `shell.rs` `(host, guest)` comment and
`crates/sweep/tests/topo_ring_nesting.rs`, which name this row's
subject as what is left, updated with the change; the sweep per
discipline §5, hit list and blind spot; out-of-fence findings filed.
Set the carried row to `review` when you open the PR.

## Verification

Hosted CI on the merged head: **six** `test (eps = …, n/2)` jobs and
**five** `k-lint (gate, …)`, read at STEP level. Own `CARGO_TARGET_DIR`
outside the worktree; private scratch; never end a turn with background
work live. Do not touch `work/atrest/log.md`. Do not merge.

## Review tier

**Single, FULL**: a new refusal on the at-rest door, reading another
program's exact decide. Class M / NUMERIC.
