1. **The sentence: I sign the two reports' wording as written** ("…two vectors times its parameter's two offsets from that point… is stated in that position. Its centre is a datum of the conic, stored or derived."). "Offsets from that point" says what the two functions are, which my "two functions of its parameter" lost. The gloss belongs in the `conic.rs` module doc, not the README.

2. **`u_ref` and `v_ref` stay on `Conic` for this decision; I withdraw removing them now.** My one change to For Ev is that bullet: `u_ref`/`v_ref` stay as principal directions, and only `frame`, `along`, `semi_axis_vectors` and `unit_in_plane` leave.
   - Reason: what replaces them depends on what the parameter becomes, and that is the general-anchor row's choice. Removing them here would design the replacement before that choice is made.
   - My "a forgotten read is silently wrong later" argument does not carry over from the circle reads. Minting an arc-held circle changes no door. Moving parameter zero to the anchor changes what `u_ref` means, so that row can rename or remove it, and the rename is the compile-time census of every remaining pairing.
   - Condition: this is written onto `work/oneround/a-section-conic-is-anchored-at-a-general-point-of-its-arc.md` as owed by that row (rename or remove, not re-document under the same name). A re-documentation under the old name would leave every hand-written pairing compiling and wrong.

3. **The order of the circle reads: yes, with the condition it is a sequencing choice, yours and not Ev's.**
   - Ev is asked the final state: circle decisions are tests on the position, and `Centre`'s callers are reporters and mints only. The condition is part of that final state: `Centre` offers `axis_line()` and one coordinate door and nothing else.
   - The order is then enforceable either way, because a forgotten read is a caller of one of those two doors and the minting unit is accepted on their caller list.
   - `axis_line()` must be on the gated list beside the coordinate door (the disallowed-methods entry with a reason per caller, which the other two propose). A `Line`'s origin is the centre, so it is a second way to the coordinates.
   - My preference among the orders is the other two's, one unit first: converting once avoids listing 79 sites under `Centre` only to convert them later. That is cost, not final state, so it is not something to put to Ev.

Nothing new was read or measured this round beyond the two reports.