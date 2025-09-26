fn count_digit(mut num:i32)->i32{
		if num==0{
			return 1;
		}
		let mut counted_digit:i32=0;
		num=num.abs();
		while num>0{
			counted_digit+=1;
			num/=10;
		}
		counted_digit as i32
}
//optimal approach 
fn count_digit_log(num:i32)->i32{
		if num==0{
			return 1;
		}
		let _n=num.abs();
		let _digit=(_n as f64).log10().floor() as i32+1; 
		_digit
}

fn main(){
	let num:i32=123;
	let count=count_digit(num.clone());
	let count_digit_by_log=count_digit_log(num.clone());
	println!("{}",count);
	println!("{}",count_digit_by_log);
}
