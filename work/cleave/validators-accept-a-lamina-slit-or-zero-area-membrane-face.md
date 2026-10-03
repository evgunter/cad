---
id: validators-accept-a-lamina-slit-or-zero-area-membrane-face
kind: issue
title: no validator refuses a zero-thickness self-slit in one plane face or a zero-area two-edge membrane face; both pass tiers 1-3 and 3' and break consumers
status: open
opened: 2026-10-03
priority: P2
cost: M
---


Measured 2026-10-03 (same measurement as `tessellator-panics-on-a-self-slit-face-every-validator-passes`). Two hand-built states pass tiers 1, 2, 3, 3′ and the at-rest gate:
- **a3, a self-slit:** a plane face carrying a two-edge ring of coincident antiparallel edges where each edge has that same face on both sides — a zero-thickness crack in one face (κ_rel = 0), which #131's rule treats as a lamina wedge to refuse at check 4. The tessellator then panics on it.
- **a2, a zero-area membrane face:** a 2-gon face of zero area bounded by two coincident antiparallel edges, on the parent face's surface. The tessellator refuses it (`Triangulation`), `union` refuses it (`NonMaximalFaces`).
No check compares ring against ring, and nothing rejects two coincident distinct edges sharing both vertices whose faces make no legal wedge. NOT the doubled-slit form #131 legalises (each edge shared with a DISTINCT other face, wedge 2π, as in a declared tangent kiss) — that one must stay legal. Owner likely RESTFRONT (validate.rs); check 4's wedge rule (#131 lamina arm) is the natural home.
