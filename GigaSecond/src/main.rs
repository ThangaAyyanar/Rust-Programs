extern crate chrono;
use chrono::prelude::*;
use chrono::Duration;
fn main() {
   let mut start = Utc.ymd(2015, 1, 24).and_hms(23, 59, 59);//Utc.ymd(2011, 4, 25).and_hms(0, 0, 0);
   let duration = Duration::seconds(1000000000);
   let gigaAnswer = start.checked_add_signed(duration);
   let result = match gigaAnswer {
	Some(a) => a,
	None => start
   };
   print!("{:?}",result);//gigaAnswer.unwarp());
   //println!("{:?}",Utc.ymd(2046, 10, 3).and_hms(1, 46, 39));
}
