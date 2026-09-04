---
id: pncad-py-tag-inventory-misses-two-measure-tags
kind: issue
title: main is red at the code tier: TAG_INVENTORY does not list two node_error_tag values tags.rs already ships
status: closed
opened: 2026-09-03
closed: 2026-09-04
---


`pncad-py tests::the_whole_tag_table_matches_its_committed_inventory`
(`crates/pncad-py/src/tests.rs:2629`) fails on `main`'s own tree:

```
src/tags.rs has moved away from TAG_INVENTORY.
  `node_error_tag`: value(s) ADDED ["measure_clearance_refused",
  "measure_selection_kind"], value(s) GONE []
```

**Seen on**: run 33788618577, both lanes, shard 2 (`test (eps = 1e-12,
2/2)` job 100761051102 and `test (interval, eps = 1e-12, 2/2)` job
100761655334), on TCOST-K2's branch — whose diff is two files in
`crates/geom-core/src/spline/` and touches no `pncad-py` line. The
merge of `main` is what carried it in. It reproduces against
`origin/main` at `be6f3145a`: `crates/pncad-py/src/tags.rs` contains
`measure_clearance_refused`, `crates/pncad-py/src/tests.rs` does not.

**Why nobody saw it.** The two values arrived with the M10 measure
work; the gate that reads them (`lib: pin the refusal-tag VALUES, the
whole table, by name`, `434964dfa`) landed after. `main`'s own push
runs classify at the docs tier and skip the test legs
(`docs/CI-MINUTES-2026-08.md` §*THE PUSH RUN CARRIES ONLY WHAT IS
UNIQUE TO IT*), so the first tree to run it was a branch's.

**Not fixed here, deliberately.** The gate's own message says the
repair is to update `TAG_INVENTORY` *and* check `pncad.pyi` and
`tests/*.py` for callers of every word that moved — the tag values are
a public Python contract. Neither new value appears in any `.pyi` or
`.py` under `crates/pncad-py/`, so the repair has to decide whether
that absence is correct surface or a missing binding, and that is the
owner of the measure work's call, not a passing lane's. A lane that
refreshed the receipt would launder the question away.


---

**Closed by M10-7** (2026-09-04). `TAG_INVENTORY`'s `node_error_tag`
list gains `measure_clearance_refused` and `measure_selection_kind`, in
sorted position beside the other `measure_*` values, which is what the
row asked for: the two tags `tags.rs` already ships are now listed.
Nothing in `pncad.pyi` or `tests/*.py` branches on either string — they
are new M10-6 surface no Python caller has met — so the fix is the
inventory line alone.

Fixed here rather than in its own PR because it is red on `main` at the
code tier, so it fails on every head: M10-7 could not have shown a green
run of its own without it.
