// sh build-n-run.sh assignments/manjitshakya/week-2/session-1/bubblesort.rs
fn bubble_sort(mut arr: [i32; 15]) {
    println!("Unsorted: {:?}", arr);
    let mut index: usize = 0;
    let mut temp: i32;
    while index < arr.len() {
        for j in 0..arr.len() - 1 {
            if arr[j] > arr[j + 1] {
                temp = arr[j + 1];
                arr[j + 1] = arr[j];
                arr[j] = temp;
            }
        }
        index = index + 1;
    }
    println!("Sorted:   {:?}", arr);
}

fn main() {
    let array: [i32; 15] = [0, 99, 1, 10, 2, 100, 87, 5, 1, 0, 12, 11, -1, 0, 0];
    bubble_sort(array);
}