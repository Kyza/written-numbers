use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short, long, default_value_t = {"random".to_string()})]
	number: String,

	#[arg(short, long, default_value_t = {"en".to_string()})]
	language: String,

	#[arg(short = 'a', long, default_value_t = false)]
	hundred_and: bool,

	#[arg(short, long, default_value_t = true)]
	commas: bool,

	#[arg(short, long, default_value_t = false)]
	ordinal: bool,

	#[arg(short = 'm', long, default_value_t = 'a')]
	match_case: char,

	#[arg(short, long, default_value_t = false)]
	memory: bool,
}

use maplit::hashmap;
use written_numbers::*;

use peak_alloc::PeakAlloc;

use rand::prelude::*;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn str_cap(s: &str) -> String {
	format!("{}{}", (&s[..1].to_string()).to_uppercase(), &s[1..])
}

fn main() {
	let args = Args::parse();

	let mut number = args.number;

	if number == "random" {
		number = format!("{}", rand::thread_rng().gen_range(1..999));
		let mut zero_count = rand::thread_rng().gen_range(2..999997);
		while zero_count > 0 {
			number += "000";
			zero_count -= 1;
		}
	}

	let result = to_words(
		&number,
		&ToWordsOptions { language: "en" },
		&hashmap! {
			"hundred_and" => if args.hundred_and {"true"} else {"false"},
			"commas" => if args.commas {"true"} else {"false"}
		},
		&mut hashmap! {},
	);

	match result {
		Ok(result) => {
			if !args.ordinal {
				if args.match_case.is_uppercase() {
					println!("{}", str_cap(&result));
				} else {
					println!("{result}");
				}
			} else {
				let ordinal = to_ordinal(
					&result,
					&ToOrdinalOptions { language: "en" },
					&mut hashmap! {},
				);
				match ordinal {
					Ok(result) => {
						if args.match_case.is_uppercase() {
							println!("{}", str_cap(&result));
						} else {
							println!("{result}");
						}
					}
					Err(err) => match err {
						ToOrdinalError::UnimplementedLanguage => {
							println!(
								"The language \"{}\" hasn't been implemented yet.",
								args.language
							);
						}
					},
				}
			}
		}
		Err(err) => match err {
			ToWordsError::NotANumber => {
				println!("\"{}\" is not a valid number.", number);
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
