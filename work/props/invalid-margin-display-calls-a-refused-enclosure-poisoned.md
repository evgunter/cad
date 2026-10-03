---
id: invalid-margin-display-calls-a-refused-enclosure-poisoned
kind: issue
title: MarginDiag's Invalid rendering calls an interval's certification refusal a 'poisoned enclosure'
status: review
opened: 2026-09-29
priority: P4
cost: E
branch: props/recourse-grammar
---

## Finding

RING-5's rule: "refused" is the certification refusal
(`!Interval::is_certified()`, below `Def`, the `Trv` clamp included);
"poison" stays for `Real::is_poison` and a lane's NaN/NaI. The one way
an enclosure reads `MarginKind::Invalid` is `Interval::sign_within`'s
`!self.is_certified()` arm (`crates/geom-core/src/interval.rs`,
`sign_within`), so the enclosure half of the Invalid rendering names
the refusal — a `Trv` clamp with ordinary endpoints included, which
`Interval::is_poison` answers `false` for. The rendering still calls
it poisoned:

- `crates/geom-core/src/predicate.rs` — `MarginDiag`'s `Display`
  (`Reading::Invalid => "invalid (NaN or a poisoned enclosure)"`, ~:1233),
  its doc (~:1239), and the pinned texts in its own tests (~:1501,
  ~:1837, ~:2284);
- `crates/sweep/src/blend/mod.rs` — the two display twins
  (`"margin invalid (NaN or a poisoned enclosure)"` ~:276,
  `"an invalid (NaN or poisoned)"` ~:301);
- `crates/editor-core/tests/refusal_concision_chains.rs` ~:3104 — an
  asserted message carrying the text.

Rows in `work/` quote the rendered text verbatim (e.g.
`work/curved/topo-mints-indeterminates-outside-the-funnel.md`,
`work/sym/a-chain-of-two-or-more-joints-poisons-its-transversality-margin.md`).

Found by CERT-NAMES' fix-pass sweep (PR #3448), which kept it out of
its own diff: the text is the lane-agnostic `Decide` surface, it is
user-facing, and it is pinned in three crates. `MarginDiag::INVALID`'s
own doc ("A poisoned margin") and `is_invalid`'s ("Whether the margin
was poisoned") are the lane-agnostic reading and are right as the
`f64` lane's word; the enclosure clause is the part that is not.

## What would close it

Re-word the enclosure clause ("NaN or a refused enclosure") at every
site above in one change, moving each pinned assertion with its text.

## Resolved (props/recourse-grammar)

"NaN or a poisoned enclosure" → "NaN or a refused enclosure" at every
site the finding lists: `MarginDiag`'s `render` (so `Display` and
`LowerExp` both), the `Display` doc (which now says why — the enclosure
half is the certification refusal, `Trv` clamp included, and poison
stays the `f64` lane's word for a NaN margin), `IndeterminatePayload`'s
`Invalid` arm, the pinned texts in `predicate.rs`'s own tests,
`sweep::blend`'s two display twins, and
`editor-core/tests/refusal_concision_chains.rs`'s asserted message.

`MarginDiag::INVALID`'s own doc and `is_invalid`'s keep "poisoned": they
are the lane-agnostic reading and the finding says so.
