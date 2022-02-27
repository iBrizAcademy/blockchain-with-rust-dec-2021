fn bubble_sort<T: Ord>(arr: &mut [T]) -> &mut [T] {
    for i in 0..arr.len() {
        let j_len = arr.len() - 1 - i;
        for j in 0..j_len {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1)
            }
        }
    }
    return arr;
}

fn main() {
    let mut numbers = [5, 1, 4, 2, 8, -10];
    println!("before sorting: {:?}", numbers);
    let result = bubble_sort(&mut numbers);
    println!("after sorting: {:?}", result);
}
