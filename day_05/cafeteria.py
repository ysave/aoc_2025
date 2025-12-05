def main():
    ranges, ingredient_ids = get_lists()
    part_one(ranges, ingredient_ids)   

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
    print(f"Total fresh ingredients found: {fresh_ingredients.__len__()}")

main()