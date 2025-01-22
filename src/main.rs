use std::io::{self, Write};

// Enum representing the order in which to sort the array.
#[derive(Debug, PartialEq)]
enum SortOrder {
    Ascending,
    Descending,
}

// Struct to hold the array and the step count for sorting.
#[derive(Debug)]
struct SortContext {
    arr: Vec<i32>,
    step_count: usize,
}

impl SortContext {
    // Implements a new function which creates a new SortContext
    // The new SortContext takes the array and sets the step count to 0
    fn new(arr: Vec<i32>) -> Self {
        Self { arr, step_count: 0 }
    }
}

// Bubble sort function
// takes in SortContext(containing array and step_count) and sorting order as params
// We increase the step count in case of any assignments or comparisons.
// We sort differently depending on whether we have Ascending or Descending SortOrder
fn bubble_sort(context: &mut SortContext, order: &SortOrder) {
    let n = context.arr.len();
    for i in (1..n).rev() {
        let mut did_swap = false;
        context.step_count += 2;
        for j in 0..i {
            context.step_count += 1;
            if (order == &SortOrder::Ascending && context.arr[j] > context.arr[j + 1])
                || (order == &SortOrder::Descending && context.arr[j] < context.arr[j + 1])
            {
                context.arr.swap(j, j + 1);
                did_swap = true;
                context.step_count += 4;
            }
            context.step_count += 2;
        }
        context.step_count += 1;
        if !did_swap {
            break;
        }
    }
}

// Selection sort function
// takes in SortContext(containing array and step_count) and sorting order as params
// We increase the step count in case of any assignments or comparisons.
// We sort differently depending on whether we have Ascending or Descending SortOrder
fn selection_sort(context: &mut SortContext, order: &SortOrder) {
    let n = context.arr.len();
    for i in 0..n - 1 {
        let mut min = i;
        context.step_count += 2;
        for j in i + 1..n {
            context.step_count += 1;
            if (order == &SortOrder::Ascending && context.arr[j] < context.arr[min])
                || (order == &SortOrder::Descending && context.arr[j] > context.arr[min])
            {
                min = j;
            }
            context.step_count += 2;
        }
        if min != i {
            context.arr.swap(i, min);
            context.step_count += 3;
        }
        context.step_count += 2;
    }
}

// Insertion sort function
// takes in SortContext(containing array and step_count) and sorting order as params
// We increase the step count in case of any assignments or comparisons.
// We sort differently depending on whether we have Ascending or Descending SortOrder
fn insertion_sort(context: &mut SortContext, order: &SortOrder) {
    let n = context.arr.len();
    for i in 1..n {
        let mut j = i;
        context.step_count += 1;
        while j > 0
            && ((order == &SortOrder::Ascending && context.arr[j] < context.arr[j - 1])
                || (order == &SortOrder::Descending && context.arr[j] > context.arr[j - 1]))
        {
            context.arr.swap(j, j - 1);
            j -= 1;
            context.step_count += 3;
        }
        context.step_count += 2;
    }
}

// Analyze step count for all algorithms
// As parameter we have sort_fn which is the sorting function that we want to analyse
// input_sizes takes in the input size of the array
// algorithm_name is a string where we specify which sorting algorithm to be applied
// order is the SortOrder that will either be Ascending or Descending
fn analyze_step_count(
    sort_fn: fn(&mut SortContext, &SortOrder),
    input_sizes: &[usize],
    algorithm_name: &str,
    order: &SortOrder,
) {
    println!(
        "\n{} Step Count Analysis for {} Order:",
        algorithm_name,
        if *order == SortOrder::Ascending {
            "Ascending"
        } else {
            "Descending"
        }
    );
    println!("---------------------------------------------------------------------");
    println!("| Input Size | Sort Case Steps | Rev. Sort Case Steps | Random Case Steps |");
    println!("---------------------------------------------------------------------");

    for &size in input_sizes {
        // We are looping through every input size and then doing the following analysing on each of them
        // Best Case (sorted array)
        let best_case_input: Vec<i32> = (0..size as i32).collect();
        let mut best_case_context = SortContext::new(best_case_input);
        sort_fn(&mut best_case_context, &order);
        let best_case_steps = best_case_context.step_count;

        // Worst Case (reversed array)
        let worst_case_input: Vec<i32> = (0..size as i32).rev().collect();
        let mut worst_case_context = SortContext::new(worst_case_input);
        sort_fn(&mut worst_case_context, &order);
        let worst_case_steps = worst_case_context.step_count;

        // Average Case (random array)
        let random_case_input: Vec<i32> = (0..size as i32)
            .map(|_| rand::random::<i32>() % 100)
            .collect();
        let mut random_case_context = SortContext::new(random_case_input);
        sort_fn(&mut random_case_context, &order);
        let average_case_steps = random_case_context.step_count;

        // Print the result for the current input size
        println!(
            "| {:>10} | {:>15} | {:>15} | {:>18} |",
            size, best_case_steps, worst_case_steps, average_case_steps
        );
    }

    println!("---------------------------------------------------------------------");
}

// Test custom array
fn test_custom_array() {
    let mut input = String::new();
    print!("Do you want to test a custom array? (y/n): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let response = input.trim().to_lowercase();

    if response == "y" {
        // Get the custom array from the user
        input.clear();
        print!("Enter the number of elements for the array: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let size: usize = input.trim().parse().unwrap();

        input.clear();
        print!("Enter {} elements for the array: ", size);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let custom_array: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // Sorting in ascending and descending order for each algorithm
        let mut arr = custom_array.clone();
        let mut bubble_context = SortContext::new(arr.clone());
        let mut selection_context = SortContext::new(arr.clone());
        let mut insertion_context = SortContext::new(arr.clone());

        // Ascending order
        println!("\nSorting in Ascending Order:");
        println!("---------------------------");

        bubble_sort(&mut bubble_context, &SortOrder::Ascending);
        println!(
            "Bubble Sort: Steps = {}, Sorted Array: {:?}",
            bubble_context.step_count, bubble_context.arr
        );

        selection_sort(&mut selection_context, &SortOrder::Ascending);
        println!(
            "Selection Sort: Steps = {}, Sorted Array: {:?}",
            selection_context.step_count, selection_context.arr
        );

        insertion_sort(&mut insertion_context, &SortOrder::Ascending);
        println!(
            "Insertion Sort: Steps = {}, Sorted Array: {:?}",
            insertion_context.step_count, insertion_context.arr
        );

        // Descending order
        arr = custom_array.clone();
        let mut bubble_context = SortContext::new(arr.clone());
        let mut selection_context = SortContext::new(arr.clone());
        let mut insertion_context = SortContext::new(arr.clone());

        println!("\nSorting in Descending Order:");
        println!("----------------------------");

        bubble_sort(&mut bubble_context, &SortOrder::Descending);
        println!(
            "Bubble Sort: Steps = {}, Sorted Array: {:?}",
            bubble_context.step_count, bubble_context.arr
        );

        selection_sort(&mut selection_context, &SortOrder::Descending);
        println!(
            "Selection Sort: Steps = {}, Sorted Array: {:?}",
            selection_context.step_count, selection_context.arr
        );

        insertion_sort(&mut insertion_context, &SortOrder::Descending);
        println!(
            "Insertion Sort: Steps = {}, Sorted Array: {:?}",
            insertion_context.step_count, insertion_context.arr
        );

        // Print step count summary
        println!("\nStep Count Summary for Input Size {}:", size);
        println!("-------------------------------------------------");
        println!("| Algorithm       | Ascending Steps | Descending Steps |");
        println!("-------------------------------------------------");

        // Ascending Step Counts
        let mut bubble_context = SortContext::new(custom_array.clone());
        bubble_sort(&mut bubble_context, &SortOrder::Ascending);

        let mut selection_context = SortContext::new(custom_array.clone());
        selection_sort(&mut selection_context, &SortOrder::Ascending);

        let mut insertion_context = SortContext::new(custom_array.clone());
        insertion_sort(&mut insertion_context, &SortOrder::Ascending);

        // Descending Step Counts
        let mut bubble_context_desc = SortContext::new(custom_array.clone());
        bubble_sort(&mut bubble_context_desc, &SortOrder::Descending);

        let mut selection_context_desc = SortContext::new(custom_array.clone());
        selection_sort(&mut selection_context_desc, &SortOrder::Descending);

        let mut insertion_context_desc = SortContext::new(custom_array.clone());
        insertion_sort(&mut insertion_context_desc, &SortOrder::Descending);

        println!(
            "| Bubble Sort     | {:>15} | {:>17} |",
            bubble_context.step_count, bubble_context_desc.step_count
        );
        println!(
            "| Selection Sort  | {:>15} | {:>17} |",
            selection_context.step_count, selection_context_desc.step_count
        );
        println!(
            "| Insertion Sort  | {:>15} | {:>17} |",
            insertion_context.step_count, insertion_context_desc.step_count
        );
        println!("-------------------------------------------------");
    } else {
        println!("No custom array testing requested. Exiting...");
    }
}

// Main function
fn main() {
    let input_sizes = vec![10, 20, 30, 40, 50];
    analyze_step_count(
        selection_sort,
        &input_sizes,
        "Selection Sort",
        &SortOrder::Ascending,
    );
    analyze_step_count(
        selection_sort,
        &input_sizes,
        "Selection Sort",
        &SortOrder::Descending,
    );

    // Bubble Sort Analysis
    analyze_step_count(
        bubble_sort,
        &input_sizes,
        "Bubble Sort",
        &SortOrder::Ascending,
    );
    analyze_step_count(
        bubble_sort,
        &input_sizes,
        "Bubble Sort",
        &SortOrder::Descending,
    );

    // Insertion Sort Analysis
    analyze_step_count(
        insertion_sort,
        &input_sizes,
        "Insertion Sort",
        &SortOrder::Ascending,
    );
    analyze_step_count(
        insertion_sort,
        &input_sizes,
        "Insertion Sort",
        &SortOrder::Descending,
    );

    println!("\nTesting with custom array...");
    test_custom_array();
}
