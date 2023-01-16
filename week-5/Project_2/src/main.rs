fn main() {
   let mut input1 = String::new();
  
   println!("\n");
  io::stdin().read_line(&mut input1).expect("Not a valid string");
    Years = input3.trim().parse().expect("Not a valid number");
  if Years >=40    {
  	println!("Your incentive is N1,560,000")
 }
   if else Years >=30 && <40 
  {  println!("Your incentive is N1,480,000")
  }
   if else Years <28 
 {
  println!("Your incentive is N1,300,000")
   
       println!("How Old are you")
  io::stdin().read_line(&mut input2).expect("Not a valid string");
     = input2.trim().parse().expect("Not a valid number");
   if Years >=40 

   {
    println!("Your incentive is N1,560,000")
   }
    if else Years >=30 && <40 

   {
    println!("Your incentive is N1,480,000")
   }
    if else Years <28 

   {
    println!("Your incentive is N1,300,000")
   }
   println!("\nAre you inexperience do you have (true or false) :");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    Years = input3.trim().parse().expect("Not a valid number");
    if true { 
   println!("Your incentive will be N100,000");
} 
    if false { 
   println!("Great! your incenive would have been stated above");
} 

}