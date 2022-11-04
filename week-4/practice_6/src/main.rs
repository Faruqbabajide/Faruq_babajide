fn main() {
   let n1 = "Eletrical".to_string();
   let n2 = "Electronic".to_string();
   let n3 = "Engineering".to_string();
   let n4 = n1 + &n2 + &n3;
   println!("\n The {:?} is informed by the aspiration to train electrical/electronic/Engineering 
   	professionals in the area of design, building and maintenenance of electronic control system,", n4);
   let w1 = "computer".to_string();
   let w2 = "Science".to_string();
   let w3 = w1 + &w2;
   println!();
   println!("{:?} is aim at building competent computer scientist", w3 );
}
