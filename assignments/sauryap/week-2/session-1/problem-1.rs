fn main() {
    
    let mut arr: [i32; 5] = [5, 6, 3, 1, 10];
    let mut j =0;

    println!("Before {:?}", arr);
    loop {
        
        if arr[j] > arr[j+1]
        {
            bubbleswap(&mut arr);
            j =0;
            
        }else{
            j = j+1;
        } 
        
        if j >=4 {
            println!("After {:?}", arr);
            break;
        }
    }
    
    
}

fn bubbleswap(arr:&mut [i32; 5]) {
    let mut x = 0;
    for i in 0..arr.len()-1{
        
        if arr[i] > arr[i+1]
        {
            println!("{} is great than {}", arr[i], arr[i+1]);
            x = arr[i];
            arr[i] = arr[i+1];
            arr[i+1] = x;
        }
    }

    // println!("{:?}", arr);
}
