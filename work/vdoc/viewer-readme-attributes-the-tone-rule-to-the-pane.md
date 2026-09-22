---
id: viewer-readme-attributes-the-tone-rule-to-the-pane
kind: issue
title: The viewer README says pane::features argues the tone rule, which a value now states
status: closed
closed: 2026-09-21
branch: vdoc/readme-attributions
opened: 2026-09-20
priority: P4
cost: E
---



Filed across the fence by `vnews`'s
`tone-is-a-value-in-frame-and-a-comment-in-two-panes` lane, whose own
diff is what made the sentence stale. `crates/viewer/README.md` is
VDOC's and a prose defect there is filed, never fixed — so this is
filed even though the ruling that governs the rest of the lane's prose
census (*prose your own diff falsifies is yours to fix*) would
otherwise have kept it in hand.

**The collision between those two rules was adjudicated, and the
boundary is worth VDOC having** (VNEWS orchestrator, 2026-09-20): *the
fix-your-own-falsified-prose rule applies inside a lane's own fence; a
named file-specific carve-out in `program.md`'s `keep_out` overrides
it.* Three reasons on the record — a carve-out is a territory contract
written on both sides and an orchestrator's message is not a territory
amendment; VDOC's whole charter is prose and citation on this file, so
the general rule would put four successor programs inside it; and the
general rule's own justification is that such prose *lands with the
change that caused it*, which is true only where the prose sits on the
lane's ground. **So a successor whose diff falsifies a sentence in this
file files it here rather than fixing it, and that is settled rather
than an omission.**

**The sentence**, in *The badges* (~`:835`):

> A `frame::Badge` carries its subject, a `frame::Tone` (`Advisory` for
> a report, `Actionable` for a verdict a reader may need to act on —
> **the rule `pane::features` argues for poisoned rows**, stated by a
> value rather than picked per call site) …

`pane::features` does not argue it. `tree::RowStatus::tone()` states
it, and the pane reads the value — which is what the rest of the
parenthesis already says is the point, so the clause contradicts its
own sentence. The repair is to cite `tree::RowStatus::tone` as the
second producer of `Tone` outside `frame`, which is also a fact the
paragraph's population sentence (*"every `frame` function returning
`Option<Badge>`"*) does not cover and does not need to: a tone is not a
badge.

The same paragraph's `app::draw_badge` clause (~`:860`) — *"the single
draw"* — is still true, and the tone MAPPING it implies now has one
home in `app::toned`, read by that draw and by the feature row.

Three sibling members of this class are `work/vnews/tone-doc-argues-
from-a-site-that-now-reads-the-value`, with the sweep rule that
produces the population.

## Closed — the rule is cited where it is STATED, not where it is read (#vdoc/readme-attributions)

**Old citation.** *The badges* (`crates/viewer/README.md`): *"the rule
`pane::features` argues for poisoned rows, stated by a value rather
than picked per call site"*.

**The subject, re-derived.** The rule is *a poisoned row stays
`Advisory`*, and it is a `match` arm:
`crates/viewer/src/tree.rs:151` —
`Self::Ok | Self::Unevaluated | Self::Poisoned { .. } => Tone::Advisory`
— inside `RowStatus::tone`, whose doc comment above it carries the
argument (*"a poisoned row shows someone else's failure and points at
the row that owns it"*). `pane::features` does not argue it: the only
tone in that file is `row.status.tone()` passed to `toned` at
`crates/viewer/src/pane/features.rs:89`, under a comment that says in
as many words *"How LOUD a drawn badge is, is not decided here — that
is `RowStatus::tone()`, read below"* (`features.rs:80-81`).

**New citation.** The parenthesis now names `tree::RowStatus::tone` as
the one function outside `frame` that DECIDES a tone, with
`pane::features` reading the value. Cited by symbol, not by line.

**The command that finds it.**

    rg -n -- "-> (crate::frame::|frame::)?Tone\b" crates/viewer/src

prints **2**: `tree.rs:149` and `frame.rs:1293`. The second is
`Badge::tone`, an accessor over a tone already stored, so
`RowStatus::tone` is the only decider outside `frame` — which is the
claim the sentence now makes.

**And the paragraph's other half, checked the same way.** The row
observed that the tone MAPPING now has one home; it did, and the page
did not say so. `app::draw_badge` is still the single draw
(`crates/viewer/src/app.rs:229`), and `app::toned`
(`crates/viewer/src/app.rs:210`) is the single tone-to-chrome mapping,
read by that draw (`app.rs:231`) and by the feature row
(`pane/features.rs:89`) and by nothing else:
`rg -n 'toned\(' crates/viewer/src` prints **3** lines, the
declaration and those two calls. That sentence is now on the page
beside the draw clause.

Re-derived on the merged tree at `fb60ba2b7f`.
