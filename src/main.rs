
mod default_mock;
use self::default_mock::Mock;

// use reqwest::header::HeaderMap;
// use serde_json::value::Value;

// async fn post() -> Result<HashMap<String, Value>, reqwest::Error>{
//     // post 请求要创建client
//     let client = reqwest::Client::new();

//     // 组装header
//     let mut headers = HeaderMap::new();
//     headers.insert("Content-Type", "application/json".parse().unwrap());

//     // 组装要提交的数据
//     let mut data = HashMap::new();
//     data.insert("user", "tangjz");
//     data.insert("password", "dev-tang.com");

//     // 发起post请求并返回
//     Ok(
//         client
//             .post("https://httpbin.org/post")
//             .headers(headers)
//             .json(&data)
//             .send().await?
//             .json::<HashMap<String, Value>>()
//             .await?
//     )
// }

#[tokio::main]
#[warn(unused_doc_comments)]
async fn main() {


     //1.根据表名，获取表结构，判断是否有 isValid
     // dataStatus、createTime、modifyTime 的映射是固定的
     //2.对于其他字段，判断名字里是否有 Time, 如果后缀是 Time, 默认固定格式
     // 3.如果 后缀是 Date，格式也写死
     //4.
    let tables = vec![
        "sy_cd_me_buss_std_dfbz_table"
    ];
    let m = Mock::table_id_map().await;
    for ele in tables {
        if let Some(value) = m.get(ele) {
            println!("正在处理 {}", ele);
            Mock::mock(value).await;
        } else {
            println!("没找到id， {}", ele);
        }
    }

    println!("Done");
    
    
}






// fn date_check<'a>(table_struc: &HashMap<&'a str, &str>, params:&mut HashMap<&'a str, &'a str>) {
//     for (column, _) in table_struc.iter().into_iter() {
//         if column.ends_with("Date") {
//             params.insert(&column, "");
//         }
//     }
// }

// fn time_check<'a>(table_struc: &HashMap<&'a str, &str>, params:&mut HashMap<&'a str, &'a str>) {
//     for (column, _) in table_struc.iter().into_iter() {
//         if column.ends_with("Time") {
//             params.insert(&column, "");
//         }
//     }
// }
// //isValid
// fn isValid_check<'a>(table_struc: &HashMap<&'a str, &str>, params:&mut HashMap<&'a str, &'a str>) {
//     for (column, _) in table_struc.iter().into_iter() {
//         if column.eq(&"isValid") {
//             params.insert(&column, "");
//         }
//     }
// }
// //url
// fn url_check<'a>(table_struc: &HashMap<&'a str, &str>, params:&mut HashMap<&'a str, &'a str>) {
//     for (column, _) in table_struc.iter().into_iter() {
//         if String::from(*column).to_lowercase().eq("url") {
//             params.insert(&column, "");
//         }
//     }
// }
// //数字
// fn sourceId_check<'a>(table_struc: &HashMap<&'a str, &str>, params:&mut HashMap<&'a str, &'a str>) {
//     for (column, _) in table_struc.iter().into_iter() {
//         if column.eq(&"sourceId") {
//             params.insert(&column, "");
//         }
//     }
// }

// fn comp_base_check<'a>(table_struc: &HashMap<&'a str, &str>, params:&mut HashMap<&'a str, &'a str>) {
//     for (column, _) in table_struc.iter().into_iter() {
//         if column.eq(&"compCode") {
//             params.insert(&column, "");
//         }
//         if column.eq(&"compName") {
//             params.insert(&column, "");
//         }
//         if column.eq(&"pubComNm") {
//             params.insert(&column, "");
//         }
//         if column.eq(&"creditCode") {
//             params.insert(&column, "");
//         }
//     }
// }

// fn uid_check<'a>(table_struc: &HashMap<&'a str, &str>, params:&mut HashMap<&'a str, &'a str>) {
//     for (column, _) in table_struc.iter().into_iter() {
//         if String::from(*column).to_lowercase().eq("uid") {
//             params.insert(&column, "");
//         }
//     }
// }
