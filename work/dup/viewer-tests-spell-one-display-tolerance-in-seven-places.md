---
id: viewer-tests-spell-one-display-tolerance-in-seven-places
kind: issue
title: One display tolerance value written out in seven places in crates/viewer/tests
status: closed
opened: 2026-09-20
priority: P4
cost: E
closed: 2026-09-20
branch: dup/viewer-shared-doors
pr: 2929
---


## Finding

- **Where**: `DisplayTolerance::new(2.0e-4).expect("a positive delta")`
  — one value, **seven** sites in six files under
  `crates/viewer/tests/`: `blend_authoring.rs:89`, `debug_dumps.rs:28`,
  `edge_pick.rs:50`, `eval_seam.rs:550`, `eval_seam.rs:813`,
  `frame_policy.rs:53`, `select_pick.rs:73`. Six of the seven carry the
  **identical** `expect` string; two spell the type path-qualified as
  `scene::DisplayTolerance`. `eval_seam.rs:813` binds it to a local
  named `coarse`, which is the same number under a name that elsewhere
  in this crate means something ten times coarser.
- **Why it is a duplicate and the per-suite `delta()` helpers are
  not**: these seven are one CONSTANT, agreeing to the digit, in
  suites that nothing distinguishes on this axis. The review suites'
  own tolerances differ on purpose — `review_gui2_r1` 1.5e-4,
  `review_gui2_r2` 3.0e-4 — and stay. The unit that measured this
  looked at the per-suite `delta()` helpers, judged them independent,
  and did not look at the constant inside them.
- **Importance**: medium. A display tolerance decides how finely a
  fixture tessellates, so it is an input to every pick these suites
  make; seven copies agreeing today is seven places to change and six
  chances to miss one.
- **Confidence**: sure about the sites and the value.
- **Instrument, and its blind spot**: `git grep -n
  'DisplayTolerance::new(2.0e-4)'` over every tracked file, no path
  argument. It is literal and line-shaped: it misses the same value
  written `2e-4`, `0.0002` or as an arithmetic expression, and any
  other value shared by two suites. The class is therefore a floor.
- **Raised by**: the S-DUP lane's fix pass on
  `viewer-review-suite-fixtures-have-no-oracle-role`, 2026-09-20,
  measured at `ab086f8c1`. The review that prompted it named five
  sites in five files; re-taking found seven in six.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), so there is no single ground-owner,
and one constant spelled seven times is S-DUP's charter. Any of the
five may claim it by `git mv`.

## Closed 2026-09-20 — re-taken at `cd9fdfd6b`, folded onto `common::{plate_delta, corpus_delta}`

### The census, re-taken

`git grep -n 'DisplayTolerance::new(2.0e-4)'` over every tracked file,
no path argument: **seven sites in six files, exactly the seven the row
names.** The row's own count did not move under re-taking — the first
on this program that did not. **Its SECOND instrument's table did**:
three cells below were wrong when first published (1.0e-3 given as 4
against 9, 1.0e-4 called a singleton against 2, and the 1.0e-6 pair
absent), caught in review. Every disposition was and is "left", so
nothing folded wrongly; what the correction costs is the claim that
this row's numbers held. Method item 18 — the census run to show a
census can be trusted is still a census.

`2e-4`, `0.0002` and an arithmetic spelling of the same value: the row
named those as its blind spot. Run as a **value-keyed census** — every
`DisplayTolerance::new(` in every tracked file, bucketed by argument —
none exists. What the second instrument DID find is the class the row
said it could not see, *"any other value shared by two suites"*:

| value | sites | disposition |
| --- | --- | --- |
| 2.0e-4 | 7 in 6 files | folded → `common::plate_delta` |
| **2.0e-3** | **6 in 6 files** | four folded → `common::corpus_delta` (`index_memo`, `pick3_acceptance`, `review_pick2_r1`, `review_pick_r2` — the corpus pick suites, and `index_memo`'s own prose gives the shared reason: the corpus holds million-triangle documents at the application's δ). The other two are `review_gui2_r1`'s `ring_delta` and `review_gui2_r2`'s `coarse`, one value for one fixture in two promoted review suites: first **left** and filed, then **folded** in the merge pass onto `common::ring_delta`, since it is two sites, carries no oracle, and was small enough to be a commit rather than a row. The parent's *kept* verdict on `coarse` was about it against its neighbour `delta()`, and still holds: that pair stays two values |
| 1.0e-3 | **9** — `common/asm.rs`, `pick_windows`, `review_gui2_r2` ×2, `frame_policy` ×3, `examples/r1_gallery_probe.rs` ×2 | **left**: different fixtures at different scales, not one value with one meaning. `frame_policy`'s three are `FittedDelta` operands, not a δ a suite indexes at |
| 1.0e-6 | 2, both `frame_policy` `FittedDelta` operands | **left**, same reason |
| 1.0e-4 | 2 — `review_gui0_r2` and `src/gpu.rs` | **left**: one test site and one shipped site |
| 1.5e-4, 3.0e-4, 5.0e-4, 1.0e-5, 5e-3 | one each | **left**: singletons, nothing to fold |

`eval_seam`'s `coarse` local — the row's sharpest observation, a name
that means something ten times coarser one file over — is gone: the
site now says `common::plate_delta()` at its one use and the misleading
binding is deleted.

### The proof: this class has NO live probe, in either direction

Baseline **626 passed / 0 failed / 1 ignored**. Direction argued before
each result was read.

| plant in `common::plate_delta` | direction | total |
| --- | --- | --- |
| ×50, 2×10⁻⁴ → 1×10⁻² | **coarsen** — harder for "this ray meets the hole's rim" | 626 / 0 |
| ×0.1, 2×10⁻⁴ → 2×10⁻⁵ | **refine** — harder only for a row keyed on the value | 626 / 0 |
| ×5000, 2×10⁻⁴ → **1.0 m**, wider than the plate | **coarsen, maximal** | 626 / 0 |

| plant in `common::corpus_delta` | direction | total | reds |
| --- | --- | --- | --- |
| ×25, 2×10⁻³ → 5×10⁻² | **coarsen** | 622 / 4 | `index_memo` 2, `pick3_acceptance` 1, `review_pick_r2` 1 |

Every row sums to 626. So the corpus δ is live and **the plate δ is
not**: three plants, two directions, three orders of magnitude, and
nothing in the crate can see it move — while three of the seven folded
sites carried prose asserting a property OF that value (*"fine enough
that the hole is a ring of facets rather than a polygon that misses the
ray"*). The fold is therefore compiler-proved, not plant-proved, and
the coverage half is not this program's: filed as
`work/tint/viewer-plate-suites-index-at-a-display-tolerance-nothing-asserts`.

### The fix pass

Three things this row's own subject caught in it.

- **The shared door's rustdoc carried the sentence the measurement had
  just falsified.** *"Finer is what makes the plate's through hole
  tessellate as a ring of facets rather than a polygon a ray can pass
  straight through"* was deleted from three private sites and then
  written into `common`, by the unit that had proved with a 1.0 m plant
  that nothing can see it. `corpus_delta` carried a measurement claim in
  the same position. Both now state only what is true of the value —
  which way the two costs pull — and the measurement lives here, dated,
  with its instruments named (method item 13).
- **The name made a claim two of its six callers falsify.**
  `pick_delta` was renamed `plate_delta`: `eval_seam` hands it to a FIT
  request and `debug_dumps` to a pick-cache sync, neither of which is a
  pick, while every caller IS a plate-scale suite.
- **Four `fn delta() { common::corpus_delta() }` one-liners were
  minted and are gone** — see the pick-index row's fix-pass note.

### The divergent control — not restated here

A `panic!` planted in `plate_delta`'s body gives **551 / 75**, which
separates *called and unasserted* from *never called* and is what makes
the three green rows above safe to read.

**The argument and its per-suite figures live in one place**,
`work/tint/viewer-plate-suites-index-at-a-display-tolerance-nothing-asserts`.
They were written out here as well, and in PR #2929, and within that one
commit the three copies drifted: one of them said two suites were
unprobed that the control's own table shows red. That row records what
that cost; this row points at it.

### After merging main

- **`fd5620e84` deleted two of the folded sites.** It removed
  `frame_policy`'s dying-worker block whole, and two
  `common::plate_delta()` calls this PR had written went with it.
  `plate_delta`'s callers are still the same six suites; its site count
  inside `frame_policy` is lower by those two. The divergent control
  was taken before the merge, and its per-suite figures in the S-TINT
  row are for that tree.
- **`crates/viewer/src/marks.rs` gained a `2.0e-4`**, in a `#[cfg(test)]`
  module that cannot reach `tests/common`. Left; see the pick-index row.
- Re-taken on the merged tree, `git grep -n
  'DisplayTolerance::new(2\.0e-[34])' -- '*.rs'` returns the three
  `common` doors and `marks.rs`'s one, and nothing else.
