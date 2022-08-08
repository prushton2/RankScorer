#![allow(non_snake_case)]

fn main() {
	let mut score: i32 = 0;
	let answerKey: [String; 3] = [String::from("Hello"), String::from("World"), String::from("Hi")];
	
	let submittedAsnwers: [String; 3] = [String::from("Hello"), String::from("Hi"), String::from("World")];

	for (i, x) in answerKey.iter().enumerate() {
		//find the index of the element in the sibmitted answers array
		let index = submittedAsnwers
							.iter()
							.position(|z| z == x)
							.unwrap();
		score -= (i as i32 - index as i32).abs();		
	}

	println!("Score: {}", score);
    
	
}

