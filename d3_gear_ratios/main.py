import aoc_utils
from typing import List, Tuple
import re


PATH = "input"
# PATH = "example"

Vec2 = Tuple[int, int]


def find_symbol(lines: List[str], top_left: Vec2, bottom_right: Vec2) -> Vec2:
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

            # if not c.isdigit() and c != ".":
            if c == "*":
                print()
                return x, y

        print()

    n = lines[top_left[1] + 1][top_left[0]+1:bottom_right[0]-1]
    print(f"{top_left=} {bottom_right=} {n}")


def part1(lines: List[str], part2: bool) -> str:
    s = 0
    near_gear = {}

    for i, line in enumerate(lines):
        for m in re.finditer(r"\d+", line):
            # print(m)

            if not part2:
                if find_symbol(lines, (m.start() - 1, i - 1), (m.end() + 1, i + 1)):
                    s += int(line[m.start():m.end()])
            else:
                coord = find_symbol(
                    lines, (m.start() - 1, i - 1), (m.end() + 1, i + 1))
                if coord is not None:
                    n = int(line[m.start():m.end()])
                    if near_gear.get(coord) is not None:
                        s += near_gear[coord] * n
                    else:
                        near_gear[coord] = n

    print(s)
    return str(s)


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
        # submit(str(part1(lines, False)), False)
        submit(str(part1(lines, True)), True)


if __name__ == "__main__":
    main()
