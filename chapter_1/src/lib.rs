use std::collections::HashMap;

//determines if a string has all unique characters
fn is_unique(s: String) -> bool {
    let mut chars:HashMap<char,bool> = HashMap::new();
    for c in s.chars() {
        if chars.contains_key(&c) {
            return false;
        } else {
            chars.insert(c, true);
        };
    };
    return true;
}


// //determines if two strings are permutations of each other
// fn checkPermutation(s1: String, s2: String) -> bool {

// }

// // Replaces all spaces with '%20'
// fn URLify(s: &String) {

// }

// //determine if a string is permutation of a palindrome
// fn palindromePermutation(s: String) -> bool { 

// }

// //Given the following operations: insert, remove, or replace
// //determine if the two strings are one or less operating away from each other
// fn OneAway(s1: String, s2: String) -> bool {

// }

// // perform basic string compression. aaabbc -> a3b2c1
fn string_compression(s: String) -> String {
    let mut comp = String::from("");
    let mut current= '\0';
    let mut count = 1;

    for (pos, c) in s.chars().enumerate() {
        if pos == 0 { //prime the pump
            current = c;
            continue;
        }
        print!("comp: {}; Current: {}; Count: {}\n",  comp,current,count);
        if  current != c {
            print!("Here 1\n");
            if count <= 1 {
                print!("Here 2\n");
                comp = format!("{}{}", comp,current);
            } else  {
                print!("Here 3\n");
                comp = format!("{}{}{}", comp,current,count);
            }
            count =1;
            current = c;
        } else {
            print!("Here 4\n");
            count +=1;
        }
    }
    print!("comp: {}; Current: {}; Count: {}\n",  comp,current,count);
    if count != 1{
        comp = format!("{}{}{}", comp,current,count);
    } else {
        comp = format!("{}{}", comp,current);
    }
    
    if comp.len() >= s.len() {
        return s
    } else {
        return comp
    }
    
}

// //rotate a matrix 90 degrees
// fn rotateMatrix(m: &Vec<Vec<T>>) {

// }

// //if a position is 0, set entire column and row to zero
// fn zeroMatrix(m: &Vec<Vec<T>>) {

// }

// //check if one of the strings is a rotation of the other. waterbottle -> tlewaterbot
// //Only one call to a substring function
fn string_rotation(s1: String, s2: String) -> bool {
    println!("Here are the params: {} and {}", s1,s2);
    if s1.len() != s2.len() {
        return false
    }
    let doubles = format!("{}{}", s1, s1);
    return doubles.contains(&s2);
}

#[cfg(test)]
mod tests;