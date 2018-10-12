mod s01c01_hex_to_base64;
mod s01c02_fixed_xor;
mod s01c03_single_byte_xor;
mod s01c04_detect_char_xor;

// Executes the code for each of the CryptoPals challenges that are in self-contained modules.
//https://cryptopals.com/

fn main() {
    // Set1 - Basics
    // Challenge 1 - Hex to base64 string conversion
    println!("Running Set 1 - Challenge 1");
    assert_eq!(s01c01_hex_to_base64::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
    	"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    assert_eq!(s01c01_hex_to_base64::base64_to_hex("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"),
    	"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("Set 1 - Challenge 1 OK");
    // Challenge 2 - Fix length XOR of hex strings
    println!("Running Set 1 - Challenge 2");
    assert_eq!(s01c02_fixed_xor::hex_fixed_xor("1c0111001f010100061a024b53535009181c","686974207468652062756c6c277320657965"),
    	"746865206b696420646f6e277420706c6179");
    println!("Set 1 - Challenge 2 OK");
    // Challenge 3 - Decode single byte XOR cypher
    println!("Running Set 1 - Challenge 3");
    assert_eq!(s01c03_single_byte_xor::single_bit_xor_cypher("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").0.to_string(),
    	"X");
    assert_eq!(s01c03_single_byte_xor::single_bit_xor_cypher("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").1,
    	"Cooking MC's like a pound of bacon");
    println!("Set 1 - Challenge 3 OK");
    // Challenge 4 - 
    println!("Running Set 1 - Challenge 4");
    assert_eq!(s01c04_detect_char_xor::detect_char_xor("s01c04_ciphertexts.txt").0,
    	 170);
    assert_eq!(s01c04_detect_char_xor::detect_char_xor("s01c04_ciphertexts.txt").1.to_string(),
    	 5.to_string());
    assert_eq!(s01c04_detect_char_xor::detect_char_xor("s01c04_ciphertexts.txt").2,
    	 "Now that the party is jumping\n");
    println!("Set 1 - Challenge 4 OK");
}
