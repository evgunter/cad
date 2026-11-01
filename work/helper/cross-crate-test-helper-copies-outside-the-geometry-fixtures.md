---
id: cross-crate-test-helper-copies-outside-the-geometry-fixtures
kind: issue
title: cross-crate duplicated test helpers outside the geometry-fixture rows: validated/vp at 10 byte-identical copies, and twelve more classes no row names
status: open
opened: 2026-09-15
priority: P4
cost: E
---


## Finding

- **Where**: `crates/*/tests/**/*.rs`, thirteen helper classes listed below
- **Importance**: medium — none of these is a geometry fixture, so none is
  covered by the fixture rows this slate already carries, and the largest is
  ten byte-identical copies across three crates
- **Confidence**: sure for the hit list (measured, method below); the
  *disposition* of each class is a judgement this row does not make
- **Raised by**: the TINT-3 lane (`tint/3-aggregation-guard`, PR #2680), in
  the sweep it owed after collapsing fifteen copies of
  `every_suite_file_is_aggregated`

### Method, so the numbers can be re-taken

Every top-level `fn` under `crates/*/tests/**/*.rs`, body taken line-wise from
the signature line to the next column-0 `}`, bodies of three or more lines
hashed (`md5`) and grouped by hash; reported where one hash occurs in two or
more crates. Name-independent, so a renamed copy is caught. Counts below are
`copies / distinct bodies` for the NAME group, with the byte-identical group
called out.

### The classes, with the hit list

**No `work/**` row names any of these** (`git grep` over `work/` for each
name; the two exceptions are called out in the reconciliation below).

| class | copies / bodies | largest byte-identical group | crates |
| --- | --- | --- | --- |
| `validated` / `vp` | 24 / 8 | **10** (`79e84db1`) | mesh, stl, step-export, sweep |
| `axis_y` | 12 / 5 | 4 (`5c1e3edb`) | mesh, stl, sweep |
| `assembly` | 10 / 6 | 4 (`dc5347b3`) | editor-core, sweep, topo |
| `lp` | 6 / 2 | 5 (`5ce47feb`) | mesh, stl, sweep |
| `sup3` / `supp` / `supv` / `sup_dist` | 5 / **1** | 5 (`20bc234a`) | geom, sweep |
| `ray` | 5 / **1** | 5 (`e87acaa5`) | bvh, editor-core |
| `quad` | 4 / 2 | 3 (`3a4cb6e4`) | mesh, step-export, sweep |
| `sub_arc2` / `sub_arc3` | 4 / **1** | 4 (`b6bf67af`) | mesh, topo |
| `half_disc` / `ball_ccw` | 4 / **1** | 4 (`9a949ec3`) | mesh, sweep |
| `outcome_str` | 3 / **1** | 3 (`c9b25aac`) | editor-core, sweep |
| `cross` / `v_cross` | 3 / **1** | 3 (`3eed4fa0`) | editor-core, geom-brep, step-export |
| `split_carrier` / `fitted_like_carrier` / `nurbs_carrier` | 3 / **1** | 3 (`88c12e77`) | geom, topo |
| `on_pool`, `missing_pairs`, `ends`, `vertices_at`, `disc`, `bits`, `offset_square_prism`, `scalars`, `embed`/`loft_walk`, `disp`/`parse_disp`, `wedge` | 2 each / 1 each | 2 | various |

The largest, in full, because it is the one that needs a decision:

```
79e84db1  crates/mesh/tests/common/mod.rs:66              fn validated
79e84db1  crates/mesh/tests/r1_probe_hash.rs:21           fn vp
79e84db1  crates/stl/tests/common/mod.rs:21               fn validated
79e84db1  crates/sweep/tests/extrude_acceptance.rs:33     fn validated
79e84db1  crates/sweep/tests/k_report.rs:78               fn validated
79e84db1  crates/sweep/tests/m5_s11_concave_sense_interval.rs:37  fn validated
79e84db1  crates/sweep/tests/review_m2_pr4.rs:39          fn validated
79e84db1  crates/sweep/tests/review_m2_pr5_interval.rs:21 fn validated
79e84db1  crates/sweep/tests/review_m3_pr1_sweep.rs:20    fn validated
79e84db1  crates/sweep/tests/revolve_common/mod.rs:27     fn validated
f0c9784d  crates/mesh/tests/issue111_az_needle.rs:77      fn validated   (+4 siblings)
7e946640  crates/step-export/tests/common/mod.rs:16       fn validated   (+1 sibling)
```

**`vp` is the rename-only shape**, byte-identical to nine `validated`s under a
different name — the thing `body-hash-census-misses-rename-only-duplicates`
says a name census cannot see, caught here because this sweep is body-keyed.
`sup3`/`supp`/`supv`/`sup_dist` is the same shape at five copies and four
names.

**Eight of the eleven bodies named `validated` have already drifted.** That is
the drift the duplication class predicts, and it means whoever takes this
reconciles rather than merges: for each hash group, decide which body is right
and say what moved.

### Reconciliation with the rows this slate already holds

The TINT-3 PR body originally dispositioned this whole sweep as "already
covered", which was wrong, and this section is what that claim should have
been. The existing rows are **geometry-fixture** rows and cover the fixture
names only:

- `work/tint/tests-common-body-fixtures-triplicated.md` — `ball`, `donut`,
  `l_prism`, `cone`, `washer`, `holed_prism` across the three `tests/common/`
  trees. It DOES name `axis_y ×2` and `validated ×2` at its line 47, and
  defers them as *"three-line vocabulary … worth deciding explicitly rather
  than sweeping in"* — **scoped to those three trees**. The measurement here
  is 12 and 24 copies across four crates, an order of magnitude past what that
  sentence was deferring, so the deferral does not carry.
- `topo-tests-brick-copies`, `sweep-boolean-suite-brick-and-prism-copies`,
  `the-two-vertex-bulge-one-circle-fixture-has-eight-copies`,
  `origin-anchored-fan-oracle-copies`,
  `geom-curve-test-frames-hand-roll-the-helper-axis-cone` — bricks, prisms,
  bulge circles, fan oracles, axis cones. None names anything in the table.

Two classes from the original sweep are **genuinely covered elsewhere and are
excluded from the table above**, so this row does not duplicate them:

- `fnv` / `fnv_str` / `digest` (26 copies, 17 bodies) —
  `work/perf/fnv-digest-and-memo-machinery-copies.md`, open, which lists the
  FNV sites by path. Cross-crate byte-identical groups measured here, as
  evidence for it: `6455c0f5` ×4 (`mesh` d9_mesh_goldens/mesh10r1_digest/
  patch_memo, `viewer` index_memo), `afee9d4b` ×3, `e8fb8426` ×2, and
  `39d19dc9` ×3 (`geom` span_bit_identity{,_ext}, `geom-core`
  coeffs_bit_identity) which that row does not name.
- `missing_pairs` — `work/perf/sweep-trace-has-two-spellings.md`, which names
  it directly and records it as deliberately left at two spellings.

### What the sweep could not match

- a duplicate identical only after alpha-renaming LOCALS or PARAMETERS —
  `body-hash-census-misses-rename-only-duplicates` is the open row for that,
  and it is why the `vp`/`sup*` hits here are a lower bound;
- bodies under three lines, and any `fn` whose closing brace is not at
  column 0 — anything nested inside a `mod`;
- `#[cfg(test)]` modules under `crates/*/src/**`;
- the cargo roots outside the workspace: `demos/`, `tools/`, `benches/`,
  `interval-transcendentals/`;
- two copies WITHIN one crate: the grouping requires two distinct crates, so
  an in-crate pair is invisible here. `axis_y` and `assembly` both show
  in-crate clusters at the edge of that blind spot.

### Not taken by TINT-3, on purpose

TINT-3's fence is the aggregation guard. Every hit here is a fixture or
vocabulary helper whose home is a judgement about
`crates/sweep/src/test_support.rs` and the routing rule in its header — the
same decision `tests-common-body-fixtures-triplicated` is waiting on, and one
that wants a unit that can act on the whole table at once.
