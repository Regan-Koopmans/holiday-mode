extern crate chrono;
extern crate pop3_rs;
extern crate regex;

use regex::Regex;
use std::net::TcpStream;
use pop3_rs::{POP3Connection, AccountConfig};
use std::io::prelude::*;
use std::path::PathBuf;
use std::str;

fn main() {
    let email_fmt = Regex::new(r"From: .+ <[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+>").unwrap();
    let email_sub = Regex::new(r"Subject: prac7").unwrap();
    let path = PathBuf::from("/home/regan/mail/");
    let mut connection = POP3Connection::new(
        AccountConfig { host: "pop.gmail.com".to_string(),
                        port: 995,
                        username : "u15043143@tuks.co.za".to_string(),
                        password : "rk?Amsterdam777".to_string(),
                        auth : "SSL".to_string(),
                        maildir : path
                    }).unwrap();
    connection.login().unwrap();
    let mail_count = connection.list(None).unwrap();
    let length = mail_count.mailbox.len();
    println!("Found {} emails.", length);
    for id in mail_count.mailbox {
        let mail = connection.retr(id.msg_id).unwrap();
        println!("{}/{}...", id.msg_id, length);
        let address = email_fmt.find(&mail.msg_data).unwrap().as_str();
        let address:Vec<&str> = address.split(" ").collect();
        let subject = email_sub.find(&mail.msg_data);
        let address = address.last().unwrap();
        let address = &address[1..address.len()-1];
        if subject != None {
            println!("Sending auto-response to\n{}", address);
            send_mail("Regan is currently on break, and will get back to you as soon as possible.", address);
        }
    }
    connection.quit().unwrap();
}

fn send_mail(msg: &str, addr:&str) {
    let mut stream = TcpStream::connect("127.0.0.1:25").unwrap();
    stream.write("HELO regan\r\n".as_bytes()).unwrap();
    stream.write("MAIL FROM: holiday@tuks.co.za\r\n".as_bytes()).unwrap();
    stream.write(format!("RCPT TO: {}\r\n",addr).as_bytes()).unwrap();
    stream.write("DATA\r\n".as_bytes()).unwrap();
    stream.write(format!("Subject: Auto-response\r\n\r\n").as_bytes()).unwrap();
    stream.write(format!("{}\r\n", msg).as_bytes()).unwrap();
    stream.write(".\r\n".as_bytes()).unwrap();
    stream.write("QUIT\r\n".as_bytes()).unwrap();
}
