extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("\n\nRace to 100\n");
    print_rules();
    let mut game_sum = 0;
    println!("Game Sum {}",game_sum);
    while game_sum < 100{
    	game_sum = game_sum + get_user_input(game_sum);
	println!("Game Sum {}",game_sum);
	if game_sum == 100 {
		println!("You Win ! :-D");
		break;}
        game_sum = game_sum + get_computer_input(game_sum);
    	println!("Game Sum {}",game_sum);
	if game_sum ==100 {
		println!("Computer wins")
	}
    }
}

fn get_computer_input(sum: u32) -> u32 {
 let mut input = rand::thread_rng().gen_range(1,11);

 if sum >34 {
	let n = sum/11;
	input =  ((11*(n+1))+1) - sum ;
	if input >10{
		input = rand::thread_rng().gen_range(1,11);
	}
 }

 if sum+input >100{
 	get_computer_input(sum)
 }else{
	 println!("Computer entered {}",input);
	 return input;
 }
}

fn get_user_input(sum: u32) -> u32 {
	let mut input = String::new();
	println!("Please enter a number between 1 and 10");
	io::stdin().read_line(&mut input).expect("failed to read input");
	let input: u32 = input.trim().parse().expect("Not a Number");
	if input>0 && input<11{
		return input;	
	}else if input + sum > 100{
		println!("Input causing game sum to exceed 100\nInvalid Input");
		get_user_input(sum)
	}else{
		println!("Invalid Input");
		get_user_input(sum)
	}
}

fn print_rules(){
	println!("Rules: Enter digit between 1 and 10\n");
}
