use std::error;
use libloading::{Library, Symbol};
use once_cell::sync::Lazy;

static DLL: Lazy<Library> = Lazy::new(|| unsafe { Library::new("asserts/x64/Termb.dll").expect("DLL文件异常") });

/// 设置波特率
pub unsafe fn cvr_set_com_baudrate(u: i32) -> Result<i32, Box<dyn error::Error>> {
    let func: Symbol<unsafe extern fn(i32) -> i32> = DLL.get(b"CVR_SetBaudRate")?;
    Ok(func(u))
}

/// 原    型：int CVR_InitComm (int Port)
// 说    明：本函数用于PC与华视电子第二代居民身份证阅读器的连接。
// 参    数：Port：连接串口（COM1~COM16）或USB口(1001~1016)
//
// 参数名	含义	取值范围
// int Port	端口编号	见下表
// 端口编号：
// 值	意义
// 1	串口1
// 2	串口2
// 3	串口3
// ...	...
// 16	串口16
//
// 1001	USB口1
// 1002	USB口2
// 1003	USB口3
// ...	...
// 1016	USB口16
// 返回值：
// 值	意义
// 1	正确
// 2	端口打开失败
// -1	未知错误
// -2	动态库加载失败
pub unsafe fn cvr_initcomm(u: i32) -> Result<i32, Box<dyn error::Error>> {
    let func: Symbol<unsafe extern fn(i32) -> i32> = DLL.get(b"CVR_InitComm")?;
    Ok(func(u))
}


/// 原    型：int CVR_Authenticate (void)
// 说    明：本函数用于读卡器和卡片之间的合法身份确认。卡认证循环间隔大于300ms。
// 参    数：
// 返 回 值：
// 值	意义	说明
// 1	正确	卡片认证成功
// 2	错误	寻卡失败
// 3	错误	选卡失败
// 4	错误	未连接读卡器
// 0	错误	动态库未加载
// （若卡片放置后发生认证错误时，请移走卡片重新放置。）
pub unsafe fn cvr_authenticate() -> Result<i32, Box<dyn error::Error>> {
    let func: Symbol<unsafe extern fn() -> i32> = DLL.get(b"CVR_Authenticate")?;
    Ok(func())
}

pub unsafe fn cvr_read_content(u: u32) -> Result<i32, Box<dyn error::Error>> {
    let func: Symbol<unsafe extern fn(u32) -> i32> = DLL.get(b"CVR_Read_Content")?;
    Ok(func(u))
}

pub unsafe fn cvr_people_name(name: &u8, len: &u32) -> Result<i32, Box<dyn error::Error>> {
    let func: Symbol<unsafe extern fn(&u8, &u32) -> i32> = DLL.get(b"GetPeopleName")?;
    Ok(func(name,len))
}

pub unsafe fn cvr_people_code(name: &u8, len: &u32) -> Result<i32, Box<dyn error::Error>> {
    let func: Symbol<unsafe extern fn(&u8, &u32) -> i32> = DLL.get(b"GetPeopleIDCode")?;
    Ok(func(name,len))
}

/// 原    型：
//  		int CVR_CloseComm(void)
// 说    明：本函数用于关闭PC到阅读器的连接。
// 参    数：无
// 返 回 值：
// 值	意义
// 1	关闭成功
// 0	端口号不合法
// -1	端口已经关闭
// -2	动态库加载失败
pub unsafe fn cvr_close_comm() -> Result<i32, Box<dyn error::Error>> {
    let func: Symbol<unsafe extern fn() -> i32> = DLL.get(b"CVR_CloseComm")?;
    Ok(func())
}

