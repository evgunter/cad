---
id: shell-glue-relation-has-three-implementations
kind: issue
title: the per-shell component glue relation is implemented three times, and the test-support copy is the one that drifts
status: closed
opened: 2026-09-06
refs: [S69, 2014]
priority: P1
cost: M
design: true
pr: 7438
branch: walks/glue-home
closed: 2026-11-23
---

## What

One relation — *a face glues all its loops; a cycle loop glues across
each edge via `mate`; an empty-loop face is its own dartless
component* — is written out three times in `crates/topo/src`:

1. **`movefac.rs`'s `Body::movefac`**, the operator's plan phase. It labels
   components to decide the partition, and its labelling IS the
   partition: whatever it computes is what the shells become.
2. **`validate.rs`'s `shell_component`**, tier 1's pass 11, whose per-shell counts
   land in `Tier1Report.shell_components` and are reused by tier 2's
   `c = 1` check. `Tier1Report` is a private struct and the counts are
   not reachable from outside the validator.
3. **`seqgen.rs`'s `shell_components`**, added by `S69` (PR 2014) so the
   `movefac` catalog row can offer the two-component shells and so
   `fusion_remake_shell` can refuse a multi-component side. Since PR
   4053 the walk is `shell_component_faces` and copies `movefac`'s
   LABELLING too, not only its count: `movefac_remake_sites` reads
   which component stays in the shell and which moves to `movefac`'s
   `k`-th minted shell from it, and `roundtrip`'s `Movefac` arm
   asserts each site landed where the copy said, so a drift between
   the two is loud there rather than silent.

Three copies of one definition is the ordinary duplication complaint.
The reason it is worth a row is the DIRECTION of the risk: (3) is test
support and it copies (1), the kernel. A generator that re-derives the
kernel's own decision procedure and then tests the kernel against it
cannot detect a defect in that procedure — if `movefac`'s labelling is
wrong, `movefac_candidates` is wrong the same way and the row keeps
choosing sites the operator agrees with. (2) is the independent
implementation and would catch it, which is why the tier-1 assert
inside `assert_euler_postcondition` is the check that actually holds
here, not the generator's site filter.

## What closing it looks like

The natural fix is one relation with the other two as thin callers, and
the shape of the API is the whole question:

- `movefac` needs the LABELS (which face in which component), in its own
  deterministic seed order, and it needs them from the plan phase before
  any mutation.
- pass 11 needs COUNTS per shell, and it runs under a gate (passes 1–7
  clean) that lets it assume things `seqgen` cannot.
- `seqgen` needs a count per shell, cheaply, on every step, with no
  allocation it can avoid — the cost measured in `S69` was ~7% of
  `choose_op` + `apply` for the two new rows together.
- `seqgen` also needs `movefac`'s LABELS before the partition (PR
  4053): `movefac_remake_sites` picks the face that stays and one
  face per component that moves, by `movefac`'s component order. A
  plan-phase component reader exposed by `movefac` itself would make
  `seqgen` a caller of (1) here instead of a copy of its labelling.

A single `pub(crate)` labelling routine returning the component map,
with counts derived from it, serves all three; the risk to weigh is
that pass 11 stops being an INDEPENDENT implementation of the relation
it validates, which is a real loss and may argue for keeping (2)
separate and unifying only (1) and (3).

Not done in `S69` (PR 2014): that unit's fence was the generator, the
ledger and one postcondition site, and moving a relation the operator
and the validator both depend on is not a change to make from a
generator branch.

`crates/topo/src/review_m1_pr4.rs`'s `shell_components` is not a
fourth copy. It glues faces that share a VERTEX (an empty loop glues
through its vertex too), so two components that touch only at a vertex
are one component there and two under this relation. It counts the
per-component v, e, f, r of an Euler–Poincaré derivation in a review
module, and is out of this row's scope.

## Evidence (PR 3669)

`movefac`'s copy now proves more than the other two read. Its mate hop
goes through `Body::proven_mate` and proves that the face the mate's
loop names lists that loop, and that no half-edge claims an empty
loop; since PR 4029 each panics naming the record (D2 row 4), where
PR 3669 refused `NotOwned { child: Loop, owner: Face }` and
`LoopCycleBroken`. On a
valid body the relation is unchanged
(`movefac::tests::valid_fixtures_partition_as_their_records_do`, now
from each shell's face list and its reverse). `seqgen::shell_components`
does neither, which is sound only because it reads bodies the generator
builds through the operators; pass 11 runs only once passes 1–7 are
clean, and pass 7 checks every loop's owner both ways. A unified routine has to
carry the proofs to the copy that reads unvalidated input, so the
`seqgen` register entry in `scripts/gates/loop-boundary-discards.sh`
now says it lacks the proof that audits `movefac`'s.

## Staleness audit (2026-11-22, WALKS, origin/main 7709b0ea)

Still live, and a fourth copy: movefac.rs's test-module `claimed_components` (glues by edge via `parent_loop`, a deliberately record-based oracle). validate.rs `shell_component` seeds in face-arena order, not the shell's list order, and skips a mate in another shell; seqgen.rs `shell_component_faces` is still registered `unaudited` in scripts/gates/loop-boundary-discards.sh. Weighed with `euler-characteristic-has-three-carriers` as fork F5.

## Closed (2026-11-23, PR 7438)

The movefac / generator / open-face copies are one walk. Pass 11
stays out of it, as Ev ruled on PR 7424: it is the independent
instrument for the relation it validates.

- `crates/topo/src/glue.rs`'s `face_set_glue` is the one `pub(crate)`
  walk of the relation over a face set: seeded in the set's order, it
  hops through each cycle member's mate and returns each face's
  component label, numbered in that order, the count, and every mate
  hop that leaves the set. It carries `movefac`'s loop-owner and claim
  proofs and panics naming the record a proof finds torn (D2 row 4).
  The per-loop claim counts (`LoopClaims`) are read once per body and
  passed in, so a caller looping over shells reads the arena once.
- `movefac`'s plan reads its partition from it and panics on any hop
  leaving the shell. The face-owner proof (each listed face names the
  shell) is `movefac`'s own pre-pass: a fact about a shell, not a face
  set.
- `seqgen`'s `shell_components` and `shell_component_faces` read it.
  Their `unaudited` register entry is gone; `movefac`'s audited one
  moved to `face_set_glue`.
- `shell`'s `count_components` reads it over the shell's non-open
  faces and requires each leaving hop to land on one of the shell's
  open faces. `face_neighbours` stays, for the antiparallel-walls pair
  check, which asks about adjacency and not components.
- Left independent: pass 11's `shell_component`, `movefac`'s test
  oracle `claimed_components` (checked against `face_set_glue` over
  `movefac`'s valid fixtures, one with a dartless face seeded after the
  first component), and `review_m1_pr4`'s vertex-glue
  `shell_components`.
