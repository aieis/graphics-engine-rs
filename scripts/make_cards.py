import os

import cv2
import numpy as np

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

TEXTURE_DIR = os.path.abspath (
    os.path.join(SCRIPT_DIR, "..", "assets/textures/card_pre")
)

class Suit(enum.IntEnum):
    Club    = 0
    Diamond = enum.auto()
    Heart   = enum.auto()
    Spade   = enum.auto()

RANK_JACK = 11
RANK_QUEEN = 12
RANK_KING = 13

RANK_NAME_ARR = [f"{rank}.png" for rank in range(1, RANK_KING + 1)]
RANK_NAME_ARR[RANK_JACK] = f"J.png"
RANK_NAME_ARR[RANK_QUEEN] = f"Q.png"
RANK_NAME_ARR[RANK_KING] = f"K.png"

SUIT_NAME_ARR = [
    "club."
]

COLUMNS   = 13
NUM_CARDS = 13 * 4

def count_amount(rank: int):
    if rank <= 10:
        return rank
    else:
        return 1


def get_rank_path(rank: int):
    return os.path.join(TEXTURE_DIR, RANK_NAME_ARR[rank])


def get_index(suit: Suit, rank: int):
    return suit * 4 + rank

def make_card(suit: Suit, rank: int):
    
