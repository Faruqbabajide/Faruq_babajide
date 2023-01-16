use std::io;

 fn trapezium () {
println! ("What is the height of the trapezium? ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Nota avalid number");
println!("What is the length of base A?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");
println!("What is the length of base B ?");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");

    let d = a / 2.00 * (c + b);
 println!("The trapezium's surface area is {}", d);
 }

 fn rhombus () {
    println! ("length of the 1st diagonal? ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Nota avalid number");
println!("What is the length of the 2nd diagonal?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    let s = 0.50 * a * b;
 println!("The area of the rhombus is {}", s);
 }

 fn parallelogram () {
    println! (" length of the base of the parallelogram? ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let s:f64 = input1.trim().parse().expect("Not a valid number");
println!("What is the height of the parallelogram?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let t:f64 = input2.trim().parse().expect("Not a valid number");

    let y = s * t;
 println!("The area of the parallelogram is {}", y);
 }

 fn cube () {
    println! ("What is the lenght of a side of the cube? ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let p:f64 = input1.trim().parse().expect("Nota avalid number");
    
    let q = 6.00 * p * p;
 println!("The surface area of the cube is {}", q);
 }

 fn cylinder () {
    println! ("What is the radius of the cylinder? ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let m:f64 = input1.trim().parse().expect("Nota avalid number");
println!("What is the height of the cylinder?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let n:f64 = input2.trim().parse().expect("Not a valid number");


    let o = 3.14159265359 * m * m * n;
 println!("The volume of the cylinder is {}", o);
 }

 fn main () {
    println!("This program was created to carry out the computations listed below.
Calculation that is important
1. The trapezium's surface area
2. The circumference of a rhombus
3. The parallelogram's surface area
4. A cube's outside surface area
5. Cylinder volume  ");

    println!("Type in the no of the caluclation you would like to perform!");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let z:i32 = input1.trim().parse().expect("Not a valid number");

    if z == 1 {
        trapezium();
    }
    if z == 2 {
         rhombus();
    }
    if z == 3 {
        parallelogram();
    }
    if z == 4 {
        cube();
    }
    if z == 5 {
        cylinder();
    }












}