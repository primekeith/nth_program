use std::io;

fn main() {
    //get values from terminal
    println!("     Program Start . . .! \n");
    loop {
	println!("Input Base :");
	
	let mut base = String::new();

	io::stdin()
	    .read_line(&mut base)
	    .expect("Failed to read base.");
	
	let base2: i128 = match base.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};
	
	println!("Input Power :");

	let mut power = String::new();

	io::stdin()
	    .read_line(&mut power)
	    .expect("Failed to read power.");
	
	let power: i32 = match power.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};
	
	let mut i = 1;
	let mut result: i128 = base2;
	
	while i <= power {
	    if i == power {
		break;
	    }
	    result = result * base2;
	    i += 1;
	}

	println!("\n");
	println!("-> Base: | {} |, Power: | {} | \n", base2, power);
	println!("-> Result: || {} ||", result);
	println!("\n \nProgram Complete! \n \n \n \n \n      Program restarted . . . \n");

    }
}
