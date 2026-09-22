---
id: m5-pr12-refusals-header-claims-the-whole-vocabulary-and-names-seven-of-twenty-three
kind: issue
title: m5_pr12_refusals' header claims the OQ6 refusal vocabulary is 'pinned variant by variant'; sixteen of BlendError's twenty-three arms appear nowhere in the file
status: open
opened: 2026-09-15
---



## Finding

Found by TINT-6's class sweep while closing
`interrogate-ladder-header-claims-every-rung-and-pins-five`. Same
shape, one crate over. Accurate at `f8f8e648e`.

`crates/sweep/tests/m5_pr12_refusals.rs`'s module header opens
*"**The OQ6 refusal vocabulary, pinned variant by variant** (M5
PR 12 §3)"*. `BlendError` (`crates/sweep/src/blend/mod.rs`) has
**twenty-three** variants. **Seven** are named in the file; **sixteen**
are not named in it in any form.

The denominator, derived rather than read off:

```
git show origin/main:crates/sweep/src/blend/mod.rs \
  | awk '/^pub enum BlendError \{/{f=1;next} f&&/^\}/{exit} f' \
  | grep -cE '^    [A-Za-z_]'        # -> 23
```

and per arm, over `crates/sweep/tests/m5_pr12_refusals.rs`:
`grep -c "BlendError::<arm>\b"` (qualified) and `grep -c "\b<arm>\b"`
(the bare word anywhere). Both count LINES, not occurrences.

### The seven the file names

| arm | `BlendError::<arm>` | the bare word |
| --- | --- | --- |
| `FaceClearanceUncertified` | 3 | 3 |
| `TangentialEdge` | 2 | 2 |
| `SpineIrregular` | 1 | 1 |
| `UnsupportedCorner` | 6 | 7 |
| `SpineUnsupported` | 2 | 2 |
| `Escalated` | 5 | 5 |
| `RingClearance` | 2 | 2 |

### The sixteen it does not

Zero for both greps, every one of them. Beside each, how many OTHER
suites under `crates/*/tests/**` (`.rs` only) name it as
`BlendError::<arm>` — `grep -rl --include=*.rs "BlendError::<arm>\b"
crates/*/tests/ | wc -l`:

| arm | suites elsewhere |
| --- | --- |
| `Band` | 2 |
| `ChainNotConnected` | 2 |
| `RadiusHeadroom` | 6 |
| `ChainNotG1` | 12 |
| `ConvexitySignFlip` | 2 |
| `ChamferArmUnsupported` | 7 |
| `RepeatedEdge` | 8 |
| `NonpositiveSize` | 7 |
| `UnsupportedBody` | 5 |
| `UnsupportedChain` | 14 |
| `UnsupportedRunOut` | 11 |
| `UnsupportedGeometry` | 4 |
| `BodyNotIntact` | 2 |
| `SurgeryInvariant` | 1 |
| `Certify` | 3 |
| `Op` | 3 |

**There is no residue, and that is the whole disposition.** Every one
of the sixteen is named in at least one sibling suite (`SurgeryInvariant`
is the thinnest, at one). So `BlendError` is pinned across
`crates/sweep/tests/**`; what overclaims is the sentence saying THIS
file is where that happens, and it overclaims by sixteen arms.

**The header's claim is weaker than the interrogate row it was found
beside**, and a taker should read that first: the interrogate row was a
ladder nothing else drove, so narrowing its header left rungs genuinely
unpinned. Here nothing is unpinned. The repair is a sentence, not a
suite.

**What the header does get right**, and a taker should not undo: it
already states one arm's reachability honestly — *"`UnsupportedCorner`
has zero constructor surface: neither `RunOutPolicy` variant is ever
taken, both are only NAMED"*. That is the shape the rest of the
sentence is missing, not a thing to remove.

## What "the OQ6 refusal vocabulary" denotes is UNVERIFIED here

The finding above equates the header's subject with the whole of
`BlendError`, and this row does not establish that equation — it is the
reading that makes the sentence checkable, not one the source states.
What the source does say points narrower: `blend/mod.rs`'s scope
section speaks of *"the OQ6 payload vocabulary"* and then names two
arms (`BlendError::UnsupportedCorner` carrying a `CornerConfig`, and
`BlendError::SpineUnsupported`), `UnsupportedCorner`'s own doc calls it
*"**Predicate 6** and the OQ6 refusal vocabulary"*, and
`RunOutPolicy`'s doc calls THAT enum *"the run-out policy vocabulary
(OQ6…)"*. On that reading the header is a claim about a handful of
arms, and the file names all of them.

So a taker owes the reading before the repair: settle from M5 PR 12 §3
and `blend/mod.rs` what OQ6's vocabulary is, then either narrow the
sentence to it or narrow it to the seven arms the file pins. Under
either reading the sentence as written is wider than its evidence;
only the size of the gap is in question.

## Blind spot of this sweep

Whole-word grep over file text, and `grep -c` counts lines rather than
occurrences. An arm constructed through a helper that names it
elsewhere reads 0 here, and an arm named only in a doc comment reads as
present. The seven "named" rows were not each checked by eye for
construction-vs-mention — `RingClearance`'s two are `matches!` patterns
over a returned refusal, the rest were not inspected. The sixteen zeros
are absolute: those identifiers do not occur in the file in any form.

The "suites elsewhere" column is the same grep one directory wider and
inherits the same blind spot; it also counts a suite that merely
mentions the arm the same as one that drives it.

## Fence

`crates/sweep/tests/*` is S-TINT's and S-TCOST's shared glob
(`scripts/work.py territory` reports a double claim, not a crossing).
Filed here rather than on S-TCOST for the same reason the interrogate
row was: the defect is an enumeration nobody updated, not a cost
lever.

## What a taker owes

The reading (above), then the header narrowed to what the file pins
with each exclusion's reason measured — and, if narrowed, the note that
the narrowed sentence is still unwelded prose. A row per absent arm is
NOT owed here: all sixteen are pinned elsewhere.

TINT-6's `the_reachable_ladder_is_driven_through_its_doors`
(`crates/editor-core/tests/lib_u5_interrogate.rs`) is the worked shape
for the harder case: one row driving each arm through a door, welded to
the enum by `test_utils::f6_variants!`, with the undriven arms
constructed rather than named in strings and the measurement that
excludes them re-taken on every run.
