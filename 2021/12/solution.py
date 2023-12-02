def is_small_cave(node):
    return node.islower()

def dfs(graph, visited, node, can_visit_twice = False):
    if node == 'end':
        return 1
    if is_small_cave(node) and visited[node] >= 1:
        if not can_visit_twice:
            return 0
        can_visit_twice = False

    paths = 0
    visited[node] += 1

    for neighbour in graph[node]:
        paths += dfs(graph, visited, neighbour, can_visit_twice)

    visited[node] -= 1

    return paths

def part1(graph):
    visited = {}

    for key in graph:
        visited[key] = 0

    return dfs(graph, visited, 'start')

def part2(graph):
    visited = {}

    for key in graph:
        visited[key] = 0

    return dfs(graph, visited, 'start', True)

if __name__ == '__main__':
    graph = {}

    with open('input', 'r') as f:
        for line in f:
            node_one, node_two = line.rstrip().split('-')

            if node_one not in graph:
                graph[node_one] = []
            if node_two not in graph:
                graph[node_two] = []

            if node_one == 'start' or node_two == 'end':
                graph[node_one].append(node_two)
                continue
            if node_two == 'start' or node_one == 'end':
                graph[node_two].append(node_one)
                continue

            graph[node_one].append(node_two)
            graph[node_two].append(node_one)

    part_one_results = part1(graph)
    part_two_results = part2(graph)

    print(f'part1: {part_one_results}')
    print(f'part2: {part_two_results}')
