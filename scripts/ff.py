import cv2
import numpy as np

def load_ff(file_path: str):
    with open(file_path, "rb") as fd:
        data = fd.read()

    ptr = 0
    header = data[:8]
    ptr += 8

    header = header.decode("ascii")
    if "farbfeld" != header:
        print(f"Image is not farbfeld: {header=}")
        return None

    wb = data[ptr:ptr+4]
    ptr += 4

    hb = data[ptr:ptr+4]
    ptr += 4

    w = int.from_bytes(wb, byteorder="big")
    h = int.from_bytes(hb, byteorder="big")

    print(f"Read farbfeld image: {w}x{h}")

    size = w*h*4
    data = np.frombuffer(data[ptr:ptr+size*2], dtype=np.uint16).reshape((h,w,4))

    im = (data >> 8).astype(np.uint8)
    return im

def show_font_atlas()
    if (im := load_ff("../assets/fonts/Atlas-Iosevka-Regular.ff")) is not None:

        h, w = im.shape[:2]

        print(f"{w}x{h}")

        ch = h // 256

        N = 256 // 16
        tw = w * N
        th = (h + N) // N

        tim = np.zeros((th, tw), np.uint8)
        for i in range(N):
            b = min((i+1)*th, h)
            tim[0:b-th*i, w*i:w*i+w] = im[th*i:b, :, 3]

        cv2.imshow("Image", tim)
        cv2.waitKey(0)

def main():
    pass

if __name__ == "__main__":
    main()
