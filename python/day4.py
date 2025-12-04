#!/usr/bin/env python3

import itertools
from pathlib import Path
import typing as t

import numpy
from numpy.typing import NDArray
import scipy.ndimage


def load_array(input_path: t.Union[str, Path]) -> NDArray[numpy.bool_]:
    arr = []
    with open(input_path, 'rb') as f:
        for line in f:
            arr.append(numpy.frombuffer(line.strip(), dtype=numpy.uint8))

    arr = numpy.stack(arr, axis=0)
    return arr == b'@'[0]

def print_array(arr: NDArray[numpy.bool_]):
    print(numpy.concatenate([
        numpy.where(arr, b'@'[0], b'.'[0]),
        numpy.full((arr.shape[0], 1), b'\n'[0])
    ], axis=1).tobytes().decode('ascii'), end='\n')

def process(input_path: t.Union[str, Path], verbose: bool = False):
    arr = load_array(input_path)
    weights = numpy.ones((3,), numpy.uint8)
    n_total = numpy.sum(arr, dtype=numpy.uint64)

    if verbose:
        print("Initial state:")
        print_array(arr)

    for i in itertools.count(1):
        convolved = arr.astype(numpy.uint8)
        for axis in (0, 1):
            convolved = scipy.ndimage.convolve1d(convolved, weights, axis=axis, mode='constant')

        available = arr & (convolved < 5)  # 4 neighbors + self
        n_available = numpy.sum(available)
        print(f"Step {i}, {n_available:3} box(es) available")
        if not n_available:
            break
        if verbose:
            print_array(arr)

        arr ^= available

    print(f"Finished in {i} step(s), final state:")
    print_array(arr)
    n_final = numpy.sum(arr, dtype=numpy.uint64)
    print(f"{n_total} -> {n_final} boxes (removed {n_total - n_final})")


def main():
    input_folder = Path(__file__).absolute().parent.parent / 'input'
    input_path = input_folder / 'day4.txt'

    process(input_path)

if __name__ == '__main__':
    main()