import os
import sys
import enum
import math

import cv2
import numpy as np

from ff import convert_ff

VIEW_INTERNAL = len(sys.argv) == 2 and sys.argv[1] == "--view-internal"

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

TEXTURE_DIR = os.path.abspath (
    os.path.join(SCRIPT_DIR, "..", "assets/textures/card_pre")
)

OUTPUT_FILE = os.path.abspath (
    os.path.join(SCRIPT_DIR, "..", "assets/textures/full-deck.png")
)


class Suit(enum.IntEnum):
    Club    = 0
    Diamond = enum.auto()
    Heart   = enum.auto()
    Spade   = enum.auto()
    Count   = enum.auto()

class Colour:
    Type  = tuple[int, int, int]

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

SUIT_DATA: list[tuple[str, Colour.Type]] = [("", Colour.White)] * (Suit.Count + 1)
SUIT_DATA[Suit.Club]    = ("club.png", Colour.Black)
SUIT_DATA[Suit.Diamond] = ("diamond.png", Colour.Red)
SUIT_DATA[Suit.Heart]   = ("heart.png", Colour.Red)
SUIT_DATA[Suit.Spade]   = ("spade.png", Colour.Black)

COLUMNS   = 13
NUM_CARDS = 13 * 4

CARD_WIDTH  = 240
CARD_HEIGHT = 360
CARD_WIDTH_MARGIN = 1
CARD_HEIGHT_MARGIN = 10

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
        return CARD_WIDTH / 2.0

    return (CARD_WIDTH - CARD_WIDTH_MARGIN) / 4.0


def get_rank_emblem_width():
    return CARD_WIDTH / 7.0

def suit_emblem_layout_internal(count: int, items_per_col: int):
    col_count = count // items_per_col
    cols = [items_per_col for _ in range(col_count)]
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
    colour = get_suit_colour(suit)


    # Prepare Suit Emblem
    count  = get_emblem_count(rank)
    suit_emblem_dim = int(get_suit_emblem_width(count))
    cols = suit_emblem_layout(count)

    suit_texture = cv2.imread(get_suit_path(suit))
    suit_emblem = cv2.resize(suit_texture, (suit_emblem_dim, suit_emblem_dim))
    suit_mask = suit_emblem[:,:,:3] / 255
    suit_stamp = ((1 - suit_mask) * 255 + suit_mask * colour).astype(np.uint8)
    suit_stamp_rot = cv2.rotate(suit_stamp, cv2.ROTATE_180)


    # Prepare Rank Emblem
    rank_emblem_dim = int(get_rank_emblem_width())
    rank_texture = cv2.imread(get_rank_path(rank))
    rank_emblem = cv2.resize(rank_texture, (rank_emblem_dim, rank_emblem_dim))
    suit_mask = rank_emblem[:,:,:3] / 255
    rank_stamp = ((1 - suit_mask) * 255 + suit_mask * colour).astype(np.uint8)
    rank_stamp_rot = cv2.rotate(rank_stamp, cv2.ROTATE_180)

    card[:rank_emblem_dim,:rank_emblem_dim] = rank_stamp
    card[-rank_emblem_dim:,-rank_emblem_dim:] = rank_stamp_rot

    suit_emblem_small_dim = int(rank_emblem_dim * 2 / 3)
    suit_stamp_small = cv2.resize(suit_stamp, (suit_emblem_small_dim, suit_emblem_small_dim))
    suit_stamp_small_rot = cv2.rotate(suit_stamp_small, cv2.ROTATE_180)

    px = int((rank_emblem_dim - suit_emblem_small_dim) / 2)
    py = rank_emblem_dim
    card[py:py+suit_emblem_small_dim,px:px+suit_emblem_small_dim] = suit_stamp_small

    px = CARD_WIDTH - px - suit_emblem_small_dim
    py = CARD_HEIGHT - py - suit_emblem_small_dim
    card[py:py+suit_emblem_small_dim,px:px+suit_emblem_small_dim] = suit_stamp_small_rot


    # Center Emblems

    if len(cols) > 1:
        total_width_of_emblems = CARD_WIDTH - rank_emblem_dim * 2 - CARD_WIDTH_MARGIN * 2
        col_spacing = (total_width_of_emblems - len(cols) * suit_emblem_dim) / (len(cols) - 1)
    else:
        total_width_of_emblems = suit_emblem_dim
        col_spacing = 0

    px_delta = suit_emblem_dim + col_spacing

    px_start = (CARD_WIDTH - total_width_of_emblems) // 2

    max_c = cols[0]
    if max_c > 1:
        max_total_height_of_emblems = CARD_HEIGHT - CARD_HEIGHT_MARGIN * 2
        max_row_spacing = (max_total_height_of_emblems -  max_c * suit_emblem_dim) / (max_c - 1)
    else:
        total_height_of_emblems = suit_emblem_dim
        max_row_spacing = 0

    for i, c in enumerate(cols):

        if c > 1:
            penalty = suit_emblem_dim + (max_row_spacing - suit_emblem_dim) / 2 if c != cols[0] else 0 # Bring it in by 1 suit if it doesn't fit
            total_height_of_emblems = CARD_HEIGHT - CARD_HEIGHT_MARGIN * 2 - penalty * 2
            row_spacing = (total_height_of_emblems - c * suit_emblem_dim) / (c - 1)
        else:
            total_height_of_emblems = suit_emblem_dim
            row_spacing = 0

        py_delta = suit_emblem_dim + row_spacing
        py_start = (CARD_HEIGHT - total_height_of_emblems) // 2

        px = int(px_start + i * px_delta)
        cx = px + suit_emblem_dim // 2

        for j in range(c):
            py = int(py_start + j * py_delta)
            cy = py + suit_emblem_dim // 2

            p1 = (cx, cy)
            beg = (0, 0)
            end = (CARD_WIDTH, CARD_HEIGHT)

            if cy < CARD_HEIGHT // 2 - 1e1:
                target = suit_stamp
            elif cy > CARD_HEIGHT - (CARD_HEIGHT // 2 - 1e1):
                target = suit_stamp_rot
            elif dist(p1, beg) < dist(p1, end) + 1e-3:
                target = suit_stamp
            else:
                target = suit_stamp_rot


            card[py:py+suit_emblem_dim,px:px+suit_emblem_dim] = target

    return card


def make_deck():
    back_texture = cv2.imread(os.path.join(TEXTURE_DIR, "back.png"))
    back_card    = cv2.resize(back_texture, (CARD_WIDTH, CARD_HEIGHT))
    empty_card   = np.ones((CARD_HEIGHT, CARD_WIDTH, 3), np.uint8) * 255

    backs = [back_card] + [empty_card for _ in range(1, RANK_KING+1 - 1)]

    suit_stack = [
        np.concatenate(backs, axis=1)
    ]

    for suit in [Suit.Heart, Suit.Spade, Suit.Diamond, Suit.Club]:
        cards = [make_card(suit, rank) for rank in range(1, RANK_KING+1)]
        suit_stack.append(np.concatenate(cards, axis=1))

    deck = np.concatenate(suit_stack, axis=0)
    cv2.imwrite(OUTPUT_FILE, deck)

    convert_ff(OUTPUT_FILE)

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
