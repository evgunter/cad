---
id: kernel-verbs-teapot-paragraph-predates-the-canal
kind: issue
title: docs/KERNEL-VERBS.md's teapot paragraph names two wrong boolean pairs and a spout the scene no longer builds
status: open
opened: 2026-09-15
---


## Finding (SUITE `D403`'s sweep, PR 2626)

`docs/KERNEL-VERBS.md`'s paragraph beginning *"The Utah teapot has been
MET"* describes a scene that no longer exists. Three claims, each false
against `demos/tour/src/teapot.rs` as it stands:

1. **"handle ∪ pot is torus × cylinder"** — the pot's belly is a
   SPHERE. The scene's own source says so at the pin (*"The pot's belly
   is a SPHERE now, not the squared pot's cylinder"*) and
   `teapot::wall_probes` prints the payload it pinned:
   `CurvedPairUnsupported { op: None, operand: B, kind: Torus,
   other_kind: Sphere }`. `demos/README.md` carried the same stale pair
   and was corrected in PR 2626.
2. **"spout ∪ pot is cone × plane, both `CurvedPairUnsupported`"** —
   the spout union is not a pair-rung refusal at all any more. A loft's
   walls are `Nurbs` and the pair gate HAS an arm for `Nurbs`, so the
   request gets past that rung and dies one door in, on an EDGE of
   operand B: `CurvedEdgeUnsupported { operand: B, .. }`, the canal's
   own seams. `demos/README.md` already says this correctly, so the two
   documents disagree with each other today.
3. **"the teapot's own spout is not attempted … the scene's spout is a
   straight cone frustum tilted into place, a spout the way a LATHE
   would make one"** — the spout is seven annular sections on the
   tangent frames of a circular arc, skinned by ONE `Node::Loft` and
   placed by `Node::Transform`. It bends, tapers and thins, which is
   why no revolve reaches it at any axis.

## Why this is a row and not a substitution

(1) and (2) are stale facts: a reader with the payloads in hand can
correct them. **(3) is a wrong mental model of the scene**, and the
sentences around it are built on it — the paragraph reasons from "a
LATHE would make one" to what the teapot still WAITS ON (*"taper /
variable-section sweep and the canal family"*), and that inference is
about a shape the scene replaced. Substituting nouns would leave an
argument standing whose premise is gone. It needs a reader who knows
what the paragraph was FOR — which of these waits are still live on
CURVED's register — rather than a search and replace, which is why
`D403`'s lane stopped and filed instead of editing.

Precedent for the shape: `work/blend/kernel-verbs-cap-pair-ulp-claim-stale.md`.

## Home

CURVED — `plan.md`'s charter is *"`docs/KERNEL-VERBS.md`'s Wave 2
remainder plus S-MATE's exit residue"*, so this document's claims are
this program's ground. `work.py territory` reports the path as unowned
because it reads `paths` globs and the charter claims the document in
prose.
