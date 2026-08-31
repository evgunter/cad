# S-MATE log

Newest entries at the bottom; the tail is the program's live status.
Plan: `docs/S-MATE-PLAN.md`. A/B band 1300–1399
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

---

**2026-08-31 — program opened.** Picked up on Evan's in-chat
direction as the last unclaimed stream of
`docs/WORK-STREAMS-2026-08.md` (verified: S-CERT/S-QA graduated in
the doc, S-BLEND graduated and closed, S-MESH/S-BOOL opened in PR
#1373). Plan drafted with four rulings sought (Q1 #946 seam
semantics, Q2 #973 backing, Q3 #943 curved-residue timing, Q4 #968
pickup); band 1200–1299 claimed in the banding entry this same
commit. Substrate corrections folded into the plan rather than the
cut: #945 is RULED (banked implementation unit), #943's planar gaps
are BUILT (#969/#1063) leaving only the curved residue. Ruling-
independent units MATE-1 (#945), MATE-2 (#1032) and MATE-3 (#941
items 1–2) are dispatchable pre-ratification; MATE-4 through MATE-7
wait on their rulings as scoped. Next: open the design-conversation
PR for this plan, then dispatch MATE-1.

**2026-08-31 — rulings received (in-chat, same day); PR #1392
updated in place.** Q1 RULED (minting moves to evaluation —
measured as a drift-closure against A3's own ratified sentence;
MATE-6 now dispatchable), Q2(a) RULED (the `ef_bound_backed` rung
extension — MATE-4's impl half dispatchable), Q2(b) DIRECTED
(eventual machinery; design pass proposes shape/staging; the
declared-interpenetration forward constraint recorded), Q4 RULED
(MATE-7 scheduled last). Q3's dependency question answered (no M10;
S-CERT #1191 touchpoint recorded at MATE-5); its scheduling half
awaits confirmation. MATE-1 lane in flight (fable arm, block
MATE-B1 slot 1, branch `mate/1-member-vocab`).

**2026-08-31 — Q3 scheduling half RULED (in-chat): build now.**
MATE-5 proceeds as scoped (cylinder-first), sequenced after MATE-2
and serialized with MATE-4 on `census.rs`.

**2026-08-31 — band renumber 1200→1300 (main-is-authority).**
While PR #1392 awaited sign-off, SEAT claimed 1000–1099 on main and
S-MESH renumbered onto 1200–1299, reaching main via PR #1373's
merge. No S-MATE ordinal had been claimed on main, so only the band
moves: **S-MATE = 1300–1399**; plan, banding entry and the cut's
graduation marker all updated in the resolving merge. MATE-1's
review-dispatch ordinal will claim 1300.

**2026-08-31 — MATE-1 MERGED (PR #1400; issue 945 closed).** The A11
member-vocabulary rider is implemented: pattern-placed mate heads
resolve, the derived offset enters the solve as the pair's static
left factor, and the loop clause falls out of member-keyed pairs.
Dual ordinal 1300 (sample #78 at merge), both arms
MERGEABLE-AFTER-FIXES converging on the under-toothed conjugation;
at fix the stepped rule was RE-HOMED onto eval's single home (the
review's drift hazard closed by construction) and both probe suites
adopted. Filed: issue 1405 (split crossing collection skips pattern
heads), issue 1411 (nested/transform head narrowing — small ruling
or extension), issue 1412 (viewer pick gate, GAUTH ground). Process
notes: three gate runs died to the exhausted Actions spending limit
(memory updated); the fix pass ran as a FRESH lane on the same arm
(the implementer lane's worktree was reclaimed at report — reclaim
only after the fix pass, next time). Next: MATE-2 (issue 1032) spec
+ dispatch on block slot 2.
