extern crate hex;

// CryptoPals Set 1 Challenge 5 - Implement repeating-key XOR
// https://cryptopals.com/sets/1/challenges/5

pub fn repeating_key_xor(plaintext: &str, key: &str) -> String {
	let key_bytes: Vec<u8> = key.to_string().into_bytes();
	let pt_bytes: Vec<u8> = plaintext.to_string().into_bytes();
	let xored: Vec<u8> = pt_bytes
		.into_iter()
		.enumerate()
		.map(|(iter,pt_byte)| pt_byte ^ key_bytes[iter % key_bytes.len()])
		.collect();
	hex::encode(&xored)
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_repeating_key_xor() {
		assert_eq!(repeating_key_xor("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal", "ICE"),
    	 "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
	}
}