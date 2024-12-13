fn main() {
    let mut s1 = String::from("Hello");
    let s2 = &mut s1; //Mutable reference to s1
    let s3 = &mut s1;  //Attempt to crete another mutable reference, which is not allowed

    printLn("{}", s1);
}

fn main() {
    let mut s1 = String::from("Hello");
    let s2 = &s1; // Immutable reference
    let s3 = &mut s1; // Mutable reference

    printLn("{}", s2);
    printLn("{}", s3); // This will cause an error
}