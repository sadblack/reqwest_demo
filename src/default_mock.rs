
/*
创建一个 struc，表示表结构，从 接口里解析出来

创建一个 struc，表示响应体

其他的，根据字段类型，给默认值
*/

use std::collections::HashMap;
use serde::{Deserialize, Serialize, de::value};


#[derive(Debug, Serialize, Deserialize)]
struct RequestEntity {

    code: String,
    msg: String,
    time: String,
    data: Option<DataList>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataList {

    data_list: Vec<Data>,
    // total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {

    id: i32,
    #[serde(rename = "dbId")]
    db_id: Option<i32>,
    #[serde(rename = "dbSize")]
    db_size: Option<i32>,
    flag: Option<i32>,
    #[serde(rename = "tableType")]
    table_type: Option<i32>,
    table: Option<String>,
    #[serde(rename = "tableName")]
    table_name: Option<String>,
    // #[serde(rename = "dbName")]
    // db_name: Option<String>,
    // #[serde(rename = "dbPosition")]
    // db_position: Option<String>,
    // #[serde(rename = "dbUrl")]
    // db_url: Option<String>,
    // #[serde(rename = "tableSql")]
    // table_sql: Option<String>,

}



pub struct Mock;

impl Mock {
    


    pub async fn table_id_map() -> HashMap<String, i32> {
        let params = [("pageSize", "2000"), ("pageNum", "1")];
        let client = reqwest::Client::new();
        let res = client.post("http://192.168.1.26:8082/mock/tableManager/query")
            .form(&params)
            .send()
            .await.unwrap()
            .json::<RequestEntity>()
            .await.unwrap();
    
        let mut table_id_map: HashMap<String, i32> = HashMap::new();
    
        if let Some(data_list) = res.data {
            for element in data_list.data_list {
                if let Some(table) = element.table {
                    table_id_map.insert(table, element.id);
                }
            }
        }
        
        table_id_map
    }

    //根据id 获取表结构
    pub async fn mock(table_id: &i32) {

        let params = [("pageSize", "100"), ("pageNum", "1"),("mockTableId", &table_id.to_string())];
        let client = reqwest::Client::new();
        let res = client.post("http://192.168.1.26:8082/mock/tableFieldManager/query")
            .form(&params)
            .send()
            .await.unwrap()
            .json::<TableEntityRequest>()
            .await.unwrap();

        if let Some(data_list) = res.data {
            for element in data_list {
                if !element.field_name.eq("id") {
                    if let Ok(_) = date_check(&element, &client).await {
                        continue;
                    }
                    if let Ok(_) = time_check(&element, &client).await {
                        continue;
                    }
                    if let Ok(_) = dataStatus_check(&element, &client).await {
                        continue;
                    }
                    if let Ok(_) = valid_check(&element, &client).await {
                        continue;
                    }
                    if let Ok(_) = url_check(&element, &client).await {
                        continue;
                    }
                    if let Ok(_) = sourceId_check(&element, &client).await {
                        continue;
                    }
                    if let Ok(_) = compCode_check(&element, &client).await {
                        continue;
                    }
                    if let Ok(_) = compName_check(&element, &client).await {
                        continue;
                    }
                    if let Ok(_) = pubComNm_check(&element, &client).await {
                        continue;
                    }
                    if let Ok(_) = creditCode_check(&element, &client).await {
                        continue;
                    }
                    if let Ok(_) = uid_check(&element, &client).await {
                        continue;
                    }
                    if let Ok(_) = fingerId_check(&element, &client).await {
                        continue;
                    }
                    if let Ok(_) = person_check(&element, &client).await {
                        continue;
                    }
                    if let Ok(_) = varchar_check(&element, &client).await {
                        continue;
                    }
                    if let Ok(_) = int_check(&element, &client).await {
                        continue;
                    }
                    //根据变量类型确定

                    let _ = default(&element, &client).await;
                }
            }
        }
        
        
    }
    
    // fn mock(table_name: &str) {
    
    
        
    // }

}


async fn date_check<'a>(column_desc: &TableQueryDetail, client: &reqwest::Client) -> Result<(),String> {
    
    if column_desc.field_name.ends_with("Date") {
        
        let mut result: TableUpdateDetail = TableUpdateDetail::from(column_desc);

        result.mock_type = Option::Some("日期".to_string());
        result.mock_expression = Option::Some("@date(\"yyyy-MM-dd\")".to_string());
        
        update(result, client).await;
    } else {
        return Err("不匹配".to_string());
    }
    Ok(())
}

async fn time_check<'a>(column_desc: &TableQueryDetail, client: &reqwest::Client) -> Result<(),String> {
    
    if column_desc.field_name.ends_with("Time") {
        
        let mut result: TableUpdateDetail = TableUpdateDetail::from(column_desc);

        result.mock_type = Option::Some("日期".to_string());
        result.mock_expression = Option::Some("@datetime(\"yyyy-MM-dd HH:mm:ss\")".to_string());
        
        update(result, client).await;
    } else {
        return Err("不匹配".to_string());
    }
    Ok(())
}

async fn valid_check<'a>(column_desc: &TableQueryDetail, client: &reqwest::Client) -> Result<(),String> {
    
    if column_desc.field_name.eq("isValid") {
        
        let mut result: TableUpdateDetail = TableUpdateDetail::from(column_desc);

        result.mock_type = Option::Some("枚举".to_string());
        result.mock_expression = Option::Some("0,1".to_string());
        
        update(result, client).await;
    } else {
        return Err("不匹配".to_string());
    }
    Ok(())
}

async fn url_check<'a>(column_desc: &TableQueryDetail, client: &reqwest::Client) -> Result<(),String> {
    
    if column_desc.field_name.to_lowercase().eq("url") {
        
        let mut result: TableUpdateDetail = TableUpdateDetail::from(column_desc);

        result.mock_type = Option::Some("Web".to_string());
        result.mock_expression = Option::Some("@url()".to_string());
        
        update(result, client).await;
    } else {
        return Err("不匹配".to_string());
    }
    Ok(())
}

async fn dataStatus_check<'a>(column_desc: &TableQueryDetail, client: &reqwest::Client) -> Result<(),String> {
    
    if column_desc.field_name.eq("dataStatus") {
        
        let mut result: TableUpdateDetail = TableUpdateDetail::from(column_desc);

        result.mock_type = Option::Some("枚举".to_string());
        result.mock_expression = Option::Some("1,2,3".to_string());
        
        update(result, client).await;
    } else {
        return Err("不匹配".to_string());
    }
    Ok(())
}

async fn sourceId_check<'a>(column_desc: &TableQueryDetail, client: &reqwest::Client) -> Result<(),String> {
    
    if column_desc.field_name.eq("sourceId") {
        
        let mut result: TableUpdateDetail = TableUpdateDetail::from(column_desc);

        result.mock_type = Option::Some("Web".to_string());
        result.mock_expression = Option::Some("@guid()".to_string());
        
        update(result, client).await;
    } else {
        return Err("不匹配".to_string());
    }
    Ok(())
}

async fn compCode_check<'a>(column_desc: &TableQueryDetail, client: &reqwest::Client) -> Result<(),String> {
    
    if column_desc.field_name.to_lowercase().eq("compcode") {
        
        let mut result: TableUpdateDetail = TableUpdateDetail::from(column_desc);

        result.mock_type = Option::Some("外键".to_string());
        result.db_id = Option::Some("14".to_string());
        result.query_sql = Option::Some("select compCode from sy_cd_ms_base_comp_info".to_string());
        
        update(result, client).await;
    } else {
        return Err("不匹配".to_string());
    }
    Ok(())
}

async fn compName_check<'a>(column_desc: &TableQueryDetail, client: &reqwest::Client) -> Result<(),String> {
    
    if column_desc.field_name.to_lowercase().eq("compname") {
        
        let mut result: TableUpdateDetail = TableUpdateDetail::from(column_desc);

        result.mock_type = Option::Some("外键".to_string());
        result.db_id = Option::Some("14".to_string());
        result.dependent_fields = Option::Some("compCode".to_string());
        result.query_sql = Option::Some("select compCode, compName from sy_cd_ms_base_comp_info where compCode in (#{compCode})".to_string());
        
        update(result, client).await;
    } else {
        return Err("不匹配".to_string());
    }
    Ok(())
}

async fn pubComNm_check<'a>(column_desc: &TableQueryDetail, client: &reqwest::Client) -> Result<(),String> {
    
    if column_desc.field_name.to_lowercase().eq("pubcomnm") {
        
        let mut result: TableUpdateDetail = TableUpdateDetail::from(column_desc);

        result.mock_type = Option::Some("外键".to_string());
        result.db_id = Option::Some("14".to_string());
        result.dependent_fields = Option::Some("compCode".to_string());
        result.query_sql = Option::Some("select compCode, compName as pubComNm from sy_cd_ms_base_comp_info where compCode in (#{compCode})".to_string());
        
        update(result, client).await;
    } else {
        return Err("不匹配".to_string());
    }
    Ok(())
}

async fn creditCode_check<'a>(column_desc: &TableQueryDetail, client: &reqwest::Client) -> Result<(),String> {
    
    if column_desc.field_name.to_lowercase().eq("creditcode") {
        
        let mut result: TableUpdateDetail = TableUpdateDetail::from(column_desc);

        result.mock_type = Option::Some("外键".to_string());
        result.db_id = Option::Some("14".to_string());
        result.dependent_fields = Option::Some("compCode".to_string());
        result.query_sql = Option::Some("select compCode, creditCode from sy_cd_ms_base_comp_info where compCode in (#{compCode})".to_string());
        
        update(result, client).await;
    } else {
        return Err("不匹配".to_string());
    }
    Ok(())
}

async fn uid_check<'a>(column_desc: &TableQueryDetail, client: &reqwest::Client) -> Result<(),String> {
    
    if column_desc.field_name.to_lowercase().eq("uid") {

        let mut result: TableUpdateDetail = TableUpdateDetail::from(column_desc);

        result.mock_type = Option::Some("Web".to_string());
        result.mock_expression = Option::Some("@guid()".to_string());
        
        update(result, client).await;
    } else {
        return Err("不匹配".to_string());
    }
    Ok(())
}

async fn fingerId_check<'a>(column_desc: &TableQueryDetail, client: &reqwest::Client) -> Result<(),String> {
    
    if column_desc.field_name.to_lowercase().eq("fingerid") {

        let mut result: TableUpdateDetail = TableUpdateDetail::from(column_desc);

        result.mock_type = Option::Some("Web".to_string());
        result.mock_expression = Option::Some("@guid()".to_string());
        
        update(result, client).await;
    } else {
        return Err("不匹配".to_string());
    }
    Ok(())
}

async fn person_check<'a>(column_desc: &TableQueryDetail, client: &reqwest::Client) -> Result<(),String> {

    match &column_desc.field_notes {
        Some(value) => {
            if value.ends_with("人") {
                let mut result: TableUpdateDetail = TableUpdateDetail::from(column_desc);
        
                result.mock_type = Option::Some("文本".to_string());
                result.mock_expression = Option::Some("@cname()".to_string());
                
                update(result, client).await;
            } else {
                return Err("不匹配".to_string());
            }
        },
        None => {
            return Err("不匹配".to_string());
        },
    }
    Ok(())
}

async fn varchar_check<'a>(column_desc: &TableQueryDetail, client: &reqwest::Client) -> Result<(),String> {
    
    let f_type = &column_desc.field_type;
    if f_type.eq("varchar") || f_type.eq("text") {

        let mut result: TableUpdateDetail = TableUpdateDetail::from(column_desc);

        result.mock_type = Option::Some("文本".to_string());
        result.mock_expression = Option::Some("@string(7, 10)".to_string());
            
        update(result, client).await;
    } else {
        return Err("不匹配".to_string());
    }
    Ok(())
}

async fn int_check<'a>(column_desc: &TableQueryDetail, client: &reqwest::Client) -> Result<(),String> {
    
    let f_type = &column_desc.field_type;
    if f_type.eq("int") || f_type.eq("bigint") {

        let mut result: TableUpdateDetail = TableUpdateDetail::from(column_desc);

        result.mock_type = Option::Some("数值".to_string());
        result.mock_expression = Option::Some("@integer(1, 10)".to_string());
        
        update(result, client).await;
    } else {
        return Err("不匹配".to_string());
    }
    Ok(())
}



async fn default<'a>(column_desc: &TableQueryDetail, client: &reqwest::Client) -> Result<(),String> {
    
    let mut result: TableUpdateDetail = TableUpdateDetail::from(column_desc);

    result.mock_type = Option::Some("文本".to_string());
    result.mock_expression = Option::Some("@string(7, 10)".to_string());
        
    update(result, client).await;
    Ok(())
}



#[derive(Debug, Serialize, Deserialize)]
struct TableEntityRequest {

    code: String,
    msg: String,
    time: String,
    data: Option<Vec<TableQueryDetail>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TableQueryDetail {

    id: i32,
    field_name: String,
    field_type: String,
    field_cn_name: Option<String>,
    field_notes: Option<String>,
    table: Option<String>,
    // #[serde(rename = "dbName")]
    // db_name: Option<String>,
    // #[serde(rename = "dbPosition")]
    // db_position: Option<String>,
    // #[serde(rename = "dbUrl")]
    // db_url: Option<String>,
    // #[serde(rename = "tableSql")]
    // table_sql: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
struct TableUpdateDetail {

    id: i32,
    field_name: String,
    field_type: String,
    #[serde(rename = "fieldCnName")]
    field_cn_name: Option<String>,
    #[serde(rename = "fieldNotes")]
    field_notes: Option<String>,
    #[serde(rename = "mockType")]
    mock_type: Option<String>,
    #[serde(rename = "mockExpression")]
    mock_expression: Option<String>,
    #[serde(rename = "dbId")]
    db_id: Option<String>,
    #[serde(rename = "dependentFields")]
    dependent_fields: Option<String>,
    #[serde(rename = "querySql")]
    query_sql: Option<String>,

    // #[serde(rename = "dbName")]
    // db_name: Option<String>,
    // #[serde(rename = "dbPosition")]
    // db_position: Option<String>,
    // #[serde(rename = "dbUrl")]
    // db_url: Option<String>,
    // #[serde(rename = "tableSql")]
    // table_sql: Option<String>,

}

impl TableUpdateDetail {

    fn from(column_desc: &TableQueryDetail) -> Self {
        TableUpdateDetail {
            id:                 column_desc.id,
            field_name:         column_desc.field_name.clone(),
            field_type:         column_desc.field_type.clone(),
            field_cn_name:      column_desc.field_cn_name.clone(),
            field_notes:        column_desc.field_notes.clone(),
            mock_type:          Option::None,
            mock_expression:    Option::None,
            db_id:              Option::None,
            dependent_fields:   Option::None,
            query_sql:          Option::None,
        }
    }
}

async fn update(result: TableUpdateDetail, client: &reqwest::Client) {
    let _ = client.post("http://192.168.1.26:8082/mock/tableFieldManager/update")
    .form(&result)
    .send()
    .await.unwrap()
    .json::<TableEntityRequest>()
    .await.unwrap();
}

