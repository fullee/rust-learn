// extern crate lb;
use lb::movies::play;
use std::{env, thread};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;

struct Video(String);

struct Audio(String);

// mod media;
// use media::Playable;
//
// impl Playable for Video {
//     fn play(&self) {
//         println!("Video Go!! - {}",self.0);
//     }
// }
//
// impl Playable for Audio {
//     fn play(&self) {
//         println!("Audio Go!! - {}",self.0);
//     }
// }

trait Vehicle {
    fn get_price(&self) -> u64;
}

trait Car: Vehicle {
    fn model(&self) -> String;
}

struct TeslaRoadster {
    model: String,
    release_date: u16,
}

impl TeslaRoadster {
    fn neww(model: &str, release_date: u16) -> Self {
        Self { model: model.to_string(), release_date }
    }
}

impl Vehicle for TeslaRoadster {
    fn get_price(&self) -> u64 {
        200_000
    }
}

impl Car for TeslaRoadster {
    fn model(&self) -> String {
        "Tesla II".to_string()
    }
}


#[derive(Debug)]
struct Foo(String);

fn main() {
    println!("启动");
    let listener = TcpListener::bind("0.0.0.0:9527").unwrap();
    loop {
        let mut x = listener.accept().unwrap();

        println!("{}", x.1);

        thread::spawn(move || {
            let mut buf = [0u8; 12];

            x.0.read_exact(&mut buf).unwrap();

            println!("{:?}",String::from_utf8_lossy(&buf));
            x.0.write_all(b"hello,you")

        });
    }
}


fn large_loop() {
    for _ in 0..1_000_000_000 {};
}

pub fn sum_1(a: i8, b: i8) -> i8 {
    a + b
}

// #[bench]
// fn bench_fast(b: &mut Bencher) {
//     b.iter(|| sum_1(1,2))
// }

// cargo test test_panic

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        println!("{}", "😔");
        assert!(true)
    }

    /**
    cargo test 时，会忽略这个方法
     */
    #[test]
    // #[ignore]
    fn test_ignore() {
        crate::large_loop()
    }

    /**
    断言这个方法会panic，测试会通过
     */
    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("出错了");
    }
}
