# CERT-M1 — Track M's trait-ground rows (H3+H4, H10+S211, D78, D221)

**Binding at dispatch** (S-CERT program, `docs/S-CERT-PLAN.md` §CERT-M;
difficulty logged at spec: **M**). Read
`docs/prompts/implementer-discipline.md` in full before starting. The
absorbed Track M table in `docs/SMELL-SCAN-2026-08.md` (§"Track M — the
scalar and certification traits") is the primary specification for each
row; this document fixes which rows this unit takes, in what order, and
the fence. Branch `cert/m1-trait-ground`.

## What this unit is, and is not

Track M has seven items. This unit takes the four that sit on the
trait ground WITHOUT the lane-trait collapse: **H3+H4** (one lane),
**H10 with S211**, **D78**, **D221**. It does NOT take `H5`
(`S90-impl` folded in — the H-R16 three-function split, its census,
and the ADV `Dual`-rewriting sub-lane; those are CERT-M2/M3) and it
does NOT take `S213` (its bound half is the supertrait obligation on
`topo::validate_geometric`, which H-R16's split reshapes — it goes with
H5). **`#883` stays parked** (reserved as lane H-f; not this program's
to unpark). Read `docs/SMELL-H-LOG.md` H-R3 and H-R16 anyway: they
define the vocabulary (a tightened bound does not *refuse*; it makes the
call unwritable) and you will be editing the doc block that has to say
so correctly.

**Fence (file territory — the repartition's rule 1):**
`crates/geom-core/src/{real,ring_interval,dual,interval,k_stats}.rs`,
`interval-transcendentals/`, `crates/bvh/`, plus the tests that live
beside them (`crates/geom-core/tests/`, `interval-transcendentals/tests/`).
Where a row's work reaches a path outside this fence — S89's two alias
sites in `geom-brep/src/ssi/enclose.rs` and `topo/src/props.rs`, the
bounds-allowlist gate under `scripts/gates/` (Track K's `D68`/`D103`) —
you do NOT edit it: you file the reaching half as a row on the owning
track (draft the row text in your report; the orchestrator lands it in
SMELL-SCAN) and land without it. Filing IS the handoff.

## The rows, in order

1. **D221 first** (smallest; it settles a habit the rest reuses).
   `real.rs`'s `abs_properties` asserts `abs(x) >= 0` beside an exact
   `prop_assert_eq!` that implies it. The test's own header names
   non-negativity as covered. Decide which the header should describe
   — the exact value (then the implied assertion goes and the header
   changes with it) or the three properties (then the redundant line
   is the honest one and the exact pin is the extra) — and make the
   header and the assertions say one thing. Do not delete first and
   describe after.
2. **H3+H4 — one lane on `real.rs` and `from_certified`.**
   (a) S85: `Bounds`' headline still calls it the door for
   "certification and driver code" — the sentence D1 demoted it out of;
   the fix that retargeted the siblings left the anchor. Retarget it,
   and take the 234-line doc block back to a size where a reader finds
   the rule: the three entries edited in place AND annotated with what
   they used to say carry both texts — the corrected text stays, the
   prose about the replaced text goes (the PR is the record). Read
   `interval.rs`'s `Bounds` impl doc and `Bounds::lo`/`hi`'s method docs
   for the same rot. (b) S89: `RingInterval::from_certified` is the
   declared one home, and three private one-line aliases sit on it
   (`bracket` in `ring_interval.rs`, `ring` in `ssi/enclose.rs`, `br` in
   `topo/props.rs`), each restating the rule at paragraph length, plus a
   PROSE CENSUS of callers in the door's doc that nothing enforces and
   has already needed a correcting commit. Delete the in-fence alias
   and its restatement; replace the prose census with something that
   cannot drift (a grep-backed row that counts the crossings, or no
   count at all — a count nothing checks is the defect); the two
   out-of-fence aliases are FILED as rows for their tracks with the
   exact edit named (inline `from_certified`, delete the restatement).
   (c) The decoration-seam suite (`geom-core/tests/decoration_seam.rs`)
   claims its rows pin "the three C9-ring crossings follow the second
   door" while every executable row reaches the ring through ONE
   crossing — make the claim true or make the rows cover what the
   claim says (measure which crossings actually run; red-first on the
   uncovered ones if you add rows).
3. **H10 with S211 — the rule with no instrument.** `real.rs`'s
   `Bounds` scope rule says bracket extraction appears only in
   certification/driver code, written as the parameter's SOLE bound —
   and the sole form is exactly what `bounds-allowlist.sh` plants as
   its must-not-fire case, so nothing watches it (S210). The gate half
   is Track K's and not yours. Yours: (a) produce the census of the
   sole-`T: Bounds` class ACROSS the workspace (S210 has `geom`'s only;
   S88 is the shape) as a checked artifact — an in-fence test that
   walks the tree and pins the roster, so the rule has an instrument
   that is not the gate, OR an argued statement of why no in-fence
   instrument can see it and where the K-side row must land; (b) S211:
   `bvh/src/lib.rs:56-61` tells a reader the CI grep allowlists its
   `Bounds` reads — false, and it cannot become true (`aabb.rs:87` is a
   sole bound). Correct the sentence to the truth (which instrument, if
   any, watches this crate) — the class of "a false mitigation is worse
   than a disclosed hole".
4. **D78 — what is still one-directional in the interval backend after
   G1 (S134).** Three items, each a measurement before a change:
   (a) `powi`'s tightness ceiling: `certify.rs` passes `None` because
   the steps are exponent-dependent — derive the exponent-dependent
   ceiling (S134 says it is derivable and that a constant would be
   wrong; `review_m0_pr4.rs`'s containment row and the measured worst
   ratios 117/122 at |n| ≤ 31 are your instrument); (b) the oracle
   tier's upper constraint is a scale-free RATIO, so a fixed absolute
   over-widening on a non-monotone shape at large oracle width matches
   no fixture — add the per-endpoint oracle-relative bound #786 declined,
   or re-argue #786's stated reason (extremum capture, huge arguments)
   with a measurement; (c) `interval.rs:135-143`'s consumer-side caveat
   — state what it protects against today or retire it. Each of the
   three lands with its digits; a declined one lands with the argument.

## Posture

- ε: the interval-backend rows are band-independent arithmetic
  (state that at each row); `CI-Config: lane=both` (the interval lane
  is this unit's subject) with the ε argument stated per the issue-1356
  practice; three-ε local sweep on every new/changed row.
- Review: the program's standard v6 dual; none of these rows is marked
  ADV, so the reviewers weigh the two track questions above all — *is
  the original problem completely gone* and *was it closed in the best
  way available* — and §D rule 5: does the fix mint a fresh instance of
  the defect it closes (a one-home fix that mints an alias; a doc fix
  that adds prose about the prose).
- Landing conventions (repartition rule 3): the landing PR DELETES the
  closed rows from the Track M table and the closed findings' text
  (S85, S89 if fully closed, S134's closed members, S211's clause) —
  member by member where partly closed; relocate any standing rule the
  finding text carries into surviving text before deleting it. Every
  landing PR edits SMELL-SCAN, so expect the merge conflict and take it.
  Struck rows: check the two-things rule (the work landed; every
  rides-along re-homed).
- No `Co-Authored-By`; rows and findings spelled out ("row H3", "S89");
  push early to `cert/m1-trait-ground`; the gate runs when the
  orchestrator opens the PR — report local evidence as local; the
  lane rules in the discipline doc apply in full (build-slot mutex,
  foreground polling, disk).

## Acceptance

- D221 header and assertions saying one thing; H3's doc block
  retargeted and shrunk with the replaced-text prose gone; H4's
  in-fence alias gone, the census replaced by an instrument or by
  nothing, the decoration-seam claim true; H10's sole-bound census as a
  checked artifact (or the argued impossibility) and S211's sentence
  true; D78's three items each measured and landed or declined with
  the argument.
- Rows filed for the two out-of-fence alias sites and any other
  reaching work; the SMELL-SCAN deletions in the landing PR.
- Sweep obligation: assume each is a class — other doc blocks in the
  fence carrying "what this used to say" prose; other prose censuses of
  callers; other false "a gate watches this" sentences in the fence —
  hit list with dispositions; state what the pattern cannot match.
- Deviations stated; D2-addendum classification for anything minted or
  retired.
