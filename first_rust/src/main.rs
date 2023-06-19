#![allow(unused)]


use std::io::{self, BufReader, ErrorKind};
use rand::Rng;
use std::io::{Write, BufRead};
use std::fs::File;
use std::cmp::Ordering;




fn main() {


    let mut st1 = String::new();
    st1.push('A');
    st1.push_str("str");
    for word in st1.split_whitespace(){
        println!("{}", word)
    };


    let st2 = st1.replace("A", "Another");
    println!("{}", st2);






    let my_tuple: (u8, String, f64) = (57,"Michal".to_string(), 50_0000.00);
    println!("Tuple {} ", my_tuple.1);

    let(v1,v2,v3) = my_tuple;
    println!("Tupleeee {}", v1);



    let myar = [1,2,3,4];
    println!("First: {}", myar[0]);
    println!("Length: {} ", myar.len());

    let newar = [1,2,3,4,5,6,7,8,9];
    let mut lindex = 0;
    loop {
        if newar[lindex] % 2 == 0{
            lindex +=1;
            continue;
        }
        if newar[lindex] == 9{
            break;
        }
        println!("Od val {}", newar[lindex]);
        lindex +=1;
    }

    let oldar = [1,2,3,4,5,6,7,8,9];
    let mut newindex = 0;
    while newindex < oldar.len(){
        println!("Test: {} ", oldar[newindex]);
        newindex +=1;
    }

    newindex = 0;
    for val in oldar.iter(){
        println!("La: {}", val);

    }

    


    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age){
        Ordering::Less =>  println!("Cant Vote"),
        Ordering::Greater =>  println!("Can Vote"),
        Ordering::Equal =>  println!("Just can vote"),
    };



    let age2 = 8;
    match age2{
        1..=18 =>  println!("Important birthday"),
        21 | 50 =>  println!("Important birthday"),
        65..=i32::MAX =>  println!("Important NOT birthday"),
        _ =>  println!("Important NOT birthday"),
    };




    let mut age = 47;
    let can_vote: bool = if age >= 18{
        true
    } else{
        false
    };
    println!("you can yote {}", can_vote);




    let age: i32 = 65;
    if( age >= 1) && (age <= 18){
        println!("Important birthday")
    } else if (age ==21) || (age == 50) {
        println!("Important birthday")
        
    } else if age >=65 {
        println!("Important birthday")
    }
    else{
        println!("Important NOT birthday")
    }




    println!("Hello, world!");

    // println!("What is your name:");
    // let mut name = String::new();
    // let greeting = "Nice to meet you";
    // io::stdin().read_line(&mut name)
    //     .expect("Didnt recieve input");

    // println!("Hello {} {}", name.trim_end(), greeting);


    const ONE_MIL: u32 = 1000000;
    const PI: f64 = 3.1415;
    let age: &str = "56";
    let mut age: u32 = age.trim().parse()
        .expect("age wasnt asigned a number");
    age = age + 1;
    println!("Im {} {}", age, ONE_MIL);


    let is_true : bool = true;
    let my_grade = 'A';
    let num_1: f32 = 1.1111111111111;
    println!("f32 : {}", num_1 + 0.222222222222);

    let mut num_3: u32 = 5;
    let num_5: u32 = 4;
    println!("{}", num_3/num_5);
    num_3 += 1;



    let random_num = rand::thread_rng().gen_range(1..100);
    println!("{}",random_num);












}
