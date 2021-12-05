def part1(arr):
    if len(arr) < 1:
        return

    length = len(arr[0]) - 1

    bit_zero_count = [0] * length
    bit_one_count = [0] * length
    epsilon = [0] * length
    gamma = [0] * length

    for binary in arr:
        for i in range(length):
            if int(binary[i]):
                bit_one_count[i] += 1
            else:
                bit_zero_count[i] += 1

    for i in range(length):
        gamma[i] = '0' if bit_zero_count[i] > bit_one_count[i] else '1'
        epsilon[i] = '1' if bit_zero_count[i] > bit_one_count[i] else '0'

    return int(''.join(epsilon), 2) * int(''.join(gamma), 2)

def iteration(arr, iteration, is_o2_generation = False):
    if len(arr) <= 1:
        return arr

    bit_one_count = 0
    bit_zero_count = 0

    for binary in arr:
        if int(binary[iteration]):
            bit_one_count += 1
        else:
            bit_zero_count += 1

    if is_o2_generation:
        check_bit = '0' if bit_zero_count > bit_one_count else '1'
    else:
        check_bit = '1' if bit_one_count < bit_zero_count else '0'

    return list(filter(lambda binary: binary[iteration] == check_bit, arr))

def part2(arr):
    if len(arr) < 1:
        return

    length = len(arr[0]) - 1

    o2_generator_rating = arr
    co2_scrubber_rating = arr

    for i in range(length):
        o2_generator_rating = iteration(o2_generator_rating, i, is_o2_generation = True)
        co2_scrubber_rating = iteration(co2_scrubber_rating, i)

    return int(''.join(o2_generator_rating[0]), 2) * int(''.join(co2_scrubber_rating[0]), 2)

if __name__ == '__main__':
    with open('input', 'r') as f:
        arr = f.readlines()

    part_one_results = part1(arr)
    part_two_results = part2(arr)

    print(f'part1: {part_one_results}')
    print(f'part2: {part_two_results}')
