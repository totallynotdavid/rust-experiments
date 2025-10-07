fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    let (name, age) = cat;

    // so, this works, but was not really the task
    // println!("{name} is {age} years old", name = cat.0, age = cat.1);
    println!("{name} is {age} years old");
}
