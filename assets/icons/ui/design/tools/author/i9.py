from lib import *
from i2 import pt, arrowhead
from i4 import G,B
import random
SEED=1
ICONS={}
def icon(fn): ICONS[fn.__name__]=fn; return fn
DISC=ell(32,32,24)
def disc(ic,fillc,cid):
    ic.fill(DISC,fillc); ic.clip_begin(cid,DISC)
def disc_end(ic):
    ic.clip_end(); ic.stroke(DISC,INK,OW)           # the same circle on every analysis icon
def blob(cx,cy,r,rnd,n=7):
    ps=[]
    for k in range(n):
        a=2*math.pi*k/n+rnd.uniform(-.25,.25); rr=r*rnd.uniform(.6,1.15); ps.append((cx+rr*math.cos(a),cy+rr*math.sin(a)))
    mid=lambda p,q:((p[0]+q[0])/2,(p[1]+q[1])/2)
    m0=mid(ps[0],ps[1]); d='M%s %s'%(f(m0[0]),f(m0[1]))
    for i in range(1,n+1):
        p=ps[i%n]; m=mid(ps[i%n],ps[(i+1)%n]); d+='Q%s %s %s %s'%(f(p[0]),f(p[1]),f(m[0]),f(m[1]))
    return d+'Z'
def smooth5(t): return t*t*t*(t*(t*6-15)+10)

def contour_loops(node,G,x0,cell,t):
    """marching squares: closed loops where the field crosses t; the outer ring of nodes counts as below t"""
    def val(i,j): return node[j][i] if 0<i<G and 0<j<G else -1e9
    def pt(e):
        k,i,j=e
        a,b=(val(i,j),val(i+1,j)) if k=='h' else (val(i,j),val(i,j+1))
        a=max(a,t-1.0); b=max(b,t-1.0); u=(t-a)/(b-a) if b!=a else .5
        return (x0+(i+u)*cell,x0+j*cell) if k=='h' else (x0+i*cell,x0+(j+u)*cell)
    table={1:[('L','B')],2:[('B','R')],3:[('L','R')],4:[('T','R')],5:[('L','T'),('B','R')],6:[('T','B')],7:[('L','T')],
           8:[('L','T')],9:[('T','B')],10:[('L','B'),('T','R')],11:[('T','R')],12:[('L','R')],13:[('B','R')],14:[('L','B')]}
    adj={}
    for j in range(G):
        for i in range(G):
            idx=(val(i,j)>t)*8+(val(i+1,j)>t)*4+(val(i+1,j+1)>t)*2+(val(i,j+1)>t)
            e={'T':('h',i,j),'B':('h',i,j+1),'L':('v',i,j),'R':('v',i+1,j)}
            for p,q in table.get(idx,[]):
                adj.setdefault(e[p],[]).append(e[q]); adj.setdefault(e[q],[]).append(e[p])
    seen=set(); loops=[]
    for start in adj:
        if start in seen: continue
        loop=[start]; seen.add(start); prev=None; cur=start
        while True:
            nxt=[n for n in adj[cur] if n!=prev]
            nxt=[n for n in nxt if n not in seen] or [n for n in nxt if n==start]
            if not nxt: break
            prev,cur=cur,nxt[0]
            if cur==start: break
            seen.add(cur); loop.append(cur)
        if len(loop)>3: loops.append([pt(e) for e in loop])
    return loops

def smooth_loop(p,passes=2):
    for _ in range(passes):
        n=len(p); p=[((p[i-1][0]+2*p[i][0]+p[(i+1)%n][0])/4,(p[i-1][1]+2*p[i][1]+p[(i+1)%n][1])/4) for i in range(n)]
    return p
def loop_path(p):
    p=p[::2] if len(p)>24 else p; n=len(p)
    mid=lambda a,b:((a[0]+b[0])/2,(a[1]+b[1])/2)
    m0=mid(p[0],p[1]); d='M%s %s'%(f(m0[0]),f(m0[1]))
    for i in range(1,n+1):
        q=p[i%n]; m=mid(q,p[(i+1)%n]); d+='Q%s %s %s %s'%(f(q[0]),f(q[1]),f(m[0]),f(m[1]))
    return d+'Z'

@icon
def curvature_map():
    ic=Icon('curvature_map','Curvature map'); disc(ic,P2,'disc-curv')
    rnd=random.Random(SEED)
    hills=[(22,25,1.0,7.6),(45,42,0.95,8.0),(46,15,0.8,5.2),(12,47,0.75,5.4)]     # a few hills of different size, so the bands nest and merge
    lat=[[rnd.random() for _ in range(5)] for _ in range(5)]
    def ripple(x,y):
        gx,gy=(x-4)/56*3,(y-4)/56*3; i,j=int(min(max(gx,0),2.99)),int(min(max(gy,0),2.99)); fx,fy=smooth5(gx-i),smooth5(gy-j)
        a=lat[j][i]*(1-fx)+lat[j][i+1]*fx; b=lat[j+1][i]*(1-fx)+lat[j+1][i+1]*fx
        return a*(1-fy)+b*fy
    G=48; x0=4.0; cell=56.0/G
    def field(x,y): return sum(h*math.exp(-((x-hx)**2+(y-hy)**2)/(2*s_*s_)) for hx,hy,h,s_ in hills)+0.14*ripple(x,y)
    node=[[field(x0+i*cell,x0+j*cell) for i in range(G+1)] for j in range(G+1)]
    for t,col in zip((0.36,0.6,0.85),(A1,A2,A3)):
        loops=contour_loops(node,G,x0,cell,t)
        d=''.join(loop_path(smooth_loop(l)) for l in loops)
        if d: ic.fill(d,col,rule=True)
    disc_end(ic); return ic
def curvature_map():
    ic=Icon('curvature_map','Curvature map'); disc(ic,P2,'disc-curv')
    rnd=random.Random(SEED); N=3
    lat=[[rnd.random() for _ in range(N+2)] for _ in range(N+2)]
    lat2=[[rnd.random() for _ in range(2*N+2)] for _ in range(2*N+2)]
    def vnoise(L,n,x,y):
        gx,gy=x*n,y*n; i,j=int(min(gx,n-1e-6)),int(min(gy,n-1e-6)); fx,fy=smooth5(gx-i),smooth5(gy-j)
        a=L[j][i]*(1-fx)+L[j][i+1]*fx; b=L[j+1][i]*(1-fx)+L[j+1][i+1]*fx
        return a*(1-fy)+b*fy
    G=48; x0=4.0; cell=56.0/G
    node=[[0.8*vnoise(lat,N,i/G,j/G)+0.2*vnoise(lat2,2*N,i/G,j/G) for i in range(G+1)] for j in range(G+1)]
    inside=sorted(node[j][i] for j in range(G+1) for i in range(G+1) if math.hypot(x0+i*cell-32,x0+j*cell-32)<24)
    levels=[inside[int(len(inside)*p)] for p in (0.40,0.62,0.82)]
    for t,col in zip(levels,(A1,A2,A3)):
        loops=contour_loops(node,G,x0,cell,t)
        d=''.join(loop_path(smooth_loop(l)) for l in loops)
        if d: ic.fill(d,col,rule=True)
    disc_end(ic); return ic
@icon
def environment_map():
    ic=Icon('environment_map','Environment map'); disc(ic,A1,'disc-env')
    ic.fill('M6 42C16 30 30 28 42 35C50 40 56 40 62 37L62 64L6 64Z',P3)           # far hill
    ic.fill('M2 54C14 40 30 44 42 49C50 52 58 50 64 47L64 66L2 66Z',A3)            # near hill
    ic.fill(ell(45,19,6),GAP)                                                      # sun
    disc_end(ic); return ic
@icon
def iso_analysis():
    ic=Icon('iso_analysis','Isoparametric analysis'); disc(ic,P1,'disc-iso')
    r=24
    ic.stroke('M32 8L32 56',A3,2.2)
    for ph in (30,60):
        rx=r*math.sin(math.radians(ph)); ic.stroke(ell(32,32,rx,r),A3,2.2)
    for lat in (-60,-30,0,30,60):
        y=32-r*math.sin(math.radians(lat)); rx=r*math.cos(math.radians(lat))
        ic.stroke(arcpts(32,y,rx,rx*0.32,0,180),A3,2.2)
    disc_end(ic); return ic
@icon
def zebra():
    ic=Icon('zebra','Zebra'); disc(ic,P1,'disc-zebra')
    def curve(base,bulge,n=14):
        return [(8+48*i/n, base-bulge*(1-((8+48*i/n-32)/26)**2)) for i in range(n+1)]
    for k in range(7):
        top=curve(-2+9.5*k,4+1.4*k); bot=[(x,y+5) for x,y in curve(-2+9.5*k,4+1.4*k)]
        ic.fill(poly(top+bot[::-1]),INK)
    disc_end(ic); return ic
@icon
def iso_analysis_small():
    ic=Icon('iso_analysis','Isoparametric analysis'); ic.fam='small'; disc(ic,P1,'disc-iso')
    r=24
    ic.stroke('M32 8L32 56',A3,3.2); ic.stroke(ell(32,32,r*0.6,r),A3,3.2); ic.stroke(arcpts(32,32,r,r*0.32,0,180),A3,3.2)
    disc_end(ic); return ic
@icon
def draft_analysis():
    ic=Icon('draft_analysis','Draft analysis')
    ic.stroke(poly([(8,56),(8,8),(56,8),(56,56)],False),P3,3.2,dash='0.1 5.5')
    ic.stroke(poly([(8,56),(56,56),(44,8),(20,8)]),INK,OW,fill=A2)       # fills the square, wide at the bottom, top on the top edge
    return ic
def draft_analysis():
    ic=Icon('draft_analysis','Draft analysis')
    d=poly([(14,54),(50,54),(41,17),(23,17)]); ic.stroke(d,INK,OW,fill=A2)
    ic.stroke(poly([(8,54),(8,9),(56,9),(56,54)],False),P3,3.2,dash='0.1 5.5')
    return ic
@icon
def min_radius():
    ic=Icon('min_radius','Minimum radius')
    t,ri,H,Ly,W=6,3.5,34,34,24; ro=ri+t
    def arc(r,a0,a1,n=10): return [(ro+r*math.cos(math.radians(a0+(a1-a0)*i/n)),ro+r*math.sin(math.radians(a0+(a1-a0)*i/n))) for i in range(n+1)]
    outer=arc(ro,180,270); inner=arc(ri,270,180)         # profile in (y, z): the bend's outer and inner arcs
    allp=[(0,H)]+outer+[(Ly,0),(Ly,t)]+inner+[(t,H)]
    I=Iso().fit([(0,y,z) for y,z in allp]+[(W,y,z) for y,z in allp],58,58)
    P=lambda x,y,z:I.p(x,y,z)
    # end face (x = W): grey, with the bend sector in accent
    ic.fill(poly([P(W,y,z) for y,z in allp]),P3)
    ic.fill(poly([P(W,y,z) for y,z in outer+inner]),A3)
    def strip(p,q,col): ic.fill(poly([P(0,*p),P(W,*p),P(W,*q),P(0,*q)]),col)
    strip((Ly,0),(Ly,t),P2)                      # thickness at the free end
    strip((Ly,t),(ro,t),P1)                      # flat top
    for i in range(len(inner)-1): strip(inner[i],inner[i+1],A2)      # the bend, highlighted
    strip((t,ro),(t,H),P2)                       # inside of the upright
    strip((t,H),(0,H),P1)                        # top edge of the upright
    # lines only where the surface changes angle
    for (y,z) in ((Ly,0),(Ly,t),(t,H),(0,H)): ic.stroke(poly([P(0,y,z),P(W,y,z)],False),INK,OW)
    ic.stroke(poly([P(0,y,z) for y,z in [(Ly,0),(Ly,t),(ro,t)]+inner+[(t,H),(0,H)]],False),INK,OW)   # far edge
    ic.stroke(poly([P(W,y,z) for y,z in allp]),INK,OW)                                              # outline of the end face
    # two arrows lying in the centres of the two inside faces, each along its face and pointing at the bend
    def arrow3(c,d,half=11.5,hl=9,hw=5):
        (cx,cy,cz)=c; (dx,dy,dz)=d
        p0=(cx-dx*half,cy-dy*half,cz-dz*half); p1=(cx+dx*half,cy+dy*half,cz+dz*half)     # tail and tip
        base=(p1[0]-dx*hl,p1[1]-dy*hl,p1[2]-dz*hl)
        wing=lambda sg:P(base[0]+sg*hw,base[1],base[2])
        shaft=poly([P(*p0),P(base[0],base[1],base[2])],False); head=poly([P(*p1),wing(1),wing(-1)])
        ic.stroke(shaft,GAP,AW+2.4); ic.stroke(head,GAP,2.4,fill=GAP)           # thin punch-out outline
        ic.stroke(shaft,A3,AW); ic.fill(head,A3)
    arrow3((W*0.5,(ro+Ly)/2+1,t),(0,-1,0))               # on the flat face, towards the upright
    arrow3((W*0.5,t,(ro+H)/2+1),(0,0,-1))                # on the upright's face, towards the floor
    return ic.recenter()

@icon
def sheet():
    ic=Icon('sheet','Sheet metal')
    t,ri,H,Ly,W=6,3.5,34,34,24; ro=ri+t                                  # the same sheet as min_radius, without its highlight and arrows
    def arc(r,a0,a1,n=10): return [(ro+r*math.cos(math.radians(a0+(a1-a0)*i/n)),ro+r*math.sin(math.radians(a0+(a1-a0)*i/n))) for i in range(n+1)]
    outer=arc(ro,180,270); inner=arc(ri,270,180)
    allp=[(0,H)]+outer+[(Ly,0),(Ly,t)]+inner+[(t,H)]
    I=Iso().fit([(0,y,z) for y,z in allp]+[(W,y,z) for y,z in allp],58,58)
    P=lambda x,y,z:I.p(x,y,z)
    ic.fill(poly([P(W,y,z) for y,z in allp]),P3)                          # the cut end
    def strip(p,q,col): ic.fill(poly([P(0,*p),P(W,*p),P(W,*q),P(0,*q)]),col)
    strip((Ly,0),(Ly,t),P2)
    strip((Ly,t),(ro,t),P1)
    for i in range(len(inner)-1): strip(inner[i],inner[i+1],P1 if i<(len(inner)-1)/2 else P2)     # the bend takes the light of the flat it leaves
    strip((t,ro),(t,H),P2)
    strip((t,H),(0,H),P1)
    # lines only where the surface changes angle, none across the flat faces or the bend
    for (y,z) in ((Ly,0),(Ly,t),(t,H),(0,H)): ic.stroke(poly([P(0,y,z),P(W,y,z)],False),INK,OW)
    ic.stroke(poly([P(0,y,z) for y,z in [(Ly,0),(Ly,t),(ro,t)]+inner+[(t,H),(0,H)]],False),INK,OW)
    ic.stroke(poly([P(W,y,z) for y,z in allp]),INK,OW)
    return ic.recenter()
@icon
def ws_animate():
    ic=Icon('ws_animate','Animate workspace'); ic.fam='workspace'
    s=19; I=Iso(1,0,0); f_=0.42                      # one large object in the frame, ink outlines, a single blue accent: like the other workspace icons
    top=I.p(0,s,s); bot=I.p(0,s,0)
    lerp=lambda a,b:(a[0]+(b[0]-a[0])*f_,a[1]+(b[1]-a[1])*f_)
    a1=lerp(top,I.p(0,0,s)); a2=lerp(top,I.p(s,s,s)); b1=lerp(bot,I.p(s,s,0))
    def frag(dx):
        sh=lambda p:(p[0]+dx,p[1])
        return poly([sh(a1),sh(top),sh(bot),sh(b1)],False)+poly([sh(top),sh(a2)],False)
    for dx in (-24,-12):                              # the cube's leftmost edge and its three shortened lines, drawn left to right
        d=frag(dx); ic.stroke(d,GAP,OW+5); ic.stroke(d,INK,OW)
    hexa=I.poly((0,0,s),(s,0,s),(s,s,s),(s,s,0),(0,s,0),(0,s,s))
    ic.stroke(hexa,GAP,OW+5,fill=GAP)
    I.box(ic,0,0,0,s,s,s,G)
    ys=I.p(0,0,s)[1]-10
    xl=top[0]+2; xr=I.p(s,0,0)[0]-1
    ic.stroke(poly([(xl,ys),(xr-9,ys)],False),A3,AW); ic.fill(arrowhead(xr+3,ys,1,0,12,13),A3)      # the blue arrow above, pointing right
    return ic.recenter()
