---
id: the-split-leaves-curved-pieces-touching-along-a-curve-with-no-guard-the-census-reads
kind: issue
title: The plane split can leave a side's pieces touching only along a curve on curved faces, with no PartsTouch counterpart, where the census does not read whether curved faces of two parts touch
status: open
opened: 2026-11-25
priority: P2
cost: M
---


Filed by `heldrecords/parting-division` from PR 7783's review (m3).

## What

The Boolean's parting refuses `EdgeInFaceGap::PartsTouch` where it
would divide a curved face between two pieces of the result that touch
only along the parting (`boolean/slit.rs` `part_riders`). It does so
because tier 3′'s census reads such a body `CensusUndecidable` ("the
kernel cannot yet tell whether curved faces of two parts touch",
`work/atrest/the-census-cannot-record-a-curved-seat-between-two-solids.md`).
Measured: half ball ∖ tube at a=0.7 with the gap lifted ships two
solids that fail tier 3′.

The plane split (`crates/topo/src/splitting/`) does not reach the
parting: no item under `splitting/` names `boolean::slit`. It mints its
own coincident pairs where a side's material is pinched along an
operand edge lying in the plane (`join::Tree::slit`,
`finish::Sides::minted_contacts`), and it sorts each side into pieces
(`pieces::sort_into_pieces`). A side whose pieces touch only along such
a line, on curved faces, gets no guard of this kind. Witness: the full
annulus less the full V split by `y = 1`, read from the V's side
(`crates/sweep/tests/a_curved_keel_in_the_plane.rs` `keeled_round`):
two pieces touching along the keel's circle, bounded there by the V's
cones (`a-splits-curved-pinch-along-a-closed-keel-records-no-edge-edge-pair`
records its missing edge–edge pair).

## The question

When that pair is recorded, does the side then pass tier 3′, or read
`CensusUndecidable` on the cones' face pairs as the Boolean's does? If
it reads undecidable, the split needs the Boolean's guard, or the
census's curved-touch reading (`the-gate-raises-no-unbacked-meeting-until-the-census-reads-what-doors-build`
is where the gate's reading of what doors build is held). Not measured
by this lane.
