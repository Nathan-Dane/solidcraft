from lib import *
ICONS={}
def icon(fn): ICONS[fn.__name__]=fn; return fn

def blue_cube(ic,I,x0,y0,z0,x1,y1,z1): I.box(ic,x0,y0,z0,x1,y1,z1,(A1,A2,A3))

@icon
def box():
    ic=Icon('box','Box'); I=Iso(1,32,32); s=27
    I.box(ic,0,0,0,s,s,s,(A1,A2,A3)); 
    # offset so centre lands on (32,32)
    return ic
@icon
def cylinder_():
    ic=Icon('cylinder','Cylinder'); cylinder(ic,32,15,20,34,(A1,A2,A3)); return ic
@icon
def sphere():
    ic=Icon('sphere','Sphere')
    ic.fill(ell(32,32,26),A1)
    p0=arc_pt=(32+26*math.cos(math.radians(-35)),32+26*math.sin(math.radians(-35)))
    p1=(32+26*math.cos(math.radians(125)),32+26*math.sin(math.radians(125)))
    ic.fill('M%s %sA26 26 0 0 1 %s %sA34 34 0 0 0 %s %sZ'%(f(p0[0]),f(p0[1]),f(p1[0]),f(p1[1]),f(p0[0]),f(p0[1])),A2)
    q0=(32+26*math.cos(math.radians(10)),32+26*math.sin(math.radians(10)))
    q1=(32+26*math.cos(math.radians(100)),32+26*math.sin(math.radians(100)))
    ic.fill('M%s %sA26 26 0 0 1 %s %sA30 30 0 0 0 %s %sZ'%(f(q0[0]),f(q0[1]),f(q1[0]),f(q1[1]),f(q0[0]),f(q0[1])),A3)
    ic.stroke(ell(32,32,26),INK,OW)
    ic.stroke(arcpts(32,32,26,8,8,172),INK,OW)
    return ic
def lune(cx,cy,rx,ry,dx,dy):
    """the part of ellipse E(cx,cy,rx,ry) that lies outside the same ellipse moved by (dx,dy): a crescent on the side away from the move"""
    k=rx/ry; d=math.hypot(dx,dy*k); ux,uy=dx/d,dy*k/d       # direction in the circle-scaled plane
    h=math.sqrt(max(rx*rx-d*d/4,0)); mx,my=ux*d/2,uy*d/2
    pts=[(mx-uy*h,my+ux*h),(mx+uy*h,my-ux*h)]               # intersection points, relative to the centre of E, scaled plane
    phi=math.degrees(math.atan2(-uy,-ux))                   # direction away from the move
    def span(c0x,c0y,p,q):
        """angles (degrees) from p to q around circle centred (c0x,c0y) passing through phi-side"""
        ta=math.degrees(math.atan2(p[1]-c0y,p[0]-c0x)); tb=math.degrees(math.atan2(q[1]-c0y,q[0]-c0x))
        mid=math.radians(phi)
        for t1 in (tb,tb+360,tb-360):
            # candidate arc from ta to t1: check phi lies within
            lo,hi=min(ta,t1),max(ta,t1)
            for ph in (phi,phi+360,phi-360):
                if lo<=ph<=hi and hi-lo<=360: return ta,t1
        return ta,tb
    t0,t1=span(0,0,pts[0],pts[1])
    # E' (centre moved by d): the arc of E' that lies inside E, from pts[1] back to pts[0]
    u0,u1=span(ux*d,uy*d,pts[1],pts[0])
    return arcpts(cx,cy,rx,ry,t0,t1)+'A%s %s 0 %d %d %s %sZ'%(f(rx),f(ry),1 if abs(u1-u0)>180 else 0,1 if u1>u0 else 0,f(cx+pts[0][0]),f(cy+pts[0][1]/k))

@icon
def torus():
    ic=Icon('torus','Torus')
    cx,cy,rx,ry,w=32,32,18.5,14.5,15          # centre line of the tube and its width
    d=ell(cx,cy,rx,ry)
    ic.stroke(d,INK,w+2*OW); ic.stroke(d,A2,w)
    Ro=(rx+w/2,ry+w/2); Ri=(rx-w/2,ry-w/2)
    ic.fill(lune(cx,cy,Ro[0],Ro[1],-3.6,-3.4),A3)             # front arc: lower right of the outer edge
    ic.fill(lune(cx+3.2,cy-3.4,Ri[0],Ri[1],-3.2,3.4),A3)      # back arc: upper right of the hole
    return ic
