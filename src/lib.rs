use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

pub fn uniq<R: BufRead, W: Write>(input: R, mut output: W, count: bool) -> std::io::Result<()> {
    let mut lines = input.lines();
    let mut prev_line = None;
    let mut current_count = 0;

    let mut write = |line: Option<&String>, line_count: usize| -> std::io::Result<()> {
        if let Some(line) = line {
            if count {
                writeln!(output, "{} {}", line_count, line)?;
            } else {
                writeln!(output, "{}", line)?;
            }
        }
        Ok(())
    };

    while let Some(line) = lines.next() {
        let line = line?;
        if Some(&line) == prev_line.as_ref() {
            current_count += 1;
        } else {
            write(prev_line.as_ref(), current_count)?;
            prev_line = Some(line);
            current_count = 1;
        }
    }

    write(prev_line.as_ref(), current_count)?;

    Ok(())
}

pub fn build_reader(path: Option<PathBuf>) -> std::io::Result<Box<dyn BufRead>> {
    if let Some(path) = path {
        if path.to_str() == Some("-") {
            return Ok(Box::new(BufReader::new(stdin())));
        }
        File::open(&path)
            .map(|file| Box::new(BufReader::new(file)) as Box<dyn BufRead>)
    } else {
        Ok(Box::new(BufReader::new(stdin())))
    }
}

pub fn build_writer(path: Option<PathBuf>) -> std::io::Result<Box<dyn Write>> {
    if let Some(path) = path {
        File::create(&path)
            .map(|file| Box::new(BufWriter::new(file)) as Box<dyn Write>)
    } else {
        Ok(Box::new(BufWriter::new(stdout())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_uniq() {
        let input = Cursor::new("apple\napple\nbanana\napple\nbanana\nbanana\n");
        let mut output = Vec::new();

        uniq(input, &mut output, false).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "apple\nbanana\napple\nbanana\n");
    }

    #[test]
    fn test_uniq_with_count() {
        let input = Cursor::new("apple\napple\nbanana\napple\nbanana\nbanana\n");
        let mut output = Vec::new();

        uniq(input, &mut output, true).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "2 apple\n1 banana\n1 apple\n2 banana\n");
    }

    #[test]
    fn test_uniq_with_unicode() {
        let input = Cursor::new("São Tomé\nSão Tomé\nBarthélemy\nSão Tomé\nBarthélemy\nBarthélemy\n");
        let mut output = Vec::new();

        uniq(input, &mut output, true).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "2 São Tomé\n1 Barthélemy\n1 São Tomé\n2 Barthélemy\n");
    }
}