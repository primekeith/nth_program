use std::io;

fn main() {
    //get values from terminal
    println!("Welcome to nth program . . .! \n");
    loop {
	println!("Input your base :");
	
	let mut base = String::new();

	io::stdin()
	    .read_line(&mut base)
	    .expect("Failed to read base.");
	
	let base2: i64 = match base.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};
	
	println!("Input your power :");

	let mut power = String::new();

	io::stdin()
	    .read_line(&mut power)
	    .expect("Failed to read power.");
	
	let power: i32 = match power.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};
	
	let mut i = 1;
	let mut result: i64 = base2;
	
	while i <= power {
	    if i == power {
		break;
	    }
	    result = result * result;
	    i += 1;
	}

	println!("\n");
	println!("->Base: {}, Power: {}", base2, power);
	println!("->Result: {}", result);
	println!("\n successful . . . \n \n \n \n \n Program restarted . . . \n");

    }
}
