"""Every fin-scene cell's kernel volume against a manifold3d (Mesh64) oracle
over the operands' tessellations, plus exact self-crossing and winding of
each built result."""
import json, sys, os
import numpy as np, manifold3d as M
sys.path.insert(0, os.path.dirname(__file__))
from check import selfx, windings, signed_volume

def man(tris):
    t=np.array(tris,dtype=np.float64).reshape(-1,3)
    uniq,inv=np.unique(t,axis=0,return_inverse=True)
    m=M.Manifold(M.Mesh64(vert_properties=np.ascontiguousarray(uniq),tri_verts=np.ascontiguousarray(inv.reshape(-1,3).astype(np.uint32))))
    assert m.status()==M.Error.NoError, m.status()
    return m

rows=[json.loads(l) for l in open(sys.argv[1])]
ring={(r['name'],r['pose']):r for r in rows if r['kind']=='ring'}
probe={(r['k'],r['pose']):r for r in rows if r['kind']=='probe'}
void={(r['bottom'],r['pose']):r for r in rows if r['kind']=='void'}
block={r['pose']:r for r in rows if r['kind']=='block'}
rng=np.random.default_rng(2)
check_geo = len(sys.argv)>2
for r in rows:
    if r['kind']=='void':
        y=man(ring[('fin '+r['bottom'],r['pose'])]['tris']); b=man(block[r['pose']]['tris'])
        print(json.dumps({'kind':'void','bottom':r['bottom'],'pose':r['pose'],'kernel':r['volume'],'oracle':(b-y).volume()}))
    if r['kind']!='cell': continue
    x=man(probe[(r['k'],r['pose'])]['tris'])
    y=man(ring[('fin '+r['bottom'],r['pose'])]['tris']) if r['scene']=='fin' else man(void[(r['bottom'],r['pose'])]['tris'])
    op=r['op']
    o={'x - y':x-y,'y - x':y-x,'x u y':x+y,'y u x':y+x,'x n y':x^y,'y n x':y^x}[op].volume()
    res={k:r[k] for k in ('bottom','scene','k','op','pose','outcome','volume')}
    res['oracle']=o
    if check_geo and r['tris']:
        res['selfx']=len(selfx(r['tris'])); res['wind']=windings(r['tris'],rng,nrand=1000); res['meshvol']=signed_volume(r['tris'])
    print(json.dumps(res),flush=True)
