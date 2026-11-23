//! **The frame seam census** — one home for the classification the
//! three corpus instruments (`editor-core`'s Band 4 documents,
//! `step-export`'s fixture bodies, `step-import`'s wild files) each
//! used to carry a verbatim copy of.
//!
//! `Vec3::orthonormal_basis` crosses the normal with the world axis its
//! own components choose: `e_z` when `|n.z| ≤ max(|n.x|, |n.y|)/2`,
//! `e_y` otherwise. The equality is the construction's one
//! discontinuity, and the only class an enclosure of positive width can
//! fail to decide. It is NOT a cone: its elevation runs from
//! `atan(1/(2√2)) ≈ 19.47°` on the diagonal meridians to
//! `atan(1/2) ≈ 26.57°` on `n.x = 0` and `n.y = 0`, a band that carries
//! no axis direction, no wall, no cap and no 30°, 45° or 60° chamfer.
//!
//! **What a count of normals on that set measures.** It bounds the
//! EXPOSURE rather than measuring it: a face whose normal is on the
//! seam only stores a hulled `u_ref` if its frame came from this
//! constructor at an enclosure scalar. A blend's strips and corner
//! patches mint their own `u_ref` (`sweep`'s `blend::arms` stores the
//! tangent for a strip and a `perp_unit` for a corner patch), so their
//! frames are exact whatever the normal does. A census row of zero is
//! therefore a strong statement and a row of N is an upper bound on N
//! faces at risk.
//!
//! The three-component order — smallest magnitude wins, the rule the
//! spec first named — is counted beside it as the contrast: its tie set
//! is where the two smallest magnitudes are equal, which every
//! axis-aligned normal sits exactly on.

/// The ratio `ρ` in the comparison `|n.z| ≤ ρ·max(|n.x|, |n.y|)`.
/// Dyadic, so the comparison is exact at every scalar.
pub const SEAM_RATIO: f64 = 0.5;

/// One corpus's planar normals, classified.
#[derive(Default, Clone, Copy)]
pub struct SeamClasses {
    /// Normals exactly on `|n.z| = ρ·max(|n.x|, |n.y|)`.
    pub on_seam: usize,
    /// Normals off it.
    pub off_seam: usize,
    /// Normals the comparison sends to the `e_z` candidate.
    pub e_z_arm: usize,
    /// …and to the `e_y` candidate.
    pub e_y_arm: usize,
    /// Normals on the tie set of the rule this construction did NOT
    /// take: an order over all THREE components, tied wherever the two
    /// smallest magnitudes are equal.
    pub three_way_tie: usize,
}

impl SeamClasses {
    /// Classifies one planar normal, given componentwise.
    pub fn add(&mut self, (x, y, z): (f64, f64, f64)) {
        let other = SEAM_RATIO * x.abs().max(y.abs());
        if z.abs() <= other {
            self.e_z_arm += 1;
        } else {
            self.e_y_arm += 1;
        }
        if z.abs() == other {
            self.on_seam += 1;
        } else {
            self.off_seam += 1;
        }
        let mut m = [x.abs(), y.abs(), z.abs()];
        m.sort_by(f64::total_cmp);
        if m[0] == m[1] {
            self.three_way_tie += 1;
        }
    }

    /// Folds another body's or document's counts in.
    pub fn merge(&mut self, o: Self) {
        self.on_seam += o.on_seam;
        self.off_seam += o.off_seam;
        self.e_z_arm += o.e_z_arm;
        self.e_y_arm += o.e_y_arm;
        self.three_way_tie += o.three_way_tie;
    }

    /// Planar faces counted.
    pub fn planes(&self) -> usize {
        self.on_seam + self.off_seam
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The classification itself, at the directions whose class the
    /// construction's placement argument depends on.
    #[test]
    fn the_seam_carries_no_axis_direction_and_no_chamfer() {
        let mut c = SeamClasses::default();
        for n in [
            (1.0, 0.0, 0.0),
            (0.0, -1.0, 0.0),
            (0.0, 0.0, 1.0),
            (0.6, 0.8, 0.0),
            // 45°, 30° and 60° chamfers, and a corner facet.
            (
                0.0,
                core::f64::consts::FRAC_1_SQRT_2,
                core::f64::consts::FRAC_1_SQRT_2,
            ),
            (0.0, 0.5, 3f64.sqrt() / 2.0),
            (0.0, 3f64.sqrt() / 2.0, 0.5),
            (1.0 / 3f64.sqrt(), 1.0 / 3f64.sqrt(), 1.0 / 3f64.sqrt()),
        ] {
            c.add(n);
        }
        assert_eq!(c.on_seam, 0, "a direction CAD geometry uses is on the seam");
        assert_eq!(c.planes(), 8);
        // The contrast: every axis direction ties the three-component
        // order, and so does the body diagonal.
        assert_eq!(c.three_way_tie, 4);
    }

    /// The seam's two ends, which ARE on it: the exact rational corner
    /// `(2/3, 2/3, 1/3)` and the meridian point `(0, 2, 1)/√5`.
    #[test]
    fn the_seams_own_points_are_counted_on_it() {
        let mut c = SeamClasses::default();
        c.add((2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0));
        let r = 2.0 / 5f64.sqrt();
        c.add((0.0, r, 0.5 * r));
        assert_eq!(c.on_seam, 2);
        assert_eq!(c.e_z_arm, 2, "the tie takes the e_z arm");
    }
}
