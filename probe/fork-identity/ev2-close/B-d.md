Designer B — addendum (d). I read `[unrepresentable]`'s "A signature is a claim too" and designer.md's two new rules on main.

**The one-sentence test, applied** (likely):
- **`expression()` at `f64`, `Interval`, `Dual`, `Probe`:** "this scalar carries no expression of the document's variables, so nothing symbolic is offered and the general spelling is the only one." That is a fact about the type, not an answer to a question. Its result type, `Option` of a type with no values, has `None` as its only inhabitant. No body decides anything, and the compiler proves it. It is the same kind of sentence `Real::Carries` already gives `Dual` ("carries no certificate", `Uncertified`). So the accessor passes.
- **`expression()` at `Sym`:** "this value's node in the document's expression graph."
- **The question itself (`Expression::identically_zero`)** is admitted only by `Sym`'s expression type. No numeric scalar is admitted to it, which is the bound Ev's clause asks for, by type. Its sentence there: "a proof that the value is zero as a function of the document's variables wherever the build takes the decisions it took."
- **The mint** (`T: Decide`) has a meaning at every scalar: "the segment's unit direction, spelled exactly where `T` can prove an axis, as the quotient otherwise." So bounding the mint to exclude numeric scalars would be wrong. It must run everywhere.

**Bounding the question's callers instead.**
- The alternative reading is a `Symbolic` trait implemented only by `Sym`, with the mint bounded by it. That fails the test the other way: the mint has a meaning at `f64`, and Rust gives a generic body no way to call `Symbolic` methods for some `T` only.
- The bounded form is therefore a separate mint per scalar: a sealed per-scalar trait, as `ProfileLane` is, with the quotient written at each numeric impl.
- That answers the clause no better, since each of its impls is still a sentence per admitted type. It splits one meaning across four bodies.
- So the accessor is the answer that keeps one signature per meaning.

**"Ask whether it is needed."** Without `Real::Expression`, the mint must either ask a question on `Real` that numeric scalars answer "no" (what Ev rejected), or be written per scalar. One associated type is the smallest thing that removes both. Nothing else in the tree needs it yet, so it is not added for future use. `register_equal` and `free_identity` would use it only when they are moved.

**Cite the clause: yes.** Append to the merged `[scalars]` bullet's first sentence:

> … where there is no expression. This is [unrepresentable]'s "a signature is a claim": a numeric scalar has no meaning for the question, so no signature admits it, and its sentence for `Real::Expression` is that it carries none.

This changes nothing else in my closing-round reply, including the two amendments.
