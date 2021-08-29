use std::env;
use markov_chain::MarkovChain;

fn main()
{
	let args: Vec<String> = env::args()
		.skip(1)
		.collect();
	
	let context: String;
	let wordmap = match args.len() {
		
		0 => {
			context = String::new();
			MarkovChain::new("input.txt")
		},
		1 => {
			context = String::new();
			MarkovChain::new(&args[0])
		},
		_ => {
			context = args[1..].join(" ");
			MarkovChain::new(&args[0])
		},
	};
	
	println!("{} {}",
		context,
		wordmap.generate_paragraph(&context),
	);
	
	return;
}
