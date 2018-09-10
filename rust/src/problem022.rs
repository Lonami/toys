/*
Using names.txt (right click and 'Save Link/Target As...'), a 46K text
file containing over five-thousand first names, begin by sorting it into
alphabetical order. Then working out the alphabetical value for each name,
multiply this value by its alphabetical position in the list to obtain a
name score.

For example, when the list is sorted into alphabetical order, COLIN, which
is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN
would obtain a score of 938 × 53 = 49714.

What is the total of all the name scores in the file?
*/
use std::fs::File;
use std::io::{BufRead, BufReader, Result};


#[test]
fn solve() -> Result<()> {
    let file = File::open("resources/p022_names.txt")?;
    let mut names = BufReader::new(file)
        .split(b',')
        .map(|r| r.map(|x| x.iter()
                            .filter(|&&c| c != b'"')
                            .map(|&x| x as char)
                            .collect::<String>()))
        .collect::<Result<Vec<String>>>()?;

    names.sort();
    let result = names.iter()
        .enumerate()
        .map(|(i, x)| // Not sure how to style this cleanly
             (i as u64 + 1)
             * (x.as_bytes()
                 .iter()
                 .map(|&c| (c - b'A' + 1) as u64)
                 .sum::<u64>()) as u64)
        .sum::<u64>();

    assert_eq!(result, 871198282);
    Ok(())
}
