//! A simple evolutionary algorithm written in Rust.

extern crate rand;

use rand::Rng;
use std::collections::HashMap;

fn main() {
    let target = "METHINKS IT IS LIKE A WEASEL".to_string();
    println!("{}", target);

    let mut parent = String::new();
    generate_first_sentence(&mut parent);

    println!("{}", parent);

    let nb_copy = 400;
    let mutation_rate = 0.05;
    let mut counter = 0;

    while fitness(&target, &parent) != 0 {
        counter += 1;

        let mut sentences = HashMap::new();
        let mut f_min = 30;

        for _ in 0..nb_copy {
            let sentence = mutate(&mut parent, mutation_rate);
            let f = fitness(&target, &sentence);

            sentences.insert(f, sentence);

            if f < f_min {
                f_min = f
            }
        }

        if fitness(&target, &parent) > f_min {
            match sentences.get(&f_min) {
                Some(s) => {
                    parent = s.clone();
                    println!("{} : {}", parent, counter);
                }
                None => println!("Error, fitness minimum but no sentence."),
            }
        }
    }
}


fn fitness(target: &str, sentence: &str) -> u32 {
    target.chars()
          .zip(sentence.chars())
          .fold(0, |acc, (c1, c2)| {
              if c1 != c2 {
                  acc + 1
              } else {
                  acc
              }
          })
}

fn mutate(sentence: &str, mutation_rate: f64) -> String {
    let mut rng = rand::thread_rng();
    let mut mutation = String::new();
    
    for c in sentence.chars() {
        match mutation_rate > rng.gen_range(0.0, 1.0) {
            false => mutation.push(c),               // no mutation
            true => mutation.push(random_char()),   // mutation
        }
    }

    mutation
}


fn generate_first_sentence(parent: &mut String) {
    for _ in 0..28 {
        parent.push(random_char());
    }
}

fn random_char() -> char {
    match rand::thread_rng().gen_range('A' as u8, '\\' as u8) as char {
        '[' => ' ',
        c => c,
    }
}
