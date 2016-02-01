use std::io::{stdin, Read, BufReader, BufRead};
use std::fmt;

type CountTable = std::collections::HashMap<String, usize>;

fn read_words<R: Read>(reader: R) -> CountTable{
    let mut count_table = CountTable::new();
    let mut lines = BufReader::new(reader).lines();

    while let Some(Ok(line)) = lines.next() {
        let mut word  = String::new();
        for c in line.chars() {
            if c.is_alphanumeric() {
                word.push(c.to_lowercase().next().unwrap());
            } else {
                if word.len() != 0 {
                    *count_table.entry(word.clone()).or_insert(0) += 1;
                    word.clear();
                }
            }
        }
        if word.len() != 0 {
            *count_table.entry(word).or_insert(0) += 1;
        }
    }
    return count_table;
}

struct ResultVec(Vec<(usize, String)>);

impl fmt::Display for ResultVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for each in self.0.iter() {
            output.push_str(&each.1);
            output.push_str(" : ");
            output.push_str(&each.0.to_string());
            output.push_str("\n");
        }
        write!(f, "{}", output)
    }
}

fn main() {
    let count_table = read_words(stdin());
    let mut results = ResultVec(vec![]);
    for (word, count) in count_table.iter() {
        results.0.push((count.clone(), word.clone()));
    }
    results.0.sort_by(|a, b| b.cmp(a));
    println!("{}", results);
}

#[cfg(test)]
mod word_freq_test {
    use std::io::{Read, Result};
    use super::{read_words, ResultVec};

    struct StringReader {
        contents: Vec<u8>,
        position: usize,
    }

    impl StringReader {
        fn new(s: String) -> Self {
            StringReader {
                contents: s.into_bytes(),
                position: 0,
            }
        }
    }

    impl Read for StringReader {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let mut count = 0;

            while self.position < self.contents.len() && count < buf.len() {
                buf[count] = self.contents[self.position];
                count += 1;
                self.position += 1;
            }

            return Ok(count);
        }
    }

    fn test(expected: &str, input: &str) {
        let mock_read = StringReader::new(input.to_owned());
        assert_eq!(expected.to_owned(), word_freq_test(mock_read));
    }

    fn word_freq_test<R: Read>(reader: R) -> String {
        let count_table = read_words(reader);
        let mut results = ResultVec(vec![]);
        for (word, count) in count_table.iter() {
            results.0.push((count.clone(), word.clone()));
        }
        results.0.sort_by(|a, b| b.cmp(a));
        results.to_string()
    }

    #[test]
    fn reads_one_word() {
        test("hello : 1\n", "hello!!!\n");
    }

    #[test]
    fn reads_two_lines() {
        test("world : 2\nhello : 1\ngoodbye : 1\n",
             "hello world!!!!\n ** goodbye -- world!");
    }

    #[test]
    fn reads_two_lines_case_sensitive() {
        test("world : 2\nhello : 1\ngoodbye : 1\n",
             "heLLo World!!!!\n ** GoodBye -- world!");
    }
}
