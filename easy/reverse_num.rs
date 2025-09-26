
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
//optimal approach
 fn safe_reverse(mut num:i32)->i32{
	 let mut reversed:i32=0;
	 let _negative=num<0;
	 num=num.abs();
	 while num>0{
		 let digit =num%10;
		 if let Some(new_reversed)=reversed.checked_mul(10).and_then(|r| r.checked_add(digit)){
			 reversed=new_reversed
		 }else{
			 return 0;
		 }
		 num/=10;
	 }
	 reversed
 }
 
fn main(){
	let num:i32 =12345;
	let reversed_num=reverse(num.clone());
	let safe_reversed=safe_reverse(num.clone());
	println!("{}",reversed_num);
	println!("{}",safe_reversed);
}
