use std::collections::HashMap;

//determines if a string has all unique characters
fn is_unique(s: String) -> bool {
    let mut chars:HashMap<char,bool> = HashMap::new();
    for c in s.chars() {
        if let std::collections::hash_map::Entry::Vacant(e) = chars.entry(c) {
            e.insert(true);
        } else {
            return false;
        };
    };
    true
}


// //determines if two strings are permutations of each other
fn check_permutation(s1: String, s2: String) -> bool {
    let s1_clone = s1.clone();
    let s2_clone = s2.clone();
    for c in s1.chars() {
        if s1_clone.is_empty() || s2_clone.is_empty() {
            break
        }
        s1_clone.replace_first(c, "");
        s2_clone.replace_first(c,"")
    }
    s1_clone.is_empty() && s2_clone.is_empty()
}

// Replaces all spaces with '%20'
//This is a lot closer to what rust can do according to the original requirements in the problem
fn urlify(s: &mut String) {
    const SPACE_REPLACEMENT: &[u8] = b"%20";

    let mut buffer = std::mem::take(s).into_bytes();
    let old_len = buffer.len();

    // Get new size for string. This will be the size of the new string plus the size of the (SPACE_REPLACEMENT-1)*num_spaces
    let space_count = buffer.iter().filter(|&&byte| byte == b' ').count();
    let new_len = buffer.len() + (SPACE_REPLACEMENT.len() - 1) * space_count;
    buffer.resize(new_len, b'\0');

    let mut write_pos = new_len;

    //Do this backwards to account for the larger size required for the SPACE_REPLACEMENT
    for read_pos in (0..old_len).rev() {
        let byte = buffer[read_pos];

        if byte == b' ' {
            write_pos -= SPACE_REPLACEMENT.len();
            buffer[write_pos..write_pos + SPACE_REPLACEMENT.len()]
                .copy_from_slice(SPACE_REPLACEMENT);
        } else {
            write_pos -= 1;
            buffer[write_pos] = byte;
        }
    }

    *s = String::from_utf8(buffer).expect("invalid UTF-8 during URL-ification");
}

//Probably wouldnt consider this as a replace in place
fn urlify2(s: &mut String) {
    *s = s.replace(" ", "%20")
}

// //determine if a string is permutation of a palindrome
// fn palindromePermutation(s: String) -> bool { 

// }

// //Given the following operations: insert, remove, or replace
// //determine if the two strings are one or less operating away from each other
// fn one_away(s1: String, s2: String) -> bool {

// }

// // perform basic string compression. aaabbc -> a3b2c1
fn string_compression(s: String) -> String {
    let mut comp = String::from("");
    let mut current= '\0'; // null char
    let mut count = 1;

    for (pos, c) in s.chars().enumerate() {
        if pos == 0 { //prime the pump
            current = c;
            continue;
        }
        if  current != c {
            if count <= 1 {
                comp = format!("{}{}", comp,current);
            } else  {
                comp = format!("{}{}{}", comp,current,count);
            }
            count =1;
            current = c;
        } else {
            count +=1;
        }
    }
    println!("comp: {}; Current: {}; Count: {}",  comp,current,count);
    // We have to capture the last char in the string. 
    if count != 1{ // if the count of the last char is equal to 1 we need to print the count
        comp = format!("{}{}{}", comp,current,count);
    } else { // dont print a "1" count to preserve space
        comp = format!("{}{}", comp,current);
    }
    
    if comp.len() >= s.len() { // if the compression didnt compress just use original string
        s
    } else {
        comp
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
    doubles.contains(&s2)
}

#[cfg(test)]
mod tests;