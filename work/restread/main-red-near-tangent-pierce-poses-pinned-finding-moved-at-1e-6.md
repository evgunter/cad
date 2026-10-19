---
id: main-red-near-tangent-pierce-poses-pinned-finding-moved-at-1e-6
kind: issue
title: Main red at ε 1e-6: near_tangent_pierce_poses' pinned EdgeEdgeCross finding moved its numbers since PR 5297 (restread/no-shared-key-crossing)
status: open
opened: 2026-10-19
priority: P1
cost: E
---


Found by CLEAVE, in PR 5185's review.

**The red.** `sweep::all near_tangent_pierce_poses::near_tangent_pierce_poses_build_their_closed_form_or_refuse_typed` fails at ε = 1e-6 only (`near_tangent_pierce_poses.rs:337`), on `origin/main` 759dcbe3. It reads: "1 of the poses' runs are wrong: w345 nt e1 a3.8 d1e-7, prism ∪ cube: the pinned findings moved". The finding is the same `UnrecordedContact{EdgeEdgeCross{16v1,31v3}}` the pin holds; only its numbers moved:

- witness y: 1.0186570888026658e-5, pinned 1.018657088811564e-5;
- margin: 5.8157933616838646e-15, pinned 2.1931706656334435e-15.

It passes at default, 1e-9 and 1e-12.

**Bisected** on main's first-parent line: it passes at 9d57b2ca40, f2efcf156b and 34cb623b55, and fails at **468d0e2f73**, the merge of PR #5297 (`restread/no-shared-key-crossing`), whose parent passes. The pin was written in 3fe334dae4 (2026-10-11).

**Owed.** Decide whether the new numbers are the census's correct reading after #5297. If they are, re-pin the numbers and say why they moved. If not, fix the reading.
