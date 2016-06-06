//! Author : Thibault Barbie
//!
//! A simple evolutionary algorithm written in Rust.

extern crate rand;

use std::collections::{HashMap};
use rand::{Rng};

fn main() {
    let target: String = String::from("METHINKS IT IS LIKE A WEASEL");
    let mut parent: String = "".to_string();
    let nb_copy = 400;
    let mutation_rate : f64 = 0.05;
    let mut counter=0;

    generate_first_sentence(&mut parent);

    println!("{}", target);
    println!("{}", parent);
    
    while fitness(&target, &parent) != 0 {
        let mut sentences: HashMap<u32, String> = HashMap::new();
        let mut f_min: u32 = 30;

        counter+=1;

        for _ in 0..nb_copy {
            let sentence = mutate(&mut parent, mutation_rate);
            let f = fitness(&target, &sentence);

            sentences.insert(f,sentence);
            if f<f_min {
                f_min = f;
            }
        }
        
        if fitness(&target, &parent) > f_min {
            match sentences.get(&f_min) {
                Some(s) => {
                    parent = s.clone();
                    println!("{} : {}", parent, counter);
                },
                None => panic!("Error, fitness minimum but no sentence."),
            }
        }
    }
}

/// Computes the fitness of a sentence against a target string.
fn fitness(target: &String, sentence: &String) -> u32 {
    let mut fitness = 0;

    for (c1, c2) in target.chars().zip(sentence.chars()) {
        if c1 != c2 {
            fitness += 1;
        }
    }

    fitness
}

/// Mutation algorithm.
///
/// It mutates each character of a string, according to a `mutation_rate`.
/// Please note that for full usefullness, `mutation_rate` should be between
/// 0 and 1.
fn mutate(sentence: &mut String, mutation_rate: f64) -> String {
    let mut rng = rand::thread_rng();
    let mut mutation: String = "".to_string();

    for c in sentence.chars() {
        if mutation_rate > rng.gen_range(0f64, 1f64) {
            mutation.push(c);
        } else {
            mutation.push(random_char());
        }
    }

    mutation
}

/// Generates a random sentence of length 28 from completly random chars.
fn generate_first_sentence(parent: &mut String) {
    for _ in 0..28 {
        parent.push(random_char());
    }
}

/// Generates a random char (between 'A' and '\\').
fn random_char() -> char {
    match rand::thread_rng().gen_range('A' as u8, '\\' as u8) as char {
        '['     => ' ',
        c @ _   => c
    }
}
