import sys,collections
def load(p):
    d={}
    for l in open(p):
        if l.startswith('TALLY'): continue
        f=l.rstrip('\n').split('\t'); d[f[0]]=f
    return d
import os
for k in [x for x in ['sph','ps','cyl','wc','cone','dis'] if os.path.exists(sys.argv[1]+'.'+x)]:
    b,h=load(sys.argv[1]+'.'+k),load(sys.argv[2]+'.'+k)
    c=collections.Counter()
    for lab in h:
        x,y=b.get(lab),h[lab]
        if x is None: c['missing']+=1; continue
        if x[1]=='BUILT' and y[1]=='BUILT':
            c['built-same' if x[2:4]==y[2:4] else 'built-DIFF']+=1
            if x[2:4]!=y[2:4]: print('DIFF',lab,x[2:4],y[2:4])
        elif x[1]=='BUILT': c['base-built/head-'+y[1]]+=1; print('LOST',lab,y[2][:120])
        elif y[1]=='BUILT': c['base-'+x[1]+'/head-built']+=1
        else: c['both-'+x[1]+'/'+y[1]+(' same' if x[2:]==y[2:] else ' changed')]+=1
    print(k,dict(c))
