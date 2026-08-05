---
name: concurrent-orchestrators
description: Two orchestrators run concurrently (M6 + M7, different accounts, Evan 2026-08-04) — static 1+1 cargo-slot split via cargo-slots.txt, scope fence, separate watchlists, A/B blocks prefixed per milestone
metadata:
  type: project
---

Evan started a second orchestrator (cad-implement-m7, different
account) on 2026-08-04 because the M6 account's Fable limit was
expected to hit that day. The M6 orchestrator continues M6; the M7
orchestrator owns M7's first slice (import of the STEP subset we
export). Briefing: ~/.local/share/cad-work/handoff-prompt-m7.md.

**The protocol:**
- **Cargo slots: static 1+1** — each orchestrator owns ONE of the
  machine's two lanes. Claims live in
  `~/.local/share/cad-work/cargo-slots.txt`; lending only by editing
  that file, never by assumption.
- **Scope fence**: M7 touches only its new import crate + tests +
  docs/M7-PLAN.md; no step-export edits, no CI structure changes
  (append jobs only, post-collapse), no M6-owned files, no touching
  the other orchestrator's open branches. Export-pin /
  check_step.sh semantic changes go through a design-conversation
  PR both orchestrators and Evan see.
- **Watchlists**: M7's away-channel monitor uses
  CAD_SIGNOFF_WATCHLIST=~/.local/share/cad-work/signoff-watchlist-m7.txt
  (the default file belongs to M6).
- **A/B**: both continue the experiment in docs/MODEL-AB-LOG.md;
  M7's blocks are numbered "M7-1, M7-2, …" (append-only rows merge
  trivially). Protocol otherwise unchanged ([[model-ab-experiment]]).
- **Coordination channel**: GitHub — each orchestrator's
  away-channel monitor surfaces the other's PRs/comments.
  Questions between orchestrators go to a fresh issue, never a
  merged-PR thread.

**Why:** limit-stop continuity beats slot-sharing throughput; the
import slice has near-zero file overlap with M6's remaining units.

**How to apply:** before any build, verify your slot in
cargo-slots.txt; at seams, update it. If the OTHER orchestrator's
account dies mid-lane, its lanes are resumable only from its own
session — do not adopt them; report to Evan instead. When one
milestone finishes, ratify the wind-down of this split explicitly
(update this memory).

**WOUND DOWN (Evan, in-chat, 2026-08-05): the M6 orchestrator is
done.** The M7 orchestrator (cad-implement-m7) is sole
orchestrator: both cargo slots, both watchlists' scope (the
default signoff-watchlist.txt is retired — new entries go to
signoff-watchlist-m7.txt), M6-owned files unfenced (M6-LOG etc.
now maintained by the sole orchestrator). Evan's pickup
instruction transferred PR #192 (M6-3 partial) explicitly; its
in-flight unit completes under the standing process with the
arm the M6 orchestrator assigned (FABLE, block-21 remainder).
The dead session's agents are unresumable — completion runs as
FRESH dispatches per [[resume-vs-fresh-subagent]].
