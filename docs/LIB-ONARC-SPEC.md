# LIB-ONARC spec — the OnArc dissolution (§2c amendment 2026-08-16; binding)

Mandate: execute the ratified §2c dissolution amendment
(docs/PATHS-DESIGN.md, "§2c dissolution amendment — OnArc
RETIRES"): arc arrivals emit their run at the verb and land on an
ordinary directed point; arc extension replaces `Radius@OnArc`;
the OnArc state and its whole surface delete; the LoopBuilder
test shim's last caller class migrates and the shim DELETES. Read
first: the amendment (every clause is binding), the census in
docs/LIB-LOG.md (2026-08-16 — your per-site work map, R1–R6c +
Python + doc surfaces), `crates/profile/src/path/family.rs`
(OnArc at :147, OnArcIncoming :848, the fused verbs :1209-1273,
resolve_arc_arrival :215-247), `crates/profile/src/path.rs`
(emit_fillet_in :1639, extend_leg_to :1682 — the ray-extension
vertex-move you are giving an arc analog).

## 0. Discipline (absolute)

docs/LIB-PYG1-SPEC.md §0 verbatim and binding (foreground builds
one at a time, build slots, no parking + kill-your-own-waiter,
commit+push per chunk, NO Co-Authored-By, no model names,
merge-main-before-open + re-merge on movement, checks STARTED,
cold clippy CI scope both lanes, k-lint discipline, comments
state the INVARIANT).

## 1. Deliverables, in dependency order

1. **The probe, first, at the pre-change head**: an executed test
   demonstrating the mismatched-r `Radius@OnArc` hole (author a
   continuation whose `r` differs from the arrival carrier's;
   show the emitted run's bulge/ArcData/declared-tangency
   inconsistency, or the late validate failure — record WHICH in
   the report). Commit it red-or-characterizing before the
   mechanism moves; it survives re-pointed at the new semantics
   (mismatched r = a legal new tangent carrier, sound by
   construction).
2. **Emission moves to the arrival verb**: `resolve_arc_arrival`
   emits the run head→anchor itself (it holds the authored
   carrier; `bulge_from_center` with the verb's own spec) and
   returns an ordinary directed point at the anchor with
   intrinsic `Incoming { ang, arm, carrier }`.
3. **Arc extension**: the `Radius{r, side}` fused incoming
   becomes available from any directed point (the existing
   derivation — it already consumes only a `DirectedPoint`).
   Same-carrier continuation (decide via the chain's
   `Incoming.carrier` bookkeeping — chain-side, axiom-clean)
   MOVES the incoming segment's end vertex to the trim point
   (the `extend_leg_to` analog for arcs; §4 item 4 exemption);
   a different carrier emits its run from the tip with a
   constructed tangency there. Trim-eats-anchor keeps refusing
   `AnchorOutsideTrimmedExtent` (existing gate — do not re-mint).
4. **Deletions**: `OnArc`, `OnArcIncoming`, `TipState::OnArc`,
   the `DynTip::OnArc` replay arms + builder rows
   (`crates/profile/src/path/program.rs:321,437,455,596-605,
   696-714,970-1009`), Python `PathOnArc` + its five
   continuation overloads, the arrival builders re-target the
   directed-point return (`crates/pncad-py/src/py/path.rs:366,
   369-372,421-467,1112-1137,1211`; `pncad.pyi:19,315-341,
   358-385,396-408,434,463,486,511,527`). Step vocabulary
   (`ArcFillet`/`ArcFilletArc`) is UNCHANGED — only tip-state
   plumbing moves; check `TipState` ripple into error payloads
   (pncad-py tags, editor-core mirrors) and report what you
   find. Doc surfaces per the census §7 list (family.rs docs +
   matrix table, path.rs module doc + compile_fail doctests —
   the E0599 doctest flips to a POSITIVE row: sharp-after-arc
   compiles — test_support.rs header, GUIDE untouched per
   census, north-star audit page rows for `PathOnArc`).
5. **Bit-identity, executed**: every census chain (family.rs
   doctest R1; rocker boss/hub R2/R3; path_program R4/R5;
   path_property R6a-c; the Python matrix rows) re-emits the
   IDENTICAL final vertex chain — assert against the existing
   pinned rows (`common::pinned` round-trips) and state the
   executed evidence in the report. If any chain moves a bit,
   STOP and report — that falsifies the amendment's unchanged
   claim and is evidence, not a fix-up target.
6. **Sharp-after-arc rows**: positive tests for the restored
   junction (arc arrival → director → line leg; the arc×arc
   fillet corner with DIFFERING far points — the shim's last
   class, authored on the lattice).
7. **Shim deletion**: migrate `crates/profile/tests/review_s2.rs`
   (keep its arc×arc coverage floor and independent far-point
   draws), `review_s8_probe.rs::check`, and the two
   `arc_fillet.rs` fixtures to lattice spellings; DELETE
   `crates/profile/src/test_support.rs` and its feature wiring.
   Note in the PR body that #377's retirement completes (the
   orchestrator closes the issue).

## 2. Fence

- NO entry/seam machinery changes — the all-blended-entry gap
  stays named and open; `p: Start` closes untouched.
- NO RESPELL-TABLE work (it stays gated behind this unit).
- NO ProfileLoop field/accessor work (LIB-SEAL's fence; this
  unit dispatches after SEAL merges — absorb its main state at
  routine re-merges).
- NO schema claim expected; verify main's live SCHEMA_VERSION by
  eye at final re-merge anyway (standing discipline).
- No new public surface beyond the amendment's own rows.

## 3. Acceptance

1. Hosted matrix green incl. python-suite; ty gate green with
   the re-targeted stubs.
2. The §1.1 probe recorded red-then-repointed; §1.5 bit-identity
   stated as executed (which suites, which command).
3. `git grep -i onarc` returns only historical-record docs
   (LIB-LOG, MODEL-AB-LOG, this spec, the amendment).
4. The shim absent; review_s2's coverage floor demonstrably
   preserved (state how).
5. Report ≤150 lines, deviations enumerated.

## 4. PR discipline

One PR, branch `lib/onarc`. PR body: the amendment cited as the
ratified basis, the probe's finding, the bit-identity evidence,
the TipState-ripple finding, #377-completion note.
Merge-main-before-open; re-merge on movement; checks STARTED.
