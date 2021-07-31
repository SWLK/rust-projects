// pig_latin

// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and "ay" is added,
// so "first" becomes "irst-fay".
// Words that start with a vowel have "hay" added to the end instead ("apple-hay").
// ===================================================================================

pub fn main() {
	let sentence = String::from("Mama ain't raise no bitch");

	// Create a String to hold the converted words
	let mut pig_latin = String::new();

	// Split the String into words
	for word in sentence.split_whitespace() {
		if first_letter_consonant(&word) {
			pig_latin = format!("{} {}", pig_latin, consonant_conversion(&word));
		} else {
			pig_latin = format!("{} {}", pig_latin, vowel_conversion(&word));
		}
	}

	println!("{}", pig_latin);
}

fn first_letter_consonant(word: &str) -> bool {
	// word.chars() returns an iterable that returns Option<char> with .next()
	match word.chars().next() {
		Some(letter) => {
			// if letter is a vowel then return false, else return true
			match letter {
				'a' | 'e' | 'i' | 'o' | 'u' => return false,
				_ => return true,
			}
		},
		None => return false,
	};
}

fn consonant_conversion(word: &str) -> String {
	// new String with a char with placeholder for the first letter
	let mut pig_latin = String::new();
	let mut first_letter: char = '*';

	// char_indices() returns Option<index, char> in iterable
	for (index, letter) in word.char_indices() {
		// if first letter store in first_letter, else push to String
		if index == 0 {
			first_letter = letter;
		} else {
			pig_latin.push(letter);
		}
	}

	// format to String then string slice the entire String, then push to String
	pig_latin.push_str(&format!("-{}ay", first_letter)[..]);

	pig_latin
}

fn vowel_conversion(word: &str) -> String {
	let mut pig_latin = String::new();
	
	pig_latin.push_str(word);
	pig_latin.push_str("-hay");

	pig_latin
}