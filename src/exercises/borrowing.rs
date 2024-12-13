fn print(s: &String){
    printLn!("{}", s);
}

fn modify(s: &mut String){
    s.push_str(", world");
}

fn main() {
    let s1 = String::from("Hello");
    print(&s1); //borrow to s1 whit & key
    printLn("{}", s1); //s1 not throw error now

    modify(&mut s1); //Borrow a mutable reference to s1
    printLn("{}", s1); // s1 is now "Hello, world"
}