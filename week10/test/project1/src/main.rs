
use std::io::Write;
fn code_7 (){
    let mut input = String::new();

 println!("\nInput '7' to create a text file for  Akpevwe Iloka and Aigbona Juliet. ");
 println!("\nInput '8' to create a text file for  Eahis Ero and Maria Akinsola. ");
 println!("\nInput '9' to create a text file for  Adamu Sagamu and Gbenga Daniels.");
  std::io::stdin().read_line(&mut input).expect("failed to read input");
 let number:i32 = input.trim().parse().expect("Invalid input");
  
 if number == 7{
    println!("A file would be created for Aigbona Juliet if you \nInput '1' or '2' for Akpevwe Iloka");
    let mut file_create = String::new();
    std::io::stdin().read_line(&mut file_create).expect("Invalid Input, try again");
       let file_create:i32= file_create.trim().parse().expect("invalid input");
        if file_create==1 {
            let announce = "Department: Consulting\n";
            let dept ="Department of Consulting";
            let mut file = std::fs::File::create("Aigbona_juliet.txt").expect("create failed");
            file.write_all("Name : Aigbona Juliet \n".as_bytes()).expect("write failed");
            file.write_all(announce.as_bytes()).expect("write failed");
            file.write_all("Qualification: B.sc. \n".as_bytes()).expect("write failed");
            file.write_all("Job Service: \n﻿Audit servicesClimate change and sustainability servicesFinancial\n﻿accounting advisory services Forensic and Integrity services\n﻿Private client audit experience Accounting Link Assurance".as_bytes()).expect("write failed");
            

            println!("\nData has been written to file");

        }

        if file_create==2 {
             let announce = "Department: Assurance\n";
            let dept ="Department of assurance";
            let mut file = std::fs::File::create("AkpevweIloka.txt").expect("create failed");
            file.write_all("Name :Akpevwe Iloka \n".as_bytes()).expect("write failed");
            file.write_all(announce.as_bytes()).expect("write failed");
            file.write_all("Qualification: HND \n".as_bytes()).expect("write failed");
            file.write_all("Job Service: \nAudit  climatic change and sustainabilty service \nCybersecurity, strategy, risk, compliance and resilience  \ndigital transformation Risk consulting services Supply chain and operations Technology transformation".as_bytes()).expect("write failed");
            

            println!("\nData has been written to file");
        }
     
       }
        
    else if number == 8 {
            println!("A file would be created for Adamu Sagamu if you \nInput '1' or '2' for Gbenga Daniels");
    let mut file_create = String::new();
    std::io::stdin().read_line(&mut file_create).expect("Invalid Input, try again");
       let file_create:i32= file_create.trim().parse().expect("invalid input");
        if file_create==1 {
                 let announce = "Department: Tax\n";
            let dept ="Department of Tax";
            let mut file = std::fs::File::create("Adamu_Sagamu.txt").expect("create failed");
            file.write_all("Name : Adamu Sagamu \n".as_bytes()).expect("write failed");
            file.write_all(announce.as_bytes()).expect("write failed");
            file.write_all("Qualification: B.sc. \n".as_bytes()).expect("write failed");
            file.write_all("Job Service: \n﻿Tax planning Tax function operations Tax policy and controversy \nGlobal trade \n﻿Tax accounting Tax compliance \n﻿Transaction tax".as_bytes()).expect("write failed");
            

            println!("\nData has been written to file");
        } 
        if file_create==2 {
            let announce = "Department: People and workforce\n";
            let dept ="Department of People and wok force";
            let mut file = std::fs::File::create("Gbenga_daniels.txt").expect("create failed");
            file.write_all("Name : Gbenga daniels \n".as_bytes()).expect("write failed");
            file.write_all(announce.as_bytes()).expect("write failed");
            file.write_all("Qualification: HND. \n".as_bytes()).expect("write failed");
            file.write_all("Job Service: \n﻿Change management and
\n﻿experience HR transformation \n﻿Learning and development consulting Recognition and reward advisory \n﻿Workforce analytics People and workforce".as_bytes()).expect("write failed");
            

            println!("\nData has been written to file");
        }
     
        }
        else if number == 9 {
            println!("A file would be created for Maria Akinsola if you \nInput '1' or '2' for Ehis Ero");
    let mut file_create = String::new();
    std::io::stdin().read_line(&mut file_create).expect("Invalid Input, try again");
       let file_create:i32= file_create.trim().parse().expect("invalid input");
        if file_create==1 {
            let announce = "Department: Trasaction and corporate finance \n";
            let dept ="Department of Trasaction and corporate finance";
            let mut file = std::fs::File::create("Maria_Akinsola.txt").expect("create failed");
            file.write_all("Name :  Maria Akinsola\n".as_bytes()).expect("write failed");
            file.write_all(announce.as_bytes()).expect("write failed");
            file.write_all("Qualification: M.sc. \n".as_bytes()).expect("write failed");
            file.write_all("Job Service: \n﻿Corporate finance \n﻿Sustainability and ESG Services M&A advisor \n﻿M&A integration M&A technology and tools \n﻿M&A advanced analytics".as_bytes()).expect("write failed");
            

            println!("\nData has been written to file");
        }
        if file_create==2 {
             let announce = "Department: Strategy\n";
            let dept ="Department of strategy";
            let mut file = std::fs::File::create("Ehis_Ero.txt").expect("create failed");
            file.write_all("Name : Ehis Ero\n".as_bytes()).expect("write failed");
            file.write_all(announce.as_bytes()).expect("write failed");
            file.write_all("Qualification: M.sc. \n".as_bytes()).expect("write failed");
            file.write_all("Job Service: \n﻿ Strategy consulting Corporate and growth strategy \n﻿ Transaction strategy and execution Restructuring and\n﻿  turnaround strategy Industry strategy Digital business building Commercial strategy".as_bytes()).expect("write failed");
            

            println!("\nData has been written to file");
        }
     
        }
    }
  
fn main() {
    println!("Welcome! This program help to sort staff to their positions");
 code_7();

}