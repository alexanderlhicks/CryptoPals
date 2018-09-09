extern crate hex;

// CryptoPals Set 1 Challenge 2 - Fixed XOR
// https://cryptopals.com/sets/1/challenges/2

// XOR two fixed length hex strings together
pub fn hex_fixed_xor(in1: &str, in2: &str) -> String {
	let in1_bytes: Vec<u8> = hex::decode(in1.to_string()).expect("Failed to decode the first input hex string");
	let in2_bytes: Vec<u8> = hex::decode(in2.to_string()).expect("Failed to decode the second input hex string");
	if in1_bytes.len() != in2_bytes.len() {
		panic!("The decoded hex strings are of uneven lenghts and cannot be XOR-ed against each other");
	}
	let xored: Vec<u8> = in1_bytes.iter().zip(in2_bytes.iter()).map(|(b1,b2)| b1^b2).collect();
	hex::encode(&xored)
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_fixed_xor() {
		assert_eq!(hex_fixed_xor("1c0111001f010100061a024b53535009181c","686974207468652062756c6c277320657965"),
    	"746865206b696420646f6e277420706c6179");
	}
}