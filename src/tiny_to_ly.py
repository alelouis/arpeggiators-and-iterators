"""
Takes as input a TinyNotation string and computes .ly Lilypond file
"""

from music21 import *
import argparse

parser = argparse.ArgumentParser(description='Music iterators mxml conversion')
parser.add_argument('file', type=str, help='file name')
parser.add_argument('tinynotation', type=str, help='Tiny notation string')
args = parser.parse_args()

filepath = f"../out/{args.file}"
obj = converter.parse(f"tinynotation: 4/4 {args.tinynotation}")

conv = lily.translate.LilypondConverter()
conv.loadFromMusic21Object(obj)
convertedFilePath = conv.writeLyFile('.ly', filepath)