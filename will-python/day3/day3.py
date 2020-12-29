#! /usr/bin/env python3

import sys
from functools import reduce
from typing import List


def count_trees(layout: List[str], dx: int, dy: int) -> int:
    x = 0
    y = 0

    tree_count = 0

    while y < len(layout):
        # We want to wrap around so our X direction is infinite
        tree_count += int(layout[y][x % len(layout[y])] == '#')

        # Slope is 1/3
        y += dy
        x += dx

    return tree_count


def part1(layout: List[str]) -> None:
    print(count_trees(layout, 3, 1))


def part2(layout: List[str]) -> None:
    slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    print(
        reduce(lambda a, b: a * b, [count_trees(layout, dx, dy) for (dx, dy) in slopes])
    )


if __name__ == '__main__':
    inp = list(filter(lambda x: x, sys.stdin.read().split('\n')))

    part1(inp)
    part2(inp)
