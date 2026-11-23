(4c implementer) **Measured: the bored ball's signed volume on origin/main 7691e4f872 against the head 6e9fa7091b.** Ball 1.05, bore 1, placed obliquely, default ε, `mesh::tessellate` then `validate::signed_volume`. Cells read: triangles, signed volume, main | head. `check_mesh` passes on every Ok cell on both trees.

**Seam split at 0.5:**
- sign −1, δ 3: 62, **−0.097454** | 64, **−0.071501**
- sign −1, δ 1: 62, **−0.097454** | 64, **−0.071501**
- sign −1, δ 0.3: 62, **−0.097454** | 64, **−0.071501**
- sign −1, δ 0.1: 64, **−0.008671** | 88, **−0.038972**
- sign +1, δ 3: 62, +0.097454 | 64, +0.071501
- sign +1, δ 1: 62, +0.097454 | 64, +0.071501
- sign +1, δ 0.3: 102, +0.038676 | 104, +0.028201
- sign +1, δ 0.1: 152, +0.059071 | 220, +0.107613

Finer cells, for scale:
- sign −1, δ 0.05: 120, +0.099292 | 120, +0.099292 (identical).
- sign −1, δ 0.04: 134, +0.094792 | 136, +0.099944.
- sign −1, δ 0.035: 142, +0.095464 | 110, +0.100192.
- sign +1, δ 0.05 to 0.035: main +0.082 to +0.103, head +0.102 to +0.117.

**Unsplit** (both trees identical): `OverusedSide` at sign +1 δ 3 to 0.3 and sign −1 δ 3 to 0.1. It meshes at the finer δ, at +0.0976 to +0.1167.

**Outcome: the first.** Main returns the same sign at all four sign −1 cells: negative at δ 3, 1, 0.3 and 0.1, as the head does. Main's own chart lane meshes this body inside out there. **It is the regime's (δ past the shell's thickness, both sheets on one set of chords), not this unit's.** By cell:
- At δ 3, 1, 0.3 the head is less negative than main (−0.0715 against −0.0975).
- At δ 0.1 it is more negative (−0.039 against −0.0087).
- On sign +1 both trees stay positive but well short of the body. The head is closer at δ 0.1 and below, and farther at δ 3 to 0.3.
- Neither tree is right in this regime.
- The magnitudes differ because the trees chord and cut differently. The sign agrees.

**What `a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge` holds and does not hold:**
- **Holds:**
  - the cell meshes;
  - `check_mesh` passes (manifold, every edge two uses);
  - the cut count per cell, and the count relation where main's unsplit body meshes;
  - `geometry_enclosing`: distances within δ of the body, and the volume against the body's within `check_volume`'s δ·area bound.
- **Does not hold:** the volume's SIGN or size on these coarse cells. `check_volume`'s δ·area bound there exceeds the body's whole volume, so a mesh turned inside out passes it, on main's chart lane as on the walk. The row says nothing about orientation once δ is past the shell's thickness. If you file it, that is the gap, together with `check_volume`'s bound being vacuous there.

Nothing built; the tree is clean at 6e9fa7091b. The probe lives only in `/home/user/wt-base` (uncommitted).
