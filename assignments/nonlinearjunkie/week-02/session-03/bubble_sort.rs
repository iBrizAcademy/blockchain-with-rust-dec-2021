fn main() {

    let unsorted_vec = vec![6,7,3,2,8,9,1,5];
    println!("Unsorted vector:");
    display_vec(unsorted_vec.clone());
    let sorted_vec=sort(unsorted_vec);
    println!("Sorted vector:");
    display_vec(sorted_vec);
}

fn sort(mut my_vec: Vec<usize>) -> Vec<usize>{

    let vec_len=my_vec.len();

    for i in 0..vec_len-1{

        let mut swap_flag: bool=false;

        for j in 0..(vec_len-1-i){

            if my_vec[j]>my_vec[j+1]{
                my_vec.swap(j,j+1);
                swap_flag=true;
            }
        }

        if !swap_flag {
            break;
        }
        
    }
    my_vec
}

fn display_vec(my_vec: Vec<usize>){

     for num in &my_vec{
         print!("{}  ",num);
     }
     println!(" ")
}