from functools import reduce

def preprocess(bits, num_packets = 1):
    packets = []

    while len(bits) > 0:
        version = int(bits[:3], 2)
        type_id = int(bits[3:6], 2)
        bits = bits[6:]

        packet = {
            'version': version,
            'type_id': type_id
        }

        if type_id == 4:
            literal = ''
            is_done = False

            while not is_done:
                literal += bits[1:5]
                is_done = bits[0] == '0'
                bits = bits[5:]

            packet['literal'] = int(literal, 2)
        else:
            length_id = int(bits[0])
            bits = bits[1:]
            subpackets = []

            if length_id == 0:
                num_subpacket_bits = int(bits[:15], 2)
                bits = bits[15:]
                subpackets = preprocess(bits[:num_subpacket_bits], -1)[1]
                bits = bits[num_subpacket_bits:]
            else:
                num_subpackets = int(bits[:11], 2)
                bits = bits[11:]
                bits, subpackets = preprocess(bits, num_subpackets)

            packet['length_id'] = length_id
            packet['subpackets'] = subpackets

        packets.append(packet)
        num_packets -= 1

        if num_packets == 0:
            break

    return bits, packets

def part1(packets):
    total = 0

    for packet in packets:
        total += packet['version']
        type_id = packet['type_id']

        if type_id != 4:
            total += part1(packet['subpackets'])

    return total

def part2(packet):
    type_id = packet['type_id']

    if type_id == 4:
        return packet['literal']

    subpackets = map(lambda subpacket: part2(subpacket), packet['subpackets'])

    if type_id == 0:
        return reduce(lambda x, y: x + y, subpackets, 0)
    if type_id == 1:
        return reduce(lambda x, y: x * y, subpackets, 1)
    if type_id == 2:
        return min(subpackets)
    if type_id == 3:
        return max(subpackets)
    if type_id == 5:
        return 1 if next(subpackets) > next(subpackets) else 0
    if type_id == 6:
        return 1 if next(subpackets) < next(subpackets) else 0
    if type_id == 7:
        return 1 if next(subpackets) == next(subpackets) else 0

if __name__ == '__main__':
    with open('input', 'r') as f:
        transmission = f.readline()
        binary = format(int(transmission, 16), f'0>{4 * len(transmission.rstrip())}b')
        packets = preprocess(binary)[1]

    part_one_results = part1(packets)
    part_two_results = part2(packets[0])

    print(f'part1: {part_one_results}')
    print(f'part2: {part_two_results}')
