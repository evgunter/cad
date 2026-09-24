---
id: a-split-circle-fixture-sits-inside-the-1e-6-escalation-band
kind: issue
title: A pane::profile fixture's chord-side margin lands in the 1e-6 escalation band and the row fails at that eps
status: closed
branch: chrome/split-circle-eps
opened: 2026-09-21
priority: P2
cost: E
closed: 2026-09-22
pr: 3059
---


## Finding

`crates/viewer/src/pane/profile.rs`,
`tests::drawing_a_locked_split_circle_above_the_cap_leaves_it_alone`.
It builds a `circle_split` of radius `0.01 m` with
`MAX_CIRCLE_SPLIT + 1` segments and `expect`s the document to admit
it — *"the document admits a split circle above the form's cap"*.

Under `CAD_TOLERANCE_EPS=1e-6` the document does not admit it. The
`chord_side` predicate answers a margin of `1.1272608421844756e-6`
against `Band { zero: 1e-6, escalate: 9.999999999999999e-6 }`, so the
pair escalates as `Indeterminate` and `apply` refuses. The row then
panics on its own `expect`.

Passes at the default eps and at `1e-12`. Executed on `origin/main`,
twice — `cargo test -p viewer --features app --lib` and
`cargo nextest run -p viewer --features app -E
'test(drawing_a_locked_split_circle)'`.

**The row is right and the fixture is the problem.** Nothing about
"drawing an over-cap split circle leaves it alone" is a claim about
tolerance; the fixture just happens to sit a tenth of a decade inside
one eps row's escalation band, because a `0.01 m` circle cut into
`MAX_CIRCLE_SPLIT + 1` pieces puts its chords there. A fixture whose
scale is near a gated eps row's is a fixture about that row. The fix
is a radius (or a count) that clears all three bands, and saying in
the row why the number was chosen.

## Why nobody had seen it

The viewer's `#[cfg(feature = "app")]` rows run in exactly one CI step
and that step carries no `CAD_TOLERANCE_EPS`, so they are gated at the
default row only —
`work/ciw/the-viewer-app-feature-rows-gate-one-eps-of-three.md`. This
row is live on `main` today and the gate is green.

## Fence

`crates/viewer/src/pane/profile.rs` — chrome's and view's. Filed from
`vgeom/sketch-infinity`, which found it by running the viewer suite at
all three eps rows after CI reded one of its own rows at `1e-6`.

## Resolution (on `chrome/split-circle-eps`, in review)

**No constant radius clears all three bands, and the miss is by a
factor of 1.45.** That is measured, not argued. The figure is
conditioned purely relatively: the `segment_straightness` sagitta
`r·(1 − cos(π/n))` and the `carrier_circles_identity` floating-point
residue are both proportional to `r` and both read against (ε, K·ε),
so the admissible radii are an interval in ε and both its walls are
walls in `r/ε`.

At `n = MAX_CIRCLE_SPLIT + 1 = 1025`, bisecting the radius through
`apply(InsertNode { Profile })` at five ε:

| ε | lower wall | closed form `K·ε/(1 − cos(π/n))` | upper wall |
|---|---|---|---|
| 1e-5 | 2.1290e6·ε | 2.1290e6·ε | 1.5358e12·ε |
| 1e-6 | 2.1290e6·ε | 2.1290e6·ε | 1.5340e12·ε |
| 1e-9 | 2.1290e6·ε | 2.1290e6·ε | 1.4937e12·ε |
| 1e-12 | 2.1290e6·ε | 2.1290e6·ε | 1.4666e12·ε |
| 1e-13 | 2.1290e6·ε | 2.1290e6·ε | 1.5179e12·ε |

So the window is **5.84–5.86 decades** wide and a constant radius
across ε = 1e-6 … 1e-12 needs **6.00**: ε = 1e-6 wants `r > 2.129 m`,
ε = 1e-12 wants `r < 1.467 m`, disjoint by 1.45×. **The miss hangs
entirely on the softer wall.** The lower one is exact and
ε-independent — measured equals closed form to five digits at every ε
above. The upper one is an empirical f64 residue: it moves a few per
cent with ε, it falls as ~1/n (at ε = 1e-9: n=256 → 1.08e13·ε,
512 → 5.07e12, 1024 → 2.57e12, 2048 → 1.24e12, 4096 → 6.17e11), and
odd counts pay about another 1.7×, which is why n = 1025 reads
1.49e12 and n = 1024 reads 2.57e12. Had the residue come out at
3.2e12·ε a constant radius would have worked.

Dropping to the chord regime does not help either, and the reason is
general rather than particular to this figure: for any regular polygon
whose sagitta falls below ε the `chord_side` ladder
`dⱼ = r(cos(π/n) − cos((2j+1)π/n))` starts at `8s`, climbs with
consecutive ratio `(j+2)/j ≤ 3` and tops out near `2r`, so a bottom
under the band and a top over it force some rung into (ε, K·ε). Both
"8 < K" and "3 < K" are facts about the **ratified** K = 10, not about
the structure — `CAD_AMBIGUITY_K` admits any finite K > 1, and nothing
in CI varies it. Below `r ≈ 1.6e3·ε` the chord itself is in band or
under ε and `vertex_separation` answers first.

So the radius is tied to the run's ε: `1e8 * Tol::witness().get().eps`,
with the derivation written beside it. **1e8 is a model-envelope
choice, not the window's centre.** The centre is 1.77e9 and would
stand ~700× clear of both walls, but it makes the fixture a 1.5 km
circle at ε = 1e-6 and a 15 km one at 1e-5 — outside D4 ¶1's ratified
micron-to-kilometre coverage. `1e8·ε` is 47× clear of the exact lower
wall and ~1.5e4× clear of the empirical upper one, and holds the
figure between 10 µm (ε = 1e-13) and 1 km (ε = 1e-5) at every ε the
suite is run at. The trade — margin on the exact wall spent to buy
margin on the soft one — is named at the literal.

**What guards it, and what does not.** A multiplier under the lower
wall or over the upper one reds this row at every ε it runs, so the
admissibility claim is mechanically guarded; the clearance figures are
decorative and nothing computes with them. But the fixture is now the
viewer's **only ε-relative literal**, and an ε-relative fixture is
immune by construction to the instrument that catches an
absolutely-scaled one drifting — running the population at
neighbouring ε. Its own `expect` is the guard; the sweep is not.

The sibling found while sweeping is
`work/forms/the-circle-split-cap-offers-counts-the-document-refuses.md`:
the same two walls, on the product surface rather than a fixture. The
`tests/profile_edit.rs` split circle (`r = 0.01`, `n = 3`) is **not**
in danger but its margin is smaller than first reported: sagitta
5.0e-3 and residue 6.9e-18 (measured: the n = 3 upper wall is
1.4545e15·ε, so the residue at r = 0.01 is 6.875e-18), which puts it
**50× above `escalate` at ε = 1e-5** — 1.70 decades, not the "≥3
decades from every wall" first written, and not ≥3 until ε = 1e-7. It
lands in band at ε = 1e-3, two decades past the sweep's edge; nothing
is live there, since 1e-3 is neither gated nor swept.

Also filed from this pass:
`work/forms/the-kernel-takes-any-count-has-four-homes-in-the-viewer.md`.
