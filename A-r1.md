# Fork `shared-vertex-pairing` — designer A, round 1

Harness token count (first report): 211,859.

## For Ev

**Recommendation (likely).** Stop letting the op choose the pairing, and change two things:

1. **Pair by the pose.** Where several crossing pairs share one vertex, each pair's runs follow its own In arcs in the shared solid. The pieces of one operand don't overlap at the point, so these runs never hold another pair's cut. The one exception is a strut that is forced onto its segment. It nests inside another strut as it does today (`holds_whole`).
2. **Match seams by pair record (by corner), not by vertex.** The finish's seam correspondence should use the pair records directly. The op-based pairing start exists only to serve the vertex map.

Together these should build both classes: the 982 lines and the three cleave ops. They retire four things:
- the reconcile's turning loop;
- `hang_at_shared` and its cover backstop;
- the finish's "conflicting seam vertex correspondence" `JoinDesync`;
- the insertion's only read of the op.

The insertion half rests on an argument, not a measurement. Change (2) is unbuilt and was not measured.

**Terms.**
- *Pair*: the crossing records of one vertex-vertex contact.
- *Germ*: one crossing of the two link boundaries on the small sphere around the point.
- *Run*: the stretch of one solid's orbit that a null edge splits off to a copy vertex.
- *In arc / Out arc*: a stretch of the shared solid's link boundary inside / outside the partner piece.
- *Pairing start*: which of the two consecutive matchings of a pair's 2k germs is used.
- *Kept side*: the side of A the op keeps (Out for ∪ and ∖).
- *Corner*: a corner of a kept section face, at a null edge's surviving end.

**Premise check.**
- **The pairing changes the plan, not the body (likely).** Either start splits every vertex into copies that are each wholly In or wholly Out. The zip then splits any kept vertex per cone (`zip::split_cones`, "a pinch is one vertex per cone", Ev on PR 4057). So both plans should give the same body. The question is which plan makes the code total and simple, not which body is right.
- **The op reaches the insertion in one place only (sure).** In `insert.rs` the op is read only in `pairing_start_turns`. Its doc says why: "A's is the one the finish needs". It puts A's runs on A's kept side so that no kept A vertex holds two null edges of one contact.
- **That need comes from the finish's representation (likely).**
  - The pair records tie each A null edge to its B partner by key. `setopfinish` collapses them into a map from each A vertex to a set of B vertices.
  - It then refuses an A vertex with several B correspondents from one contact. Two cases are exempted by hand (`shared_cut`, `one_ring`).
  - Several A vertices per B vertex pass; several B vertices per A vertex do not.
  - Each kept corner is one null edge's end, so the per-corner correspondence already exists in the records.
- **The fault sits upstream of the refusal (likely).** The kept-side start lays a pair with four or more crossings along its Out arcs, and other pieces can sit on those arcs. The `SharedVertexCrossings` refusal and the experiment's `JoinDesync` are two faces of one constraint: the finish's vertex map, pushed back into the insertion. The cleave row's history shows the same thing. The first-germ start desynced, the kept-side start was added, and it refused instead.
- The brief's "982 poses" are 982 lines (pose × operand order × op).

**Ratified text.** None governs this choice (sure).
- The insertion contract in the `insert.rs` module docs and the `pairing_start_turns` doc are agent-written (`c50f2ff22e6`, 2026-10-04).
- The topo README and DESIGN.md have no clause on vertex-vertex pairing.
- The only ratified clause this relies on is Ev's "a pinch is one vertex per cone" (PR 4057, commit `52cbed0cfa0`). It stays as it is.

**Options, as final states.**

*A. Pair by the pose, match seams by corner (recommended, likely).*
- **Rule.** A pair pairs consecutively in its pairing solid's walk, along that solid's In arcs. The pairing solid is the one whose vertex several pairs share, and A everywhere else.
  - The other solid's runs nest as B's do today (`b_runs`), so that code becomes per solid.
  - It has to, because at six or more crossings the shared solid's In matching can be nested in A's walk (unsure how often; the 40 six-crossing lines would tell).
- **What becomes true.**
  - No run at a shared vertex holds another pair's cut.
  - Two runs crossing becomes an invariant violation: it would mean two pieces of one operand overlap.
  - `SharedVertexCrossings` shrinks to the one-arc case, which has its own row.
  - The insertion depends only on the pose.
- **Finish change.** For each pair record, the finish records the kept loop's half-edge at its surviving end, and `align` matches seams by record. The vertex-set matching and its two exemptions go.
- **Consequences.**
  - Every vertex with more than two crossings in ∪ or ∖ re-pairs (A's In arcs instead of its Out arcs). Poses that build today then depend on (2) and `split_cones`; the battery re-baselines this.
  - The 524 lines that the other start sent to `JoinDesync` go through (2).
  - The closed one-arc row was put back to a typed refusal because one insertion order hit `JoinDesync`. It may build under (2) as well (unsure).
- **Reversible:** yes; it is a rule plus a change of key.

*B. Keep the op's start, and read every run at the vertex as one laminar family (the row's option 3).*
- **What it is.** One tree over every pair's runs, in one germ order. A run that another pair's Out run holds mints at that run's copy. This merges `holds_whole`, `hang_at_shared` and `b_runs`' per-plan tree into one relation. Turns remain only to keep the family rooted.
- **Pros.** It touches the insertion only, the finish is untouched, and it covers the 966 laminar lines and the cleave ops by construction (likely).
- **Cons.**
  - The 16 crossing lines stay refusals of bodies that exist. Their only laminar plan re-pairs, which runs into the finish again.
  - The insertion still reads the op for the finish's sake.
  - It keeps more mechanism than A.
- Unmeasured. Reversible.

*C. Keep refusing typed (today).* Rejected (sure). Both classes have a defined body. The oracle's volume exists, and the other start's builds pass the volume, tier 2, 3′ and certificate checks. The refusal is a limit of the kernel's own plan, not of the pose or of a reading. So a typed refusal is not an acceptable final state for either class.

*D. Fall back to the other start only where the kept-side start fails (the row's options 2 and 4).* Rejected (likely). It is two rules for one choice. It still needs (2) for the 524 lines, and once (2) exists the fallback has no reason to.

**Answers to the four questions.**
- **What these ops should produce:** the oracle's body, one vertex per cone. The fault is a pairing chosen by the op to compensate for a seam map keyed by vertex.
- **How to pair at a shared vertex:** along each pair's own In arcs in the shared solid, so its runs stay clear of the other pairs' runs.
  - *Insertion contract:* the "from the germ whose run lies on the side the op keeps of A" sentence goes, and so do the turned-run clauses. "Other pairs' runs are disjoint, apart from strut-in-strut" becomes a consequence rather than a target the reconcile steers toward.
  - *Finish:* the seam correspondence is per corner.
- **Typed refusal:** not acceptable for either class (see C).
- **Effect on what builds today:** poses with a pair crossing more than twice in ∪ or ∖ re-pair, and the same bodies are expected. The experiment's `JoinDesync` is what (2) removes.

**Confidence.**
- Recommendation: likely.
- Sure: the op's single use in the insertion; the finish's one-sided check (several B vertices per A vertex refused, the reverse allowed).
- Likely: the pairing differs only before the zip; corner keys can be derived from the records; In arcs are disjoint.
- Unsure: the six-crossing nesting in A; the one-arc row.

## For the orchestrator

- **The experiment I set up did not run.**
  - The setup is in the clone `/root/.local/share/cad-work/fork-svc-a/cad`. It holds the lane's probe-and-retry patch, plus `finish.rs`'s "conflicting seam vertex correspondence" guard gated on `SVC_A_RELAX` (`&& std::env::var_os("SVC_A_RELAX").is_none()`).
  - The planned run was `GAUGE_SVC_RETRY=1 SVC_A_RELAX=1` on `four_pairs_battery` and the cleave pin.
  - It would show whether `align`'s vertex-set matching can already tell the 524 desync lines apart, or whether the corner key is needed. Either result informs change (2).
  - Release builds lost about 80 minutes to build-slot contention from other lanes, and a sweep compile never fit inside a 10-minute foreground window. The target directory is deleted.
  - This is worth running before Ev rules. It would also help to measure option A's rule itself: flip every shared vertex to In arcs, not only the refusing pair.
- **Assumed, not checked:** that `promote_solid` runs while the null edges still resolve, so the per-record corner can be read there.
- **Loose doc, not a separate defect:** the `pairing_start_turns` doc says "no kept vertex of A holds two null edges". That already fails at shared vertices. A copy holding another pair's nested run, or a turned run's copy, holds several null edges, and the finish lets this through only via `shared_cut`. It is more evidence for (2).
- I met no foreign GitHub content. Nothing was pushed, and the main checkout is untouched.
- My draft of this report is at `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/fork-svc-A-r1.md`.
