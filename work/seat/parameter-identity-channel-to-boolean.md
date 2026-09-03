---
id: parameter-identity-channel-to-boolean
kind: issue
title: design - no channel carries parameter-identity (declared radius equality) from the recipe layer to boolean dispatch
status: parked
blocked_on: [SEAT-6]
opened: 2026-08-31
github: 1372
refs: [1345, 1353, 1388]
---

## From GitHub issue 1372

Opened 2026-08-31; 2 comments.

## The finding (VERBS-GERMARMS PR-2, #1353; also blocks VERBS-SPHSPH arms)

The exact cyl×cyl germ closed form (two bisector-plane ellipses) is valid only for **exactly equal radii**; near-equal radii produce a space quartic that differs qualitatively near the pinch. The design contract requires structural facts to be **declared, never inferred** — comparing two stored f64 radii is measurement masquerading as structure, and the near-pinch family is exactly where it lies.

PR-2 measured that **no declaration channel exists at all**: by the time `pair_section_frame` dispatches, the boolean sees two evaluated carriers with stored f64 radii; `ContactClass` is `{Rest, Tangent}` and carries no radius fact; `cylinder_cylinder_section`'s `RadiusEvidence` parameter has no production caller that could ever supply it. The same absence blocks SPHSPH's equal-radius arms (its spec's option (a) presumes a structural-parallelism/equality declaration that nothing can currently make).

## The direction ratified in principle (Ev, in-chat 2026-08-31)

Parameter identity — both surfaces built from the **same parameter** — is the right *form* of evidence: equality by provenance, true by construction, immune to tolerance arguments. Ev: "we definitely need info from parameter identity to be available here but i'm not sure how to do it nicely."

## The open design questions (this issue is the conversation)

1. **Where does the identity live?** The recipe/profile layer knows two features share an `r`; evaluation into carriers erases it. Candidates: a provenance token/ID on the carrier (shared handle), a side table keyed by carrier, or a typed `RadiusEvidence` minted at elaboration time and threaded through the boolean.
2. **What preserves it?** Rigid transforms should (they preserve radii); re-fitting, offsetting (r → r±t), STEP import, and hand-edited geometry should not. Offsets are interesting: offsetting BOTH walls of a shared-r pair by the same declared t arguably preserves the identity — is that a rule or a new declaration?
3. **Scope**: radius equality is one instance. Axis-parallelism declarations (SPHSPH's polar gate option (a)), seam-azimuth conventions, and the D-series "conventional data is data" family have the same shape — is this one channel or several?
4. **Failure honesty**: when the channel is absent (imported geometry), the family refuses typed — the current PR-2 behavior is then the permanent fallback, not a gap.

Design conversation — needs Ev's sign-off before any implementation unit is specced.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_016pYMaeU4woYZN8YGdTLfSK

## Comments

**2026-08-31** — comment:

Two observations for the open questions, and a pointer: this conversation now has a proposed mechanism as §3 of PR #1388 (`docs/VERB-SEAT-DESIGN.md`), argued jointly with issue #1345's one-verb-vocabulary item because the two turn out to need each other.

**Q1 (where does the identity live) has a shipped precedent one level coarser.** `GeomSource` (N6) already carries recipe-source identity INTO the kernel: `topo/src/source.rs`, stored as opt-in `SecondaryMap`s beside the geometry arenas (`body.rs:180-181`), lowered to pure data (`u64` node ids, structural `SourceExpr` addresses), attached by `editor-core` (`set_surface_source`), compared syntactically by the boolean's coincidence rungs (`plane_eq.rs:166` `same_base`), composed through rigid placement (`SourceExpr::Placed`), and simply absent for a kernel-direct caller. This issue's need is that pattern at per-stored-field granularity instead of per-description — a `ParamSource` side record for the scalar fields of minted descriptions — rather than a new mechanism. That also disposes of the candidates list: side table (per the precedent), not an on-carrier token and not a threaded `RadiusEvidence` (which stays as the *consumer-side* evidence type, finally with a production caller).

**Q2 (what preserves it) mostly answers itself if the channel carries lowered *expression* identity rather than a parameter handle.** Same discipline as `SourceExpr`: syntactic identity of the lowered expression address. Then "offset BOTH walls by the same declared `t`" preserves equality by construction — both walls carry the same `r ± t` address — while `r` vs `r ± t` differ; no new declaration and no rule needed. Rigid transforms compose exactly as `GeomSource` composes today; re-fitting, import, and hand-built geometry attach nothing and the family refuses typed (Q4's permanent-fallback reading — agreed, and the alternative of comparing stored radii is the measurement-as-structure move the contract forbids). The one genuinely open sub-case is a value the KERNEL derives arithmetically (e.g. the hollow tube's `minor_radius − wall`): v1 proposal is that identity ends where `editor-core` did not evaluate the expression, with kernel-minted composite sources recorded as a rejected-for-now alternative (PR #1388, ledger VS-Q3).

**Q3 (one channel or several): one mechanism, several typed positions.** Radius equality and SPHSPH's axis-parallelism read the same side-record channel at different field positions; the consumer-side evidence types stay per-family and typed.

**The coupling to issue #1345.** Attaching a field's source at mint time requires knowing which parameter flows into which field of which minted description — per-op knowledge that has no home today (nothing between a `wire_*` function's opaque argument list and the minted carrier records it). That parameter→field flow is the "provenance minting" item in #1345's decided per-verb declaration, which is why the two conversations are one PR: this channel is the first concrete consumer of params-reified-as-data at the kernel seat.

---
_Generated by [Claude Code](https://claude.ai/code)_

**2026-09-02** — comment:

**SEAT → VERBS handoff (SEAT-6 dispatching now).** Per `docs/SEAT-PLAN.md` Wave 3, the SEAT program is dispatching SEAT-6, which lands this issue's channel as ratified in `docs/VERB-SEAT-DESIGN.md` §3 (P1–P3, ledger VS-Q3/VS-Q4):

- **What SEAT-6 lands**: the opaque `ParamSource` token (editor-core-minted, deterministic from the lowered expression address; `Eq`-only to the kernel), per-field side records beside the geometry arenas keyed like `surface_sources`, attach-at-mint driven by the migrated verbs' declared `param_flow` (the SEAT-4/SEAT-5 substrate), propagation by key identity through survivors (rigid placement carries verbatim; kills drop), and **the first consumer: `cylinder_cylinder_section`'s `RadiusEvidence` gains its production caller** (same field sources on the two carriers ⇒ `Declared`).
- **What stays VERBS-owned**: the cyl×cyl equal-radius germ's geometry-side acceptance (the closed form itself, its ε discipline, and the germ's behavior once evidence arrives) is the germ lane's; SEAT-6 supplies the evidence channel and the production call, pinned so that the germ's existing declared-evidence tests are the contract. If the germ lane has in-flight work touching `RadiusEvidence`'s shape or `cylinder_cylinder_section`'s signature, flag it here before SEAT-6's PR goes ready — otherwise SEAT-6 treats the current shape as frozen and any conflict resolves by merge order.
- **Boundaries fixed by the ratified doc** (not re-litigated in the unit): no kernel-minted sources for kernel-derived fields in v1 (VS-Q3); absence refuses typed, permanently — no numeric fallback (P3); identity is per-evaluation against the current document (the `topo/src/source.rs` scope caveat verbatim).

SPHSPH's structural-parallelism option reads the same channel at its own position later; nothing in SEAT-6 forecloses it.

---
_Generated by [Claude Code](https://claude.ai/code)_

## Home

SEAT: the design question is ratified as `docs/VERB-SEAT-DESIGN.md` §3 (the lowered parameter-identity channel, `ParamSource`), which SEAT's charter executes, and unit `SEAT-6` is landing it — so the issue parks on that unit.
