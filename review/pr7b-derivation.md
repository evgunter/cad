# PR 7b hand re-derivation (Charter A)

## 1. Boehm insertion window (insert_once_ring, compose.rs:296-331)
Insert u (current mult s) into U, degree p, span k = last index with U_k <= u.
Textbook (NURBS Book A5.1 / Boehm): new coefficients
  Q_i = c_i                      for i <= k-p
  Q_i = (1-a_i) c_{i-1} + a_i c_i, a_i = (u-U_i)/(U_{i+p}-U_i), for k-p+1 <= i <= k-s
  Q_i = c_{i-1}                  for i >= k-s+1
Code branches: `i+p <= k` -> c_i  (i <= k-p)  MATCH
               `i+s <= k` -> c_{i-1} + (c_i - c_{i-1})*alpha (i <= k-s)  MATCH
               else c_{i-1}  MATCH
c_{i-1}+(c_i-c_{i-1})a == (1-a)c_{i-1}+a c_i in exact arithmetic; ring evaluates
the written association with outward rounding -> enclosure of the true value.
alpha formed as ring quotient of point knot enclosures -> quotient rounding kept.
Repeated insertion to full multiplicity (m..p per distinct interior value) gives
Bezier segments; span j owns coeffs j*p..=j*p+p (full-mult control layout). OK.

## 2. Tensor product (tensor_channel)
Knot insertion in u is a linear operator on the iu index of C[iu,iv], identity on
iv; hence per-v-column u-decomposition followed by per-(u-span,a) v-row
decomposition = the tensor of the two univariate decompositions. Layout check:
stage1[su][a][jv]; stage2 extends cells[sv] per a in order -> patches[su][sv]
row-major a*(mv+1)+b, matching cell_residual's F index i*(mv+1)+j. OK.

## 3. Denominator-cleared basis rows (cell_residual)
Cell [ua,ub]x[va,vb], local ut=(u-ua)/(ub-ua). With rational u(t)=U(t)/W(t):
  ut = G/((ub-ua)W),  1-ut = H/((ub-ua)W),  G=U-ua*W, H=ub*W-U.
  W^mu B_i^mu(ut) = C(mu,i) G^i H^{mu-i} / (ub-ua)^mu.
Code computes Nc = sum_ij F_ij C(mu,i)C(mv,j) Gu^i Hu^{mu-i} Gv^j Hv^{mv-j}
   = s * W^{mu+mv} * (F cell polynomial at (u(t),v(t))),  s = (ub-ua)^mu (vb-va)^mv > 0,
s COMMON to all 4 channels (F = wx,wy,wz,w cell coeffs).
  num_d = N_d*W_C - A_d*N_w = s W^{mu+mv} [ (S^h_d o P) W_C - A_d (S^h_w o P) ]
  den   = N_w*W_C          = s W^{mu+mv} (S^h_w o P) W_C
  num_d(t)/den(t) = S_d(P(t)) - C_d(t)   EXACTLY per t (common factor cancels).
Bernstein coefficient hulls contain values on the span (convexity); interval
quotient hull(num)/hull(den) contains num(t)/den(t) for every t provided den
hull is zero-free (ring poisons otherwise). Multi-cell windows: truth at t uses
cell(t)'s polynomial, covered by cell(t)'s hull, subset of across-cell hull. SOUND.
Out-of-domain windows: clamped to edge cell = polynomial extension; kernel eval
(find_span) clamps identically -> consistent semantics.

## 4. Power tables and binomial pairs
power_table: gp[i]=g^i, hp[i]=h^i via repeated bern_mul_row (exact binomial
quotient weights per product, ring); T_i = g^i h^{m-i}, each entry degree m*q.
Binomial pair bmu[i]*bmv[j] in f64: exact because C(mu,i)C(mv,j) <= C(mu+mv,i+j)
(Vandermonde) and mu+mv <= 54 is enforced (early poison in cell_residual for the
q=0 hole; otherwise bern_mul_row's binom_row(deg>54) all-NaN poisons first).
Budget: final num product degree q(mu+mv)+q -> binom_row(q(mu+mv+1)); >54 poisons.
q=3: mu+mv<=17 (e.g. (9,8) = 54 exactly OK); (9,9)=57 NaN. Matches docs.

## 5. Window selection
wu = hull(U row)/hull(W row) contains u(t)=U(t)/W(t) on span (W>0: weights
positive, insertion = convex combos, positivity preserved). cells_touched uses
closed overlap; miss -> nearest edge cell. Conservative. OK.

Conclusion: algebra correct on paper; remaining risk is implementation slips,
attacked by the executable pinch + falsification probes.
