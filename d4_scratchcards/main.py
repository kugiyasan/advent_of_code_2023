import aoc_utils
from typing import List


PATH = "input"
# PATH = "example"


def part1(lines: List[str]) -> str:
    s = 0

    for line in lines:
        line = line.split(": ")[1]
        w, h = line.split(" | ")
        winning_numbers = {int(n) for n in w.split()}
        hand = {int(n) for n in h.split()}

        n = len(winning_numbers.intersection(hand))

        if n > 0:
            s += 2**(n - 1)

    return str(s)


def part2(lines: List[str]) -> str:
    arr = []

    for line in lines:
        line = line.split(": ")[1]
        w, h = line.split(" | ")
        winning_numbers = {int(n) for n in w.split()}
        hand = {int(n) for n in h.split()}

        n = len(winning_numbers.intersection(hand))

        arr.append(n)

    cards = [1] * len(arr)

    for i, scratch_card in enumerate(arr):
        for j in range(arr[i]):
            cards[i + j + 1] += cards[i]

    return str(sum(cards))


def submit(answer: str, part2: bool) -> None:
    part = "2" if part2 else "1"
    i = input(f"Press 'y' to submit '{answer}' for part {part}?: ")

    if i == "y":
        print("Submitting...")
        aoc_utils.submit(answer, part2)


def main() -> None:
    with open(PATH) as f:
        lines = f.readlines()
        # submit(str(part1(lines)), False)
        submit(str(part2(lines)), True)


if __name__ == "__main__":
    main()
