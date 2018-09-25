extern crate hex;
// CryptoPals Set 1 Challenge 4 - Detect single-character XOR
// https://cryptopals.com/sets/1/challenges/4

// Detect
pub fn detect_char_xor(file_path: &str) -> (usize, char, String) {
	use std::str::from_utf8;
	// Read the file into one string
	use std::fs::File;
	use std::io::Read;
	let mut file = File::open(file_path.to_string())
		.expect("File not found or error opening file");
	let mut ciphertexts_string = String::new();
	file.read_to_string(&mut ciphertexts_string)
		.expect("Error reading the file");
	// Split up all the ciphertexts and decode to bytes
	let ciphertexts: Vec<_> = ciphertexts_string.split('\n').map(|ct| hex::decode(ct.to_string()).expect("Failed to decode the first input hex string")).collect();
	// Iterate ove the ciphertexts to find the highest scoring key for each (as in challenge 3)
	// and find highest scoring overall plaintext line number
	max_score_ct = (0..ciphertexts.len())
		.map(|line| (line, single_bit_xor_cypher(&ciphertexts[line])))
		.max_by_key(|(line, key)| (vector_score(ciphertexts[line]).clone(), key))
		.unwrap()
	// Return the ciphertext number, key and plaintext
	//(max_score_ct,
		//single_bit_xor_cypher(&ciphertexts[max_score_ct]).0,
		//from_utf8(&single_bit_xor_cypher(&ciphertexts[max_score_ct]).1)
		//.expect("Failed to get String from vector")
		//.to_string())
}

fn single_bit_xor_cypher(ct_bytes: &Vec<u8>) -> (char, Vec<u8>) {
	//use std::str::from_utf8;
	// Convert hex string to bytes
	//let ct_bytes: Vec<u8> = hex::decode(ct.to_string())
	//	.expect("Failed to decode the first input hex string");
	// Find the key that maximises the score by XOR-ing the ciphertext with all possible keys (i.e., u8 range)
	let max_score_key: u8 = (0..=255)
		.max_by_key(|&key| vector_score(key_xor(ct_bytes.clone(), key)))
		.unwrap();
	// Return the plaintext key and plaintext
	(max_score_key as char,key_xor(ct_bytes.clone(), max_score_key))
}

// Assign score to characters based on frequency in the english language
// https://en.wikipedia.org/wiki/Letter_frequency#Relative_frequencies_of_letters_in_the_English_language
fn char_score(character: char) -> i32 {
	match character {
		' ' => 13000, // Space has slightly higher frequency than e
		'e' => 12702,
		't' => 9056,
		'a' => 8167,
		'o' => 7507,
		'i' => 6966,
		'n' => 6749,
		's' => 6327,
		'h' => 6094,
		'r' => 5987,
		'd' => 4253,
		'l' => 4025,
		'c' => 2782,
		'u' => 2758,
		'm' => 2406,
		'w' => 2236,
		'f' => 228,
		'g' => 2015,
		'y' => 1974,
		'p' => 1929,
		'b' => 1492,
		'v' => 978,
		'k' => 772,
		'j' => 153,
		'x' => 150,
		'q' => 95,
		'z' => 74,
		_ => 1, // non alphabetic or space characters
	}
}

// Give a score to a vector based on the score for each character
fn vector_score(vec: Vec<u8>) -> i32 {
	vec
		.iter()
		.fold(0, |score, character| score + char_score(*character as char))
}

// XOR a vector with a key
fn key_xor(v1: Vec<u8>, key: u8) -> Vec<u8> {
	v1
		.iter()
		.map(|b1| b1^key)
		.collect()
}