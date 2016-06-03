/* Author : Thibault Barbie

A simple evolutionary algorithm written in Rust.

*/

extern crate rand;

use rand::Rng;
use std::collections::HashMap;

fn main() {
    let target : String ="METHINKS IT IS LIKE A WEASEL".to_string();
    println!("{}",target);

    let mut parent : String="".to_string();
    generate_first_sentence(&mut parent);
    println!("{}",parent);

    let nb_copy = 400;
    let mutation_rate : f64 = 0.05;
    let mut counter=0;
    
    while fitness(&target, &parent)!=0 {
        counter+=1;
        let mut sentences=HashMap::new();
        let mut f_min : u32 = 30;
        for i in 0..nb_copy {
            let sentence=mutate(&mut parent, mutation_rate);
            let f=fitness(&target, &sentence);
            sentences.insert(f,sentence);

            if f<f_min {f_min=f}
        }
        
        if fitness(&target, &parent)>f_min {
            match sentences.get(&f_min) {
                Some(s) => {
                    parent=s.clone();
                    println!("{} : {}",parent,counter);},
                None => println!("Error, fitness minimum but no sentence."),
            }
        }
    }
}


fn fitness(target : &String, sentence : &String) -> u32 {
    let mut fitness = 0;
    for (c1, c2) in target.chars().zip(sentence.chars())
    {
        if c1!=c2 { fitness+=1 };
    }
    fitness
}

fn mutate(sentence : &mut String, mutation_rate : f64) -> String {
    let mut rng=rand::thread_rng();
    
    let mut mutation : String ="".to_string();
    for c in sentence.chars() {
        match mutation_rate>rng.gen_range(0f64,1f64) {
            false => mutation.push(c),               // no mutation
            true  => mutation.push(random_char()),   // mutation
        }
    }
    mutation
}


fn generate_first_sentence (parent : &mut String) {
    for i in 0..28 {
        parent.push(random_char());
    }
}

fn random_char() -> char {
    let mut c : char=' ';

    let mut rng=rand::thread_rng();
    let x=rng.gen_range(0,27);
    
    match x {
        0 => c = 'A',
        1 => c = 'B',
        2 => c = 'C',
        3 => c = 'D',
        4 => c = 'E',
        5 => c = 'F',
        6 => c = 'G',
        7 => c = 'H',
        8 => c = 'I',
        9 => c = 'J',
        10 => c = 'K',
        11 => c = 'L',
        12 => c = 'M',
        13 => c = 'N',
        14 => c = 'O',
        15 => c = 'P',
        16 => c = 'Q',
        17 => c = 'R',
        18 => c = 'S',
        19 => c = 'T',
        20 => c = 'U',
        21 => c = 'V',
        22 => c = 'W',
        23 => c = 'X',
        24 => c = 'Y',
        25 => c = 'Z',
        26 => c = ' ',
        _ => {},
    }
    c
}
