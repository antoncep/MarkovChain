use std::env;
use markov_chain::MarkovChain;

fn main()
{
	let args: Vec<String> = env::args().collect();
	
	let context: String;
	let mut chain = match args.len() {
		
		1 => {
			context = String::from("");
			MarkovChain::new("input.txt")
		},
		_ => {
			context = args[2..].join(" ");
			MarkovChain::new(&args[1])
		},
	};
	MarkovChain::build(&chain.content, &mut chain.wordmap);
	
	println!("{} {}",
		context,
		chain.generate_paragraph(&context),
	);
	
	return;
}
