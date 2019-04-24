pub fn verse(n: i32) -> String {
	if n==0{
		return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
	}
	else if n==1{
		return "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string()
	}
	else if n==2{
		return "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string()
	}
	else {
		
		return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n , n, n-1)
	}
}

pub fn sing(start: i32, end: i32) -> String {
    let mut hasil = "".to_string();
	
	for i in (end..=start).rev() {
		hasil.push_str(&verse(i));
		//hasil.pop();
		hasil.push_str(&"\n");
	}
	hasil.pop();
	hasil
	
	//unimplemented!("sing verses {} to {}, inclusive", start, end)
	
}
