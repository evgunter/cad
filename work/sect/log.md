# SECT — the log

## 2026-10-10 — opened

Opened by JOIN's orchestrator at JOIN's close (Ev approved the plan in
chat on 2026-10-10; `docs/doc-ledger/join-leaves-the-tracker.md`). Its
charter is the section-first join Ev ratified in `[ev]` PR 4313 (fork
row 100), whose build was gated on INTENT stage 4's PR A; PR 4364
merged 2026-10-09T21:06Z, so the track is startable.

Ten rows moved from `work/join/` by `git mv`, ids and bodies unchanged:
the cylinder × sphere lane and tangency rows, the skew-cylinder and
torus frames, the one-site and along-edge rings, the in-band axis
offset, and three order and margin rows. Two unpriced rows were priced
at the move. SECT owns the curved section-frame and lane rows outright:
no per-pair stopgap lane is built elsewhere. Rows dispatchable: 10, for
31 points against a budget of 31.
— (JOIN orchestrator)

- 2026-10-14 — `closed-in-face-section-loop-has-one-site` closed by PATHS on `paths/in-face-section-self-loop` (`one_site_segments`: the wrap crossing and the closed-edge arm in one owner). (PATHS)

- 2026-10-14 — #5159's fix pass files two rows here: `a-closed-edge-lying-in-a-cone-cylinder-or-torus-face-has-no-join-arm` (P1, the one-site arm's residue; refused today at the crossing layer) and `a-closed-edges-germ-read-on-a-planar-face-would-take-the-placeholder-chord` (P3, the PR's disclosed blind spot, unreached). `RimSide::of` is now the one table of pierced faces the join and `join_lone_ring` read. (PATHS)

- 2026-10-25 — **Re-cut (refold approved in chat):** in, `the-lilys-stem-and-arch-walls-have-no-certificate-arm` from ORBIT (P0, lily wall 1) and `c5-plane-torus-cone-cylinder-arms` from GERM (P1), beside the torus frame (P0, teapot wall 2); `cylinder-sphere-germ-pair-has-no-join-lane` to P0 as the section object every P0 here is built on; `in-band-axis-offset-…` to P1 as it rides with step 1. Out, the six P2/P3 rows to SECTTAIL on the priority seam. Budget 36 (`program.md` says why). SPLICE, opened the same day, needs this track's rung-3 arm for klein wall 3 and teapot wall 3.

## 2026-10-25 — a row filed by SPLICE

(SPLICE orchestrator) Filed `a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm`
(P0 H, design open, parked on step 1 and on SPLICE's gate deletion): the
rung-3 arm that klein wall 3 and teapot wall 3 need. Step 4 of this
plan mentioned it only as a sentence. Agreed in chat that SECT should
get its own orchestrator rather than SPLICE taking these rows.

## 2026-10-27 — picked up

(SECT orchestrator) Track claimed, status `ready` → `active`. INTENT E
(`booleans-glue-on-zero`) closed 2026-10-10, so the section door's
coincidence seam the plan names is no longer a wait: E's record exists
and moves to the door with the frame's retirement. First dispatches:

- **Step 1's open choices to the designer pair** (one Opus, one Fable,
  `docs/prompts/designer.md`): the section object's type and home, its
  certificate's rung per kind pair, and the door's coincidence/tangency
  seam with E's record, as the brief for
  `cylinder-sphere-germ-pair-has-no-join-lane` (and its two riders)
  finds them open. Converged and inside fork row 100 → the brief is
  written from it; a split → an `[ev]` PR.
- **`the-lilys-stem-and-arch-walls-have-no-certificate-arm`** (P0 M) to
  an implementer now: a section-certificate arm in `section_cert.rs`,
  which does not wait on the section object. Review tier: single FULL
  (a certificate deciding a pair clear is a soundness claim a read
  cannot believe on its own).
