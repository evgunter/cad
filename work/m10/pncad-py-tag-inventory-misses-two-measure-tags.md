---
id: pncad-py-tag-inventory-misses-two-measure-tags
kind: issue
title: main is red at the code tier: TAG_INVENTORY does not list two node_error_tag values tags.rs already ships
status: closed
opened: 2026-09-03
closed: 2026-09-04
---

**If your run is red on this, append an instance below — do not open a
new item.** Every code-tier PR carries this one, and two CIW lanes have
already opened a second file for it and had to withdraw it:
`work/ciw/perf-history-cannot-identify-its-host` and
`work/ciw/render-lanes-red-at-missing-merge-ref` (PR 1724). And if you
are only passing through, do not repair it by refreshing the receipt —
the paragraph headed *Not fixed here, deliberately* says why the repair
is the measure work owner's call and not a lane's.


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

## Re-confirmed 2026-09-04 (CIW unit 5, PR 1722)

Still live, and now on a second unrelated branch: runs
[33822129170](https://github.com/evgunter/cad/actions/runs/33822129170)
(`test (eps = 1e-12, 2/2)`) and
[33822742334](https://github.com/evgunter/cad/actions/runs/33822742334)
(`test (interval, eps = 1e-12, 2/2)`). Same test, same two ADDED values,
same shard, both lanes — and it is the ONLY failure in either run
(2862/2863 and 3146/3147 passed under `--no-fail-fast`, so each summary
is that shard's whole failure surface, not its first failure). The
carrying diff was two `scripts/` files, one `crates/editor-core/tests/`
file and three `docs/perf-data/*/README.md` files.

**Sharper than "main's push runs classify at the docs tier":** the
skipping is observable per run and it has been unbroken. `main`'s three
most recent runs at the time of writing — `59c461c`, `ae25e03`,
`d8d0256` — each show BOTH test rows as `skipped`, still carrying their
unexpanded matrix names (`test (eps = ${{ needs.filter.outputs.eps }},
${{ matrix.shard }}/2)`). So it is not that `main` runs this test and
tolerates it, nor that the lane draw has been unlucky: **no run on
`main` has drawn a point that executes this test since the two values
landed.** Every instance of this red is therefore a branch's, and every
branch that draws a code-tier test row will keep paying for it until the
repair lands. That is worth stating because the symptom — "my unrelated
PR is red" — is going to recur and will keep costing a lane its triage
time before it gets here.

Nothing above revises the disposition: still not fixed by a passing
lane, for the reason the section above gives.

## Further instances 2026-09-04 (CIW unit 1, PR 1724)

**Shard `2/2` only, and no sampled axis selects it.** Over the evening it
fires at `default` and at `interval`, and at all three tolerance rows —
`default`, `1e-6` and `1e-12` — while `1/2` is green in every one of those
runs. The shard is where this test lands, not a property of the sample.

**The draw is ruled out from one branch.** PR 1724 pushed twice: run
33822327502 drew `interval` at `1e-12`, run 33823102468 drew `default` at
`1e-12`, and both reproduced it on a diff of `.github/workflows/` and
`work/` that contains no Rust at all. A re-run draws the same point, so
two heads drawing two points and agreeing is the cheapest evidence that
the configuration is not what is selecting the failure.

**Breadth on the night of 2026-09-03/04**, no two of these sharing a diff:
`tcost/k3-unit` (33803928081) and `tcost/k3-cost-probe` (33804838111),
each red in BOTH of its `2/2` jobs; `tcost/10-blend-fixture-home`
(33809383460, 33812074765); `tcost/11-aggregation-guard-home` (33810572043,
33811688528, 33814096354); `tcost/reviewer-rows-are-ordinary-rows`
(33820183319); and the sibling CIW lanes `ciw/perf-host-identity`
(33822129170) and `ciw/one-pin-reader` (33822152938).

**What this lane read and what it inferred.** The failing test name was
read directly from its own two runs. For the other seven branches it read
only the job name and the `exit code 100` shape, and inferred the rest
from the identical shard; the run ids are listed so that is checkable
rather than taken.

## Re-homed to LIB, with provenance (2026-09-04, CIW orchestrator)

Ev, 2026-09-04: keep merging over it, but pass it off to actually get
fixed. Re-verified unfixed on `origin/main` at that moment —
`crates/pncad-py/src/tags.rs:318` and `:322` ship the two values,
`crates/pncad-py/src/tests.rs` names neither, and no `.pyi` or `.py`
under the repository names either.

**Why LIB and not M10.** `crates/pncad-py/*` is LIB's territory
(`work/lib/program.md`); M10's `paths` do not reach it. Both possible
repairs — adding the two values to `TAG_INVENTORY`, or adding the
bindings whose absence would make that wrong — are edits inside LIB's
fence. What LIB does **not** own is the one question that decides
which: *is the absence of these two tags from every `.pyi` and
`tests/*.py` correct surface, or a missing binding?* That is the
measure work's intent and it is M10's to answer (LIB's own `keep_out`:
"the analysis lane is M10's"). One question, then a small edit.

## How it happened: two green PRs, seven minutes apart

Established by CIW on 2026-09-04, because "who broke it" was being
guessed at and the answer bears on the class rather than on blame.

| commit | program | what | merged (UTC) |
|---|---|---|---|
| `5a3fc838` | **M10**, `m10/m10-6-reporting` (PR 1685) | added both `node_error_tag` values | 17:48 |
| `434964df` | **LIB**, `claude/lib-mechanical-clippy-ci-tadd42` (PR 1696) | added the `TAG_INVENTORY` gate that reads them | 17:55 |

LIB last merged `main` into its branch at **16:31**, an hour and a
quarter before M10 landed, so its branch never contained M10's values —
`461e0f9a` is not an ancestor of LIB's final head. **LIB enumerated the
table correctly against the tree it could see, and was made stale seven
minutes later by a merge it had no way to know about.** No lane was
careless; nothing here is a finding against either program.

**Neither PR run could have caught it.** M10's run had the values and
no gate. LIB's run had the gate and no values. The combination first
existed only on `main`, whose push runs classify docs-tier and skip the
test rows entirely — so the first tree to execute it was an unrelated
third party's branch, days later, and it has been billed to every
code-tier PR since.

That is F3's accepted residue (`docs/CI-MINUTES-2026-08.md` §F3: "the
landed main commit is then never itself tested") in its sharpest
observed form: not "main is untested in principle" but **two green PRs
composing into a red main, with no instrument anywhere that could see
the composition.** Recorded as evidence in
`work/ciw/f3-recosting-on-a-public-repo`, which is re-costing F3 now
that the repository is public and standard-runner minutes are free.

## Further instance 2026-09-04 (FILLET, PR 1733)

`test (interval, eps = 1e-12, 2/2)` on `fillet/ev-nocornerside` at
`1887cf54`: 2330/2331, the one red this row. The PR's code change is a
doc comment in `crates/profile/src/validate.rs`; merged over it,
annotated on the PR.

## Fixed (M10 orchestrator-direct hotfix), 2026-09-04

**The question this item held open, answered.** *Is the absence of
`measure_clearance_refused` and `measure_selection_kind` from every
`.pyi` and `tests/*.py` under `crates/pncad-py/` correct surface, or a
missing binding?* **Correct surface.** The six siblings of the family —
`measure_ref_resolve`, `measure_ref_unreadable`, `measure_unsupported`,
`measure_not_parallel`, `measure_non_finite`, `measure_malformed` —
appear in exactly two places in the whole tree, `src/tags.rs` and
`TAG_INVENTORY` in `src/tests.rs`, and in no `.pyi`, no `tests/*.py`,
no example, no docs table and no guide. The two new values were in one
of those two places, not zero, so the only missing surface was the
inventory row.

The reason the family stops there is written down, by M10-6 itself, in
`crates/pncad-py/tests/test_binding_census.py`: measure AUTHORING is a
declared census gap (`B-MEASURES`), and `MinClearanceRefusal` /
`MeasureUnavailableAt` are listed in it with the note that they "are
READING names — a caller dispatches on them after an evaluation, not
while authoring — but the read door that would surface them
(`Value.measure` on a `min_clearance`) cannot be reached until the
authoring half exists". A Python caller cannot author a measure node,
so it cannot make one refuse, so no Python test can observe either tag
on the wire. Binding them today would be surface with nothing behind
it.

**The edit.** Two lines in `crates/pncad-py/src/tests.rs`, both in the
`node_error_tag` row of `TAG_INVENTORY`, in the sort the table is kept
in: `measure_clearance_refused` after `loft`, `measure_selection_kind`
after `measure_ref_unreadable`. Nothing else moved — no receipt beyond
what the gate's own message asks for, and no binding, per the answer
above.

**LIB's routing was correct on territory; M10 is the active owner.**
`crates/pncad-py/*` is LIB's fence and the CIW orchestrator read that
correctly. Ev, 2026-09-04: LIB is not active, and the two tags are
M10's measure work — so M10 answers the question the item reserved for
it *and* makes the small edit that follows from the answer, rather than
handing a one-line change to a program that is not running.

**PR 1725 (M10-7) carries the same two inventory lines as its D11.**
Verified byte-identical rather than assumed: `crates/pncad-py/src/tests.rs`
is blob `90f0a0d96` on both this branch and `m10/m10-7-symbolic`, from the
same parent blob `e30ca0c7a`. That is one change made twice, so 1725
merges over this hotfix with nothing to resolve in the Rust.

**This tracker file will conflict, and that is expected.** 1725 leaves
the item at `work/lib/` and appends its own "M10's answer" section at
the end; this branch renames it to `work/m10/` and appends the section
above in the same place. Whichever lands second takes an add/add
conflict on the tail of the file. Resolve it by keeping this file at
`work/m10/` with `status: closed`, and folding in anything 1725's
section says that is not already here — it reaches the same answer from
a different direction ("no `.pyi` declares a constant for any other
`measure_*` tag") rather than from the B-MEASURES census gap. Nothing
in the Rust is at stake in that resolution.
