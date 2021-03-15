#!/bin/python3

import chess
import sys

fen = sys.argv[1]
board = chess.Board(fen)
WP = board.pieces(chess.PAWN, chess.WHITE)
WR = board.pieces(chess.ROOK, chess.WHITE)
WN = board.pieces(chess.KNIGHT, chess.WHITE)
WB = board.pieces(chess.BISHOP, chess.WHITE)
WK = board.pieces(chess.KING, chess.WHITE)
WQ = board.pieces(chess.QUEEN, chess.WHITE)

BP = board.pieces(chess.PAWN, chess.BLACK)
BR = board.pieces(chess.ROOK, chess.BLACK)
BN = board.pieces(chess.KNIGHT, chess.BLACK)
BB = board.pieces(chess.BISHOP, chess.BLACK)
BK = board.pieces(chess.KING, chess.BLACK)
BQ = board.pieces(chess.QUEEN, chess.BLACK)

white=WP|WR|WN|WB|WK|WQ
black=BP|BR|BN|BB|BK|BQ
all=white|black

def bb_to_int(b):
    s=str(b)
    s = s.replace(".", "0")
    s = s.replace(" ", "")
    s = s.replace("\n", "")
    return int(s, 2)

print("White Pawn:", bb_to_int(WP))
print("White Rooks:", bb_to_int(WR))
print("White Knights:", bb_to_int(WN))
print("White Bishops:", bb_to_int(WB))
print("White King:", bb_to_int(WK))
print("White Queen:", bb_to_int(WQ))
print()

print("Black Pawn:", bb_to_int(BP))
print("Black Rooks:", bb_to_int(BR))
print("Black Knights:", bb_to_int(BN))
print("Black Bishops:", bb_to_int(BB))
print("Black King:", bb_to_int(BK))
print("Black Queen:", bb_to_int(BQ))
print()

print("All Pieces:", bb_to_int(all))
