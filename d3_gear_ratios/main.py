import aoc_utils
from typing import List, Tuple
import re


PATH = "input"
# PATH = "example"

Vec2 = Tuple[int, int]


def find_symbol(lines: List[str], top_left: Vec2, bottom_right: Vec2) -> bool:
    for y in range(top_left[1], bottom_right[1] + 1):
        if y < 0 or len(lines) <= y:
            print("skipping y")
            continue

        for x in range(top_left[0], bottom_right[0]):
            if x < 0 or len(lines[y]) <= x:
                print("skipping x")
                continue

            c = lines[y][x]
            print(f"{c}", end="")

            if not c.isdigit() and c != ".":
                print()
                return True

        print()

    n = lines[top_left[1] + 1][top_left[0]+1:bottom_right[0]-1]
    print(f"{top_left=} {bottom_right=} {n}")
    return False


def part1(lines: List[str]) -> str:
    # lines = lines[102:105]
    s = 0

    for i, line in enumerate(lines):
        for m in re.finditer(r"\d+", line):
            # print(m)

            if find_symbol(lines, (m.start() - 1, i - 1), (m.end() + 1, i + 1)):
                s += int(line[m.start():m.end()])

    print(s)
    return str(s)


def part2(lines: List[str]) -> str:
    pass


def submit(answer: str, part2: bool) -> None:
    part = "2" if part2 else "1"
    try:
        i = input(f"Press 'y' to submit '{answer}' for part {part}?: ")

        if i == "y":
            print("Submitting...")
            aoc_utils.submit(answer, part2)
    except EOFError:
        pass


def main() -> None:
    with open(PATH) as f:
        lines = [s.strip() for s in f.readlines()]
        submit(str(part1(lines)), False)
        # submit(str(part2(lines)), True)


if __name__ == "__main__":
    main()
