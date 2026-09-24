---
id: half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete
kind: issue
title: mev, mef and mekr mint half-edges into a cached curved face and no pcurve row with them, leaving the face half-minted
status: open
opened: 2026-09-13
refs: [validate-pcurves-never-recertifies-a-face-it-finds-incomplete]
priority: P0
cost: H
---



Found by this program's `split-edge-children-lack-pcurve-rows-on-curved-charts`
lane as the class sweep of its own defect, and measured.

Every operator that mints half-edges into an EXISTING loop has the
shape the split-edge finding named: two fresh half-edge keys join a
face that may already carry pcurve rows, no row is minted with them,
and the face is left half-minted — tier-3 invalid with one
`Pcurve MissingCache` per new half until a caller re-mints.
The minting sites are `Body::mint_halves`'s callers
(`crates/topo/src/euler.rs`: `mev_fan_execute`, `mev_lone_execute`,
`mef_chords`, `mef_lone`; `crates/topo/src/euler_ring.rs`:
`mekr_mint`, the shared mint of `mekr`'s four entries;
`crates/topo/src/split.rs`: `split_edge`, now closed).

Measured on the minted cylinder wall of
`crates/topo/tests/split_edge_pcurve_rows.rs`: `mev_line` at a `Fan`
site on the wall face's own loop returns `Ok`, and
`validate_pcurves` then reports two `MissingCache` findings for the
half-edges it minted.

**Why `split_edge`'s closing does not reach them, which is the whole of
this row.** A split had the material to do better: a `Pcurve` is a
function of the carrier parameter and holds no interval of its own, so
each child's chart image IS the parent's restricted, and the
restriction re-certifies through `PcurveCache::certify`, which sits in
`geom_brep`'s `impl<T: Decide>` block. `mev`/`mef`/`mekr` mint a
BRAND-NEW edge with no parent row to restrict; its chart image has to
be DERIVED, and every derivation door in `topo::pcurves` (`pcurve_of`,
`nurbs_iso_derive`, `mint_pcurves_of`) carries `T: PcurveFittedLane` —
the bound ripple `mint_faces`'s own comment banks. So closing this
either widens those operators' bound from `Decide` (and every
generic caller's with it) or keeps the declared primitive posture and
says so at each op.

The posture is declared today —
`pcurves::staleness_posture::DECLARED` carries each of these as
`Neither`, "Euler operator" — so this is not an undisclosed defect. It
is open because the declaration's safety argument leans on a backstop
that does not fire in this exact state (`work/trim/`'s
`validate-pcurves-never-recertifies-a-face-it-finds-incomplete`), and
because a caller reading `mev`'s own entry is not told that a curved
cached face will be tier-3 invalid on return.

## For Ev, on the open `[ev]` PR (TOPO, 2026-09-14)

Closing this is either a bound ripple — every half-edge-minting
operator (`mev`, `mef`, `mekr`) and every generic caller widened from
`Decide` to `PcurveFittedLane` so the op can derive the new edge's
chart image — or the declared primitive posture stated at each op
("this operator mints no pcurve row; a caller on a cached face runs
`mint_pcurves_of` at its door's close"), which is what the producers
do today. The loop-re-parenting fix pass measured the ripple's shape
for a sibling bound (`Bounds` on three doors: forty signatures across
four crates without converging), so the ripple is not a small diff.
TOPO recommends the **declared posture**, stated at each op and in
`pcurves.rs`'s posture table, with the `Fitted`/`General` frontier and
this row as the record; the ripple stays available if a consumer
appears that needs a minted edge to arrive cached.

## Context for Ev (TOPO, 2026-09-14, PR 2527)

**What a pcurve row is.** For a face on a curved chart, each half-edge
stores the 2D image of its edge in the face's parameter chart,
certified against the 3D carrier; tier 3 validates curved faces
through these rows (`crates/topo/src/pcurves.rs`). A face is
"cached"/"minted" when every half-edge of its loops has a row.

**The defect.** `mev`, `mef` and `mekr` add new half-edges into an
existing loop. On a cached curved face the two new halves have no
row, so the face is tier-3 invalid on return (`MissingCache` per new
half) until a caller runs `mint_pcurves_of(face)`. Measured: one
`mev_line` on a minted cylinder wall, `Ok`, then two `MissingCache`.

**Why the operator does not mint.** A brand-new edge's chart image
has to be DERIVED (the 3D carrier projected into the surface's
parameters, or fitted), and every derivation door carries the bound
`T: PcurveFittedLane` — the fitting lane — while the Euler operators
are generic over `T: Decide` only. Widening them widens every generic
caller (boolean, splitting, sweep, blend, the recipe layer); a
sibling bound widening was measured at forty signatures across four
crates without converging. `split_edge` could close its own instance
because a child's image is the parent's RESTRICTED, which re-certifies
under `Decide` — no derivation.

**What holds today.** `pcurves::staleness_posture::DECLARED` records
each of these ops as `Neither` (mints nothing, drops nothing), and
every producer (extrude, boolean, blend, shell, …) runs a closing
mint at its own door — a prose convention spelled thirteen times
(`producer-closing-mint-is-a-convention-with-thirteen-copies`),
unenforced, whose backstop does not fire in this exact state.

**The choice.** (i) The bound ripple: operators self-contained, a
curved face tier-3 valid on return, at the signature cost above.
(ii) The declared primitive posture: each op's doc and the posture
table say "mints no row; a caller on a cached face runs
`mint_pcurves_of` at its door's close", which is what every producer
does. (iii) Enforce the convention at the surgery scope's close: the
scope records cached faces its operators touched and its close mints
their missing rows (or refuses) — the same scope-close list question
3's shape (a) needs for re-based runs; the convention becomes a
mechanism without moving any operator's bound.

TOPO recommends **(ii) now, with (iii) as the enforcement unit** cut
beside (a); the ripple only if a consumer appears that needs a minted
edge cached on return. The yes/no asked: yes to the declared posture
as the operators' contract.

## The cleanest long-term shape, ignoring churn (TOPO, 2026-09-14, PR 2527)

Ev: "what would be the cleanest way to do this long-term, ignoring
the cost of churn?" — the operator completes the face it touches, the
same principle question 3 settles on (discharge the obligation at the
call; no half-done state a door can produce). Concretely:

1. **An operator that mints a half-edge into a cached curved face
   mints its row at the mint site**, so a face is never half-minted
   after any Euler operator — D9 row 0 applied to "cached with holes":
   the state becomes unproducible. The closing-mint convention (thirteen
   prose copies) retires, and `validate_pcurves`'s `MissingCache` arm
   becomes a kernel-bug detector rather than a caller-obligation
   reporter.
2. **Which derivation the operator may run.** The carriers the Euler
   operators mint are lines and closed-form arcs; on the analytic
   charts (plane, cylinder, sphere, torus, cone) their chart images
   have closed forms — the door `mint_face` already routes every
   non-`General` image through. If that closed-form derivation can be
   stated under `T: Decide` (phase 1's question: today `pcurve_of`
   carries `PcurveFittedLane` because one body serves both lanes), the
   operators mint the closed-form rows under their present bound and
   REFUSE typed (`UnsupportedCarrier`-shaped) where only the fitted
   lane could derive the image — honest, and the fitted case is a
   NURBS chart, which the Euler operators reach only from callers that
   hold the fitted lane anyway. If it cannot be split, the full bound
   ripple (`Decide → PcurveFittedLane` on the minting operators and
   their generic callers) is the price, and it is a one-time signature
   cost, not a design cost.
3. The declared posture (ii) is then only the interim statement of
   what holds until 1–2 land, and (iii) the scope-close mint is
   unnecessary — the same conclusion as question 3's, where (a)'s
   scope bookkeeping was the deferral nobody needed.

So the long-term answer is (i) in its honest form: closed-form rows
minted at the site under `Decide` with a typed refusal at the fitted
frontier, the ripple only if the split fails. Recommended as the
target; the interim posture stated meanwhile.

## Ruled (2026-09-14, PR 2527)

Ev: "sounds good!" to the long-term shape above. Ratified as the
target: an Euler operator that mints a half-edge into a cached curved
face mints its row at the mint site; the closed-form derivation split
from the fitted lane under `Decide` is tried first, with a typed
refusal at the fitted frontier, the bound ripple only if the split
fails; the closing-mint convention retires with it. The declared
posture (ii) is the interim statement until the unit lands. Kernel
answer: a block slot (phase 1 is the split question); this row is now
that unit.
