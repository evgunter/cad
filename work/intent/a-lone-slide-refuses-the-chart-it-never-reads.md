---
id: a-lone-slide-refuses-the-chart-it-never-reads
kind: issue
title: A value's chart refuses at the antipode band even for a lone slide that never reads the swung axes
status: open
opened: 2026-10-18
priority: P3
cost: E
---


Found by the second review pair on PR #5267 (R4, style, `likely`), left unfixed in that PR.

**What.** A value is charted on each body's frame swung onto the mated direction. `Chart::of` (`crates/editor-core/src/mate/value.rs:71`) builds the whole chart for any mate a value names: both swings (`reference_about` on the target's direction and on the copy's), each refusing `PoseDegenerate { Chart }` when its direction sits in the antipode band. A value that reads no swung axis still pays for them: a lone `Slide(Z)` on an axis mate slides along the mated direction itself, and a slide with no spin beside it reads only the target's swing. Either refuses whenever the copy's own direction (or, for `Slide(Z)`, either direction) is within the band of its antipode.

**Why it was left.** The spec says "the chart refuses" (INTENT stage 3 §1, Values; ruling C-1), and the refusal is honest: the band cannot decide which way the swing goes. The cost is a refusal a value did not need, at a measure-zero set of poses.

**To decide.** Whether a value's chart is built lazily, swinging only what its coordinate reads (`Slide(Z)` neither swing; `Slide(X|Y)` the target's; `Spin` both), so a refusal names only the swing the value depends on. Belongs with F's measure-free door, which re-states when a value's refusal is decided.

## Built, then held back for Ev (2026-10-18)

A lazy chart was built in PR 5314: each swing decided and kept as `None` where
undecided, so a value refuses only on the swing it reads. A slide along `d`
reads neither, a slide across `d` the target's, and a spin both. It was taken
out of that PR before merge.

**Why.** It narrows a ratified clause. D10 in `docs/DESIGN.md` says "Inside the
sliver band about that one pole the chart refuses". That was ratified in
`[ev]` PR ddbc7c4f3b (fork-log row 156), and `docs/INTENT-STAGE3-SPEC.md` §1
says the same. So which values refuse there is Ev's call.

**Two findings from PR 5314's review to carry into it.** Both swings are
still *decided* at the chart's build, so an in-band verdict would escalate on
the node's log even where no value reads it; a lazy chart would have to defer
the decision itself, not only the refusal. And the k-stats move inside the
band, because the old `?` stopped before the copy's swing.

**Next.** Ev is restating placement (`a-placement-relates-two-frames`), and the
value chart may change shape there. Put this question to that restatement,
not to a separate `[ev]` PR.
