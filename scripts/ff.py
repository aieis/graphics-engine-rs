import sys
import os
import argparse
import enum
import struct

import cv2
import numpy as np

FF_MAGIC_BYTES = "farbfeld"
FF_HEADER_LEN = len(FF_MAGIC_BYTES) + 4 + 4

def convert_ff(file_path: str, output_path: str | None):

    if output_path is None:
        output_path = os.path.splitext(file_path)[0] + ".ff"

    im = cv2.imread(file_path, cv2.IMREAD_UNCHANGED)

    if len(im.shape) != 3:
        # Image is grayscale
        im = cv2.cvtColor(cv2.COLOR_GRAY2BGRA)

    elif im.shape[2] == 3:
        im = cv2.cvtColor(cv2.COLOR_BGR2BGRA)

    h, w, c = im.shape[:3]

    if c != 4:
        print(f"UNEXPECTED IMAGE FORMAT: {im.shape}")
        return

    data_len = w * h * 4 * 2;
    buf = bytearray(FF_HEADER_LEN + data_len)

    ptr = 0

    buf[ptr:ptr+len(FF_MAGIC_BYTES)] = FF_MAGIC_BYTES.encode("ascii")
    ptr += len(FF_MAGIC_BYTES)

    struct.pack_into(">i", buf, ptr, w)
    ptr += 4

    struct.pack_into(">i", buf, ptr, h)
    ptr +=4

    for row in range(h):
        for col in range(w):
            r, g, b, a = im[row,col]

            r = int(int(r)*(2**16-1)/(2**8-1))
            g = int(int(g)*(2**16-1)/(2**8-1))
            b = int(int(b)*(2**16-1)/(2**8-1))
            a = int(int(a)*(2**16-1)/(2**8-1))

            struct.pack_into(">H", buf, ptr, r)
            ptr += 2

            struct.pack_into(">H", buf, ptr, g)
            ptr += 2

            struct.pack_into(">H", buf, ptr, b)
            ptr += 2

            struct.pack_into(">H", buf, ptr, a)
            ptr += 2

    with open(output_path, "wb") as fd:
        fd.write(buf)



def load_ff(file_path: str):
    with open(file_path, "rb") as fd:
        data = fd.read()

    ptr = 0
    header = data[:8]
    ptr += 8

    header = header.decode("ascii")
    if FF_MAGIC_BYTES != header:
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

def show_ff(file_path: str):
    if (im := load_ff(file_path)) is not None:

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

def show_font_atlas():
    show_ff("../assets/fonts/Atlas-Iosevka-Regular.ff")


class Command(enum.IntEnum):
    Convert = 0
    Show  = enum.auto()
    Undefined = enum.auto()

def get_name_from_command(cmd: Command):
    match cmd:
        case Command.Convert:
            return "convert"

        case Command.Show:
            return "show"

        case Command.Undefined:
            return "undefined"


def get_command_from_name(cmd_name: str) -> Command:
    match cmd_name:
        case "convert":
            return Command.Convert

        case "show":
            return Command.Show

        case _:
            return Command.Undefined


def get_all_commands():
    cmds = [get_name_from_command(Command(cmd)) for cmd in range(0, int(Command.Undefined))]
    return cmds


def main():
    prog = sys.argv[0]

    parser = argparse.ArgumentParser(prog=prog, description="Small ff helper")
    parser.add_argument("-f", "--file", type=str)


    subparsers = parser.add_subparsers(dest="command")

    convert = subparsers.add_parser(get_name_from_command(Command.Convert))
    convert.add_argument("-o", "--output", default=None, type=str)

    show = subparsers.add_parser(get_name_from_command(Command.Show))

    args = parser.parse_args()

    cmd = get_command_from_name(args.command)

    match cmd:
        case Command.Show:
            show_ff(args.file)

        case Command.Convert:
            convert_ff(args.file, args.output)

        case Command.Undefined:
            print(f"Unknown command: '{args.command}'")

if __name__ == "__main__":
    main()
