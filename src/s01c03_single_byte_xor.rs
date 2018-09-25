extern crate hex;

// CryptoPals Set 1 Challenge 3 - Single-byte XOR cipher
// https://cryptopals.com/sets/1/challenges/3


// XOR input hex string with every character and output the most likely key and result based on character scores
pub fn single_bit_xor_cypher(ct: &str) -> (char, String) {
	use std::str::from_utf8;
	// Convert hex string to bytes
	let ct_bytes: Vec<u8> = hex::decode(ct.to_string())
		.expect("Failed to decode the first input hex string");
	// Find the key that maximises the score by XOR-ing the ciphertext with all possible keys (i.e., u8 range)
	let max_score_key: u8 = (0..=255)
		.max_by_key(|&key| vector_score(key_xor(ct_bytes.clone(), key)))
		.unwrap();
	// Return the plaintext key and plaintext
	(max_score_key as char,from_utf8(&key_xor(ct_bytes.clone(), max_score_key))
		.expect("Failed to get String from vector")
		.to_string())
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

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_fixed_xor() {
		assert_eq!(s01c03_single_byte_xor::single_bit_xor_cypher("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").0.to_string(),
    	"X");
		assert_eq!(single_bit_xor_cypher("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").1,
    	"Cooking MC's like a pound of bacon");
	}
}