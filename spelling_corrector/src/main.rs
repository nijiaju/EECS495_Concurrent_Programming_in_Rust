use std::env;
use std::fs::File;
use std::io::{stdin, Read, BufReader, BufRead};

type CountTable = std::collections::HashMap<String, usize>;
type WordSet    = std::collections::HashSet<String>;

fn format_words(line: &String) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();

    for c in line.chars() {
        if c.is_alphabetic() {
            word.push(c.to_lowercase().next().unwrap());
        } else {
            if word.len() != 0 {
                words.push(word.clone());
//                println!("{}", word);
                word.clear();
            }
        }
    }
    
    if word.len() != 0 {
        words.push(word.clone());
//        println!("{}", word);
    }
    return words;
}

fn training_words<R:Read>(reader: R) -> CountTable {
    let mut count_table = CountTable::new();
    let mut lines = BufReader::new(reader).lines();

    while let Some(Ok(line)) = lines.next() {
        for word in format_words(&line) {
            *count_table.entry(word).or_insert(0) += 1;
        }
    }
                
    return count_table;
}

fn enumerate_possibilities(word: &String) -> WordSet {
    let mut possibilities = WordSet::new();

    //delete
    for i in 0..(word.len()) {
        let mut word_del = word.clone();
        word_del.remove(i);
//        println!("{}", word_del); 
        possibilities.insert(word_del);
    }

    //insert
    for i in 0..(word.len() + 1) {
        for j in 97u8..123u8 {
            let mut word_ins = word.clone();
            word_ins.insert(i, j as char);
//            println!("{}", word_ins);
            possibilities.insert(word_ins);
        }
    }

    //replace
    for i in 0..(word.len()) {
        for j in 97u8..123u8 {
            let mut word_rep = word.clone();
            let c = word_rep.remove(i);
            if c != j as char {
                word_rep.insert(i, j as char);
            } else {
                continue;
            }
//            println!("{}", word_rep);
            possibilities.insert(word_rep);
        }
    }

    //transpose
    for i in 0..(word.len() - 1) {
        let mut word_tra = word.clone();
        let c1 = word_tra.remove(i);
        let c2 = word_tra.remove(i);
        word_tra.insert(i, c1);
        word_tra.insert(i, c2);
        if word_tra == *word {
            continue;
        }
//        println!("{}", word_tra);
        possibilities.insert(word_tra);
    }

    return possibilities;
}

fn spell_correction(word1: &String, count_table: &CountTable) -> String {

    //already correct
    if count_table.contains_key(word1) {
        return word1.clone();
    }
    //find correction in edit distance 1
    let edit1 = enumerate_possibilities(word1);
    let mut res = (String::new(), 0usize);
    for each_word in edit1 {
        if count_table.contains_key(&each_word) {
            let &count = count_table.get(&each_word).unwrap();
            if count > res.1 {
                res = (each_word, count);
            }
        }
    }
    if res.1 > 0 {
        return res.0;
    }

    for word2 in enumerate_possibilities(&word1) {
        let edit2 = enumerate_possibilities(&word2);
        for each_word in edit2 {
            if count_table.contains_key(&each_word) {
                let &count = count_table.get(&each_word).unwrap();
                if count > res.1 {
                    res = (each_word, count);
                }
            }
        }
    }
    if res.1 > 0 {
        return res.0;
    } else {
        return String::from("--");
    }
}

fn main() {
    //argument sanity check
    if env::args().count() != 2usize {
        println!("\nuseage: cargo run path/corpus.txt < input.txt\n");
        return;
    }

    //open the corpus file
    let f = match File::open(env::args().nth(1).unwrap()) {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    //training
    let count_table = training_words(f); 

    //possible corrections
    let mut lines = BufReader::new(stdin()).lines();

    while let Some(Ok(line)) = lines.next() {
        for word in format_words(&line) {
            println!("{}, {}", word, spell_correction(&word, &count_table));
        }
    }
}
