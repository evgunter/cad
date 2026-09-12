---
id: unexplained-literal-seeds-do-not-say-which-shape-they-are
kind: issue
title: Fourteen test-side PRNGs take an unexplained literal seed: none says in-file which shape licenses it
status: open
opened: 2026-09-12
---


Filed 2026-09-12 by the S-TCOST orchestrator, out of the sweep that
S-TCOST's `r1-probe-seeds-are-not-on-the-fuzz-dial` unit ran (PR 2433).
That unit's own defect was the opposite one — three rows seeded from the
CLOCK under a private door `CAD_FUZZ_SEED` could not reach — and its
sweep turned these up beside them.

## The rule this lands on

`memories/test-suite-cost.md`, §*When a fixed seed IS right*, in as many
words: **"each case must say in-file which one it is; an unexplained
literal seed is the failure mode."** The memory is not against fixed
seeds — two of its three shapes want one. It is against a literal that
does not say which shape it is serving, because the same four hex digits
are correct for a witness you cannot write down and a defect for a
counterexample search.

The failure it hides is the one this program exists for: **a shape-1
counterexample search pinned to a literal re-searches one fixed sample
forever.** It goes green every run, for the same reason every run, and
no amount of it is evidence about anything but that sample. That is a
row that cannot fail in the way its name claims it can.

## The sites, re-derived rather than inherited

Swept by the orchestrator at the merge of PR 2433's base, pattern
`(Lcg|Rng|Xorshift|SplitMix|Prng)(::new)?\(\s*0x[0-9A-Fa-f_]+` over
`crates/ --include=*.rs`, less `test-utils/src/fuzz.rs`. **Fourteen
seeding sites in seven files:**

| file | sites |
|---|---|
| `crates/geom-core/tests/props1_review_rows.rs` | 5 |
| `crates/geom/tests/curves/n1r2_fixtures/mod.rs` | 3 |
| `crates/mesh/src/planar.rs` | 2 |
| `crates/geom-core/src/linalg/affine.rs` | 1 |
| `crates/geom-core/tests/r1_p2_onb_probes.rs` | 1 |
| `crates/geom-brep/tests/offb_r2_probes.rs` | 1 |
| `crates/geom-brep/tests/cert10r2_probes.rs` | 1 |

The count is 14 sites and not 14 rows; one file's five literals are five
different rows and want five separate judgements, not one.

**Two sites were EXCLUDED after checking, and the reason generalises:**
`crates/editor-core/tests/m10_6_reports_interval.rs` and
`crates/editor-core/tests/r2_m10_6_probes_interval.rs` seed from
`editor_core::mc::DEFAULT_SEED` — the library's own public Monte Carlo
stream constant. Those are differentials against a documented stream, so
the seed is a fixture identifier and the code says so by naming the
constant instead of writing the literal. **That is what compliance looks
like here**, and it is the model for the fix: not "vary every seed", but
"say what the literal is for".

## What this row asks for

Per site, a judgement and one line in the file, not a sweep-wide policy:

1. **Shape 1 (counterexample search)** — the literal is the defect. Route
   through `test_utils::fuzz` so the row draws per run and is pinnable by
   `CAD_FUZZ_SEED`, exactly as PR 2433 did for its three.
2. **Shape 2 (a witness you can write down)** — do not search at all;
   the fixture goes in as a static.
3. **Shape 3 (a witness you cannot write down)** — the literal is
   RIGHT and stays, and the row says in-file that it is a fixture
   identifier and what class it is reaching. Ev's condition applies: K
   large enough that the row is very unlikely to pass by accident on a
   lucky seed.

## Why this is S-TINT's and not S-TCOST's

`work/tint/plan.md` §The fence with S-TCOST: *"a row justified by a claim
that cannot fail... is this program's"*, and *"a row justified by a
second — cpu or wall — is S-TCOST's"*. **Nothing here is measured and
nothing here is a cost argument** — the rows are cheap and the complaint
is that a green one is not evidence. The fence's carve-out that keeps the
fuzz-gating policy question with S-TCOST
(`r1-probe-seeds-are-not-on-the-fuzz-dial`,
`proptest-modules-in-src-ungated`) does not reach this: those rows are
about whether a randomized suite RUNS and whether the harness can pin it,
and every seed here is already deterministic and already reachable. The
question is whether a determinism is licensed, which is a different one.

## Two hazards for whoever takes it

- **Three of the fourteen are in `crates/*/src`**, not in `tests/`:
  `mesh/src/planar.rs` (2) and `geom-core/src/linalg/affine.rs` (1), in
  `#[cfg(test)]` modules. S-TINT's own `keep_out` binds — *"a fix that
  has to land in crates/*/src... is announced to the program that owns
  that crate before it lands"*.
- **A shape-1 row converted here becomes a randomized row**, and
  `memories/test-suite-cost.md`'s other rule then attaches to it: a
  fuzzer that is not gated is a defect in the fuzzer. The gate mechanism
  (`gated_to!`, `ci-filter.py --gated-check`) stays S-TCOST's by the
  fence, so a conversion that needs a marker is announced rather than
  invented.

## What the sweep could not match

One pattern, one shape: a generator constructed with a hex literal at the
call site. It cannot see a seed written in decimal, one built from a
`const` or a `let` a line earlier, one passed in as a function argument,
or a third-party generator (`proptest`'s own case generation is neither
read nor judged here). It was run over `crates/` only — the four
`--workspace`-excluded roots (`demos/`, `tools/`, `benches/`,
`interval-transcendentals/`) are unswept for this shape. And it is
accurate as of PR 2433's base, not as of whenever this row is taken up.
