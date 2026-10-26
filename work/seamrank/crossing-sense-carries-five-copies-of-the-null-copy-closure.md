---
id: crossing-sense-carries-five-copies-of-the-null-copy-closure
kind: issue
title: The crossing-sense code holds the null-copy closure in five places and two rules for matching a crossing's sides by image
status: open
opened: 2026-10-06
priority: P3
---


Found by the re-review of PR 4203 (head eba5c93881). These are style
findings, none blocking.

**Duplications:**
- **The null-copy transitive closure appears five times:**
  - in topo: `NullCopies::of` (`boolean/mod.rs:~1576`), `with_copies`
    (`mod.rs:~4404`), `Descendants::null_copies` (`ops.rs:~2598`), and
    `null_copy_rows`;
  - in editor-core: the fixpoint loop in `operand_vertex_keys`
    (`names/emit_topo.rs:~2022`).

  The kernel ships raw pairs, and the emitter closes them again.
- **Two ways to match a crossing's sides by image, with different
  strictness.** `canonical::flip_senses` (`canonical.rs:~304`) assumes a
  swap whenever an image differs, while `emit_union::resensed` refuses on
  a mismatch.
- **`cited_whole` (`emit_union.rs:~926`) restates
  `WholeMemberEdges::name` (`:~1353`) by hand.**
- **`plane_side_code` is a half-dedupe.** It is private to `sectors.rs`.
  The same `side_code(.., NO_CURVATURE(), band)` spelling remains at
  `recl.rs:459` and `:937`, and two identical `code` closures remain in
  `sectors.rs`.

**Fields and doc claims kept in step by hand:**
- `BooleanNaming.null_copies` must be set beside `edge_classes` at five
  construction sites, and a fallback's `..Default` would drop it.
- The `operand_vertex_keys` and `ops.rs:316` docs claim "one point by
  construction". Nothing checks that every null edge comes from
  `mev_null`.

**Test gap.** The `resensed` unit test covers only `at = None`, so a
drift between `cited_whole` and `WholeMemberEdges` on the `least_at`
path would pass.

**Residuals, believed unreachable:**
- At a closed edge's own vertex, F4's guard has nothing to check, since
  both sides end there.
- `UNCLASSIFIED_SIDE` fires for any vertex `senses_at` reads, not only
  crossings.

## A fifth walker in topo (PR 4740)

`ops::uncopied` (`crates/topo/src/boolean/ops.rs`), which names the
vertex a null-edge copy was minted at so that `ee_lineage`'s fresh
`VertexOnVertex` decision cites an input cell, walks the null edges
back to the original once more, beside `NullCopies::of`, `with_copies`,
`null_copy_rows`, `Descendants::null_copies` and `BooleanNaming`'s
`null_copies` field. Since PR 4981 it is total: it reads the root of
`BooleanReduction.copies` (`crate::lineage::VertexCopies`), which
refuses a non-fresh copy where the reduction records it, so it no
longer refuses `JoinDesync`. One closure, built once per reduction and read by all
of them, retires the copies.

## 2026-10-13: names retired by PR 4820 (FUSE)

PR 4820 retired `ee_lineage`. An operand's carried edge-edge rows
are now placed by `boolean::ops::carried` (`operand_fate`, then
`records::landings`). `ops::uncopied` survives, and its caller is now
`carried`'s minted-meeting decision.

PR 4981 adds one more representation for that closure to retire:
`BooleanReduction.copies` (`VertexCopies`, one per operand) is a
directed view derived from `null_edges`, carried beside
`Descendants::null_copies`, `NullCopies::of_operand`, `null_copy_rows`
and `BooleanNaming.null_copies`.

## Partly moot (WIRE orchestrator, 2026-10-26)

Closed by PR 5758 (2026-10-26, "nothing is ranked", Ev's ruling on PR 5705). The `flip_senses` bullet is moot: the function is deleted. The other bullets stand.
