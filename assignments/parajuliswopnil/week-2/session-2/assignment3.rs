// - Program to pass ownership of a struct object, to a method and then returning it.
// - Program to pass struct variable as reference and dereferencing to modify value of object

struct User{
    id: i32,
    name: String,
}

fn main(){
    // Question number 1
    let mut user = User{
        id: 1,
        name: String::from("User1"),
    };

    println!("{}, {}", user.id, user.name); // still in the scope
    let returned = transfer_ownership_and_return(user); // ownership is transferred to the function 
    println!("{}, {}", returned.id, returned.name); // ownership is returned to the calling function 

    // Question number 2
    let mut user2 = User{
        id: 2,
        name: String::from("User2")
    };

    println!("{}, {}", user2.id, user2.name); // still in the scope
    pass_struct_as_reference_and_dereferencing_to_modify(&mut user2); // mutable reference is passed on to the function
    println!("{}, {}", user2.id, user2.name); // value printed after modifying in the function because the ownership was never transferred to the function
}

// Question number 1
fn transfer_ownership_and_return(struct_object: User)-> User{
    struct_object // returned the struct object variable to the calling function thus transferring the ownership aswell
}

// Question number 2
fn pass_struct_as_reference_and_dereferencing_to_modify(struct_object: &mut User){
    struct_object.id = 3; // modified the mutalbe refrence of User object 
    struct_object.name = String::from("User3"); // modified the mutalbe refrence of User object 
}