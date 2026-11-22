# Review: PR 7471 (`heldrecords/elbow-seams`), frozen head `bc3796c8c3`, base `heldrecords/elbow-ends` (PR 7463) — DUAL review, you are one of two

Independent: do NOT read the other reviewer's branch, the PR's comments, or anything under `~/.local/share/cad-work/` but your own scratch, until your report is in. Probes only to `heldrecords/review-elbow-seams-<6 hex>`.

Read first: `CLAUDE.md`; `docs/prompts/reviewer-style-lane.md` IN FULL; the row `work/heldrecords/a-pipe-does-not-union-with-an-elbow-revolved-off-its-own-cap.md` on the head; `crates/topo/README.md` tier 3 (check 4, the wedge fold, must-carry `[edges]`); D10. Review `git diff origin/heldrecords/elbow-ends...bc3796c8c3`. Context: PR 7432 routes a rim junction at the glue door; PR 7463 reads an edge ending on a glued junction rim by its end root (deflation) in the crossing layer.

**What this PR does** (the elbow now unions: 4 outward poses, the rod, ±2ε and 1e-3 rad turns, the G1 tube chain, at closed-form volumes with t3 and t3′):
1. `reduce::junction_side`: the side premise of PR 7463's window — margin departure·spread − ½(κ_edge+κ_rim)·spread² − charge decided Positive, else typed `JunctionEndUnread { Window }`.
2. `ops::describe_edges_seamed`: a zip seam's `Scaffold` stand-in is described where it rests.
3. `geom_brep::dihedral::second_order_walk` (SELECT's ground, shared by tier 3 check 4, the fold and must-carry): a station whose signed κ_rel differs in sign from the first makes the walk `UnderDetermined`.

**Already known, do not re-derive (found by PR 7463's review; the fix pass will take them):** (a) `end_root`'s `whole` ignores the quarter-turn clip (`turned = span.min(quarter)` / `whole = !halved`), a false "no crossing" past π/2; (b) the side is decided against the full rim circle, not the glued arc, and `departures` can glue part of a circle; (c) `junction_side`'s pins call the function, not the door. You may confirm them on this head in one line each.

**Claims to falsify:**
1. **`second_order_walk`'s change is sound and complete.** A sign change between stations means κ_rel has a zero between them, so the bend is not determinate: show UnderDetermined is never too weak (a determinate seam now read under-determined, wrongly refusing or wrongly choosing the plain description). And the converse the change does NOT address: κ_rel that stays one sign at every station but dips to zero (or crosses twice) BETWEEN stations — is that still read Determinate, minting a `TangentIntersection` that cannot certify? Build one if you can (a station layout straddling two close neutral points). List every verdict the change moves across geom-brep, topo, sweep, editor-core at default ε and 1e-12 (run head vs base).
2. **The seam description is right**: the rim edge rests with a carrier that tier 3 certifies (the plain circle where no station is jet-determinate; `TangentIntersection` only where it can certify), and `describe_edges_seamed` cannot describe a scaffold that should have refused (a stale stand-in from a real defect upstream). What is the scaffold, and which stage mints it (the author left this untraced)? Is describing it at the describe pass the right layer, or does it hide a minting bug?
3. **`junction_side`'s margin** is a lower bound on the edge's distance from Q's face over the spread (its κ's, `spread` an upper bound, `charge` in metres); concavity makes checking at the spread's end enough.
4. **The unions are right**: re-derive the closed-form volumes; check t3 and t3′ on each built result independently; check the tube chain is one solid with one bent tube's topology; check the turned poses' cos δ-adjusted volume.
5. **No verdict moved wrong** elsewhere (claim 1's run covers this) and the inward poses still refuse typed.
6. **The pins can fail**: mutate each of the three changes away and see which pins go red.

Own `CARGO_TARGET_DIR` outside the checkout; per-crate runs (whole-tree OOMs). Foreground rule. Report ≤150 lines (verdict; claims; MAJOR/MINOR/NOTE; `## Style` sure/likely/unsure; probe rows) by `mcp__claude-code-remote__send_message` to `session_018u3gVKp4QiofNJ9Z1mzgD8`, then stop. No email or personal identifier anywhere; no model names in commits.
