#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

pub mod cvr_sdk;

use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;
use tauri::Manager;
use textcode::gb2312;

fn main() {
    let app = tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                // window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet,cvr])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let mut count = Box::new(0);
    app.run(|app_handle, event| match event {
        tauri::RunEvent::MainEventsCleared => {
            count = count+1;
            println!("On RunEvent::MainEventsCleared")
        }
        _ => {}
    });
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}


#[tauri::command]
fn cvr() -> String {
    let name = [0; 128];
    let len = Box::new(128);
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
                        cvr_sdk::cvr_people_name(&name[0], &len).unwrap();
                        // print!("\n姓名：{}", gb2312::decode_to_string(&name));
                        // cvr_sdk::cvr_people_code(&name[0], &len).unwrap();
                        // print!("\n身份证号：{}", gb2312::decode_to_string(&name));
                        println!("读卡操作成功!");
                        cvr_sdk::cvr_close_comm().unwrap();
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
    };

    gb2312::decode_to_string(&name).replace('\0', "")
}