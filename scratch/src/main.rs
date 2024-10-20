fn main() -> Result<(), ()> {
	let msg = "hi";
	say(msg);

	let msg_2 = "hello";
	let longest = longest(msg, msg_2);
	println!("The longer message is: {}", longest);

	let mut i = 1;
	i = add(i);
	println!("{}", i);

	let mut y: i32 = 1;
	add_mut(&mut y);
	println!("{}", y);

	Ok(())

}

fn say(msg: &str) {
	println!("{}", msg);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

fn add(x: i32) -> i32 {
	x + 1
}

fn add_mut(x: &mut i32) {
	*x = *x + 1;
}
