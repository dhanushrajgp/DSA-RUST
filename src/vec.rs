pub fn vec_examples(){
    println!("Hello, world!");
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    //initializing and declaring a vector example.
    let _vec = vec![1,2,3];


    //methods of accessing a value in vector.
    let third:&i32 = &v[2];
    println!("the third element is {third}");


    let fourth: Option<&i32> = v.get(3);
    match fourth{
        Some(fourth) => println!("The fourth element is {fourth}"),
        None => println!("There's no fourth element")
    }

    //for loop to get immutable references to each element in vector.

    for i in &v{
        println!("{i}");
    }

    //we can also iterate over mutable references to each element in vector.
    for i in &mut v{
        *i += 50;
        println!("{i}");
    }



    // * is a dereference operator to get to the value of i.
}