---
id: viewer-tests-spell-one-display-tolerance-in-seven-places
kind: issue
title: One display tolerance value written out in seven places in crates/viewer/tests
status: closed
opened: 2026-09-20
closed: 2026-09-20
branch: dup/viewer-shared-doors
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

## Closed 2026-09-20 — re-taken at `cd9fdfd6b`, folded onto `common::{pick_delta, corpus_delta}`

### The census, re-taken

`git grep -n 'DisplayTolerance::new(2.0e-4)'` over every tracked file,
no path argument: **seven sites in six files, exactly the seven the row
names.** This is the first row on this program whose count did not move
under re-taking.

`2e-4`, `0.0002` and an arithmetic spelling of the same value: the row
named those as its blind spot. Run as a **value-keyed census** — every
`DisplayTolerance::new(` in every tracked file, bucketed by argument —
none exists. What the second instrument DID find is the class the row
said it could not see, *"any other value shared by two suites"*:

| value | sites | disposition |
| --- | --- | --- |
| 2.0e-4 | 7 in 6 files | folded → `common::pick_delta` |
| **2.0e-3** | **6 in 6 files** | four folded → `common::corpus_delta` (`index_memo`, `pick3_acceptance`, `review_pick2_r1`, `review_pick_r2` — the corpus pick suites, and `index_memo`'s own prose gives the shared reason: the corpus holds million-triangle documents at the application's δ). The other two are `review_gui2_r1`'s `ring_delta` and `review_gui2_r2`'s `coarse`, one value for one fixture in two promoted review suites, **left** and filed as `viewer-gui2-suites-spell-the-gallery-ring-delta-twice` |
| 1.0e-3 | `common/asm.rs` (the assembly's own door) + `pick_windows` + two in `review_gui2_r2` | **left**: different fixtures at different scales, not one value with one meaning |
| 1.5e-4, 3.0e-4, 5.0e-4, 1.0e-5, 5e-3, 1.0e-4 | one each | **left**: singletons, nothing to fold |

`eval_seam`'s `coarse` local — the row's sharpest observation, a name
that means something ten times coarser one file over — is gone: the
site now says `common::pick_delta()` at its one use and the misleading
binding is deleted.

### The proof: this class has NO live probe, in either direction

Baseline **626 passed / 0 failed / 1 ignored**. Direction argued before
each result was read.

| plant in `common::pick_delta` | direction | total |
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
