
fn reverse(mut x:i32)->i32{
	let mut reversed=0;
	let negative=x<0;
	 x=x.abs();
	while x>0{
		let digit =x%10;
		reversed=reversed*10+digit;
		x/=10;
	}
	if negative{
		-reversed
	}else{
		reversed
	}
}
fn main(){
	let num:i32 =12345;
	let reversed_num=reverse(num);
	println!("{}",reversed_num);
}
