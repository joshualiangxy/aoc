from collections import deque

def main(lanternfish_timers):
    timers = deque([0] * 9)
    part1 = 0
    part2 = 0

    for timer in lanternfish_timers:
        timers[timer] += 1
        part2 += 1

    for day in range(256):
        if day == 80:
            part1 = part2
        birthing_fish = timers.popleft()
        timers.append(birthing_fish)
        timers[6] += birthing_fish
        part2 += birthing_fish

    return part1, part2

if __name__ == '__main__':
    with open('input', 'r') as f:
        lanternfish_timers = map(lambda x: int(x), f.readline().split(','))

    part1, part2 = main(lanternfish_timers)

    print(f'part1: {part1}')
    print(f'part2: {part2}')
