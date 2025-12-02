#!/usr/bin/env python3

from pathlib import Path
import typing as t

DIAL_SIZE = 100


def process(input_path: Path, start_pos: int = 50) -> t.Tuple[int, int]:
    pos = start_pos
    stop_count = pass_count = 0

    with open(input_path) as f:
        for line in f:
            line = line.strip()
            sign = +1 if line[0] == 'R' else -1
            value = int(line[1:])

            # number of movements to reach first zero
            offset = DIAL_SIZE - pos if sign * pos < 0 else pos
            pass_count += (offset + value) // DIAL_SIZE

            pos += sign * value
            pos %= DIAL_SIZE

            if pos == 0:
                stop_count += 1

            #print(f"{line}, pos: {pos}, pass_count: {pass_count} stop_count: {stop_count}")

    return (stop_count, pass_count)


def main():
    input_folder = Path(__file__).absolute().parent.parent / 'input'
    input_path = input_folder / 'day1.txt'

    (stop_count, pass_count) = process(input_path)

    print(f"Stopped at 0 {stop_count} times")
    print(f"Passed 0 {pass_count} times")

if __name__ == '__main__':
    main()