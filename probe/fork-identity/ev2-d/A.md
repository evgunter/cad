DESIGNER A — fork 272, point (d), final. Read B-d and C-d; checked `crates/geom-core/README.md` DL5.3 and `geom-brep/src/certify.rs` (`NurbsLaneOf`, `NoLane`) on current main. Nothing run.

**1. C's point moves me. I withdraw the bounded mint and return to the accessor.**
- DL5.3's ratified text is, word for word, the case in front of us: "A scalar's lanes are an associated type with no values at a dual, never an `Option` a dual answers `None` to." That is Ev's own statement of how a pass that runs at every scalar reaches something one scalar lacks: an associated type that is empty there.
- The tree does it exactly as C says: `pub trait NurbsLaneOf<T: Real>`, `impl<T: Certifies> NurbsLaneOf<T> for NurbsLane<T>`, and `impl<T: Real> NurbsLaneOf<T> for geom_core::NoLane` (`certify.rs` ~3127–3149), with the pass generic over the lane type. `Real::Expression` with an empty type at every numeric scalar, and a vacuous impl of the question's trait for that empty type, is the same shape.
- So my reading last round, that the clause's examples "exclude by bound, not by an absence", was wrong about the clause's own cited example. DL5.3 has both devices and says which is for what: `Certifies` bounds code whose whole meaning is certification; the empty associated type serves code that has a meaning at every scalar and uses a lane where there is one.
- What DL5.3 forbids is an impl that chooses `None`. With an empty `Expression`, `None` is the only value of the type; no impl chooses. My "it reads as a check in generic code" was about how it reads, and DL5.3 has already ruled that this is the accepted reading.
- My remaining argument (a numeric scalar "implements something that exists only for `Sym`'s sake") is answered the same way: every scalar already declares `Carries` and its lane types on those terms.
- And B and C are right about the cost I waved through: a per-scalar direction item splits the mint against [one-way], needs an impl or inherited default at `f64`, `Interval`, `Dual`, `Probe` and `Sym`, has to live in `profile` or `geom-core` below `ProfileLane`, and `arc_rim_normal_weight` would need a second one. Under the new "needed before safe" paragraph, that item is the thing that is not needed.

**2. Is there a form that is both?** Yes, and it is DL5.3's own split:
- Code whose whole meaning is symbolic is bounded: it takes `T: Symbolic` (implemented by `Sym` alone). That is where `register_equal` and `free_identity` go later, and where a helper that only makes sense with an expression lives.
- The mint is one body, generic over the scalar, reaching the question through `Real::Expression`, which is empty at every numeric scalar.
Rust cannot give one mint body without either the associated type or a per-scalar item; the associated type is the one the design already uses.

**3. Final position: the accessor, as B and C have it, with `Symbolic` as the bound for purely symbolic code. Not a choice for Ev from my side.** The three of us now recommend one shape.

The sentence per admitted type, for the record, since the clause asks for it:
- `Real::Expression` at `Sym`: this value's node in the replay's expression graph. At every numeric scalar: a type with no values; the scalar carries no expression.
- The question: only on a real expression; the empty type's impl has no value to be called on.
- The mint: at every scalar, the unit direction of the vector; at `Sym`, as the exact axis where a component is proved identically zero.

**Text.** I accept the merged `[scalars]` bullet with my (b) amendment, and with the citation in the form I gave for the accessor case, adjusted to the precedent C found, which is the better one to name: after "by a type with no values" add "([unrepresentable]: a signature is a claim; `crates/geom-core/README.md` DL5.3 is the same device for a dual's lanes)". E12's sentence as you wrote it.

Withdrawn: my bounded-form opening for the bullet and the bounded-form E12 sentence from the last message.
