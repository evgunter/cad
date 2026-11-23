1. **The sentence: I sign (y) as written.** It is the same sentence A signs, so two of us now sign it.
   - On the page it goes on, "offset" already means something else: `crates/geom-core/README.md` "Float rounding" uses it for "a residual, an offset, a stand-off" (a length that is the margin).
   - In `conic.rs`, "offsets" names only the arc-held placement's two functions (`offsets`, `running_offsets`, `offset_extent`, "the first offset read as …"). The centre form there is "the centre moved by one semi-axis".
   - So (x) would make the README use a word that the code reserves for one placement and the page uses for another thing. The gloss that (x) defers to the module doc would have to correct it.
   - (y) uses words the page already holds: "the point it stores", "datum", "parameter".
   - What would decide it: whether `conic.rs`'s module doc ends up calling the centre form's `cos θ`, `sin θ` "offsets". If C will write that gloss, I can sign (x). As the doc stands today, (y).

2. **`u_ref` and `v_ref`: Ev is shown (y) for this decision, with the end state named.**
   - The end state is (x). The unit that does it is the general-anchor row's.
   - C's reason moves me: a rename when parameter zero moves is the compile-time census of every pairing. Removing them now would design what replaces them before that row decides what the parameter is.
   - (y) is a sequencing choice with a stated end, so Ev is not asked about it separately. For Ev should carry one line: "`u_ref`/`v_ref` stay until the general-anchor row renames or removes them."
   - Condition: that row gets the obligation written in, "rename or remove, never re-document under the same name". I also take A's addition: the classification counts reads that use `axis` alone and reads that want an in-plane basis with no seam meaning.
