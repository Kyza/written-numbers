use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short, long)]
	number: String,

	#[arg(short, long, default_value_t = {"en".to_string()})]
	language: String,

	#[arg(short = 'a', long, default_value_t = false)]
	hundred_and: bool,

	#[arg(short, long, default_value_t = true)]
	commas: bool,

	#[arg(short, long, default_value_t = false)]
	memory: bool,
}

use maplit::hashmap;
use written_numbers::*;

use peak_alloc::PeakAlloc;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn main() {
	let args = Args::parse();

	let result = WrittenNumbers::new().to_words(
		&args.number,
		&ToWordsOptions { language: "en" },
		&hashmap! {
			"hundred_and" => if args.hundred_and {"true"} else {"false"},
			"commas" => if args.commas {"true"} else {"false"}
		},
	);

	match result {
		Ok(result) => {
			println!("{result}");
		}
		Err(err) => match err {
			ToWordsError::NotANumber => {
				println!("\"{}\" is not a valid number.", args.number);
			}
			ToWordsError::UnimplementedLanguage => {
				println!(
					"The language \"{}\" hasn't been implemented yet.",
					args.language
				);
			}
		},
	}

	if args.memory {
		let peak_mem = PEAK_ALLOC.peak_usage_as_mb();
		println!("Peak memory: {} MB", peak_mem);
	}
}
