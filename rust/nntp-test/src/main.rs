extern crate nntp;

use nntp::{Article, NNTPStream};

fn main() {
    let mut nntp_stream = match NNTPStream::connect("news.gmane.org", 119) {
        Ok(stream) => stream,
        Err(e) => panic!("{}", e),
    };

    match nntp_stream.capabilities() {
        Ok(lines) => {
            for line in lines.iter() {
                println!("{}", line);
            }
        }
        Err(e) => panic!(e),
    };

    match nntp_stream.group("gmane.emacs.devel") {
        Ok(_) => {
            println!("Group found!");
        }
        Err(e) => {
            println!("ERR: Group not found.");
            println!("ERR: :: {}", e);
        }
    };

    let _ = nntp_stream.quit();
}
