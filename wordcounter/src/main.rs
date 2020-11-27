use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;


fn main() {
    let file = File::open("A-Tale-of-Two-Cities.txt").expect("can't read file");
    let reader = BufReader::new(file);

    let mut counterMap: HashMap<String, usize> = HashMap::new();
    for line in reader.lines(){
        if line.is_err(){
            continue
        }
        let line = line.unwrap();
        for word in line.trim().split_ascii_whitespace(){
            let val = counterMap.get(word);
            match val {
                Some(counter) =>  counterMap.insert(word.to_string(), counter+1),
                _ =>  counterMap.insert(word.to_string(),1)
            };
        }
    }
    let mut result = vec![];
    for (k, v) in counterMap.into_iter(){
            result.push((k,v));
    }

    result.sort_by_key(|val1|{
        (*val1).1
    });
    for(k, v)in result.iter().rev().take(100){
        println!("{} = {}", k, v)
    }
}
