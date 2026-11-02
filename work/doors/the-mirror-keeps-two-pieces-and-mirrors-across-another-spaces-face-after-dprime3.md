---
id: the-mirror-keeps-two-pieces-and-mirrors-across-another-spaces-face-after-dprime3
kind: issue
title: After D′3 the mirror tool gains its two-pieces output (B ⊔ image) and a world-level mirror across another space's face
status: open
opened: 2026-10-25
priority: P2
cost: M
design: true
refs: [the-viewer-cannot-author-a-mirror]
---


The residue of the mirror arrival fork (2026-10-25, designer pair, converged; recorded in `the-viewer-cannot-author-a-mirror`'s `## Closed` once PR 5683 lands). Before D′3 the mirror tool makes only the fused symmetric part: `B ∪ Mirror{B, plane}`, taken over by B's drawn copies. Two outputs wait on D′3's operators and product slot:

- **Two pieces.** `B ⊔ Mirror{B, plane}`, taken over the same way. The image stays its own piece (a pick through the `⊔`'s path), and the seam is recorded as a touch rather than glued. It becomes the tool's second output beside the fused one, as the pattern tool offers Fused beside Instances.
- **A mirror across another space's face**, such as a bracket mirrored across a chassis's centre plane. This is a world-level creation, `Mirror { body: C, plane: Plane { face: wall.f } }`, where C is the clicked copy. The gesture adds it to the product's `⊔` (D′3's creation rule: a gesture writes `SetProduct`). Today it refuses `SpaceMismatch`, which is right until then. Both designers lean to building it, and neither is sure.

## Moved (2026-10-26)

AUTHTAIL folded into DOORS (Ev, in chat, 2026-10-26): both were AUTHOR's
successors on the viewer authoring files. Id and body unchanged.

## Re-parked on PR 6209 (2026-11-01)

D′3 (PR 5636) merged, so the old trigger fired. The row now waits on SPACES' `[ev]` PR 6209 (where a new body lands, and what the picture draws). Its recommended rule draws *bodies*, hiding a body that an operation reads as a body, and it hands the generalised takeover, re-pointing along the seen path, to SPACES (`a-takeover-re-points-along-the-seen-path-and-a-creation-joins-the-product`). Under that rule this row's defect may dissolve for an unplaced body and become SPACES' re-point for a placed copy. Re-read it against Ev's answer before building. (DOORS orchestrator)

## After 6209 (2026-11-02)

Re-read against Ev's ruling on PR 6209 and main at `822607621e`. **Still live, as a design question, smaller than filed.** Nothing is broken: both items are outputs the tool does not offer, so priority drops P1 → P2.

- **Two pieces: not built, and 6209 now pins the fused one.** `DocSession::add_mirror` (`crates/viewer/src/session.rs:3527`) writes only `body ∪ image` over the body's takeover, and the ratified landing text says so: "the mirror writes `body ∪ image`" (`crates/viewer/README.md:2183`, *A new body lands unplaced*). The viewer has no other `⊔` gesture either: `BooleanSpec` (`crates/viewer/src/session/author.rs:82`) is union, intersect and subtract, so `B ⊔ image` has no spelling in the GUI. 6209 makes the output cheap to draw (a `⊔` is drawn as its pieces, `world.rs` `pieces`), and `Takeover` already re-points readers with a `⊔` over a family (`add_pattern`, `session.rs:3640`). But a second output beside the one the README names changes that sentence, so it is a design question, not an elaboration.
- **A mirror across another space's face: half superseded, half open.** Its "adds it to the product's `⊔`" half is superseded: nothing joins the product unasked (§5′.3 (b), `docs/INTENT-STAGE3-SPEC.md:494`; README *A body reaches the product through one door*). The mirror itself still refuses `SpaceMismatch` at the insert door (`crates/viewer/src/session/op.rs:782`): the body seat seats the body a picked copy places (`crates/viewer/src/combine.rs:332`), and the plane is read on the body its copy places. Building it means reading body and plane on their world copies, so the mirror lands in their shared space, the world. The open choices are whether the gesture should do that, and whether "the mirror writes `body ∪ image`" then fuses two disjoint solids (a bracket and its image across a chassis) or wants the `⊔` above. Both designers leaned to building it, and neither was sure.

**What it touches:** `add_mirror` and `MirrorTool` (`combine.rs`), a second output choice in `pane/create.rs`'s `mirror_tool_ui`, and the README sentence. Cost M once ruled. **Owner:** DOORS; no SPACES or PLACING row carries a mirror. **Next step:** a design fork (one Opus, one Fable designer), then an `[ev]` PR editing the README sentence, if DOORS wants the outputs at all. (DOORS re-read)
