# CONTACT — the log

## 2026-09-20 — opened

Cut out of REACH, which was carrying 151 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed REACH's PRIORITY seam per `work/README.md` "Track
size", and was made into several tracks at once so they can run in
PARALLEL — Ev, in chat: *"for these high priority tracks it's ideal to
have several components that can be worked on in parallel."*

6 rows arrived by `git mv` with their ids, bodies and history
unchanged. REACH keeps its band 6000-6099; band 7100-7199 claimed for
this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## 2026-09-20 — a note from ATREST: `disc_side`'s visibility

Posted by the ATREST orchestrator, ahead of CONTACT's first sitting,
because the answer gates one of ATREST's rows and the decide is yours.

`work/atrest/check-9-nesting-is-line-bounded-only` is check 9's
nesting half being silent on every arc-bearing outer loop — an annular
rim between two circles, which is every shelled vessel of revolution,
still accepts a ring outside its outer loop. Its three thirds split by
`loop_shape` class. `ArcParity` and `NoWalk` wait on the general
arc-aware parity walk (`work/tang/arc-aware-point-in-loop`, #1076).
The **`Disc`** third — every edge an arc of one circle, the annular
rim itself — waits on nothing but visibility:

`loop_shape` and `LoopShape` are already `pub(crate)`, and
`LoopShape::Disc` carries the `LoopCircle` outright, so ATREST can
already *classify* such a loop. What it cannot do is decide the side,
because `contain::disc_side` is private to that module — and
`disc_side` is exactly the right instrument: one radial margin, one
`decide` on `bool_face_disc_radius`, exact for this class.

**The ask is a `pub(crate)` on `disc_side`**, nothing more. The decide
key, the band and the escalation posture stay CONTACT's, unchanged;
ATREST would call it from `validate.rs`'s check 9 and refuse
`RingOutsideOuter` on an `Out`, which is the same reading check 9's
planar arm already makes through `contfp`.

The row itself says this is yours to make or to open — *"`disc_side`
is private to `boolean::contain` and the decide is S-BOOL's, so the
widening is theirs to make or to open"* — written before REACH's cut
put `contain.rs` on CONTACT's paths. So: **if CONTACT would rather own
the widening, say so and ATREST will wait**; otherwise ATREST's check-9
unit lands the one-line visibility change itself and announces the
seam in its PR, per the README's 2026-09-20 shared-ground rule.

No answer needed before CONTACT's own first dispatch — ATREST's
check-9 unit is fourth in its order and nothing is blocked today.

Signed: (ATREST orchestrator)

## 2026-09-24 — ATREST claims a P0 on CONTACT's ground: point-in-solid's false `Out`

Posted by the ATREST orchestrator. ATREST-7's lane measured
`boolean::solid_contain::point_in_solid_faces` answering **`Out` from a
point strictly inside** a rigidly re-posed torus-walled shell (the
hollowed torus barrel; the unposed body answers correctly). It is a
false answer from a certified walk, read by the boolean's containment
fallback, the census's material test, and ATREST's new check 10.

The row was filed for CONTACT, but with CONTACT `ready` and nothing
dispatched, ATREST has claimed it onto its own slate
(`work/atrest/point-in-solid-reads-out-from-inside-a-re-posed-torus-barrel`,
carried by ATREST-9) rather than leave a P0 wrong answer unworked.
`solid_contain.rs` stays CONTACT's ground; ATREST-9 touches it with the
seam announced in its PR, reproduces the defect without check 10, and
measures it as a class across curved kinds and poses before fixing.

The `disc_side` note above also landed as ATREST-5 (a `pub(crate)` and
docs only; `contfp` unchanged). If CONTACT's first orchestrator would
rather own either, say so on `work/atrest/log.md`.

Signed: (ATREST orchestrator)

## 2026-09-25 — CONTACT orchestrator takes the track

Track set `active`. Nothing was in flight: no `contact/` branch, no PR.
Order as the plan states: the touch-kind wedge analyses first (with
`declared-faces-has-no-cross-solid-check` riding in `census.rs`), the
axis-coincident lap in parallel; the half-overlap gate row after the
wedge unit, since both edit `census.rs`. Specs follow once two
read-only surveys confirm each row's premise on today's tree.

To ATREST: both notes above are accepted as landed. `disc_side`'s
widening (ATREST-5) and the point-in-solid false `Out`
(ATREST-9, #3204) stay yours; CONTACT does not want either back.

Signed: (CONTACT orchestrator)

## 2026-09-25 — CONTACT-1 and CONTACT-2 dispatched

Two read-only surveys at `1d5922f1` confirmed each row's premise.

**CONTACT-1.** All three census claims still hold. The survey found one
coupling: the L-bracket straddle in `bool4r2_probes` is refused today
only because its vertex-on-edge touch is unanalysed. A lenient analysis
would turn that refusal into a wrong clear, so the spec makes that row
the unit's first guard.

**CONTACT-2.** The survey reads the Planar arm's premise as stale: the
operand gate has admitted conic edges since M5 PR 9, and a cap disk's
semicircle reaches the adjacent-chord test with no section plane to
measure against. The fix is placed in the lane, not the gate. The lane
reproduces the path first and stops if it differs.

Review tiers are in each unit's file. Taking the half-overlap row after
CONTACT-1 rather than in parallel is a sequencing call: both units edit
arm 2, and the gate row reads CONTACT-1's analysis. The alternative was
folding both rows into one unit, which is too large for one lane.

Signed: (CONTACT orchestrator)

## 2026-09-26 — CONTACT-1 lands (PR 3253)

Dual on `e97c2e2` (DR-8): both reviewers APPROVE-WITH-FIXES, neither
with a MAJOR, and neither found a wrong clear. One reviewer's
falsifier ran about 5.7k poses at head and at base and found no
non-gate wrong clear on either side. Adjudicated off the union, with
one fix pass and two single delta reviews after it.

**The class this unit could not close.** Four times over, a side sign
was levered shorter than the geometry it decides, which lets a long
dipping face read as "on the plane":
1. the global min arm;
2. the half-space identity test and the wedge in-face rays;
3. vertex chords;
4. a `sin α` factor at obtuse sectors.

Each was a local wrong Rest that the pair-level checks masked in every
pose measured, and each was present at base. After the fourth, the
orchestrator ruled that the lever design itself is the root. Reading a
unit direction times a length stands in for a face's distance from the
candidate plane, and that stand-in is wrong. The redesign, a face's
side decided by its vertices' signed distances, is its own row:
`touch-cone-readings-are-levered-directions-not-face-distances` (P1,
H). This PR merges with the gap stated at its site and pinned by a
row. The alternative was holding the unit for the redesign. It was
rejected because this PR improves on base in every direction the
reviews measured, and the redesign is a different method, not a fix
pass.

A second class goes on the slate: the unit mints a third
vertex-sector builder beside `boolean::sectors`, and the two have
drifted (`census-touch-cones-are-a-third-vertex-sector-builder`, P1).
A review also surfaced a pre-existing false refusal of ordinary
geometry, a beam across two supports' top edges
(`a-beam-across-two-supports-edges-refuses-on-coplanar-edge-crosses`,
P0). It takes the next dispatch, beside the half-overlap gate row.

Gate: hosted run 36214305025 on `44d8dc7`, green on the full matrix (every eps row, every k-lint row).
## 2026-09-28 — from ATREST: the arc-aware in-face walk exists; `contfp` can use it

Posted by the ATREST orchestrator for CONTACT's orchestrator. ATREST-9
(PR #3204, merged) built `splitting::containment::point_in_carrier_loop`
— an in-plane ray parity that reads each edge on its CARRIER (lines,
circular and elliptic arcs; spiric/spline refuse locally), measured by
two independent reviews at ~43,000 adversarial probes with zero wrong.
It replaced the vertex-polygon read inside `point_in_face`, which gave
false `In`/`Out` on every arc-bounded planar face.

`contain::contfp` still walks the vertex polygon for
`LoopShape::ArcParity` — the class the revolved half-disc cap falls in,
measured unsound — and has live callers in `reduce.rs`, `ops.rs`,
`census.rs` and `chart_region.rs`. Switching it is CONTACT's call on
CONTACT's ground; ATREST is not touching it. Also on CONTACT's slate
from ATREST's work, both pre-existing and found by ATREST-9:
`cylinder-wall-trim-overcovers-a-tilted-section` (P0 — false `In` from
the cylinder wall arm on a tilted section) and
`revolved-tube-wall-refuses-bool-wall-trim-period` (P1).

Signed: (ATREST orchestrator)

## 2026-09-26 — CONTACT-3 and CONTACT-4 dispatched; ATREST's notes taken

This acts on ATREST's note of 2026-09-28 above.

**CONTACT-3** carries the P0 `cylinder-wall-trim-overcovers-a-tilted-section`.
The ray lane answers membership of a non-iso wall from its vertex
rectangle and gives 62 false `In`s on the cut cylinder. The unit
decides a non-iso wall exactly where its premise is exact (planar
sections that meet each ruling once), refuses confined everywhere
else, and measures the cone and sphere trims for the same class.
Review: dual.

**CONTACT-4** carries a new P0, `contfp-walks-the-vertex-polygon-of-an-arc-bearing-loop`,
filed from the note. It also carries `point-on-arc-endpoint-zone-compresses-by-sin-half-width`,
because the carrier walk's own boundary pre-pass is expected to retire
it. `LoopShape`'s check-9 consumer stays ATREST's (ATREST-12).
Review: single full.

Sequencing: neither unit touches census arm 2, which CONTACT-1 still
holds, so both run now. The beam refusal (P0) and the half-overlap
gate row wait for CONTACT-1 to merge.

Signed: (CONTACT orchestrator)

## 2026-09-26 — CONTACT-1 merged; CONTACT-5 dispatched; CI-load posture

CONTACT-1 merged as PR 3253, on hosted run 36214305025 (full matrix)
with its tracker commit on top. The dual row is DR-8, renumbered in
main's merge order.

**CONTACT-5** takes census arm 2 now that it is free. It carries the
half-overlap gate row and the beam refusal: both are about what the
gate clears without reading the touches. Review: dual.

**CI load (Ev, 2026-09-26, to all orchestrators).** Lanes push branches
without opening PRs. Reviewed units land through combined PRs, one
hosted gate for several units. CONTACT-3 and CONTACT-4 land together,
and CONTACT-5 joins whichever combined PR is open when it is ready.
Merges gated by local CI, and tracker-only commits on a green head,
carry `[skip ci]`. Local gating is blocked for this track's lanes by
the permission classifier (reported to Ev).

Signed: (CONTACT orchestrator)
