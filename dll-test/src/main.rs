use std::thread::sleep;
use std::time::Duration;
use textcode::gb2312;

pub mod cvr_sdk;

fn main() {
    println!("hello！");
    cv();
}

fn cv() {
    loop {
        unsafe {
            cvr_sdk::cvr_set_com_baudrate(9600).unwrap();
            for i in 1001..=1006 {
                let init_state = cvr_sdk::cvr_initcomm(i).unwrap();
                if 1 == init_state {
                    println!("卡片开始认证: ");
                    sleep(Duration::from_secs(2));
                    let auth_code = cvr_sdk::cvr_authenticate().unwrap();
                    if auth_code == 1 {
                        let read_content = cvr_sdk::cvr_read_content(4).unwrap();
                        if read_content == 1 {
                            let name = [0; 128];
                            let len = Box::new(128);
                            cvr_sdk::cvr_people_name(&name[0], &len).unwrap();
                            print!("\n姓名：{}", gb2312::decode_to_string(&name));
                            cvr_sdk::cvr_people_code(&name[0], &len).unwrap();
                            print!("\n身份证号：{}", gb2312::decode_to_string(&name));
                            println!("读卡操作成功!");
                            break;
                        } else {
                            println!("读卡操作失败!");
                            break;
                        }
                    } else {
                        println!("请重新放入卡片");
                        break;
                    }
                }
            }
        }
        println!("\n");
        sleep(Duration::from_secs(1));
    };
}