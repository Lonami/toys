/*
You are given the following information, but you may prefer to do some
research for yourself.

    * 1 Jan 1900 was a Monday.
    * Thirty days has September,
      April, June and November.
      All the rest have thirty-one,
      Saving February alone,
      Which has twenty-eight, rain or shine.
      And on leap years, twenty-nine.
    * A leap year occurs on any year evenly divisible by 4,
      but not on a century unless it is divisible by 400.

How many Sundays fell on the first of the month during the twentieth
century (1 Jan 1901 to 31 Dec 2000)?
*/
fn is_leap(n: i32) -> bool {
    if n % 4 != 0 {
        false
    } else if n % 100 == 0 {
        n % 400 == 0
    } else {
        true
    }
}

fn days_of_month(month: i32, year: i32) -> i32 {
    match month {
        0 | 2 | 4 | 6 | 7 | 9 | 11 => 31,
        3 | 5 | 8 | 10 => 30,
        1 => if is_leap(year) { 29 } else { 28 },
        _ => panic!("month must be within 0..12")
    }
}


#[test]
fn solve() {
    let mut result = 0;
    let mut day = 365 % 7;
    for year in 1901..=2000 {
        for month in 0..12 {
            if day == 6 {
                result += 1;
            }
            day = (day + days_of_month(month, year)) % 7;
        }
    }
    assert_eq!(result, 171);
}
