---
id: m5-pr12-refusals-header-claims-the-whole-vocabulary-and-pins-six-of-eleven
kind: issue
title: m5_pr12_refusals' header claims the OQ6 refusal vocabulary 'pinned variant by variant'; five of BlendError's eleven arms appear nowhere in the file
status: open
opened: 2026-09-15
---



## Finding

Found by TINT-6's class sweep while closing
`interrogate-ladder-header-claims-every-rung-and-pins-five`. Same
shape, one crate over. Accurate at `6e8dc1931`.

`crates/sweep/tests/m5_pr12_refusals.rs`'s module header opens
*"**The OQ6 refusal vocabulary, pinned variant by variant** (M5
PR 12 §3)"*. `BlendError` (`crates/sweep/src/blend/mod.rs`) has
eleven variants. Six are named in the file; five are not named in it
in any form.

| arm | `BlendError::<arm>` in the file | the bare word in the file |
| --- | --- | --- |
| `FaceClearanceUncertified` | 3 | — |
| `TangentialEdge` | 2 | — |
| `SpineIrregular` | 1 | — |
| `UnsupportedCorner` | 6 | — |
| `SpineUnsupported` | 2 | — |
| `Escalated` | 5 | — |
| `Band` | **0** | **0** |
| `ChainNotConnected` | **0** | **0** |
| `RadiusHeadroom` | **0** | **0** |
| `ConvexitySignFlip` | **0** | **0** |
| `ChamferArmUnsupported` | **0** | **0** |

Derived by `grep -c "BlendError::<arm>\b"` and `grep -c "\b<arm>\b"`
per arm over the file; the variant list is read off `pub enum
BlendError`.

**Weaker than the interrogate row it was found beside**, and the
difference is what a taker should read first: four of the five absent
arms ARE pinned elsewhere in `crates/sweep/tests/` —
`ChainNotConnected` (`review_d2_adv_probes`,
`review_blend6_r1_probes`), `RadiusHeadroom` (six suites, including
`m5_pr12_battery` — this file's own sibling), `ConvexitySignFlip`
(`review_d2_adv_probes`, `review_blend6_r1_probes`) and
`ChamferArmUnsupported` (seven suites). So the vocabulary is largely
pinned; what overclaims is the sentence saying THIS file is where that
happens. `Band` is the one arm the grep found nowhere under
`crates/*/tests/` as `BlendError::Band`, and whether it is reachable
at all is unmeasured here.

**What the header does get right**, and a taker should not undo: it
already states one arm's reachability honestly — *"`UnsupportedCorner`
has zero constructor surface: neither `RunOutPolicy` variant is ever
taken, both are only NAMED"*. That is the shape the rest of the
sentence is missing, not a thing to remove.

**Blind spot of this sweep.** Whole-word grep over file text. An arm
constructed through a helper that names it elsewhere reads 0 here, and
an arm named only in a doc comment reads as present. The six "yes"
rows were not each checked by eye for construction-vs-mention; the
five zeros are absolute (the identifiers do not occur in the file in
any form).

## Fence

`crates/sweep/tests/*` is S-TINT's and S-TCOST's shared glob
(`scripts/work.py territory` reports a double claim, not a crossing).
Filed here rather than on S-TCOST for the same reason the interrogate
row was: the defect is an enumeration nobody updated, not a cost
lever.

## What a taker owes

Either a row per absent arm, or the header narrowed to what the file
pins with each exclusion's reason measured — and, if narrowed, the
note that the narrowed sentence is still unwelded prose. TINT-6's
`the_reachable_ladder_is_driven_through_its_doors`
(`crates/editor-core/tests/lib_u5_interrogate.rs`) is the worked shape:
one row driving each arm through a door, welded to the enum by
`test_utils::f6_variants!`, with the undriven arms constructed rather
than named in strings.
