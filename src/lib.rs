use std::collections::HashMap;
use std::fs;
use rand::Rng;

pub struct MarkovChain<'a>
{
	pub content: String,
	pub wordmap: HashMap<(&'a str, &'a str), Vec<&'a str>>,
}

impl<'a> MarkovChain<'a>
{
	pub fn new(filename: &str) -> Self
	{
		return MarkovChain {
			
			content: fs::read_to_string(filename)
				.expect("error reading file")
				.to_lowercase(),
			wordmap: HashMap::new(),
		};
	}
	
	pub fn build<'b>(content: &'b str, wordmap: &mut HashMap<(&'b str, &'b str), Vec<&'b str>>)
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
	
	pub fn generate_paragraph(&self, context: &str) -> String
	{
		let mut paragraph = Self::generate_sentence(self, context);
		
		for _ in 0..rand::thread_rng().gen_range(4..11) {
			
			let words: Vec<&str> = paragraph.split_whitespace().rev().collect();
			let context = words[1].to_string() + " " + words[0];
			paragraph = paragraph + " " + &Self::generate_sentence(self, &context);
		}
		
		return paragraph;
	}
	
	pub fn generate_sentence(&self, context: &str) -> String
	{
		let mut sentence = String::new();
		
		if context == "" {
			
			let keys: Vec<&(&str, &str)> = self.wordmap.keys().collect();
			let key = *(keys[rand::thread_rng().gen_range(0..keys.len())]);
			sentence = sentence + key.0 + " " + key.1;
		}
		else {
			
			let words: Vec<&str> = context.split_whitespace().rev().collect();
			if words.len() < 2 {
				
				let keys: Vec<&(&str, &str)> = self.wordmap.keys().collect();
				let key = (words[0], (*(keys[rand::thread_rng().gen_range(0..keys.len())])).1);
				let next = match self.wordmap.get(&key) {
					
					Some(val) => val[rand::thread_rng().gen_range(0..val.len())],
					None => "end.",
				};
				sentence = sentence + key.1 + " " + next;
			}
			else {
				
				let key = (words[1], words[0]);
				let next = match self.wordmap.get(&key) {
					
					Some(val) => val[rand::thread_rng().gen_range(0..val.len())],
					None => "the",
				};
				let key = (key.1, next);
				let next = match self.wordmap.get(&key) {
					
					Some(val) => val[rand::thread_rng().gen_range(0..val.len())],
					None => "end.",
				};
				sentence = sentence + key.1 + " " + next;
			}
		}
		
		while sentence.chars().last().unwrap() != '.' {
			
			let words: Vec<&str> = sentence.split_whitespace().rev().collect();
			let key = (words[1], words[0]);
			let next = match self.wordmap.get(&key) {
				
				Some(val) => val[rand::thread_rng().gen_range(0..val.len())],
				None => "the end.",
			};
			sentence = sentence + " " + next;
		}
		
		return sentence;
	}
}
