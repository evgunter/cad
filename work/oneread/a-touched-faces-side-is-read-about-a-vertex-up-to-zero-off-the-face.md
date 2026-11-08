---
id: a-touched-faces-side-is-read-about-a-vertex-up-to-zero-off-the-face
kind: issue
title: vtxfac reads a bound's side of a touched face about the vertex's own point, which stands up to the zero band off the face, on the band alone
status: open
opened: 2026-11-03
priority: P3
cost: E
---


Found by both reviews of PR 6407 (`the-cone-sites-read-every-bounds-side-through-bound-read`, claim 4). One rated it MINOR, the other a NOTE.

`vtxfac::cone_classes` reads each edge's side of the touched face on the plain band (`PierceDatum::given(dir, reach, band)`); so does the vertex-on-face pierce (`classify_vertex_on_face`). The datum is the face's outward normal at the vertex `p` (`face_outward_normal_at(.., p, ..)`), taken as a plane through `p`. A vertex on a face stands up to the zero band off the face's actual surface.

The exposure is a **descended** edge, tied with the face at first order. Its side is its parting's, decided past `K·zero` on its own. Two terms beside the parting go uncounted:
- the first-order residue the tie leaves;
- the vertex's offset off the face.

A parting decided by a margin inside their sum can read the wrong side. Since PR 6407's A′ clause (item 3, amended 2026-11-02), that side turns straight into an In/Out class. This is the shape `vtxfac-partner-side-reads-a-partners-bounds-on-the-band-beside-its-tie` closed for partners (`Reading::tied` over the offset).

PR 6407's orchestrator ruling ("the touched face has no exposure from the cone's tie") covered only the partners' offset, which does not move the vertex's own point. It did not cover this.

The pierce reads the same way on main, so this predates PR 6407.

Owed:
- read the touched face's sides on a reading over the vertex's offset off the face (`Reading::tied` with that offset), or show the offset cannot reach a decided side;
- a row at `K = 1.4`: a vertex 0.9·zero off a curved face, an edge tied with the face at first order, whose parting clears `K·zero` by less than the residue plus the offset.
