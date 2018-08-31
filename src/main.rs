mod c01_hex_to_base64;
mod c02_fixed_xor;

fn main() {
    //Set1
    //Challenge 1
    println!("Running Set 1 - Challenge 1");
    assert_eq!(c01_hex_to_base64::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
    	"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    assert_eq!(c01_hex_to_base64::base64_to_hex("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"),
    	"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("Set 1 - Challenge 1 OK");
    //Challenge 2
    println!("Running Set 1 - Challenge 2");
    assert_eq!(c02_fixed_xor::hex_fixed_xor("1c0111001f010100061a024b53535009181c","686974207468652062756c6c277320657965"),
    	"746865206b696420646f6e277420706c6179");
    println!("Set 1 - Challenge 2 OK");
}
