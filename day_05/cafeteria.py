def main():
    ranges, ingredient_ids = get_lists()
    part_one(ranges, ingredient_ids)
    part_two(ranges)

def get_lists():
    ranges = []
    ingredient_ids = []
    current_array = ranges
    with open("ingredients.txt") as file:
        for line in file:
            if line.strip() == "":
                current_array = ingredient_ids
                continue
            current_array.append(line.rstrip())


    return ranges, ingredient_ids


def part_one(ranges, ingredient_ids):
    ingredient_set = {int(id) for id in ingredient_ids}
    fresh_ingredients = set()
    for r in ranges:
        start, end = map(int, r.split("-"))
        for ingredient in ingredient_set:
            if start <= ingredient <= end:
                fresh_ingredients.add(ingredient)
    print(f"Solution part 1: {fresh_ingredients.__len__()}")

def part_two(ranges):
    intervals = sorted([tuple(map(int, r.split('-'))) for r in ranges])

    merged_intervals = []
    current_start, current_end = intervals[0]

    for start, end in intervals[1:]:
        if start <= current_end:
            current_end = max(current_end, end)
        else:
            merged_intervals.append((current_start, current_end))
            current_start, current_end = start, end
    
    merged_intervals.append((current_start, current_end))

    total_covered = sum(end - start + 1 for start, end in merged_intervals)
    print(f"Solution part 2: {total_covered}")


main()