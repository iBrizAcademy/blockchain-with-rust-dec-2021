fn main() {
    let unsorted: [i32; 9] = [5, 7, 3, 1, 2, 4, 6, 9, 8];
    println!("Unsorted: {:?}", unsorted);
    bubble_sort(unsorted);
}
fn bubble_sort(mut arr: [i32; 9]){
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    println!("Sorted: {:?}", arr);
}