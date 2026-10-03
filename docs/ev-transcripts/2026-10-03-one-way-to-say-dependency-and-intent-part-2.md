# Ev's words, verbatim — part 2 of the 2026-10-03 conversation behind ruling `one-way-to-say-dependency-and-intent`

Continues commit 5f7a1c71e3. Deleted in the next commit; the ruling and
PR #3990 point here. Ev's messages verbatim; the assistant's side
summarized in brackets.

[Assistant: opened PR #3990 with the designers' converged rung (2) — same construction of the same variables, canonical per kind — and the residual split on its linear part.]

**Ev, 8 (on decision 2 of PR #3990):**

> 2. i think i've changed my mind on this kind of thing a fair number of times, so i wouldn't take anything i've said before this conversation, even recently, as decisive. i am unsure on this point because it is possible to have some really obvious, must-be-silently-accepted-or-it's-super-annoying cases that still, i think, wouldn't count under the strict rule. like if you have a chain of blocks that each say "my floor is coplanar with my neighbor's floor", and then we want to have them form a loop, or we want to stack another block on top of two of them like bricks, then, if i'm understanding the setup correctly, it'd be refused until you separately declared that last block is level with first block, or block top 1 is level with block top 2. i think basically any system that makes a bunch of equal but incomparable references is vulnerable to this. if we accept any theorem we can derive as ok then yes, it depends on how we prove it, but at least the transitive property will always hold.

[Assistant: the rule must be equality of normal forms, which is transitive by construction; under the canonical-form (2) (linear offsets, each kind modulo its own symmetry, rigid motions about shared axis objects folded) the loop and the brick stack glue, while pairwise or syntactic rules fail them; the remaining choice is how strong the normal form is — (2) canonical forms, or (3) the symbolic tier's polynomial form as a lazy in-band fallback — and whether a coincidence holding only by arithmetic beyond sums (a boss of radius w/2 centred at w/2) should glue silently.]

**Ev, 9:**

> i'm still not sure, but i'm happy to build only (2) to begin with, and extend it to (3) later if necessary, if that change wouldn't be too invasive?

[Assistant: adding (3) later is one more rung at one door, which may only turn refusals into glue; D10 states that every coincidence decision goes through one door.]
