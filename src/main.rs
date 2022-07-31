use rust_embed::RustEmbed;
use rand::seq::IteratorRandom;
use serde::Deserialize;
use colored::*;

mod center;
use center::Center;

#[derive(RustEmbed)]
#[folder = "nihongo-benkyou/kotowaza"]
#[include = "*.json"]
struct KotowazaDir;

#[derive(Deserialize)]
#[allow(unused)]
struct Kotowaza {
	en: String,
	ja: String,
	kana: String,
	week: String,
}


fn main() {
	let mut rng = rand::thread_rng();

	let name = KotowazaDir::iter()
		.choose(&mut rng)
		.unwrap();

	let file = KotowazaDir::get(&name)
		.unwrap();

	let data = file.data
		.as_ref();

	let kotowaza: Kotowaza = serde_json::from_slice(data)
		.unwrap();

	let (term_width, _) = term_size::dimensions()
		.unwrap();

	print!("\n{}\n{}\n\n{}\n\n",
		kotowaza.ja
			.center(term_width)
			.bold()
			.yellow(),
		kotowaza.kana
			.center(term_width)
			.dimmed()
			.yellow(),
		kotowaza.en
			.center(term_width)
			.italic()
			.cyan());
}
