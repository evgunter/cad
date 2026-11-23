My recommendation is unchanged from round 2, so there is no For Ev section. On both items I now sign what the other two sign, so the three of us agree.

1. **The sentence: I sign (y) as written, no splice.**
   > A conic's position is the point it stores plus two vectors times two functions of its parameter, and every reading of where the conic is (at a parameter, along it, or of a point against it) is read from that position. Its centre is a datum of the whole conic, stored or derived.

   Checked against the two pages:
   - **"offsets" is already taken in `conic.rs`.** Its only use there is the arc-held pair (the private `offsets` function: "the two offsets an arc-held ellipse's point is summed from"). The centre-held formula in the module doc uses no such word. (x) would give the word a second, wider meaning on the ratified page; my round-3 reason for (x) missed that.
   - **"read from" is the README's own verb** ("one held at its centre is read from its centre").
   - **"of the whole conic" answers the page's own contrast** with "a point of the arc it serves".
   - "Stored" and "datum" are used as the page uses them.

2. **`u_ref` and `v_ref`: the end state is (x), they are off `Conic`; which unit removes them is your sequencing.**
   - **My round-3 reason for (y) does not hold.** I said the replacement depends on what the parameter becomes. `MajorFrame` in `conic.rs` already has `phase`, "the carrier parameter at" its vertex, so it states the pairing whatever the general-anchor row decides.
   - **What (y) wanted is kept by (x).** The removal is the compile-time census of every pairing, whichever unit does it.
   - **I take the other report's addition.** A read that wants an in-plane basis with no parameter gets a frame with no seam meaning.
   - **What Ev sees.** The final `Conic` surface without them, as one line: "`axis` stays; the principal frame is asked as `major_frame()` / `section_frame()`, which state the parameter at their vertex". Ev is not asked which unit does it.
   - **Sequencing preference.** The pairing reads are the same sites the readers unit converts, so removing them there avoids a second pass. That is cost, not final state.
   - **If it lands with the general-anchor row instead**, the line owed on `work/oneround/a-section-conic-is-anchored-at-a-general-point-of-its-arc.md` stands: remove, never re-document under the same name.

**The one fact that could reopen item 2.** The classification finds a body of reads that need the seam direction as such, and neither `MajorFrame` nor a seam-less in-plane frame serves them. I know of none. The STEP writer's reference direction is `major_dir` with `phase`.

Read this round: the two round-3 answers, the README's Float rounding wording, and `conic.rs`'s `MajorFrame` and `offsets` docs (from my earlier read). Nothing was built or measured.