---
id: an-instance-defines-one-output-of-its-copies
kind: ruling
title: What an instance of a part defines as its outputs, now that the edit door cannot see the part's copies (and a placement's outputs, by the same rule)
status: open
opened: 2026-10-11
priority: P0
needs_ev: true
refs: [a-placement-is-the-bundle-of-mates, an-instance-defines-one-body-per-part-placement]
---

This came out of INTENT stage 3 B (`a-placement-is-the-bundle-of-mates`, fork B3). Two designers weighed it; their first reports and the round they took are in fork-log row 116.

**The gap.** Spec §1, from FORK-S3M (row 97, PR 4326), says an instance defines "one output per world placement of the part … a re-pin that adds a world copy mints a port no placement reads yet". But output variables are minted at the insert door (`Mint::outputs_of_insert`). Neither that door nor `UpdateReference`'s `apply` can see the part, and A13 says they must not. So no door can mint those ports.

**Recommendation (converged after one round; the build proceeds on it):**
- **The instance's output.** An instance defines one fixed output, `copies: Bodies`. It enumerates the part's copies, and each member is keyed by the copy's identity in the part: the part's placement node id plus the shape read it copies, which is FORK-DM4's keying.
- **Nothing recorded on the instance.** Nothing about the part is stored there, and `UpdateReference` stays a pin.
- **Reading one copy.** A reader of one copy holds a member read, `inst.copies[c]`. Whether `c` exists in the pinned part is decided at evaluation. A key the part no longer has leaves its reader unresolved and typed, never re-pointed.
- **One rule for placements.** A placement defines one output of the shape of what it reads (the `PortKind::Placed` precedent), so `Place` and the instance follow one rule, and no port count depends on a list.
- **The instance is still a placement.** Its constraints move the whole part. The GUI's "place part" writes those constraints, not a second `Place` over the copies.

**Alternative weighed:** the instance records its part's copy list and mints one port per recorded key. `UpdateReference` would carry the list, and evaluation would check it against the pin. This keeps row 97's wording, but it stores a function of the pin on the host, kept in step by hand. It also needs a keyed port arm on both `Place` and the instance, or else two rules.

**What changes:** D10 Operations (the instance clause, and a placement's output shape) and spec §1/§3 (the instance and placement bullets, the `eval/parts.rs` line). Spec §0's "FORK-DM4 is open" is also corrected: FORK-DM4 was ratified on PR 4462 and built on PR 4527.

**Named interims for B:**
- A part placed by a pattern delivers its family flat under its one key, until `work/refs/an-indexed-read-of-rank-two-has-no-family` lands.
- Selecting one copy's face goes through the part-local name path, as today.
- Whether `Selection` must hold a member read is G's question. The designers differed on it, and neither was sure.
