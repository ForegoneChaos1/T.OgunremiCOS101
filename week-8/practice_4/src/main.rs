fn main() {

    //Name vector
    let name = vec!["Mary", "Sam", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];

//age vector
    let age = [16,17,19,22,20,21,18,23];

    print!("\nAge allocation:\n");

    for i in 0..age.len()
    {
        print!("{} is {} years old\n",name[i],age[i]);
    }
}
