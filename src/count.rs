use std::fs::File;
use std::io::{self, stdin, Stdin, BufRead, BufReader};

enum Reader<X, Y> {
    FileReader(X),
    IoReader(Y),
}

fn main() -> io::Result<()> {
    use self::Reader::*;
    // let mut filename = String::new();
    let filename = get_filename();
    let file_result = File::open(filename);

    let reader: Reader<BufReader<File>, Stdin> = match file_result {
        Ok(file_) =>  Reader::FileReader(BufReader::new(file_)),
        Err(_) => Reader::IoReader(stdin())
    };

    let count: isize = match reader {
        FileReader(reader) => count_lines(reader),
        IoReader(unbuffed) => {
                                let buffed = unbuffed.lock();
                                count_lines(buffed)
        },
    };

    println!("{}", count);
    Ok(())
}

fn count_lines<R>(reader: R) -> isize
    where R: BufRead
{
    let lines = reader.lines();
    let mut count = 0; 
    for _ in lines {
        count = count + 1;
    }   
    count
}
   
fn get_filename() -> String {
    let mut args = String::new();

    for arg in std::env::args().skip(1) {
        args.push_str(&arg);
    };
   args
}
