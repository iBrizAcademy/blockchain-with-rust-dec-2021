fn main(){
    let mut _unsorted_array = &mut [64, 34, 25, 12, 22, 11, 90, 10, 105, 0];
    for i in 0.._unsorted_array.len(){
        for j in i + 1.._unsorted_array.len(){
            if _unsorted_array[i]>_unsorted_array[j]{
                let temp = _unsorted_array[j];
                _unsorted_array[j] = _unsorted_array[i];
                _unsorted_array[i] = temp;
            }
        } 
    }
    println!("{:?}", _unsorted_array);
}