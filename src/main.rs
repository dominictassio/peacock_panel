extern crate time;

fn get_month_name(date: i32) -> &'static str {

	match date {
		0 	=> 	"JANUARY",
		1 	=> 	"FEBRUARY",
		2 	=> 	"MARCH",
		3 	=> 	"APRIL",
		4 	=> 	"MAY",
		5 	=> 	"JUNE",
		6 	=> 	"JULY",
		7 	=> 	"AUGUST",
		8 	=> 	"SEPTEMBER",
		9 	=> 	"OCTOBER",
		10 	=> 	"NOVEMBER",
		11 	=>	"DECEMBER",
		_ 	=>	"MONTH"
	}

}

fn get_weekday_name(date: i32) -> &'static str {

	match date {
		0	=>	"SUNDAY",
		1	=> 	"MONDAY",
		2	=>	"TUESDAY",
		3	=>	"WEDNESDAY",
		4	=>	"THURSDAY",
		5	=>	"FRIDAY",
		6	=>	"SATURDAY",
		_	=>	"DAY"
	}

}

fn get_date_and_time() -> String {

	let now = time::now();

	let hour = now.tm_hour % 12;
	let minute = now.tm_min;
	
	let weekday = get_weekday_name(now.tm_wday);
	let month = get_month_name(now.tm_mon);
	let day = now.tm_mday;

    return format!("{:2}:{:02} {:>9}, {:>9} {:2}", hour, minute, weekday, month, day);

}

fn make_desktops() -> String {

	let desktops: [&str; 10] = [
		"unu",
		"du",
		"tri",
		"kvar",
		"kvin",
		"ses",
		"sep",
		"ok",
		"naŭ",
		"dek"
	];

	let mut output: String = String::new();

	for i in 0..10 {
		output = 	output + 
					"%{A:bspc desktop -f " + desktops[i] + ":}" +
					"[" + desktops[i] + "]" +
					"%{A}" +
					if i != 9 { " " } else { "" };
	}

	return output;

}

fn get_system_buttons() -> String {
	String::new() +
	"%{A:bspc quit:}" +
	"%{F#e6db74}[-]%{F-}" +
	"%{A}" +
	" " +
	"%{A:sudo shutdown -h now:}%{A3:sudo shutdown -r now:}" +
	"%{F#ff5d38}[x]%{F-}" +
	"%{A}%{A}"
}

fn main() {

	loop {
		println!("%{{l}}  {desktops}  %{{l}} %{{r}}  [{time_date}]  {system}  %{{r}}",
				desktops=&make_desktops(),
				time_date=get_date_and_time(),
				system=get_system_buttons());
	}

}
