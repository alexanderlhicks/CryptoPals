extern crate hex;

// XOR two fixed length hex strings together
pub fn hex_fixed_xor(in1: &str, in2: &str) -> String {
	let in1_bytes: Vec<u8> = hex::decode(in1.to_string()).unwrap();
	let in2_bytes: Vec<u8> = hex::decode(in2.to_string()).unwrap();
	if in1_bytes.len() != in2_bytes.len() {
		panic!("Buffers of unequal lengths");
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