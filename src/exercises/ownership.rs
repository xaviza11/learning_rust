fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; //s1 transfer property to s2

    println!("{}"m s2); //print hello

    //try to print s1 throw error because rust delete property

}

fn take_property(s: String) {
    println("{}", s);
}

fn main() {
    let s1 = String::from("World");
    //take_property(s1); //Property s1 is transfer here so now print s1 throw error
}

