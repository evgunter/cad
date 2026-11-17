# PR 7044 — delta re-review at `e43ce957e9` (your first review was at `5e933bfff1`)

You reviewed this unit's first head: approve with fixes, no MAJOR, three MINORs, three NOTEs, ten style items. The fix pass did more than those findings asked: `cert::dist_point_triangle` was rewritten. So the delta comes back to you, the reviewer who holds the exact-arithmetic oracle and the mutant set. Same rules as before: commit, push and post nothing; read no other review's material and no branch under `analysis/`; report to the orchestrator by `send_message`.

**The delta:** `git diff 5e933bfff1 dfa60d7176 -- crates/mesh/src/cert.rs work/chord` (391 lines of `cert.rs`). The head `e43ce957e9` is that commit plus a merge of main that touches no `mesh` source. The PR body has been rewritten too; read it on the PR.

**What changed, as the implementer states it:**

- The distance is computed at unit scale: the corners' offsets from `p` and the sides are divided by `binade` of the furthest coordinate (a power of two), `dist_origin_triangle` computes, and the result is multiplied back. `dist_line_triangle` scales its direction the same way. The corner range guard is gone.
- The normal is `ab.cross(bc)` on the sorted corners; the two-shorter-sides select is removed, on the measurement that no row could tell them apart (worst 3.3e-15 of a unit length over 20,000 needles) and the argument in the comment at the site.
- One rule for a non-finite input, `all_finite`, asked by all five certificates; `inside_bound` is gone and the cylinder and sphere certificates end in `cert::worse(r − dist, 0.0)`; `cert_torus` no longer answers `+∞` on an infinity; `cert_spindle` reads an apex corner's azimuth.
- The bracket is three explicit branches.
- Rows: a new needle row; the table row no longer asserts bit-equality across orders; the distance fuzz asserts scale invariance at 2^±400 and 2^±900; the poison row asserts one thing for all five.

## Claims to falsify

1. **The distance is still the distance on the lanes' domain.** Re-run your oracle on the new head: inscribed sphere and cylinder triangles in your classes. Your first figures were 0.31 allowance too long and 0.27 too short at worst. A regression past one allowance on any inscribed class is a MAJOR.
2. **The fixed pair is as good as the select after the sort.** Attack the comment's argument: build needles and near-needles whose first two sides in sorted order are the two long ones, with the foot inside, at generic and at axis-aligned poses, and with ties in the first coordinate (two or three corners sharing `x`, where the sort falls to `y` and `z`). Is there a triangle with its corners on a sphere about `p`, or whose shadow has its corners on a circle about the axis, where the new normal is worse than your first review's by more than an allowance?
3. **Scale-free, exactly.** Dividing by a power of two is exact only while nothing becomes subnormal. Find where it is not: corners whose furthest coordinate from `p` is near `f64::MAX` (is `1.0 / unit` exact at `unit = 2^1023`?), near `f64::MIN_POSITIVE` and subnormal, a `p` equal to a corner, all four points equal, sides far smaller than the offsets (a small triangle far from `p`: the sides divided by the offsets' scale). The doc says a feature under 1e-77 of the scale can underflow and "is within the bracket by its own size": falsify that sentence or confirm it with numbers. Your three first-review cases (axis norm 1e155; 1e-160; corners 1e-125 from `p`) are claimed correct now.
4. **No silent wrong finite answer is reachable through the public functions**, other than what the doc names with its size. Your m2 and n2 are the precedent.
5. **No mesh byte and no verdict moves, again.** Re-run your digest of every `tessellate` result over the `mesh` suite, head against main's `cert.rs` in the same tree. The four refusal bounds that moved in their last digits: the same four, or others now that the normal's sides changed?
6. **Poisoning has one home.** Each of the five certificates answers NaN, not `+∞` and not a number, on any non-finite corner coordinate, radius, anchor or axis, a zero axis, and two finite points whose difference overflows; `+∞` remains only for finite corners that are no triangle of the surface. Did removing `cert_torus`'s `+∞` change what any caller sees (grep the readers of `is_infinite` / `INFINITY` on a certificate)?
7. **Rows and mutants.** The implementer's ten mutants each red a named row (table in the PR body). Re-run your own survivors: the fixed-pair mutant is now the code; `>=` for `>` was equivalent. Add the mutants this rewrite invites: scale by something not a power of two; scale by the nearest coordinate; drop the multiply back; `area2 == 0.0` test removed; the sort made unstable or partial. Does anything survive?
8. **The docs now say what is true.** Your Q5, Q1 and stance items, and m3, n1, n3: read the new function doc, `all_finite`'s doc, the PR body's "sibling" paragraph (per lane; corpus-measured) and "What moves" (the four bounds), and the two `work/chord` evidence lines.
9. **Cost.** You measured 73 ns against main's 37 per call. What is it now?
10. **Hosted CI** on `e43ce957e9`: state each job's conclusion, and which jobs the change filter skipped.

## Report

Verdict (APPROVE / APPROVE-WITH-FIXES / NOT-MERGEABLE-AS-IS); each claim held, corrected or falsified with what you ran; findings labelled MAJOR / MINOR / NOTE, each DEMONSTRATED or BY-INSPECTION, with file:line; which of your first findings you confirm fixed; what you executed and what you only read. Name no model.
