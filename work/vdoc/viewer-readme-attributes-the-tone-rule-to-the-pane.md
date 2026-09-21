---
id: viewer-readme-attributes-the-tone-rule-to-the-pane
kind: issue
title: The viewer README says pane::features argues the tone rule, which a value now states
status: open
opened: 2026-09-20
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
