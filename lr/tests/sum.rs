use std::io::Read;
use std::net::TcpStream;
use lb::movies::sum;
use ssh2::ErrorCode::Session;

mod commons;

use commons::{setup, teardown};

#[test]
fn sum_test1() {
    assert_eq!(sum(2, 2), 4);
}

/**
加上下面的参数可以显示print宏
cargo test sum_1_test -- --nocapture
 */
#[test]
fn sum_1_test() {
    setup();
    assert_eq!(sum(2, 2), 4);
    teardown();
}


#[test]
fn variables() {

    use ssh2::Session;

    let stream = TcpStream::connect("192.168.2.9:22").unwrap();
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(stream);
    session.handshake().unwrap();
    session.userauth_password("root", "123456").unwrap();

    if session.authenticated() {
        let mut channel = session.channel_session().unwrap();
        channel.exec("ls -al /root").unwrap();

        let mut s = String::new();
        channel.read_to_string(&mut s);
        println!("{}", s);
        channel.wait_close();
        println!("{}", channel.exit_status().unwrap())
    };
}


#[test]
fn types() {
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}

pub fn type_name_of_val<T: ?Sized>(_val: &T) -> &'static str {
    std::any::type_name::<T>()
}

