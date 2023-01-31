use std::io::Read;
use std::io;

fn admin (){
 let mut file = std::fs::File::open("globacom_db.txt").unwrap();
 let mut contents = String::new();
 file.read_to_string(&mut contents).unwrap();
 println!("{:?}",contents );
}
fn project(){
 let mut file = std::fs::File::open("project_tb.txt").unwrap();
 let mut contents = String::new();
 file.read_to_string(&mut contents).unwrap();
 println!("{:?}",contents );
}
fn employees(){
 let mut file = std::fs::File::open("staff_tb.txt").unwrap();
 let mut contents = String::new();
 file.read_to_string(&mut contents).unwrap();
 println!("{:?}",contents );
}
fn customer(){
 let mut file = std::fs::File::open("customer_tb.txt").unwrap();
 let mut contents = String::new();
 file.read_to_string(&mut contents).unwrap();
 println!("{:?}",contents );
}
fn vendor (){
 let mut file = std::fs::File::open("dataplan_tb.txt").unwrap();
 let mut contents = String::new();
 file.read_to_string(&mut contents).unwrap();
 println!("{:?}",contents );
}


fn main(){
let mut input = String::new();
	println!("Hello there!!!");
    println!("I would like to know what user you are , an Administrator ,a vendor ,a customer ,an employees, or a project manager ?");
    println!("Enter 1 for Administrator");
    println!("Enter 2 for Vendor");
    println!("Enter 3 for Customer");
    println!("Enter 4 for Employees");
    println!("Enter 5 for Project Manager");
io::stdin().read_line(&mut input).expect("Not a valid string");
let input:i32 = input.trim().parse().expect("Not a valid number");
if input == 1 {
   admin();
}
else if  input == 2 {
   vendor();
}
else if  input == 3 {
   customer();
}
else if  input == 4{
   employees();
}
else if  input == 5 {
   project();
}
}