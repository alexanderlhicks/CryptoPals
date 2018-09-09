extern crate hex;
// CryptoPals Set 1 Challenge 3 - Single-byte XOR cipher
// https://cryptopals.com/sets/1/challenges/3


// XOR input hex string with every character and output the most likely key and result based on character scores

pub fn single_bit_xor_cypher(ct: &str) -> (String, String) {
	use std::str::from_utf8;
	// Convert hex string to bytes
	let ct_bytes: Vec<u8> = hex::decode(ct.to_string()).expect("Failed to decode the first input hex string");
	// Find the key that maximises the score by XOR-ing the ciphertext with all possible characters
	let max_score_key: u8 = ('a'..'z').max_by_key(|key| vector_score(vector_xor(ct_bytes, vec![key; ct_bytes.len()]))).unwrap();
	// Return the plaintext
	(max_score_key.to_string(),from_utf8(&vector_xor(ct_bytes, vec![max_score_key; ct_bytes.len()])).expect("Failed to get String from vector"))
}

// Assign score to characters based on frequency in the english language
// https://en.wikipedia.org/wiki/Letter_frequency#Relative_frequencies_of_letters_in_the_English_language
fn char_score(character: char) -> f32 {
	match character {
		' ' => 13.0, // space has slightly higher frequncy than e
		'e' => 12.702,
		't' => 9.056,
		'a' => 8.167,
		'o' => 7.507,
		'i' => 6.966,
		'n' => 6.749,
		's' => 6.327,
		'h' => 6.094,
		'r' => 5.987,
		'd' => 4.253,
		'l' => 4.025,
		'c' => 2.782,
		'u' => 2.758,
		'm' => 2.406,
		'w' => 2.236,
		'f' => 2.28,
		'g' => 2.015,
		'y' => 1.974,
		'p' => 1.929,
		'b' => 1.492,
		'v' => 0.978,
		'k' => 0.772,
		'j' => 0.153,
		'x' => 0.150,
		'q' => 0.095,
		'z' => 0.074,
		_ => 0.0001, // non alphabetic or space characters
	}
}

// Give a score to a vector
fn vector_score(vec: Vec<u8>) -> f32 {
	vec.iter().fold(0, |score, character| score + char_score(character as char))
}

fn vector_xor(v1: Vec<u8>, v2: Vec<u8>) -> Vec<u8> {
	v1.iter().zip(v2.iter()).map(|(b1,b2)| b1^b2).collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_fixed_xor() {
		assert_eq!(single_bit_xor_cypher("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").1,
    	"Cooking MC's like a pound of bacon");
	}
}