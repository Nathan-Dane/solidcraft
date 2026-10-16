from lib import *
from i2 import pt, arrowhead, arrow, poly_reg, dashes
from i4 import G,B,face,cyl3,bez,surf_patch
ICONS={}
def icon(fn): ICONS[fn.__name__]=fn; return fn
def arc_arrow(ic,cx,cy,r,a0,a1,c=A3,w=AW,L=9,W=10):
    """clockwise (a1>a0) or counter (a1<a0) arc with head at a1"""
    ic.stroke(arcpts(cx,cy,r,r,a0,a1),c,w)
    a=math.radians(a1); sgn=1 if a1>a0 else -1
    tx,ty=-math.sin(a)*sgn,math.cos(a)*sgn
    px,py=cx+r*math.cos(a),cy+r*math.sin(a)
    ic.fill(arrowhead(px+tx*L*0.6,py+ty*L*0.6,tx,ty,L,W),c)
@icon
def boundary_fill():
    ic=Icon('boundary_fill','Boundary fill'); I=Iso().fit([(0,0,0),(40,0,0),(40,40,0),(0,40,0),(0,0,34),(40,0,34),(0,40,34)],58,56)
    face(ic,I,[(0,0,0),(40,0,0),(40,40,0),(0,40,0)],P1)
    face(ic,I,[(0,0,0),(40,0,0),(40,0,34),(0,0,34)],P2)
    face(ic,I,[(0,0,0),(0,40,0),(0,40,34),(0,0,34)],P3)
    I.box(ic,3,3,0,28,28,25,B)
    return ic
@icon
def camera():
    ic=Icon('camera','Camera')
    ic.stroke('M22 22L25 14L39 14L42 22',INK,OW,fill=P2)
    body=rrect(6,20,58,52,6) if False else rrect(6,20,58,52,6)
    ic.fill(body,P1); ic.stroke(body,INK,OW)
    ic.fill(ell(32,36,10),A1); ic.stroke(ell(32,36,10),INK,OW); ic.dot(32,36,5,A3)
    ic.dot(50,28,2.4,A3)
    return ic
@icon
def capture():
    ic=Icon('capture','Capture position'); I=Iso().fit([(0,0,0),(20,20,0),(20,20,20),(0,0,20),(0,20,0),(20,0,0)],26,26)
    I.box(ic,0,0,0,20,20,20,G)
    for sx,sy in ((1,1),(-1,1),(1,-1),(-1,-1)):
        x,y=32+sx*24,32+sy*24
        ic.stroke(poly([(x-sx*12,y),(x,y),(x,y-sy*12)],False),A3,AW)
    return ic
@icon
def compute():
    ic=Icon('compute','Compute')
    arc_arrow(ic,32,32,19,205,335); arc_arrow(ic,32,32,19,25,155); return ic
@icon
def delete():
    ic=Icon('delete','Delete')
    ic.stroke(rrect(24,6,40,16,3),INK,OW,fill=P3)
    body='M14 22L50 22L46.5 57L17.5 57Z'; ic.fill(body,P1); ic.stroke(body,INK,OW)
    lid=rrect(8,14,56,24,4); ic.stroke(lid,INK,OW,fill=P3)
    ic.stroke('M25 33L26 49M39 33L38 49',A3,AW)
    return ic
def delete():
    ic=Icon('delete','Delete')
    ic.stroke('M26 15L26 9L38 9L38 15',INK,OW)
    body='M14 20L50 20L47 56L17 56Z'; ic.fill(body,P1); ic.stroke(body,INK,OW)
    ic.stroke('M10 15L54 15',INK,AW)
    ic.stroke('M26 28L27 48M32 28L32 48M38 28L37 48',P3,AW)
    return ic
@icon
def status_error():
    ic=Icon('status_error','Error')
    ic.fill(ell(32,32,24),A1); ic.stroke(ell(32,32,24),A3,AW)
    ic.stroke('M23 23L41 41M41 23L23 41',A3,AW)
    return ic
@icon
def status_warning():
    ic=Icon('status_warning','Warning')
    t='M32 8L57 53L7 53Z'; ic.fill(t,A1); ic.stroke(t,A3,AW)
    ic.stroke('M32 24L32 37',A3,AW); ic.dot(32,45,3,A3)
    return ic
@icon
def update():
    ic=Icon('update','Update')
    ic.stroke('M10 42L10 54L54 54L54 42',INK,OW)
    ic.stroke('M32 8L32 38',A3,AW); ic.fill(arrowhead(32,46,0,1,12,14),A3)
    return ic
@icon
def new():
    ic=Icon('new','New'); doc='M12 6L34 6L46 18L46 58L12 58Z'
    ic.fill(doc,P1); ic.stroke(doc,INK,OW); ic.stroke('M34 6L34 18L46 18',INK,OW)
    ic.dot(46,44,11,A3); ic.stroke('M46 38L46 50M40 44L52 44',P1,AW*0.9)
    return ic.recenter()

@icon
def open_():
    ic=Icon('open','Open')
    back='M6 18L24 18L29 24L52 24L52 50L6 50Z'; ic.fill(back,P2); ic.stroke(back,INK,OW)
    fr='M6 50L14 30L60 30L52 50Z'; ic.fill(fr,P1); ic.stroke(fr,INK,OW)
    return ic
@icon
def export():
    ic=Icon('export','Export'); I=Iso(0.9886,32,41)
    I.box(ic,0,0,0,20,20,20,G)
    sh='M32 16L32 33'; head=poly([(32,4),(23,16),(41,16)])
    ic.stroke(sh,GAP,NW+6); ic.stroke(head,GAP,6,fill=GAP)
    ic.stroke(sh,A3,NW); ic.stroke(head,A3,2,fill=A3)
    return ic
def export():
    ic=Icon('export','Export')
    for d,c in (('M14.88 31.06L32 41.05L32 59.59L14.88 49.6Z',P2),('M32 41.05L49.12 31.06L49.12 49.6L32 59.59Z',P3),('M14.88 31.06L32 21.07L49.12 31.06L32 41.05Z',P1)): ic.fill(d,c)
    ic.stroke('M14.88 31.06L32 21.07L49.12 31.06L49.12 49.6L32 59.59L14.88 49.6Z',INK,OW)
    ic.stroke('M32 41.05L14.88 31.06M32 41.05L49.12 31.06M32 41.05L32 59.59',INK,OW)
    ic.stroke('M32 24L32 11',A3,OW); ic.fill('M32 4L28.2 10.7L35.8 10.7Z',A3)
    return ic
@icon
def material():
    ic=Icon('material','Material')
    ic.fill(ell(32,58,16,3.6),P2)
    ic.fill(ell(32,31,24),A2); ic.stroke(ell(32,31,24),INK,OW)
    ic.fill('M14 24A19 19 0 0 1 30 12A15 15 0 0 0 14 24Z',A1) if False else None
    ic.dot(23,22,6,A1)
    return ic
@icon
def decal():
    ic=Icon('decal','Decal'); I=Iso().fit([(0,0,0),(48,0,0),(48,44,0),(0,44,0)],58,50)
    face(ic,I,[(0,0,0),(48,0,0),(48,44,0),(0,44,0)],P1)
    face(ic,I,[(8,8,0),(40,8,0),(40,36,0),(8,36,0)],A1,True,OW)
    ic.fill(I.poly((8,36,0),(18,22,0),(25,31,0),(31,25,0),(40,36,0)),A2) if False else None
    ic.fill(I.poly((8,36,0),(17,21,0),(25,30,0),(30,26,0),(40,36,0)),A2)
    ic.stroke(I.poly((8,8,0),(40,8,0),(40,36,0),(8,36,0)),INK,OW)
    return ic
@icon
def emboss():
    ic=Icon('emboss','Emboss'); I=Iso().fit([(0,0,0),(46,46,0),(0,0,16),(46,46,16),(0,46,0),(46,0,0)],58,52)
    I.box(ic,0,0,0,46,46,8,G)
    E=[(10,8),(36,8),(36,14),(16,14),(16,20),(28,20),(28,26),(16,26),(16,32),(36,32),(36,38),(10,38)]
    I.extrude(ic,[E],8,14,B)
    return ic
def emboss():
    ic=Icon('emboss','Emboss'); I=Iso().fit([(0,0,0),(46,46,0),(0,0,16),(46,46,16),(0,46,0),(46,0,0)],58,52)
    I.box(ic,0,0,0,46,46,8,G)
    I.box(ic,10,8,8,16,38,14,B)
    for y in (8,20,32):
        I.box(ic,16,y,8,34 if y!=20 else 28,y+6,14,B)
    return ic
@icon
def lip():
    ic=Icon('lip','Lip'); I=Iso().fit([(0,0,0),(44,44,0),(0,0,24),(44,44,24),(0,44,0),(44,0,0)],58,56)
    I.box(ic,0,0,0,44,44,18,G)
    I.extrude(ic,[[(4,4),(40,4),(40,40),(4,40)],[(9,9),(35,9),(35,35),(9,35)]],18,23,B)
    return ic
def lip():
    ic=Icon('lip','Lip'); I=Iso().fit([(0,0,0),(44,44,0),(0,0,24),(44,44,24),(0,44,0),(44,0,0)],58,56)
    I.box(ic,0,0,0,44,44,18,G,OW,detail=False)
    z0,z1=18,23; t=5
    top=I.poly((0,0,18),(44,0,18),(44,44,18),(0,44,18)); ic.fill(top,P1)
    ic.stroke(poly([I.p(0,44,18),I.p(44,44,18),I.p(44,0,18)],False),INK,OW)
    ic.stroke(poly([I.p(44,44,18),I.p(44,44,0)],False),INK,OW)
    I.box(ic,4,4,z0,40,9,z1,B,OW,detail=False)
    I.box(ic,4,9,z0,9,35,z1,B,OW,detail=False)
    I.box(ic,35,9,z0,40,35,z1,B,OW,detail=False)
    I.box(ic,4,35,z0,40,40,z1,B,OW,detail=False)
    return ic
@icon
def rest():
    ic=Icon('rest','Rest'); I=Iso().fit([(0,0,0),(46,46,0),(0,0,16),(46,46,16),(0,46,0),(46,0,0)],58,52)
    I.box(ic,0,0,0,46,46,8,G)
    I.box(ic,7,6,8,13,40,14,B); I.box(ic,31,6,8,37,40,14,B)
    return ic
def arrowhead_path(x,y,dx,dy,L=9,W=8):
    bx,by=x-dx*L,y-dy*L; px,py=-dy,dx
    return poly([(x,y),(bx+px*W/2,by+py*W/2),(bx-px*W/2,by-py*W/2)])
def triad(ic,I,x,y,z,L=22):
    o=I.p(x,y,z)
    for (dx,dy,dz) in ((L,0,0),(0,L,0),(0,0,L)):
        e=I.p(x+dx,y+dy,z+dz)
        ic.stroke(poly([o,e],False),A3,AW)
        ic.fill(arrowhead(e[0],e[1],e[0]-o[0],e[1]-o[1],8,9),A3)
    ic.dot(o[0],o[1],4,INK)
@icon
def joint_origin():
    ic=Icon('joint_origin','Joint origin')
    ox,oy=31,34
    axes=((0,-30),(24,-6),(-11,25))          # up, slightly backward and level, mostly forward
    def tip(dx,dy,L=11):
        n=math.hypot(dx,dy); return (ox+dx,oy+dy),(dx/n,dy/n)
    for dx,dy in axes:
        (x,y),(ux,uy)=tip(dx,dy)
        sx,sy=x-ux*7,y-uy*7
        halo=GAP
        ic.stroke(poly([(ox,oy),(sx,sy)],False),halo,AW+5.5)
        ic.stroke(arrowhead_path(x,y,ux,uy,11,12),halo,5.5,fill=halo)
    for dx,dy in axes:
        (x,y),(ux,uy)=tip(dx,dy)
        sx,sy=x-ux*7,y-uy*7
        ic.stroke(poly([(ox,oy),(sx,sy)],False),A3,AW)
        ic.fill(arrowhead_path(x,y,ux,uy,11,12),A3)
    ic.dot(ox,oy,9,GAP); ic.dot(ox,oy,6.3,INK)
    return ic.recenter()
@icon
def drive():
    ic=Icon('drive','Drive')
    arc_arrow(ic,32,32,22,-50,250 if False else 235)
    ic.fill(ell(32,32,9),A2); ic.stroke(ell(32,32,9),INK,OW); ic.dot(32,32,3,INK)
    return ic
@icon
def motion_link():
    ic=Icon('motion_link','Motion link')
    ic.ring(14,32,8.5,P3,AW,P1); ic.ring(50,32,8.5,A3,AW,A1)
    ic.stroke('M26 32L38 32',A3,AW); ic.fill(arrowhead(40,32,1,0,9,10),A3); ic.fill(arrowhead(24,32,-1,0,9,10),A3)
    return ic
@icon
def convert_sheet():
    ic=Icon('convert_sheet','Convert to sheet')
    I1=Iso(1,15,42); I1.box(ic,-6.5,-6.5,-6.5,6.5,6.5,6.5,G)
    I2=Iso(1,44,44); I2.box(ic,-8.5,-8.5,-1,8.5,8.5,1,B)
    ic.stroke('M18 16L38 16',A3,AW); ic.fill(arrowhead(48,16,1,0,10,11),A3)
    return ic
@icon
def project_surface():
    ic=Icon('project_surface','Project to surface'); I=Iso().fit([(0,0,0),(40,0,0),(40,40,0),(0,40,0),(0,0,12)],58,52)
    surf_patch(ic,I,n=2,fillc=P1,line=P2,lw=OW*0.8,out=INK,ow=OW)
    size=40
    def Z(x,y): return 7*math.sin(x*math.pi/size*1.3)*math.cos(y*math.pi/size*0.9)+4.2
    pts=[]
    for i in range(25):
        u=i/24; x=7+26*u; y=20+8*math.sin(u*2*math.pi)                  # a sketch curve lying inside the surface
        pts.append(I.p(x,y,Z(x,y)))
    ic.stroke(poly(pts,False),A3,AW)
    return ic.recenter()
def project_surface():
    ic=Icon('project_surface','Project to surface'); I=Iso().fit([(0,0,0),(40,0,0),(40,40,0),(0,40,0),(0,0,12)],58,40); I.oy+=8
    outline=surf_patch(ic,I,n=2,fillc=P1,line=P2,lw=OW*0.8,out=INK,ow=OW)
    ic.stroke('M18 30C26 24 34 36 46 30',A3,AW)
    ic.stroke('M18 6C26 2 34 12 46 6',A3,AW) if False else None
    ic.stroke('M22 6C28 4 34 8 42 4',A3,AW) if False else None
    ic.fill(arrowhead(32,22,0,1,7,8),A3) if False else None
    return ic
@icon
def iso_curve():
    ic=Icon('iso_curve','Isoparametric curve'); I=Iso().fit([(0,0,0),(40,0,0),(40,40,0),(0,40,0),(0,0,12),(40,40,12)],58,52)
    outline=surf_patch(ic,I,n=2,fillc=P1,line=P2,lw=OW*0.8,out=INK,ow=OW)
    size=40
    def Z(x,y): return 7*math.sin(x*math.pi/size*1.3)*math.cos(y*math.pi/size*0.9)+4.2
    ps=[I.p(t,22,Z(t,22)) for t in [size*i/14 for i in range(15)]]
    ic.stroke(poly(ps,False),A3,AW)
    return ic
@icon
def intersection_curve():
    ic=Icon('intersection_curve','Intersection curve'); I=Iso().fit([(0,0,0),(44,0,0),(44,44,0),(0,44,0),(0,22,34)],58,52)
    face(ic,I,[(0,0,0),(44,0,0),(44,44,0),(0,44,0)],P1)
    def y(x): return 22+4.2*math.sin(2*math.pi*x/44)
    xs=[44*i/22 for i in range(23)]
    lo=[I.p(x,y(x),0) for x in xs]; hi=[I.p(x,y(x),32) for x in xs]
    sheet=poly(hi+lo[::-1]); ic.fill(sheet,P2); ic.stroke(sheet,INK,OW)       # a slightly wavy vertical sheet
    ic.stroke(poly(lo,False),GAP,AW+2.4); ic.stroke(poly(lo,False),A3,AW)     # where it meets the floor, with a thin punch-out
    return ic.recenter()
def intersection_curve():
    ic=Icon('intersection_curve','Intersection curve'); I=Iso().fit([(0,0,0),(44,0,0),(44,44,0),(0,44,0),(0,22,30)],58,52)
    face(ic,I,[(0,0,0),(44,0,0),(44,44,0),(0,44,0)],P1)
    face(ic,I,[(0,22,-4),(44,22,-4),(44,22,30),(0,22,30)],P2) if False else None
    face(ic,I,[(0,22,0),(44,22,0),(44,22,30),(0,22,30)],P2)
    ic.stroke(poly([I.p(0,22,0),I.p(44,22,0)],False),A3,AW)
    return ic
@icon
def patch():
    ic=Icon('patch','Patch')
    ic.fill(ell(32,32,26,17),A1)
    ic.stroke('M10 30C22 22 40 40 54 32',A3,OW); ic.stroke('M18 44C26 36 40 44 46 40',A3,OW) if False else None
    ic.stroke(ell(32,32,26,17),INK,AW*0.85)
    ic.stroke('M10 32C22 24 42 40 54 32',A3,AW*0.8) if False else None
    return ic
@icon
def ruled():
    ic=Icon('ruled','Ruled surface')
    t=[(6,18),(20,6),(40,26),(58,12)]; b=[(6,50),(20,38),(40,58),(58,44)]
    d='M6 18C20 6 40 26 58 12L58 44C40 58 20 38 6 50Z'
    ic.fill('M6 18C20 6 40 26 58 12L58 44C40 58 20 38 6 50Z',A1)
    for k in range(1,6):
        u=k/6; a=bez(*t,u); c=bez(*b,u); ic.stroke(poly([a,c],False),A2,OW)
    ic.stroke('M6 18C20 6 40 26 58 12',A3,AW); ic.stroke('M6 50C20 38 40 58 58 44',A3,AW)
    ic.stroke('M6 18L6 50M58 12L58 44',A3,OW)
    return ic
@icon
def reverse_normal():
    ic=Icon('reverse_normal','Reverse normal'); I=Iso(1,32,35)
    face(ic,I,[(-14,-14,0),(14,-14,0),(14,14,0),(-14,14,0)],P1)
    ic.stroke('M32 33L32 16',A3,AW); ic.fill(arrowhead(32,7,0,-1,10,11),A3)        # the normal as it was
    ic.stroke('M32 41L32 49',P3,AW); ic.fill(arrowhead(32,58,0,1,10,11),P3)        # and flipped
    return ic.recenter()
def reverse_normal():
    ic=Icon('reverse_normal','Reverse normal'); I=Iso(1,32,38)
    face(ic,I,[(-24,-24,0),(24,-24,0),(24,24,0),(-24,24,0)],P1)
    ic.stroke('M32 40L32 14',A3,AW); ic.fill(arrowhead(32,6,0,-1,9,10),A3)
    ic.stroke('M32 44L32 54',P3,AW); ic.fill(arrowhead(32,60,0,1,9,10),P3)
    return ic
@icon
def thicken():
    ic=Icon('thicken','Thicken'); I=Iso(1,32,34)
    I.box(ic,-14,-14,-12,14,14,-4,B)
    ic.stroke(I.poly((-14,-14,10),(14,-14,10),(14,14,10),(-14,14,10)),INK,OW,fill=P1)
    return ic.recenter()
def thicken():
    ic=Icon('thicken','Thicken'); I=Iso(0.9,32,36)
    I.box(ic,-24,-24,-14,24,24,-4,B)
    ic.stroke(I.poly((-24,-24,12),(24,-24,12),(24,24,12),(-24,24,12)),INK,OW,fill=P1)
    ic.stroke('M32 8L32 8',A3,AW) if False else None
    return ic
def tile(ic,I,x0,x1,y0,y1,col): face(ic,I,[(x0,y0,0),(x1,y0,0),(x1,y1,0),(x0,y1,0)],col)
@icon
def stitch():
    ic=Icon('stitch','Stitch'); I=Iso().fit([(0,0,0),(48,0,0),(48,32,0),(0,32,0)],58,50)
    tile(ic,I,0,24,0,32,P1); tile(ic,I,24,48,0,32,P2)
    for y in (4,12,20,28):
        ic.stroke(poly([I.p(20,y,0),I.p(28,y,0)],False),A3,3.4)
    return ic
@icon
def unstitch():
    ic=Icon('unstitch','Unstitch'); I=Iso().fit([(0,0,0),(52,0,0),(52,32,0),(0,32,0)],58,50)
    tile(ic,I,0,22,0,32,P1); tile(ic,I,30,52,0,32,P2)
    for y in (4,12,20,28):
        ic.stroke(poly([I.p(17,y,0),I.p(21,y,0)],False),A3,3.4); ic.stroke(poly([I.p(31,y,0),I.p(35,y,0)],False),A3,3.4)
    return ic
@icon
def untrim():
    ic=Icon('untrim','Untrim'); I=Iso().fit([(0,0,0),(48,0,0),(48,40,0),(0,40,0)],58,50)
    face(ic,I,[(0,0,0),(48,0,0),(48,40,0),(0,40,0)],P1)
    X,Y=I.p(24,20,0); rx,ry=I.ellipse_r(9)
    ic.fill(ell(X,Y,rx,ry),P3); ic.stroke(ell(X,Y,rx,ry),A3,AW)
    return ic
@icon
def rigid_group():
    ic=Icon('rigid_group','Rigid group')
    ic.stroke('M21 29L21 21A11 11 0 0 1 43 21L43 29',INK,AW)
    body=rrect(13,28,51,57,5); ic.fill(body,P1); ic.stroke(body,INK,OW)
    ic.dot(32,40,4.5,A3); ic.stroke('M32 41L32 49',A3,AW)
    return ic
@icon
def revert():
    ic=Icon('revert','Revert position'); I=Iso().fit([(0,0,0),(20,20,0),(20,20,20),(0,0,20),(0,20,0),(20,0,0)],26,26)
    I.box(ic,0,0,0,20,20,20,G)
    for sx,sy in ((1,1),(-1,1),(1,-1),(-1,-1)):
        x,y=32+sx*24,32+sy*24
        ic.stroke(poly([(x-sx*12,y),(x,y),(x,y-sy*12)],False),P3,3.4,dash='0.1 5.6')
    return ic
def revert():
    ic=Icon('revert','Revert position'); I=Iso(0.9,40,42)
    I.box(ic,-13,-13,-13,13,13,13,G)
    arc_arrow(ic,34,30,22,-30,-170 if False else -200)
    return ic
