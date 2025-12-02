#!/usr/bin/env python3

from pathlib import Path
import typing as t


def parse_range(range: str) -> t.Tuple[int, int]:
    (lower, upper) = map(int, range.strip().split('-'))
    return (lower, upper)


def is_repeated(s: str, n: int) -> bool:
    return s == s[:n] * (len(s) // n)


def process(input_path: t.Union[str, Path]) -> t.Tuple[int, int]:
    with open(input_path) as f:
        ranges = list(map(parse_range, f.read().split(',')))

    sum_part1 = 0
    sum_part2 = 0
    for (lower, upper) in ranges:
        for value in range(lower, upper+1):
            s = str(value)
            for repeat_len in range(1, len(s)//2 + 1):
                if is_repeated(s, repeat_len):
                    sum_part2 += value
                    if repeat_len * 2 == len(s):
                        sum_part1 += value
                    break

    return (sum_part1, sum_part2)



def main():
    input_folder = Path(__file__).absolute().parent.parent / 'input'
    input_path = input_folder / 'day2.txt'

    (sum_part1, sum_part2) = process(input_path)

    print(f"Part 1 sum: {sum_part1}")
    print(f"Part 2 sum: {sum_part2}")

if __name__ == '__main__':
    main()