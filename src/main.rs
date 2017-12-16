

use std::env;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::collections::hash_map::Entry;
use std::io::{BufReader, BufRead};
use std::fs::File;

//TODO add min word, max word, option to select min chars for word,
//total number of words,average word size,number of words per size, sorted list of top 10 words

fn main() {

	let args: Vec<String> = env::args().collect();
	match args.len() {
		1 => panic!("Insufficient arguments, try \"textstat <file-path>\""),
		2 => (),
		_ => panic!("Wrong number of arguments, try \"textstat <file-path>\""),
	}

	let f = File::open(&args[1]);
	let f = match f {
	    Ok(file) => file,
	    Err(e) => panic!("There was a problem opening file at \"{}\"{:?}",args[1],e),
	};

	let mut total_words: usize = 0; // total number of words
	let mut sum_length: usize = 0; // sum of lengths of individual words
	let mut word_length: usize = 0; // length of the current words
	// array holding frequencies of word lengths.i.e. index 0 holds number of words by size 1 etc.
	let mut word_frequency: [isize; 10] = [0;10]; 
	// holds words with their content as key, frequency as value
	let mut word_map: HashMap<String, isize> = HashMap::new();


	let reader = BufReader::new(f);
	for line in reader.lines() {
		for mut curr in line.as_ref().unwrap().split_whitespace(){
			if curr.len()==1 && !curr.chars().nth(0).unwrap().is_alphabetic() {
			    continue;
			}
			if !curr.chars().nth(0).unwrap().is_alphabetic() {
				curr = &curr[1..curr.len()];
			}
			if !curr.chars().nth(curr.len()-1).unwrap().is_alphabetic(){
				curr = &curr[0..curr.len()-1];
			}
			word_length = curr.len();
			if word_length<=10 { word_frequency[word_length-1] += 1;}
			total_words += 1;
			sum_length += word_length;
			match word_map.entry(curr.to_string().to_lowercase()) {
			    Entry::Occupied(entry) => {*entry.into_mut()+=1;},
			    Entry::Vacant(entry) => {entry.insert(1);},
			}
		}
	}	
	

	let mut v = Vec::from_iter(word_map);
	v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

	println!("Total number of words:{:?}", total_words );
	println!("Average word length: {:?} (rounded)", (sum_length/total_words) );
	for i in 0..10{
		println!("Text contains {:?} words with {} char(s)",word_frequency[i],(i+1) );
	}
	println!("Top common words are: ");
	let mut c = 0;
	for &(ref word, ref v) in v.iter(){
		c += 1;
		if c > 10 {break;}
		println!("{:?} is found {} times", &word,&v);
	}
}
