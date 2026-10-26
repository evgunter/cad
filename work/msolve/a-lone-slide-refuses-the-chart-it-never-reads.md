---
id: a-lone-slide-refuses-the-chart-it-never-reads
kind: issue
title: A value's chart refuses at the antipode band even for a lone slide that never reads the swung axes
status: closed
opened: 2026-10-18
closed: 2026-10-24
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

**Next (INTENT orchestrator, 2026-10-18).** No separate `[ev]` question.
D′, `a-placement-pins-by-written-values-without-a-mate` (filed on stage 3 D's
branch), rebuilds the chart without a mate. D′ settles whether the sliver-band
clause narrows, and this item follows it.

## D′2 re-read (2026-10-23)

D′2 rebuilt the chart on the target space's axes, swung onto the one
mated direction, and left the sliver-band clause as ratified:
`Chart::of` (`crates/editor-core/src/mate/value.rs`) still builds both
swings for any value under a direction-fixing mate, so a lone slide
still refuses in the band. Whether the clause narrows is still Ev's
call (D10's ratified sentence); D′2 did not decide it, and this row
stays open on that question.

## Closed (2026-10-24)

Absorbed by `the-value-chart-reads-a-written-orientation`. The two swings this row is about are gone, and with them their two antipode bands. Under F the chart has one pole, the orientation written backwards. Within its band the chart refuses whichever values the bundle holds, as `[placement.values]` states ("within the band about that one pole the chart refuses"). Away from it, the only refusal on the chart's account is the span of the slides written (`NoChartWhy::SlidesDoNotSpan`). That is decided from the source's geometry and not from where it stands. So the question of which values refuse at a swing's antipode no longer arises.
