extern crate hex;
extern crate base64;

// Convert a hex string to a base64 string
pub fn hex_to_base64(hex_input: &str) -> String {
	base64::encode(&hex::decode(hex_input.to_string()).unwrap())
}

// Convert a base64 string to a hex string
pub fn base64_to_hex(base64_input: &str) -> String {
	hex::encode(&base64::decode(&base64_input.to_string()).unwrap())
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_hex_to_base64() {
		assert_eq!(hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
	}
	#[test]
	fn test_base64_to_hex() {
		assert_eq!(base64_to_hex("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"),
			"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
	}
}