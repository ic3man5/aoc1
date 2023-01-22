if __name__ == "__main__":
    with open("calories.txt", 'r') as f:
        all_calories = []
        total_calories = 0
        for line in f:
            if not line.strip():
                all_calories.append(total_calories)
                total_calories = 0
                continue
            total_calories += int(line.strip())
        print(f"Elf with the highest calorie count is: {max(all_calories)}!")
