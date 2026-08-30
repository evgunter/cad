# CERT-3 — issue 924: the rotation-anchor round-trip

**Binding at dispatch** (S-CERT program, `docs/S-CERT-PLAN.md`;
difficulty logged at spec: **S/M**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 924 is the primary specification; this document fixes scope
and acceptance.

## The defect

`Affine3::rotation_about_axis` (`geom-core/src/linalg/affine.rs`)
builds its translation as `q − linear·q`. At `angle = 0` the
Rodrigues construction gives `linear` exactly `I`, so over the reals
the translation is zero — but at `T = Interval` the two occurrences
of `q` are evaluated independently and `x − x = [lo−hi, hi−lo]`: the
identity map exits the constructor **2·width(q) wide** on the point
it was anchored on. `MappedCurve::RevolvedPoint::eval` reaches it at
`s = 0` and `restrict` composes it into the STORED placement, so the
width persists. Every rigid transform in the kernel goes through
this constructor.

## The fix shape (the issue's own)

Spell the translation so the vanishing factor **multiplies** rather
than cancels: `q − R·q = (I − R)·q`, with `I − R` assembled so the
identity case is exactly representable (the `sin θ` and `1 − cos θ`
factors carried symbolically into the matrix difference — the same
move issue 921 made for the arc). Under the ratified bit bar
(`memories/output-stability-as-justification.md`): f64 bits may move
if ≪ ε or if a flip is semantically correct and the code cleaner —
but measure and REPORT what moves; the zero-angle case must come out
exactly zero in both lanes, and a change that moves f64 bits
anywhere re-derives affected pinned rows rather than re-baselining
them silently.

## Order of work

1. **Red-first Interval row**: a zero-angle `rotation_about_axis`
   anchored on a widened point is width-preserving (red under the
   current spelling — the issue predicts exactly 2·width). A second
   row at small nonzero angles pinning that the enclosure scales
   with `angle·width + O(width²)`, not with a constant floor.
2. The constructor fix.
3. **Re-measure the `RevolvedPoint` mapped-source enclosures**
   (the issue's named consumer): before/after widths on the
   existing fixtures, reported in the PR body.
4. **The blast-radius pass**: this constructor sits under transform,
   sweep, boolean, and fillet lanes. Run the touched-crate suites at
   default ε and the interval feature; any margin population that
   moves is re-derived with the argument in the PR body (no baseline
   is a target). If k-lint fires, follow the K-REPORT runbook —
   never adjust geometry to silence it.

## Sweep obligation

The issue records that the #921 sweep's pattern (anchor round-trips
with a named intermediate) could NOT match this site — the class
needs constructors read, not expressions grepped. Sweep
`geom-core/src/linalg/` constructors for the same
subtract-then-re-add shape (a derived quantity that cancels over the
reals paid as width at Interval); hit list with dispositions in the
PR body; state what the pattern cannot match. A correlated
expression evaluated naively under Interval that you find but do not
fix goes to issue 1143's audit as a member, not to a new issue (the
ratified routing).

## Keep-outs

`geom-core/src/linalg/vec.rs` is PCURVE P-2's until PR 1177 lands —
read freely, edit never. This is Track N fence ground: merge
origin/main before opening the PR and again if main moves.

## Acceptance

- The zero-angle Interval row red-then-green; the identity case
  exactly zero-width in both lanes.
- `RevolvedPoint` enclosure widths measured before/after, reported.
- Touched-crate suites green at default ε + interval locally; hosted
  CI green on the head, drawn point reported (a linalg change draws
  the interval lane only by the sampler — say whether it did).
- ε-three-outcome honesty on new rows; sweep receipt in the PR body.
- No `Co-Authored-By` in lane commits; keyword hygiene (write
  "issue 924" spelled out; the orchestrator closes it after merge).
- Any refusal minted or changed classified per the D2 addendum.
