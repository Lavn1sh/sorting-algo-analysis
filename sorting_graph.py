import matplotlib.pyplot as plt 

input_sizes = [10, 100, 250, 500, 750, 1000]

# Data for Insertion Sort
insertion_sort_best = [45, 4950, 31125, 124750, 280875, 499500]
insertion_sort_worst = [180, 19800, 124500, 499000, 1123500, 1998000]
insertion_sort_avg = [99, 11967, 80139, 314026, 710040, 1249878]

# Data for Selection Sort
selection_sort_best = [54, 5049, 31374, 125249, 281624, 500499]
selection_sort_worst = [69, 5199, 31749, 125999, 282749, 501999]
selection_sort_avg = [75, 5328, 32109, 126725, 283853, 503478]

# Data for Bubble Sort
bubble_sort_best = [30, 300, 750, 1500, 2250, 3000]
bubble_sort_worst = [180, 19800, 124500, 499000, 1123500, 1998000]
bubble_sort_avg = [113, 12485, 75612, 302011, 697377, 1252784]

plt.figure(figsize=(18, 12))

# Graph 1: Insertion Sort
plt.subplot(3, 1, 1)
plt.plot(input_sizes, insertion_sort_best, label="Best Case", marker='o', linestyle='-', color='blue', linewidth=2)
plt.plot(input_sizes, insertion_sort_avg, label="Average Case", marker='s', linestyle='--', color='green', linewidth=2)
plt.plot(input_sizes, insertion_sort_worst, label="Worst Case", marker='^', linestyle='-.', color='red', linewidth=2)
plt.title("Graph 1: Insertion Sort", fontsize=14)
plt.xlabel("Input Size", fontsize=12)
plt.ylabel("Steps", fontsize=12)
plt.text(input_sizes[-2], insertion_sort_best[-2] + 0.22 * insertion_sort_best[-2], 'O(n)', fontsize=10, color='blue', ha='left')
plt.text(input_sizes[-2], insertion_sort_avg[-2] + 0.22 * insertion_sort_avg[-2], 'O(n^2)', fontsize=10, color='green', ha='left')
plt.text(input_sizes[-2], insertion_sort_worst[-2] + 0.22 * insertion_sort_worst[-2], 'O(n^2)', fontsize=10, color='red', ha='left')
plt.legend()
plt.grid(True)

# Graph 2: Selection Sort
plt.subplot(3, 1, 2)
plt.plot(input_sizes, selection_sort_best, label="Best Case", marker='o', linestyle='-', color='blue', linewidth=2)
plt.plot(input_sizes, selection_sort_avg, label="Average Case", marker='s', linestyle='--', color='green', linewidth=2)
plt.plot(input_sizes, selection_sort_worst, label="Worst Case", marker='^', linestyle='-.', color='red', linewidth=2)
plt.title("Graph 2: Selection Sort", fontsize=14)
plt.xlabel("Input Size", fontsize=12)
plt.ylabel("Steps", fontsize=12)
plt.text(input_sizes[-2], selection_sort_best[-2] + 0.24 * selection_sort_best[-2], 'O(n^2)', fontsize=10, color='blue', ha='left')
plt.text(input_sizes[-2], selection_sort_avg[-2] + 0.24 * selection_sort_avg[-2], 'O(n^2)', fontsize=10, color='green', ha='left')
plt.text(input_sizes[-2], selection_sort_worst[-2] + 0.24 * selection_sort_worst[-2], 'O(n^2)', fontsize=10, color='red', ha='left')
plt.legend()
plt.grid(True)

# Graph 3: Bubble Sort
plt.subplot(3, 1, 3)
ax1 = plt.gca()
ax1.plot(input_sizes, bubble_sort_avg, label="Average Case", marker='s', linestyle='--', color='green', linewidth=2)
ax1.plot(input_sizes, bubble_sort_worst, label="Worst Case", marker='^', linestyle='-.', color='red', linewidth=2)
ax1.set_xlabel("Input Size", fontsize=12)
ax1.set_ylabel("Steps (Average/Worst)", fontsize=12)
ax1.set_yscale('log')
ax1.legend(loc='upper left')
ax1.grid(True)
ax2 = ax1.twinx()
ax2.plot(input_sizes, bubble_sort_best, label="Best Case", marker='o', linestyle='-', color='blue', linewidth=2)
ax2.set_ylabel("Steps (Best)", fontsize=12)
ax2.set_ylim(0, 3500)
ax2.legend(loc='upper right')

plt.title("Graph 3: Bubble Sort", fontsize=14)
plt.text(input_sizes[-2], bubble_sort_best[-2] + 0.12 * bubble_sort_best[-2], 'O(n)', fontsize=10, color='blue', ha='left')
plt.text(input_sizes[2], bubble_sort_avg[2] - 0.975 * bubble_sort_avg[2], 'O(n^2)', fontsize=10, color='green', ha='left')
plt.text(input_sizes[2], bubble_sort_worst[2] - 0.978 * bubble_sort_worst[2], 'O(n^2)', fontsize=10, color='red', ha='left')

plt.subplots_adjust(hspace=0.5)
plt.show()

