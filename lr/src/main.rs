// extern crate lb;
use lb::movies::play;
use std::env;
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
    let mut foo = String::from("own");
    let bar = &mut foo;
    bar.push('!');
    let bar2 = &mut foo;

    println!("{}", bar2);
    // let tr = TeslaRoadster::neww("Tesla Roadster II", 2020);
    // println!("{} is priced at ${}", tr.model, tr.get_price());
    // let image_path = env::args().skip(1).next().unwrap();
    // let path = Path::new(&image_path);
    // let img = image::open(path).unwrap();
    // let rotate = img.rotate90();
    // rotate.save(path).unwrap();
    // play("hh".to_string())

    // let audio = Audio("ama".to_string());
    // let video = Video("vv.mp4".to_string());
    // audio.play();
    // video.play();
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
