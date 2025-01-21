use rand::Rng;

fn main() {
    let input_sizes = vec![10, 100, 250, 500, 750, 1000];

    println!("Analyzing Selection Sort:");
    analyze_step_count(selection_sort, &input_sizes);

    println!("\nAnalyzing Bubble Sort:");
    analyze_step_count(bubble_sort, &input_sizes);

    println!("\nAnalyzing Insertion Sort:");
    analyze_step_count(insertion_sort, &input_sizes);
}

// Sorting algorithms with step counting
fn insertion_sort(arr: &mut Vec<i32>, step_count: &mut u64) {
    let n = arr.len();
    for i in 0..n {
        *step_count += 1;
        for j in (1..=i).rev() {
            *step_count += 1;
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
                *step_count += 3;
            }
            *step_count += 2;
        }
        *step_count += 1;
    }
}

fn selection_sort(arr: &mut Vec<i32>, step_count: &mut u64) {
    let n = arr.len();
    for i in 0..n - 1 {
        let mut min = i;
        *step_count += 2;
        for j in i..n {
            *step_count += 1;
            if arr[j] < arr[min] {
                min = j;
                *step_count += 1;
            }
            *step_count += 2;
        }
        if min != i {
            arr.swap(i, min);
            *step_count += 3;
        }
        *step_count += 2;
    }
}

fn bubble_sort(arr: &mut Vec<i32>, step_count: &mut u64) {
    let n = arr.len();
    for i in (1..n).rev() {
        let mut did_swap = false;
        *step_count += 2;
        for j in 0..i {
            *step_count += 1;
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                did_swap = true;
                *step_count += 4;
            }
            *step_count += 2;
        }
        *step_count += 1;
        if !did_swap {
            break;
        }
        *step_count += 1;
    }
}

// Input generation
fn generate_input(size: usize, input_type: &str) -> Vec<i32> {
    match input_type {
        "random" => (0..size)
            .map(|_| rand::thread_rng().gen_range(0..size as i32))
            .collect(),
        "sorted" => (0..size as i32).collect(),
        "reversed" => (0..size as i32).rev().collect(),
        _ => vec![],
    }
}

// Analyze step count for sorting algorithms
fn analyze_step_count<F>(sort_function: F, input_sizes: &[usize])
where
    F: Fn(&mut Vec<i32>, &mut u64),
{
    println!("Step Count Analysis (Best, Worst, and Average Cases):");
    println!(
        "{:<12} {:<20} {:<20} {:<20}",
        "Input Size", "Best Case Steps", "Worst Case Steps", "Average Case Steps"
    );
    println!("{}", "-".repeat(72));

    for &size in input_sizes {
        // Best Case
        let mut best_case_input = generate_input(size, "sorted");
        let mut step_count = 0;
        sort_function(&mut best_case_input, &mut step_count);
        let best_steps = step_count;

        // Worst Case
        let mut worst_case_input = generate_input(size, "reversed");
        step_count = 0;
        sort_function(&mut worst_case_input, &mut step_count);
        let worst_steps = step_count;

        // Average Case
        let mut average_case_input = generate_input(size, "random");
        step_count = 0;
        sort_function(&mut average_case_input, &mut step_count);
        let average_steps = step_count;

        // Print the results in a tabular format
        println!(
            "{:<12} {:<20} {:<20} {:<20}",
            size, best_steps, worst_steps, average_steps
        );
    }
}
