ADDENDUM — PR 7517, on 52b6423b18 (rows, slow-set lines and one filed issue on top of a87d047b5a; no production line changes).

Overall verdict unchanged: NOT-MERGEABLE-AS-IS on the bit-identity MAJOR. Claim 5's "missing witness and filing" MINOR is withdrawn.

**What I ran at 52b6423b18:**
- All 8 collapsed_rows tests, with `--ignore-default-filter`: 8/8 pass.
- `work.py lint`: ok.
- `bit-identity-consumer.sh`: still RED, now with TWO hits. The PR's `trimmed.rs:3236`, and the new row's own `trimmed_collapsed_rows.rs:465` (`one_stored_point` used to find the vertex id). The gate scans test code too.
- Mutant M5 (a 1e-9 band in place of exact identity): now red on TWO rows, the unit `holds` row and the new body witness. So M5 is now caught at body level, and the claim-8 coverage gap closes.

**Claim 5, with the witness:**
- **Does the witness show what it says?** Yes. It checks its own premise: `vertex_at_singular_point` at the band is `Ok(true)`, `holds` is false, `singular` is empty and the vertex is held once. Then the face meshes at δ 0.1 and refuses typed at 1e-2.
- **Is "sound" true of the one-copy shape?** Yes, as far as I can test. The polygon's corner at the vertex is at the side's chart point, its 3-D position is within an ulp (inside δ+ε), and the certificate decides typed. My a87d047 ulps-off probe (1/4/64 ulps on two walls, three δ) agrees: typed CertificateExceeded, never a panic or a fold.
- NOTE: the row pins today's limitation (a refusal at 1e-2). It will go red when the filed cure lands. That is intended, but the row's doc should say so, so nobody reads a red as a regression.

**The filed cure ("make the import write the vertex as the side's point"):** I agree it is the right home.
- Import is where ε_in may decide incidence, and it already heals the SIDE (`CollapsedSideMadeOne`). Writing the vertex as the healed point keeps the mesh ε-free.
- **Correction to my main report:** there I floated reading the lift's Sides decision in the mesh. That decision is taken at the run's band, so it would put ε into the mesh's choice (`[tolerance.chordal]`). Withdraw that suggestion.
- That leaves the MAJOR as a pure design question for Ev: the exact read is a new production consumer of the retired bit channel, comparing a vertex against a side point. It needs a DESIGN `[extensibility]` revision, or the gate's list extended under one.
- Minor gap in the filing: it has no priority or cost, and its claim that "step import could" make such a vertex is unmeasured. It says so itself, but no fixture exists.

**New rows:**
- `the_wall_pinned_through_three_sections_meshes_only_with_its_side_walked` pins only "meshes + check_mesh" at 3e-2 and 1e-2. It is a subset of the class row, so it adds a name, not coverage (M2 already reds the class row).
- `a_lune_walks_both_its_collapsed_sides` pins ⌈1/h_v⌉+1 copies per column, from the same `face_target`/steps formula as the code: an implementation-parity check, as with the relation row.

Restored to a clean tree; nothing committed or pushed.
