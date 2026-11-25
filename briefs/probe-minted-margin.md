# Probe lane `heldrecords/probe-minted-margin`: measure one fact for a design fork

This lane measures and reports. It does not build: it changes nothing on main and opens no PR. Probes go to branch `heldrecords/probe-minted-margin` only.

## Background

Two designers have converged on a conditional. In the boolean's carry, a record (edge E lies in face F) can land on two cells the cut minted: v, the vertex where the tool face X cut E, and s, the section edge X cut in F. They meet at a point, with v strictly inside s.

There are two candidate final states:

- **(door)** The boolean's door decides that meeting with the census's point-inside-edge reading: `census.rs` `on_edge_interior_reading`, or `curved::on_curved_interior_reading` for a curved edge. It measures a margin against the band.
- **(makers)** Where one tool cell made both v and s, the meeting is proven by construction and nothing is measured.

The (door) form is chosen unless measuring can refuse a valid body. The argument that it might: the record puts E in F only to within the band. The v-to-s distance is roughly E's gap to F divided by how steeply X crosses E, and in the pierce row X (the lying cylinder) crosses E at its top line, near-tangent, so the gap may be amplified into the sliver band, where an in-band reading escalates.

## What to measure

The witness is `sweep a_pierce_held_in_a_curved_face::a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order`. Use the branch `origin/heldrecords/removed-members-red` at `17237bee03`, where (C) sets these meetings aside and the probe hook `PROBE_LOGSET` exists on `origin/heldrecords/review-removed-members-red-798492`.

1. For every set-aside (v, s) meeting in that row, print:
   - the reading `on_edge_interior_reading` or `on_curved_interior_reading` gives (the margin and its Zero threshold);
   - where the meeting sits relative to the band: Zero, sliver (in band but not Zero) or clear;
   - the maker of v (`Mints::minted_by` / `EdgeSplit::by`);
   - the tool face s was cut from.
2. Repeat with the operand's E-in-F gap nudged across its Zero band:
   - move the second prism by 0, ±0.25, ±0.5, ±0.9 of the band width (perpendicular to F);
   - also move it so that X crosses E closer to tangent, if the fixture allows (shift the cylinder so its top line lies nearer E's ends).
   - Run each at ε default, 1e-6 and 1e-12.
3. Report a table: pose, ε, the v-to-s margin, the Zero threshold, the band verdict, and whether the body builds today.
4. **The answer:** does any single-maker meeting read sliver or clear (not Zero) where the record's own gap was decided Zero? Do the same check for the vertex–vertex door's `vertices_meet`, wherever one tool cell made both vertices: is there any such pose in the `ci` set or the pierce row?

## Rules

- Own `CARGO_TARGET_DIR` outside the checkout; build per crate.
- Foreground rule: never end a turn with background work.
- Report in at most 80 lines, by `mcp__claude-code-remote__send_message` to `session_018u3gVKp4QiofNJ9Z1mzgD8`, then stop.
- No email or personal identifier anywhere; no model names in commits.
