use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Hello i'm your RUST quadratic equation calculator. By entering the corresponding values for a, b, and c, I can help you determine the equation's roots. ");

println!("Input your value of a: ");
io::stdin().read_line(&mut input1).expect("Not a valid string");
let a:f32 = input1.trim().parse().expect("Not a valid number");

println!("Input your value of b: ");
io::stdin().read_line(&mut input2).expect("Not a valid string");
let b:f32 = input2.trim().parse().expect("Not a valid number");

println!("Input your value of c: ");
io::stdin().read_line(&mut input3).expect("Not a valid string");
let c:f32 = input3.trim().parse().expect("Not a valid number");

let d:f32 = -1.00 * b;
let e:f32 = b * b - (4.00 * a * c );
let f:f32 = e.sqrt();
let g:f32 = 2.00 * a;
let x:f32 = (d + f) / g ;

let h:f32 = -1.00 * b;
let i:f32 = b * b - (4.00 * a * c );
let j:f32 = i.sqrt();
let k:f32 = 2.00 * a;
let y:f32 = (h - j) / k;

if (b * b) - (4.00 * a * c) < 0.00
    {
        println! ("There are no real roots of this equation, Restart the process");
    }

else if (b * b) - (4.00 * a * c) == 0.00
       {
        println! ("The roots of this equation have the same value of {}", x);
    }
else 
       {
        println! ("The distinct roots of this quadratic eqaution are {}, and {} as requested for ,Thank you!", x, y);
    }
}