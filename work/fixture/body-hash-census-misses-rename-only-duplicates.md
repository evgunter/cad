---
id: body-hash-census-misses-rename-only-duplicates
kind: issue
title: the body-hash duplicate census misses rename-only twins
status: open
opened: 2026-09-03
priority: P4
cost: E
---


TCOST-8's second sweep pattern hashed every `fn` block's body TEXT
(whitespace-stripped, lines joined) and grouped by hash, which is what
found the same job under different names (`unit_cylinder`, `zcyl`) that
a name census cannot see. **A textual hash still misses a duplicate
whose only difference is a parameter or local name**, and TCOST-8's PR
body states that blind spot but does not close it.

Confirmed instances in `crates/geom-brep/tests/` (line numbers are
post-TCOST-8):

- **`width`, three spellings of one body.** `arc_eval_anchor.rs:38`
  (`x.hi() - x.lo()`) and `review_arceval_r1_probes.rs` (same) hash
  together; `revolved_point_anchor.rs:37` (`e.hi() - e.lo()`) does not,
  and is the same function under a renamed parameter. TCOST-8 kept the
  first pair apart on a stated reviewer-pair reason and never saw the
  third.
- **The max-of-three-widths variant.** `cert3r1_e2e.rs:69` writes it as
  a closure `|e: Interval| e.hi() - e.lo()` folded with `.max`;
  `r2_cert3_e2e.rs:67` writes the same fold inline over `p.x`/`p.y`/
  `p.z`; `arc_eval_anchor.rs`'s `point_width` is the two-coordinate
  form. Three shapes of "the widest coordinate of an enclosure", none of
  which hash together, and none of which any census in TCOST-7 or
  TCOST-8 reported.

**The obligation this leaves.** The sweep is a class claim, so the
pattern's blind spot is a claim too: *no rename-only duplicate exists*
is unverified across `crates/geom-brep/tests/` and equally across the
crates TCOST-B1 and TCOST-B2 converted to a shared tree, since they were
swept the same way. Closing it wants a normalized hash — alpha-rename
locals and parameters to positional placeholders before hashing, or
compare token streams with identifiers elided — re-run over every crate
that has a `tests/shared/`, with the hit list published the way TCOST-8
published its own.

Not fixed here on purpose: rewriting the census is a tool change whose
output is a new hit list, and a hit list wants a unit that can act on it.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/tcost/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Moved to S-TINT (2026-09-11)

Moved by `git mv` from `work/tcost/` at S-TINT's opening. Id, title and
body are unchanged; the directory is the claim (`work/README.md`).

**Why it moved.** S-TCOST's board was re-sorted on 2026-09-11 against the
repository going public on 2026-09-03 (`work/tcost/log.md`, the
2026-09-11 seam). That sort found this row is not a cost lever in either
currency — it neither shortens the gate's critical path nor saves a
billed minute, and it was never argued on one. It reached S-TCOST by the
tracker-wide re-home of 2026-09-04, which routed rows by PATH GLOB
(`crates/*/tests/*`, `crates/test-utils/*`) rather than by question.
This program is the question it was always about: whether the suite
asserts what it claims to assert.

## Re-derived (2026-09-15, lane C)

**VERDICT: REPRODUCES** — every named instance is live, the normalized
census has not been written, and one copy the row did not name has been
added since.

**The `width` triple, by name.** All three are `fn width(_: Interval) ->
f64` returning the same subtraction, still in three files:

- `crates/geom-brep/tests/arc_eval_anchor.rs` `fn width` — `x.hi() - x.lo()`
- `crates/geom-brep/tests/review_arceval_r1_probes.rs` `fn width` — `x.hi() - x.lo()` (hashes with the first)
- `crates/geom-brep/tests/revolved_point_anchor.rs` `fn width` — `e.hi() - e.lo()` (the rename-only twin a text hash misses)

**Not named in the row and also live: `point_width` is now a THIRD
copy.** `fn point_width` exists in all three of those files —
`arc_eval_anchor.rs` and `review_arceval_r1_probes.rs` over
`Point2<Interval>`, `revolved_point_anchor.rs` over `Point3<Interval>`.
The row named `arc_eval_anchor.rs`'s `point_width` only as one shape of
the max-of-widths variant; as a duplicate pair it is a second instance of
the same defect on the same slate.

**The max-of-three-widths variant, by name.** Still three shapes, none
hashing together:

- `crates/geom-brep/tests/cert3r1_e2e.rs` — the closure `let w = |e: Interval| e.hi() - e.lo();` folded with `.max`
- `crates/geom-brep/tests/r2_cert3_e2e.rs` — the same fold written inline over `p.x`/`p.y`/`p.z`, and written TWICE in that file (once in a helper, once inside a row)
- `crates/geom-brep/tests/arc_eval_anchor.rs` `point_width` — the two-coordinate form

**The shared tree did not absorb any of it.** `crates/geom-brep/tests/shared/`
exists (`fixture.rs`, `interval.rs`, `patch.rs`, `point.rs`, `ring.rs`,
`sample.rs`, `surf.rs`, `tol.rs`, `topo.rs`, `mod.rs`) and carries no
`width`, `point_width` or any `hi() - lo()` helper;
`shared/interval.rs`'s header states what it deliberately leaves out and
it is a different function (`revolved_point_anchor.rs`'s `w(c)` widener).

**The obligation is still open.** `grep -rln "alpha-rename\|normalized
hash\|rename-only"` over `*.rs` and `*.py` returns nothing: no
normalized-hash or token-stream census exists anywhere in the tree, so
the class claim *no rename-only duplicate exists* is as unverified today
as it was on 2026-09-03, across `crates/geom-brep/tests/` and across the
eight other crates that carry a `tests/common/` or `tests/shared/`
(`mesh`, `profile`, `step-export`, `step-import`, `stl`, `sweep`, `topo`,
`viewer`).

**How this was re-derived, and its blind spot.** By name, not by line:
`grep -rn "fn width\|fn point_width\|hi() - "` over the five named files.
Every line number in the original body still resolves (`:38`, `:37`,
`:69`, `:67`), which is luck rather than evidence — the names are what
was checked. **What this could not match**: a rename-only twin whose
body also differs in whitespace-invisible ways (a `let` binding
introduced, arguments reordered), and any duplicate outside the five
files the row names, since running the normalized census IS the unit.

**Recommendation (orchestrator's call).** Keep open, unchanged in shape;
add `point_width` to the instance list when a unit is cut.
