use std::collections::HashMap;
use std::fs;
use rand::Rng;

pub struct MarkovChain
{
	inner: HashMap<String, Vec<String>>,
}

impl MarkovChain
{
	pub fn new(filename: &str) -> Self
	{
		let content = fs::read_to_string(filename)
			.expect("error reading file")
			.to_lowercase()
			.replace("*", "")
			.replace("_", "")
			.replace("-", "")
			.replace("—", " ")
			.replace("--", " ");
		
		let mut wordmap: HashMap<String, Vec<String>> = HashMap::new();
		
		let mut ngram = ("", "");
		let mut after = "";
		
		for word in content.split_whitespace() {
			
			ngram = (ngram.1, after);
			after = word;
			
			if ngram.0 == "" { continue; }
			
			let key = [ngram.0, ngram.1].join(" ");
			
			if let Some(val) = wordmap.get_mut(&key) {
				
				val.push(after.to_owned());
			}
			else {
				
				wordmap.insert(key, vec![after.to_owned()]);
			}
		}
		
		return MarkovChain {
			
			inner: wordmap,
		};
	}
	
	pub fn generate_paragraph(&self, context: &str) -> String
	{
		let mut paragraph = Self::generate_sentence(self, context);
		
		for _ in 0..rand::thread_rng().gen_range(4..11) {
			
			let words: Vec<&str> = paragraph.split_whitespace().rev().collect();
			let context = [words[1], words[0]].join(" ");
			
			paragraph = paragraph + " " + &Self::generate_sentence(self, &context);
		}
		
		return paragraph;
	}
	
	pub fn generate_sentence(&self, context: &str) -> String
	{
		let mut context: Vec<&str> = context.split_whitespace().rev().collect();
		
		let keys: Vec<&String> = self.inner.keys().collect();
		let key = keys[rand::thread_rng().gen_range(0..keys.len())];
		
		let mut key = key.split_whitespace().rev();
		
		while context.len() < 2 {
			
			context.push(key.next().unwrap());
		}
		
		let mut sentence = [context[1], context[0]].join(" ");
		
		for _ in 0..=1 {
			
			let words: Vec<&str> = sentence.split_whitespace().rev().collect();
			let key = [words[1], words[0]].join(" ");
			
			let next: &str = match self.inner.get(&key) {
				
				Some(val) => &val[rand::thread_rng().gen_range(0..val.len())],
				None => ".",
			};
			sentence = sentence + " " + next;
		}
		
		while sentence.chars().last().unwrap() != '.' {
			
			let words: Vec<&str> = sentence.split_whitespace().rev().collect();
			let key = [words[1], words[0]].join(" ");
			
			let next: &str = match self.inner.get(&key) {
				
				Some(val) => &val[rand::thread_rng().gen_range(0..val.len())],
				None => ".",
			};
			sentence = sentence + " " + next;
		}
		
		let sentence: Vec<&str> = sentence.split_whitespace().skip(2).collect();
		return sentence.join(" ");
	}
}
