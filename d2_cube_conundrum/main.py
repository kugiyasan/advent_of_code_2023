import aoc_utils
from typing import List


PATH = "input"
# PATH = "example"

CUBES = {
    "red": 12,
    "green": 13,
    "blue": 14,
}


def part1(lines: List[str]) -> str:
    answer = []

    for line in lines:
        id, game = line.split(": ")
        sets = game.split("; ")

        possible_sets = True

        for s in sets:
            cubes = s.split(", ")

            for cube in cubes:
                print(cube)
                n, color = cube.split(" ")
                n = int(n)
                color = color.strip()

                if n > CUBES[color]:
                    possible_sets = False

        if possible_sets:
            answer.append(int(id[5:]))

    print(answer)
    return str(sum(answer))


def part2(lines: List[str]) -> str:
    answer = []

    for line in lines:
        id, game = line.split(": ")
        sets = game.split("; ")

        min_set = {
            "red": 0,
            "green": 0,
            "blue": 0,
        }

        for s in sets:
            cubes = s.split(", ")

            for cube in cubes:
                print(cube)
                n, color = cube.split(" ")
                n = int(n)
                color = color.strip()

                min_set[color] = max(min_set[color], n)

        red = min_set["red"]
        green = min_set["green"]
        blue = min_set["blue"]
        answer.append(red * green * blue)

    print(answer)
    return str(sum(answer))


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
