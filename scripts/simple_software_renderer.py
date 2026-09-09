import math

import numpy as np
import cv2

CAM_X_ANGLE_DEFAULT = -np.pi / 2
CAM_Y_ANGLE_DEFAULT = 0
CAM_LOC_DEFAULT = np.array((0, 0,  20), np.float64)

def identity():
    return np.array((
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1,
    )).reshape((4, 4))

class State:
    CamLoc   = CAM_LOC_DEFAULT.copy()
    CamDir   = np.array((0, 0, -1), np.float64)
    CamRight = np.array((1, 0,  0), np.float64)
    CamUp    = np.array((0, 1,  0), np.float64)

    CamXAngle = CAM_X_ANGLE_DEFAULT
    CamYAngle = CAM_Y_ANGLE_DEFAULT

    Proj = identity()
    View = identity()

    MousePos : np.ndarray | None = None

    PrintDebug = False


G = State()

FOV    = np.radians(45)
ASPECT = 1.0

DIM = 512

PLANES    = [np.array((0.0, 0.0, -10.0))]
PLANE_HALF_DIM = 1.0

CANVAS = np.zeros((DIM, DIM, 3), np.uint8)

DELTA = 0.1

WINDOW_NAME = "SimpleSoftwareRenderer"

def main():
    G.Proj = create_projection_matrix(FOV, ASPECT)
    update_view()

    print("Proj:")
    pretty_print(G.Proj)

    print("\nView:")
    pretty_print(G.View)


    print("\nDir:")
    pretty_print(G.CamDir)
    print()

    cv2.namedWindow(WINDOW_NAME)
    cv2.setMouseCallback(WINDOW_NAME, on_mouse_event)

    while (k := cv2.waitKey(16)) != ord('q'):
        handle_key(k)
        render_scene()

def render_scene():

    CANVAS[:,:,:] = 0

    for plane in PLANES:
        points = [pp for p in get_plane_points(plane) if (pp := project_point(p, G.View, G.Proj)) is not None]

        if len(points) != 4:
            continue

        points = np.array([((x*DIM)//2 + DIM//2, (y*DIM)//2 + DIM//2) for x, y in points], np.int32)
        points.reshape((-1, 1, 2))


        cv2.drawContours(CANVAS, [points], -1, (255, 255, 255), -1)


    G.PrintDebug = False
    cv2.imshow(WINDOW_NAME, CANVAS)


def project_point(p_orig: np.ndarray, view: np.ndarray, proj: np.ndarray):

    p_view = vec4(p_orig, 1.0).dot(view)
    p = p_view.dot(proj)

    if G.PrintDebug:
        print(f"P_ORIG: ", end="")
        pretty_print(p_orig)

        print(f"P_VIEW: ", end="")
        pretty_print(p_view)

        print(f"P_PROJ: ", end="")
        pretty_print(p)

        print()

    if abs(p[3]) < 1e-5 or p[2]/p[3] < 1e-5:
        return

    return p[:2] / p[3] # This is the divide by w that the GPU automatically does in the shader


def get_plane_points(plane_position: np.ndarray):

    x, y, z = plane_position

    return np.array((
        (x - PLANE_HALF_DIM, y - PLANE_HALF_DIM, z),
        (x - PLANE_HALF_DIM, y + PLANE_HALF_DIM, z),
        (x + PLANE_HALF_DIM, y + PLANE_HALF_DIM, z),
        (x + PLANE_HALF_DIM, y - PLANE_HALF_DIM, z),
    ))


def create_projection_matrix(fov: float, aspect: float):
    F = 100.0
    N = 0.1
    C = 1 / np.tan(fov / 2)

    X = C / aspect

    # z' = Az + B
    # z'' = z' / -z
    # So we need an A and B such that z' gets mapped to 0 when z==N and 1 when at z==F
    # (A*N+B) / (-N) = 0 and (A*F+B)/(-F) = 1
    A = -F/(F-N)
    B = -(N*F)/(F-N)

    proj = np.array((
        (X,  0,  0,  0),
        (0,  C,  0,  0),
        (0,  0,  A, -1),
        (0,  0,  B,  0)
    ))

    return proj


def create_view_matrix(pos: np.ndarray, dir: np.ndarray, up: np.ndarray):

    # /*
    #  * The premise as follows the component of a vector v onto a basis can be derived as follows
    #  * cos(t) = c' / |v| because the vector forms the hypotenuse (imagine vector (1, 1) on the cartesian grid)
    #  * -> c' = |v| * cos(t)
    #  * -> c' = 1 * |v| * cos(t) so if you are projecting onto a vector 'a' of length 1 then we get
    #  * -> c' = |a| * |v| * cos(t) which is the dot product
    #  * -> c' = dot(a,v)
    #  */

    f =  normalize(dir);
    r =  normalize(cross(f, up));
    u = -normalize(cross(f, r));
    b = -f;

    disp = vec3(-dot(r, pos), -dot(u, pos), -dot(b, pos));

    view = mat4(
        vec4(vec3(r[0], u[0], b[0]), 0.0),
        vec4(vec3(r[1], u[1], b[1]), 0.0),
        vec4(vec3(r[2], u[2], b[2]), 0.0),
        vec4(disp, 1.0),
    )

    return view

def vec3(x, y, z):
    return np.array((x, y, z))

def vec4(v: np.ndarray, s: float):
    x, y, z = v
    return np.array((x, y, z, s))

def mat4(x, y, z, w):
    return np.array((x, y, z, w)).reshape((4, 4))

def normalize(v):
    if (l := np.sqrt((v*v).sum())) > 1.0e-8:
        return v / l

    return v

def dot(u, v):
    return np.dot(u, v)

def cross(u, v):
    return np.cross(u, v)


## event handlers

def handle_key(key):

    try:
        k = chr(key)
    except Exception as _:
        return

    match k:
        case "d":
            G.CamLoc +=  DELTA * G.CamRight

        case "a":
            G.CamLoc += -DELTA * G.CamRight

        case "w":
            G.CamLoc +=  DELTA * G.CamDir

        case "s":
            G.CamLoc += -DELTA * G.CamDir

        case "t":
            print("Reset")
            G.CamLoc = CAM_LOC_DEFAULT.copy()
            G.CamXAngle = CAM_X_ANGLE_DEFAULT
            G.CamYAngle = CAM_Y_ANGLE_DEFAULT


        case "p":
            G.PrintDebug = True


    update_view()


def on_mouse_event(event: int, x: int, y: int, flags, param):
    if event == cv2.EVENT_LBUTTONDOWN:
        G.MousePos = np.array((x, y), np.float64)
        return
    elif event == cv2.EVENT_LBUTTONUP:
        G.MousePos = None

    elif event == cv2.EVENT_MOUSEMOVE and G.MousePos is not None:
        current_pos = np.array((x, y), np.float64)
        delta = current_pos - G.MousePos
        dx, dy = delta

        MOUSE_DELTA = 0.1
        if abs(dy) >= 2 or abs(dx) >= 2:
            if abs(dy) >= 2:
                G.CamYAngle += MOUSE_DELTA * dy / (DIM  / 2) * np.pi

            if abs(dx) >= 2:
                G.CamXAngle += MOUSE_DELTA * dx / (DIM  / 2) * np.pi


            G.MousePos = current_pos
            update_view()


def update_view():

    x_cos = math.cos(G.CamXAngle)
    x_sin = math.sin(G.CamXAngle)

    y_cos = math.cos(G.CamYAngle)
    y_sin = math.sin(G.CamYAngle)

    G.CamDir   = normalize(vec3(y_cos * x_cos, y_sin, y_cos * x_sin))
    G.CamRight = normalize(cross(G.CamDir, G.CamUp))
    G.View     = create_view_matrix(G.CamLoc, G.CamDir, G.CamUp)


def pretty_print(v: np.ndarray):
    if len(v.shape) > 1:
        for v in v:
            pretty_print_vec(v)
    else:
        pretty_print_vec(v)

def pretty_print_vec(v):
    ln = "["
    ln += f"{v[0]:6.2f}"
    for v in v[1:]:
        ln += f" {v:6.2f}"

    ln += "]"
    print(ln)



def test():
    i = vec4(vec3(1, 1, -20), 1).dot(
        mat4(
            vec4(vec3(0, 0, 0), 0),
            vec4(vec3(0, 0, 0), 0),
            vec4(vec3(-1, 0, 0), 0),
            vec4(vec3(0, 0, 0), 0),
        )
    )

    print(i)

if __name__ == "__main__":
    main()
