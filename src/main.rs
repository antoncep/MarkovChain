use std::collections::HashMap;
use std::env;
use std::fs;
use rand::Rng;

struct MarkovChain<'a>
{
	content: String,
	context: String,
	wordmap: HashMap<(&'a str, &'a str), Vec<&'a str>>,
}

impl<'a> MarkovChain<'a>
{
	fn new(filename: &str, context: &str) -> Self
	{
		return MarkovChain {
			
			content: fs::read_to_string(filename).expect("error reading file"),
			context: String::from(context),
			wordmap: HashMap::new(),
		};
	}
	
	fn build<'b>(content: &'b str, wordmap: &mut HashMap<(&'b str, &'b str), Vec<&'b str>>)
	{
		let mut ngram = ("", "");
		let mut after = "";
		
		for word in content.split_whitespace() {
			
			ngram = (ngram.1, after);
			after = word;
			
			if ngram.0 == "" { continue; }
			
			if let Some(val) = wordmap.get_mut(&ngram) {
				
				val.push(after);
			}
			else {
				
				wordmap.insert(ngram, vec![after]);
			}
		}
		
		return;
	}
}

fn main()
{
	let args: Vec<String> = env::args().collect();
	
	let mut markov_chain = match args.len() {
		
		1 => MarkovChain::new("input.txt", ""),
		_ => MarkovChain::new(&args[1], &args[2..].join(" ")),
	};
	MarkovChain::build(&markov_chain.content, &mut markov_chain.wordmap);
	
	println!("{} {}",
		markov_chain.context,
		generate_paragraph(&markov_chain.context, &markov_chain.wordmap)
	);
	
	return;
}

fn generate_paragraph(context: &str, wordmap: &HashMap<(&str, &str), Vec<&str>>) -> String
{
	let mut paragraph = generate_sentence(context, &wordmap);
	
	for _ in 0..rand::thread_rng().gen_range(4..11) {
		
		let words: Vec<&str> = paragraph.split_whitespace().rev().collect();
		let context = words[1].to_string() + " " + words[0];
		paragraph = paragraph + " " + &generate_sentence(&context, &wordmap);
	}
	
	return paragraph;
}

fn generate_sentence(context: &str, wordmap: &HashMap<(&str, &str), Vec<&str>>) -> String
{
	let mut sentence = String::new();
	
	if context == "" {
		
		let keys: Vec<&(&str, &str)> = wordmap.keys().collect();
		let key = *(keys[rand::thread_rng().gen_range(0..keys.len())]);
		sentence = sentence + key.0 + " " + key.1;
	}
	else {
		
		let words: Vec<&str> = context.split_whitespace().rev().collect();
		if words.len() < 2 {
			
			let keys: Vec<&(&str, &str)> = wordmap.keys().collect();
			let key = (words[0], (*(keys[rand::thread_rng().gen_range(0..keys.len())])).1);
			let next = match wordmap.get(&key) {
				
				Some(val) => val[rand::thread_rng().gen_range(0..val.len())],
				None => "empty",
			};
			sentence = sentence + key.1 + " " + next;
		}
		else {
			
			let key = (words[1], words[0]);
			let next = match wordmap.get(&key) {
				
				Some(val) => val[rand::thread_rng().gen_range(0..val.len())],
				None => "empty",
			};
			let key = (key.1, next);
			let next = match wordmap.get(&key) {
				
				Some(val) => val[rand::thread_rng().gen_range(0..val.len())],
				None => "empty",
			};
			sentence = sentence + key.1 + " " + next;
		}
	}
	
	while sentence.chars().last().unwrap() != '.' {
		
		let words: Vec<&str> = sentence.split_whitespace().rev().collect();
		let key = (words[1], words[0]);
		let next = match wordmap.get(&key) {
			
			Some(val) => val[rand::thread_rng().gen_range(0..val.len())],
			None => "the end.",
		};
		sentence = sentence + " " + next;
	}
	
	return sentence;
}
