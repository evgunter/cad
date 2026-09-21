---
id: loud-skip-marker-text-is-unchecked-against-its-own-file
kind: issue
title: Nothing checks a loud-skip marker's text against its own file; three markers shipped saying their file's rows are the gated ones when most were not
status: open
opened: 2026-09-15
priority: P4
cost: E
---



Filed by S-TINT unit TINT-2, at its style reviewer's prompting: the
reviewer observed that a cheap mechanical guard exists and that the
unit's PR enumerated only the two expensive options. `scripts/check-*.py`
is this program's (`work/ciw/program.md` `paths`), so the guard is filed
rather than built. **Routing note:** the orchestrator's brief sent this
to S-TCOST; `python3 scripts/work.py territory --files -` puts
`scripts/check-*.py` here, and S-TCOST holds only `ci-filter.py`,
`slowest-tests.py` and `base-test-listing.sh`. If the gate is instead
spelled under `scripts/gates/`, it is code-quality Track K's.

## The evidence, which is not hypothetical

TINT-2 rewrote the ten `#[cfg]`-gated `*_lane_skipped_*` marker bodies
to stop hand-keeping a list of the rows that did not compile. The
replacement wording — *"its rows are the `#[cfg(feature = "…")]` block
below"* — was written against the five `crates/sweep/tests/*_interval.rs`
binaries, which genuinely contain nothing else, and pasted into files
that differ. Three of the nine shipped false in the unit's own first
push, and were caught by a human reading them, not by any gate. Counting
`#[test]` before each file's first gated block (the marker itself is one
of them):

| file | `#[test]` before the gated block | claim was |
| --- | --- | --- |
| the five `crates/sweep/tests/*_interval.rs` | 1 (the marker) | true |
| `crates/viewer/tests/chrome_labels.rs` | 1 (the marker) | true |
| `crates/topo/tests/m6_2_fitted_at_rest.rs` | 4 | **false** — 3 rows run at default features |
| `crates/viewer/tests/error_display.rs` | 16 | **false** — 15 rows run at default features |
| `crates/viewer/tests/panel_display.rs` | 17 | **false** — 16 rows run at default features |

Reproduce with:

```
grep -n '^\s*#\[test\]' <file> | awk -F: -v b=$(grep -n '^#\[cfg(feature' <file> | head -1 | cut -d: -f1) '$1<b'
```

The unit's repair was to put the marker behind
`test_utils::vacuity::loud_skip_marker!`, whose text is now uniform and
true of any file. **That removes today's instances and does not add a
guard**: the next hand-written marker, or a second argument to the macro
that drifts, fails exactly as silently.

## The guard the reviewer described

A `scripts/check-*.py` row, in the shape of the neighbouring
`scripts/check-interval-cfg-additive.py`: for every `*_lane_skipped_*`
marker row or `loud_skip_marker!` invocation, assert that **the feature
token it names occurs in a `#[cfg(feature = "…")]` elsewhere in the same
file**. Cheap, purely textual, no build, and it would have gone red
today on two things at once:

1. a marker naming a feature its file does not gate anything behind —
   the rename hazard, where `git mv` or a feature rename leaves the
   marker pointing at nothing;
2. the class above, if the assertion is extended to compare the marker's
   claim about WHAT is gated against the block it points at.

(1) alone is worth more than it costs. (2) is the harder half and may not
be worth it — a marker's `absent` string is prose about a subject, and a
gate that tried to parse it would be the hand-kept enumeration one level
up, which is the defect TINT-2 was closing. **Take (1); leave (2)
un-taken deliberately and say so**, rather than shipping a checker that
appears to cover both.

## Why it belongs in the per-PR gate

Its inputs are `crates/*/tests/*.rs` and `crates/test-utils/src/*`, which
are exactly the change set the per-PR gate already runs on, so
`docs/prompts/implementer-discipline.md`'s rule about a row demoted to
the nightly applies in advance: there is no reason for this one to fire
anywhere but on the PR that breaks it.

Related: `work/ciw/gating-nextest-jobs-discard-every-passing-tests-stdout`
(why these bodies have no reader at all on the gate) and
`work/tint/loud-skip-marker-is-a-hand-kept-idiom` (the class).
