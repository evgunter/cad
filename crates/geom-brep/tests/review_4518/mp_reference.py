import sys, struct, mpmath as m
m.mp.prec = 300
f = lambda h: struct.unpack('<d', bytes.fromhex(h)[::-1])[0]
def ulp(x):
    import math; return math.ulp(abs(x)) if x else 5e-324
stats = {}
for line in open(sys.argv[1]):
    v = [f(h) for h in line.split()]
    q, p, n, a, new, old = v[0:3], v[3:6], v[6:9], v[9], v[10:13], v[13:16]
    Q=[m.mpf(x) for x in q]; P=[m.mpf(x) for x in p]; N=[m.mpf(x) for x in n]
    L=m.sqrt(sum(x*x for x in N)); N=[x/L for x in N]
    s, c = m.sin(m.mpf(a)), m.cos(m.mpf(a))
    V=[P[i]-Q[i] for i in range(3)]
    d=sum(N[i]*V[i] for i in range(3))
    cr=[N[1]*V[2]-N[2]*V[1], N[2]*V[0]-N[0]*V[2], N[0]*V[1]-N[1]*V[0]]
    T=[Q[i]+V[i]*c+cr[i]*s+N[i]*d*(1-c) for i in range(3)]
    rad = float(m.sqrt(sum(x*x for x in V)))
    sc = max(abs(x) for x in p+q)
    mq=max(abs(x) for x in q)
    key=(0 if mq==0 else min([1,1e3,1e5,3.7e7],key=lambda b:abs(mq/b-0.6)), min([1e-3,1,1e3,2.3e6],key=lambda b:abs(rad/b-1)))
    en = max(abs(float(T[i]-new[i])) for i in range(3))
    eo = max(abs(float(T[i]-old[i])) for i in range(3))
    st = stats.setdefault(key, [0,0,0,0.0,0.0,0])
    st[0]+=1; st[1]+= en>eo; st[2]+= eo>en; st[3]=max(st[3],en/ulp(sc)); st[4]=max(st[4],eo/ulp(sc))
for k in sorted(stats): print(k, "n=%d new_worse=%d old_worse=%d max_err_new=%.1f max_err_old=%.1f (ulps of max coord)" % tuple(stats[k][:5]))
