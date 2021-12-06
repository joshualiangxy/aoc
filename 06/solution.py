from collections import deque
from copy import deepcopy

def part1(lanternfish_timers):
    timers = deque([0] * 9)
    total = 0

    for timer in lanternfish_timers:
        timers[timer] += 1
        total += 1

    for _ in range(80):
        birthing_fish = timers.popleft()
        timers.append(birthing_fish)
        timers[6] += birthing_fish
        total += birthing_fish

    return total

def part2(lanternfish_timers):
    timers = deque([0] * 9)
    total = 0

    for timer in lanternfish_timers:
        timers[timer] += 1
        total += 1

    for _ in range(256):
        birthing_fish = timers.popleft()
        timers.append(birthing_fish)
        timers[6] += birthing_fish
        total += birthing_fish

    return total

if __name__ == '__main__':
    with open('input', 'r') as f:
        lanternfish_timers = map(lambda x: int(x), f.readline().split(','))

    part_one_results = part1(deepcopy(lanternfish_timers))
    part_two_results = part2(deepcopy(lanternfish_timers))

    print(f'part1: {part_one_results}')
    print(f'part2: {part_two_results}')
