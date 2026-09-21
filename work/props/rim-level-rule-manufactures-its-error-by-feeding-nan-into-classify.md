---
id: rim-level-rule-manufactures-its-error-by-feeding-nan-into-classify
kind: issue
title: The rim-level rule's structurally-impossible arm throws by feeding f64::NAN into classify, and unreachable_zero returns a 4-tuple of NaNs into live flux arithmetic
status: review
opened: 2026-09-11
refs: [877, S40]
priority: P0
cost: H
branch: props/curved-residues
pr: 2924
---


## Carved out of `S40` (2026-09-11, by the WIRE orchestrator)

`S40` ("Residue and editing artifacts", accepted by Ev 2026-08-18)
was one file carrying four unrelated residues on three programs'
territory. WIRE inherited it whole in the cut of 2026-09-11 and owns
only one of the four; the rest are filed where the code lives, per
`work/README.md` ("when the owning program is clear, file the item
straight onto that program's slate"). `S40` keeps its id and its
`WitnessSlot` row; this is the second of its four bullets, verbatim
below. **Nothing about it was re-judged in the move** — the citations
are `S40`'s, written against a tree `#877` has since moved, and the row
was last read on 2026-08-18.

## Finding (`S40` bullet 2, verbatim)

- **Confidence**: sure

The rim-level rule's structurally-impossible arm manufactures its error
by feeding `f64::NAN` into `classify` and letting the funnel escalate —
a decision predicate used as a `throw`; `unreachable_zero` returns a
4-tuple of NaNs into live flux arithmetic (`props/curved.rs`,
`mixed_levels` and `unreachable_zero` — cited by target name per
**S176(a)**; **`same_level` no longer exists**, the two rim-level
spellings having been unified into `level_coincides` by **#877 / S81**,
and this bullet named it). **STILL OPEN** — the idiom survived the
unification unchanged, it is now at one site instead of two, and it is
D2 (bug-vs-invalid-state) territory.

## Why it is PROPS's

`crates/geom-brep/src/props/*` is PROPS's glob and no other open
program claims it. `S40`'s own verdict scoped this row out in 2026-08-19
as "a design call or belonging to a later wave"; the wave it named was
`S40`'s own, and `S40`'s program is gone. The D2 taxonomy is
`docs/DESIGN.md`'s (the bug-vs-invalid-state addendum, ~line 846), so
the call is PROPS's to make against it, not a ruling to escalate.

## Fixed — the manufactured escalation only (PR 2924, branch `props/curved-residues`)

**Re-read before fixing, as the dispatch required.** The citation is
from 2026-08-18 and names `mixed_levels` and `unreachable_zero`. As
the tree stands, `same_level` is indeed gone (unified into
`level_coincides` by #877/S81) and `mixed_levels` is reached from
`level_coincides`'s two arms — the rim-vs-group key and the rim-vs-
ends pair — not from two independent spellings.

**What was fixed.** `mixed_levels` built its error by feeding
`Margin::of(T::from_f64(f64::NAN))` into `classify` and keeping
whatever the funnel returned. A mixed pair has no comparand at all —
the two representations are not two values of one quantity — so the
margin measured nothing, spent a recorded verdict, and made the
error's TYPE depend on how the funnel happens to treat a poisoned
value. It now returns `PropsError::NotIsoRectangle { what: name }`
directly: the same outcome CLASS its callers always got (an `Err`,
never `Ok(false)`, so a mixed-representation face neither measures nor
groups), stated rather than manufactured. `mixed_levels` loses its
type parameter and its `Band`. The adopted review probe keeps its
assertion and is renamed `mixed_kind_levels_refuse_typed`.

**Which reading of D9 was left, and why.** Untouched. The file
answers a kernel-bug-only state two ways — `torus_meridian_orient`'s
`unreachable!` and `unreachable_zero`'s poison 4-tuple, the latter
still returning NaNs into `torus_ends`' live flux arithmetic — and
choosing between them is the file-wide census
`props-curved-carries-two-readings-of-d9-unreachable-vs-poison` asks
for. A decision taken at one site would pre-empt it, so this arm keeps
the typed refusal it already produced and says so at the site. **The
second half of this row's title is therefore still open, on that
sibling.**

**The sweep.** Pattern
`(classify|decide|require_zero|Margin::(of|levered|norm3))[^;]*NAN`
over `crates/` — after the fix, zero hits (one doc-comment match on
the word "decided"). The idiom survives nowhere else. **What that
pattern cannot match**: a NaN bound to a local first and passed on a
later line, a NaN produced arithmetically rather than written, a
`±inf` poison used the same way, and a NaN reaching a margin through a
helper. For the first and last of those, the widening sweep is every
`f64::NAN` in `crates/*/src/`: inside `props/` the only remaining site
is `unreachable_zero` (left, above) and `quad.rs`'s `is_nan` guards
and one `return f64::NAN` value, none of which feeds a decision
predicate; outside it the hits are poison/sentinel values
(`bvh`'s AABB and SAH sentinels, `implicit.rs`'s `poison`,
`ssi/jet.rs`'s `sqrt`, `ssi/certify.rs`'s collapsed `SupSpeed` —
itself already a filed TRIM finding — `patch_bound.rs`, `editor-core`)
and none of them manufactures an error through a decide.
