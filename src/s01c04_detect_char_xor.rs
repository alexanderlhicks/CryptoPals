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
	let ciphertexts: Vec<Vec<u8>> = ciphertexts_string
		.split('\n')
		.map(|ct| hex::decode(ct.to_string()).expect("Failed to decode the first input hex string"))
		.collect();
	// Iterate ove the ciphertexts to find the highest scoring key for each (as in challenge 3)
	// and find highest scoring overall plaintext
	let max_score_ct = (0..ciphertexts.len())
		.map(|line| (line, single_bit_xor_cypher(&ciphertexts[line]).0))
		.max_by_key(|&(line, key)| (vector_score(&ciphertexts[line]), key))
		.unwrap();
	// Return the ciphertext number, key and plaintext
	//(max_score_ct.0,max_score_ct.1,from_utf8(&single_bit_xor_cypher(&ciphertexts[max_score_ct]).1).expect("Failed to get String from vector").to_string())
	(max_score_ct.0,
		single_bit_xor_cypher(&ciphertexts[max_score_ct.0]).0 as char,
		from_utf8(&single_bit_xor_cypher(&ciphertexts[max_score_ct.0]).1).expect("Failed to get String from vector").to_string())
}

fn single_bit_xor_cypher(ct_bytes: &Vec<u8>) -> (u8, Vec<u8>) {
	//use std::str::from_utf8;
	// Convert hex string to bytes
	//let ct_bytes: Vec<u8> = hex::decode(ct.to_string())
	//	.expect("Failed to decode the first input hex string");
	// Find the key that maximises the score by XOR-ing the ciphertext with all possible keys (i.e., u8 range)
	let max_score_key: u8 = (0..=255)
		.max_by_key(|&key| vector_score(&key_xor(&ct_bytes, key)))
		.unwrap();
	// Return the plaintext key and plaintext
	(max_score_key,key_xor(&ct_bytes, max_score_key))
}

// Assign score to characters based on frequency in the english language
// https://en.wikipedia.org/wiki/Letter_frequency#Relative_frequencies_of_letters_in_the_English_language
fn char_score(character: char) -> isize {
	match character {
		' ' => 13000, // Space has slightly higher frequency than e
		'e' | 'E' => 12702,
		't' | 'T' => 9056,
		'a' | 'A' => 8167,
		'o' | 'O' => 7507,
		'i' | 'I' => 6966,
		'n' | 'N' => 6749,
		's' | 'S' => 6327,
		'h' | 'H' => 6094,
		'r' | 'R' => 5987,
		'd' | 'D' => 4253,
		'l' | 'L' => 4025,
		'c' | 'C' => 2782,
		'u' | 'U' => 2758,
		'm' | 'M' => 2406,
		'w' | 'W' => 2236,
		'f' | 'F' => 228,
		'g' | 'G' => 2015,
		'y' | 'Y' => 1974,
		'p' | 'P' => 1929,
		'b' | 'B' => 1492,
		'v' | 'V' => 978,
		'k' | 'K' => 772,
		'j' | 'J' => 153,
		'x' | 'X' => 150,
		'q' | 'Q' => 95,
		'z' | 'Z' => 74,
		_ => 1, // non alphabetic
	}
}

// Give a score to a vector based on the score for each character
fn vector_score(vec: &Vec<u8>) -> isize {
	vec
		.iter()
		.fold(0, |score, character| score + char_score(*character as char))
}

// XOR a vector with a key
fn key_xor(v1: &Vec<u8>, key: u8) -> Vec<u8> {
	v1
		.iter()
		.map(|b1| b1^key)
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_detect_char_xor() {
		assert_eq!(s01c04_detect_char_xor::detect_char_xor("s01c04_ciphertexts.txt").0,
    	 170);
		assert_eq!(s01c04_detect_char_xor::detect_char_xor("s01c04_ciphertexts.txt").1.to_string(),
			53.to_string());
		assert_eq!(s01c04_detect_char_xor::detect_char_xor("s01c04_ciphertexts.txt").2,
			"Now that the party is jumping\n");
	}
}