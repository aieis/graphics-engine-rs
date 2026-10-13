import os
import sys
import enum
import math

import cv2
import numpy as np

VIEW_INTERNAL = len(sys.argv) == 2 and sys.argv[1] == "--view-internal"

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

TEXTURE_DIR = os.path.abspath (
    os.path.join(SCRIPT_DIR, "..", "assets/textures/card_pre")
)

class Suit(enum.IntEnum):
    Club    = 0
    Diamond = enum.auto()
    Heart   = enum.auto()
    Spade   = enum.auto()
    Count   = enum.auto()

class Colour:
    White = (255, 255, 255)
    Black = (0, 0, 0)
    Red   = (0, 0, 255)

RANK_JACK  = 11
RANK_QUEEN = 12
RANK_KING  = 13

RANK_NAME = [f"{rank}.png" for rank in range(0, RANK_KING + 1)]
RANK_NAME[RANK_JACK] = f"J.png"
RANK_NAME[RANK_QUEEN] = f"Q.png"
RANK_NAME[RANK_KING] = f"K.png"

SUIT_DATA = [("", Colour.White)] * (Suit.Count + 1)
SUIT_DATA[Suit.Club]    = ("club.png", Colour.Black)
SUIT_DATA[Suit.Diamond] = ("diamond.png", Colour.Red)
SUIT_DATA[Suit.Heart]   = ("heart.png", Colour.Red)
SUIT_DATA[Suit.Spade]   = ("spade.png", Colour.Black)

COLUMNS   = 13
NUM_CARDS = 13 * 4

CARD_WIDTH  = 240
CARD_HEIGHT = 360

def get_emblem_count(rank: int):
    if rank <= 10:
        return rank
    else:
        return 1


def get_rank_path(rank: int):
    return os.path.join(TEXTURE_DIR, RANK_NAME[rank])

def get_suit_path(suit: Suit):
    return os.path.join(TEXTURE_DIR, SUIT_DATA[suit][0])

def get_suit_colour(suit: Suit):
    return SUIT_DATA[suit][1]


def get_index(suit: Suit, rank: int):
    return suit * 4 + rank

def get_suit_emblem_width(count: int):
    if count == 1:
        return CARD_WIDTH / 3.0

    return CARD_WIDTH / 6.0


def get_rank_emblem_width():
    return CARD_WIDTH / 5.0

def suit_emblem_layout_internal(count: int, items_per_col: int):
    col_count = count // items_per_col
    cols = [items_per_col for c in range(col_count)]
    extras = count % items_per_col
    if extras != 0:
        cols.insert(col_count // 2, extras)

    return cols

def suit_emblem_layout(count: int):
    if count < 6 and count != 3:
        return suit_emblem_layout_internal(count, 2)

    elif count < 10:
        return suit_emblem_layout_internal(count, 3)

    else:
        return suit_emblem_layout_internal(count, 4)


def dist(p1: tuple[int, int], p2: tuple[int, int]):
    return math.sqrt((p1[0] - p2[0])**2 + (p1[1] - p2[1])**2)

def make_card(suit: Suit, rank: int):
    print(f"Making Card: {rank} of {suit.name}s")
    card = np.zeros((CARD_HEIGHT, CARD_WIDTH, 3), np.uint8)
    card[:,:] = Colour.White

    emblem_dim = int(get_suit_emblem_width(rank))
    count  = get_emblem_count(rank)
    cols = suit_emblem_layout(count)
    colour = get_suit_colour(suit)

    suit_texture = cv2.imread(get_suit_path(suit))
    emblem = cv2.resize(suit_texture, (emblem_dim, emblem_dim))
    mask = emblem[:,:,:3] / 255
    stamp = ((1 - mask) * 255 + mask * colour).astype(np.uint8)
    stamp_rot = cv2.rotate(stamp, cv2.ROTATE_180)

    total_width_of_emblems = (len(cols) * 2 - 1) * emblem_dim

    px_start = (CARD_WIDTH - total_width_of_emblems) // 2

    for i, c in enumerate(cols):
        total_height_of_emblems = (c * 2 - 1) * emblem_dim
        py_start = (CARD_HEIGHT - total_height_of_emblems) // 2

        px = px_start + i * emblem_dim * 2
        cx = px + emblem_dim // 2

        for j in range(c):
            py = py_start + j * emblem_dim * 2
            cy = py + emblem_dim // 2

            p1 = (cx, cy)
            beg = (0, 0)
            end = (CARD_WIDTH, CARD_HEIGHT)

            if cy < CARD_HEIGHT // 2 or dist(p1, beg) < dist(p1, end) + 1e-3 :
                card[py:py+emblem_dim,px:px+emblem_dim] = stamp
            else:
                card[py:py+emblem_dim,px:px+emblem_dim] = stamp_rot


    emblem_dim = int(get_rank_emblem_width())
    rank_texture = cv2.imread(get_rank_path(rank))
    emblem = cv2.resize(rank_texture, (emblem_dim, emblem_dim))
    mask = emblem[:,:,:3] / 255
    stamp = ((1 - mask) * 255 + mask * colour).astype(np.uint8)
    stamp_rot = cv2.rotate(stamp, cv2.ROTATE_180)

    card[:emblem_dim,:emblem_dim] = stamp
    card[-emblem_dim:,-emblem_dim:] = stamp_rot

    return card


def make_deck():

    suit_stack = []
    for suit in [Suit.Heart, Suit.Spade, Suit.Diamond, Suit.Club]:
        cards = [make_card(suit, rank) for rank in range(1, RANK_KING+1)]
        suit_stack.append(np.concatenate(cards, axis=1))

    deck = np.concatenate(suit_stack, axis=0)
    cv2.imwrite(os.path.join(SCRIPT_DIR, "..", "target", "tmp.png"), deck)


def main():
    if VIEW_INTERNAL:
        show_card_layouts()
    else:
        make_deck()

def show_card_layouts():
    for i in range(1, 11):
        print(f"Rank {i} => {suit_emblem_layout(i)}")

if __name__ == "__main__":
    main()
