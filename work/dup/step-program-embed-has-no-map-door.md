---
id: step-program-embed-has-no-map-door
kind: issue
title: The recorded Step program is walked through a scalar map by hand seven times in six test files; profile has no Step::map_scalar
status: closed
opened: 2026-09-24
priority: P4
cost: D
closed: 2026-09-26
branch: dup/b6-c
---


## Finding

Filed by the `dup/scalar-lift-home` lane. Folding the `Point2` lifts
onto `Point2::map` put this lane inside **seven hand-written copies of
one walk**: a recorded `profile` program `Step<f64>` re-expressed
through a scalar map (six to a lane scalar, one a rescale at `f64`),
arm by arm — `Target::Point(p) => Target::Point(pt(p))`,
`ArcData::Center { c, winding, target } => …`, the radius through
`T::from_f64` — with `pt` the only piece that had a door.

Measured at the lane's fold head, two instruments over every tracked
file with no path argument:

- the arm every copy must contain,
  `git grep -n 'Target::StartArriving => Target::StartArriving'`:
  **7 hits in 6 files** — `crates/editor-core/tests/cert3r1_dump.rs`,
  `crates/editor-core/tests/m10_p_fence.rs`,
  `crates/profile/tests/cert4r1_e2e.rs` (`embed_step`),
  `crates/profile/tests/generic_replay.rs` ×2 (`embed_step`, and
  `scaled` further down the file — the same walk with `f` a SCALE,
  `f64` to `f64`, which the same door would serve as
  `map_scalar(|c| c * s)`),
  `crates/profile/tests/guided_replay.rs` (`embed`),
  `crates/profile/tests/review_fillet_stored_tangency_r1_probes.rs`
  (`embed`);
- signatures, `(Step|Target|ArcData)<f64>` `->` a `Step`/`Target`/
  `ArcData` at another scalar: the same six files, nothing else.

The copies are not identical: `cert4r1_e2e` and `generic_replay`'s
`embed_step` cover every `Step` variant, and the other five cover only
the variants their fixture records and `panic!`/`unreachable!` on the
rest. So the copies have already DRIFTED in coverage. A variant added
to `Step` fails to compile in the two exhaustive copies; the other five
compile and reach their catch-all arm only if a fixture records the new
variant.

**Band: P4.** All seven copies are tests. No `src` code builds on them,
and nothing in `src` walks a `Step` through a scalar map that a
`Step::map_scalar` door would serve. Two copies already catch a new
variant. That makes this non-architectural code improvement under
`work/README.md`'s bands, not P1.

**Blind spot.** A copy that never names `Target::StartArriving` (one
that embeds only `Step`s with no target) is invisible to the first
instrument; the second catches it only when the copy is a named
function or a typed closure.

## The shape of a home

`Step`, `Target` and `ArcData` are declared in
`crates/profile/src/path/program.rs` (one macro), which has no `map`.
By `geom`'s `scalar_lift` convention (`map` on a leaf, `map_scalar` on a
type with structure to carry) the door would be `Step::map_scalar`,
exhaustive over the variants, with `Point2::map` at the leaves. That
file is `paths`' ground; the row sits here because the copies are this
program's class and were measured by this program's unit. `paths` may
claim it by `git mv`.


## Closed (2026-09-26, PR: batch 6)

**Re-measured at the merge base `032999ff2`**, over every tracked file
with no path argument, with four instruments:

1. The row's arm, `git grep -n 'Target::StartArriving =>
   Target::StartArriving'`: **7 hits in 6 files**, as the row said.
2. The row's signature instrument: the same six files.
3. **The scalar-lift atom**, which does not need the copy to carry a
   target: `git grep -n -E 'from_f64\((radius|r|b|theta|len|angle|delta|phase|dx)\)'`.
   Its only `Step` hits are in the six files. The others are geometry
   lifts in `geom*`, plus `profile/tests/bool9r1_probes.rs`'s two
   `ProfileLoop` walks. Those two are oracles kept verbatim against
   `ProfileLoop::map_scalar`, so they are not members.
4. **A lane-typed step**, `git grep -n -E 'Step<(Interval|Dual64|T)>'`
   over `*/tests/*`, `demos/`, `tools/` and `benches/`: the same six
   files.

In `src`, nothing is a member. `editor-core/src/eval/mod.rs`'s two
`Step` walks are digest FOLDS, not maps. `editor-core/src/program.rs`'s
`res_step` resolves an Expr-bearing `ProgramStep`, not a `Step<f64>`.
`viewer/src/sketch.rs` builds default steps from a `Verb`.

**It is one construction.** Each of the six embeds sends every scalar
field through one function (`T::from_f64`, or `Interval::from_f64`) and
carries every structural field (target form, winding, side, split
count) verbatim. Two of the six were exhaustive (`cert4r1_e2e`'s and
`generic_replay`'s `embed_step`). The other four differ only in
COVERAGE: they `panic!` or `unreachable!` on variants their fixture does
not record. (The row's "other five" counted `scaled` as well; see
below.)
So the door is `profile::Step::map_scalar`
(`crates/profile/src/path/program.rs`, after the transition table). It
takes `&self`, like every other `map_scalar` in `profile` and `geom`.
It is exhaustive over the verbs, the arc modes and the target forms,
with no wildcard arm (a code comment says so; the rustdoc does not),
and `Point2::map` sits at the leaves. Its two rungs,
`Target::map_scalar` and `ArcData::map_scalar`, also take `&self` and
are `pub(crate)`, because nothing outside the ladder lifts a bare
target or spec.

**Routed through it — all six embeds:** `cert4r1_e2e.rs` and
`generic_replay.rs` (`embed_step`), `guided_replay.rs` (`embed`),
`review_fillet_stored_tangency_r1_probes.rs` (`pt`/`tgt`/`spec`/`embed`,
two call sites), and `editor-core/tests/{cert3r1_dump,m10_p_fence}.rs`
(the `embed` closure). The four partial copies are now total: a
fixture that records another verb embeds rather than panics. The
plants below show which rows read the door, and none of them relied on
those panics. The exhaustiveness argument `generic_replay`'s
`embed_step` made for itself now belongs to the door.

**Folded with it: `try_replay_at`.** It was byte-identical in
`cert4r1_e2e.rs` and `generic_replay.rs`, which share one test binary,
and it is now `profile/tests/common/mod.rs`'s, imported by both.

**Not given a door: the lift of a whole program.** At the head it is
six sites: `common::try_replay_at`, `guided_replay.rs`, the r1 probes
×2, and `editor-core/tests/{cert3r1_dump,m10_p_fence}.rs`. Each is
`program.iter().map(|s| s.map_scalar(f)).collect()`. That is one
iterator adaptor over the door, not a construction, so a second door
would only rename it.

**Filed, not folded:** the two `editor-core` files also build a
byte-identical `programs` fixture list. That is
`work/dup/the-arc-carrier-fence-fixture-is-written-twice-in-editor-core-tests.md`,
which waits on the tint row that may retire `cert3r1_dump`.
`m10_p_fence.rs`'s claim that the file *"travels to a pre-lift tree"*
was made false by routing it through the new door. It now says that on
a tree older than `Step::map_scalar` the embed is written out by hand.

**Divergent member, kept: `generic_replay.rs`'s `scaled`.** The row
said the door would serve it as `map_scalar(|c| c * s)`. It would not.
`scaled` is a similarity SCALE: it moves lengths, and leaves angles and
bulges alone. `map_scalar` sends every scalar through `f` alike. The
two agree only on the fixture `scaled` sees (a Center-mode
`ArcFilletArc`, all lengths), and routing it through the door would
silently mis-scale the first Bulge, Sweep or `Angle` step a later
fixture adds. It keeps its walk, and its refusal of the variants it has
not sorted. Its `pt` (`generic_replay.rs`, ~:348,
`Point2::new(p.x * s, p.y * s)`) is a scale, not a lift. Its doc now says why, and the door's rustdoc says that a
scale is not spelled with it.

**Plants** (profile: `cargo nextest run -p profile --no-fail-fast`,
baseline 530 run / 530 passed / 2 skipped). Each plant was applied and
restored by an exact-string patch pair, and `git diff --quiet HEAD` was
checked on the file after each restore.

| plant in `Step::map_scalar` | direction | red |
| --- | --- | --- |
| `panic!` on entry | reach | 9 / 530: `cert4r1_e2e` ×3, `generic_replay` ×4, `guided_replay` ×1, the r1 probes' report row ×1 |
| `Center`'s `c` with x and y swapped | a different answer | 7: the reach set minus `cert4r1_the_enclosure_width_scales_with_the_profile` (a relative width predicate) and the report row (prints, asserts nothing) |
| `Target::Point` with x and y swapped | a different answer | 7, the same set |
| `Toward`'s `dx`/`dy` swapped | a different answer | 3: `generic_replay`'s Dual bit-identity, Interval containment and escalation rows |
| `Line`'s length doubled | a different answer | 3, the same `generic_replay` rows |

Each row sums to 530. The door's answer is asserted by `generic_replay`
at every verb the coverage corpus replays, and by `cert4r1_e2e` and
`guided_replay` at the fused Center-mode step.

In `editor-core`, the same patch pair was run over the two routed files
(`cargo nextest run -p editor-core -E
'test(/^(m10_p_fence|cert3r1_dump)::/)'`, 4 rows). Under `panic!` on
entry, **4/4 red**, each on the plant's own message. Under the swapped
`Center` centre, **2/4 red**: `m10_p_fence`'s bit fences at `f64` and at
`Interval`. The two `cert3r1_dump` rows print and assert nothing, so
they are reached and unasserted by design; that is filed as
`work/tint/cert3r1-dump-is-a-print-only-replica-of-the-m10-p-fence.md`.
Unplanted, both `m10_p_fence` fences pass, which pins the door
bit-identical to the hand walk it replaced at that fixture.

**Re-planted after the fix pass** (`&self` receivers, `try_replay_at`
homed). All five profile plants gave the same red sets as above (9, 7,
7, 3, 3 of 530, each summing to 530). The editor-core pair gave 4/4 on
`panic!` and 2/4 on the swapped centre, as before. Two plants in the
homed `common::try_replay_at`:
- `panic!` on entry reds 7 of 530: every `cert4r1_e2e` and
  `generic_replay` row that calls it.
- Lifting through `|c| T::from_f64(c * 2.0)` (a different answer)
  reds 3: `generic_replay`'s Dual bit-identity, Interval containment
  and escalation rows. The `cert4r1_e2e` rows stay green. Their one
  fixture is a Center-mode step whose every scalar is a length, so
  doubling all of them is a uniform scale. That is the likely reason,
  but it was not traced row by row.

