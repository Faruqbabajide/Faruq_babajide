use std::io;
fn GeoPo_Merger() {
   let Geo_arr:[&str;5] =["South West","North East","South South","South West","South East"];
   println!("Geoplolitical Zones Are: {:?}", Geo_arr);
   let Minstry_arr:[&str;5] =["Internal Affairs","Justice","Defense","Power and Steel","Petroleum"];
    println!("Names of ministry includes: {:?}", Minstry_arr);
   let Commisioner_arr:[&str;5] =["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okroacha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Atieye"] ;
   println!("Names of commissioners are: {:?}", Commisioner_arr);
}
fn Pub_service(){
		println!("Public servant Profession sorter for Office Manager.How many years of experience do you have :");	let mut input1 = String::new();
io::stdin();
	  .read_line(&mut input1);
	let yrs1:f32= .input1.trim().parse();
	if yrs1 = 1 && yrs1=2;
	println!(" You are an intern");
	else if yrs1 =3 && <=5;
	println!(" You are an Administrator");
	else if yrs1 >=5 && <=8;
	println!(" You are a Senior Administrator");
	else if yrs1 >=8 && <=10;
	println!(" You are a Office Manager");
	else if yrs1 >=10 && <=13;
	println!(" You are a Director");
	else if yrs1 =14 
	println!(" You are a CEO");
println!("Public service Profession sorter for Academic .How many years of experience do you have :");	let mut input1 = String::new();
io::stdin();
	  .read_line(&mut input1);
	let yrs1:f32= .input1.trim().parse();
	if yrs1 = 1 && yrs1=2;
	println!(" You are an nothing");
	else if yrs1 =3 && <=5;
	println!(" You are an Research assistant");
	else if yrs1 >=5 && <=8;
	println!(" You are a PhD Candidate");
	else if yrs1 >=8 && <=10;
	println!(" You are a Post - Doc Researcher");
	else if yrs1 >=10 && <=13;
	println!(" You are a Senior Lecturer");
	else if yrs1 =14 
	println!(" You are a Dean");
}
println!("Public service Profession sorter for Lawyer .How many years of experience do you have :");	let mut input1 = String::new();
io::stdin();
	  .read_line(&mut input1);
	let yrs1:f32= .input1.trim().parse();
	if yrs1 = 1 && yrs1=2;
	println!(" You are an Paralegal");
	else if yrs1 =3 && <=5;
	println!(" You are an Associate");
	else if yrs1 >=5 && <=8;
	println!(" You are a Senior Associate 1-2");
	else if yrs1 >=8 && <=10;
	println!(" You are a Senior Associate 3-4");
	else if yrs1 >=10 && <=13;
	println!(" You are a Partner");
	else if yrs1 =14 
	
	println!("Public service Profession sorter for  Teacher.How many years of experience do you have :");	
	let mut input1 = String::new();
io::stdin();
	  .read_line(&mut input1);
	let yrs1:f32= .input1.trim().parse();
	if yrs1 = 1 && yrs1=2;
	println!(" You are an Placement");
	else if yrs1 =3 && <=5;
	println!(" You are an Classroom Teacher");
	else if yrs1 >=5 && <=8;
	println!(" You are a Senior Teacher");
	else if yrs1 >=8 && <=10;
	println!(" You are a Leading Teacher");
	else if yrs1 >=10 && <=13;
	println!(" You are a Deputy Principal");
	else if yrs1 =14 
	println!(" You are a Principal");
}
}
fn main(){
	//calling function 
	let mut input 1 = String::new()
println!("Do you want to review the Geopolitical  division or Know you Profession ");

GeoPo_Merger()
Pub_service()
}



