use crate::utils::common;
use regex::Regex;
use crate::model::gen_table_model::{GenTableAddPayload, GenTableColumnAddPayload};

/**
 * 代码生成通用常量
 *
 * @author ruoyi
 */

/** 单表（增删改查） */
#[allow(dead_code)]
pub const TPL_CRUD: &str = "crud";

/** 树表（增删改查） */
#[allow(dead_code)]
pub const TPL_TREE: &str = "tree";

/** 主子表（增删改查） */
#[allow(dead_code)]
pub const TPL_SUB: &str = "sub";

/** 树编码字段 */
#[allow(dead_code)]
pub const TREE_CODE: &str = "treeCode";

/** 树父编码字段 */
#[allow(dead_code)]
pub const TREE_PARENT_CODE: &str = "treeParentCode";

/** 树名称字段 */
#[allow(dead_code)]
pub const TREE_NAME: &str = "treeName";

/** 上级菜单ID字段 */
#[allow(dead_code)]
pub const PARENT_MENU_ID: &str = "parentMenuId";

/** 上级菜单名称字段 */
#[allow(dead_code)]
pub const PARENT_MENU_NAME: &str = "parentMenuName";

/** 数据库字符串类型 */
#[allow(dead_code)]
pub const COLUMNTYPE_STR: [&str;4] = [ "char", "varchar", "nvarchar", "varchar2" ];

/** 数据库文本类型 */
#[allow(dead_code)]
pub const COLUMNTYPE_TEXT: [&str;4] = [ "tinytext", "text", "mediumtext", "longtext" ];

/** 数据库时间类型 */
#[allow(dead_code)]
pub const COLUMNTYPE_TIME: [&str;4] = [ "datetime", "time", "date", "timestamp" ];

/** 数据库数字类型 */
#[allow(dead_code)]
pub const COLUMNTYPE_NUMBER: [&str;11] = [ "tinyint", "smallint", "mediumint", "int", "number",
    "integer", "bit", "bigint", "float", "double", "decimal" ];

/** 页面不需要编辑字段 */
#[allow(dead_code)]
pub const COLUMNNAME_NOT_EDIT: [&str;4] = [ "id", "create_by", "create_time", "del_flag" ];

/** 页面不需要显示的列表字段 */
#[allow(dead_code)]
pub const COLUMNNAME_NOT_LIST: [&str;6] = [ "id", "create_by", "create_time", "del_flag", "update_by",
"update_time" ];

/** 页面不需要查询字段 */
#[allow(dead_code)]
pub const COLUMNNAME_NOT_QUERY: [&str;7] = [ "id", "create_by", "create_time", "del_flag", "update_by",
"update_time", "remark" ];

/** Entity基类字段 */
#[allow(dead_code)]
pub const BASE_ENTITY: [&str;5] = [ "createBy", "createTime", "updateBy", "updateTime", "remark" ];

/** Tree基类字段 */
#[allow(dead_code)]
pub const TREE_ENTITY: [&str;5] = [ "parentName", "parentId", "orderNum", "ancestors", "children" ];

/** 文本框 */
#[allow(dead_code)]
pub const HTML_INPUT: &str = "input";

/** 文本域 */
#[allow(dead_code)]
pub const HTML_TEXTAREA: &str = "textarea";

/** 下拉框 */
#[allow(dead_code)]
pub const HTML_SELECT: &str = "select";

/** 单选框 */
#[allow(dead_code)]
pub const HTML_RADIO: &str = "radio";

/** 复选框 */
#[allow(dead_code)]
pub const HTML_CHECKBOX: &str = "checkbox";

/** 日期控件 */
#[allow(dead_code)]
pub const HTML_DATETIME: &str = "datetime";

/** 图片上传控件 */
#[allow(dead_code)]
pub const HTML_IMAGE_UPLOAD: &str = "imageUpload";

/** 文件上传控件 */
#[allow(dead_code)]
pub const HTML_FILE_UPLOAD: &str = "fileUpload";

/** 富文本控件 */
#[allow(dead_code)]
pub const HTML_EDITOR: &str = "editor";

/** 字符串类型 */
#[allow(dead_code)]
pub const TYPE_STRING: &str = "String";

/** 整型 */
#[allow(dead_code)]
pub const TYPE_INTEGER: &str = "i32";

/** 长整型 */
#[allow(dead_code)]
pub const TYPE_LONG: &str = "i64";

/** 浮点型 */
#[allow(dead_code)]
pub const TYPE_DOUBLE: &str = "f64";

/** 高精度计算类型 */
#[allow(dead_code)]
pub const TYPE_BIGDECIMAL: &str = "f64";

/** 时间类型 */
#[allow(dead_code)]
pub const TYPE_DATE: &str = "DateTime";

/** 模糊查询 */
#[allow(dead_code)]
pub const QUERY_LIKE: &str = "LIKE";

/** 相等查询 */
#[allow(dead_code)]
pub const QUERY_EQ: &str = "EQ";

/** 需要 */
#[allow(dead_code)]
pub const REQUIRE: &str = "1";

pub fn init_column_field(column: &mut GenTableColumnAddPayload, table: &GenTableAddPayload) {
    let column_type = column.column_type.clone().unwrap_or("".to_string());
    let column_name_lowercase = column.column_name.clone().unwrap_or("".to_string())
        .to_lowercase();
    let data_type = get_db_type(column_type.as_str());
    let column_name = column_name_lowercase.as_str();
    column.table_id = table.table_id.map(|v| v.to_string());
    column.create_by = table.create_by.clone();
    column.java_field = Some(common::to_pascal_case(column_name));
    // 设置默认字段类型
    column.java_type = Some("String".to_string());
    column.query_type = Some("EQ".to_string());

    if COLUMNTYPE_STR.contains(&data_type) || COLUMNTYPE_TEXT.contains(&data_type) {
        // 字符串长度超过500设置为文本域
        let column_length = get_column_length(&data_type);
        let html_type = match column_length >= 500 || COLUMNTYPE_TEXT.contains(&data_type) {
            true => HTML_TEXTAREA,
            false => HTML_INPUT,
        };
        column.html_type = Some(html_type.to_string());
    }
    else if COLUMNTYPE_TIME.contains(&data_type) {
        column.java_type = Some(TYPE_DATE.to_string());
        column.html_type = Some(HTML_DATETIME.to_string());
    }
    else if COLUMNTYPE_NUMBER.contains(&data_type) {
        column.html_type= Some(HTML_INPUT.to_string());

        // 如果是浮点型 统一用BigDecimal
        let num_str = common::sub_string_between(&data_type, "(", ")");
        let str: Vec<&str> = num_str.split( ",").collect();
        if str.len() != 0 && str.len() == 2 && str[1].parse::<i32>().unwrap_or(0) > 0 {
            column.java_type = Some(TYPE_BIGDECIMAL.to_string());
        }
        // 如果是整形
        else if str.len() != 0 && str.len() == 1 && str[0].parse::<i32>().unwrap_or(0) <= 10 {
            column.java_type = Some(TYPE_INTEGER.to_string());
        }
        // 长整形
        else
        {
            column.java_type = Some(TYPE_LONG.to_string());
        }
    }

    // 插入字段（默认所有字段都需要插入）
    column.is_insert = Some(REQUIRE.to_string());

    // 编辑字段
    if !COLUMNNAME_NOT_EDIT.contains(&column_name) && column.is_pk != Some("1".to_string()) {
        column.is_edit = Some(REQUIRE.to_string());
    }
    // 列表字段
    if !COLUMNNAME_NOT_LIST.contains(&column_name) && column.is_pk != Some("1".to_string()) {
        column.is_list= Some(REQUIRE.to_string());
    }
    // 查询字段
    if !COLUMNNAME_NOT_QUERY.contains(&column_name) && column.is_pk != Some("1".to_string()) {
        column.is_query = Some(REQUIRE.to_string());
    }

    // 查询字段类型
    if column_name.ends_with("name") {
        column.query_type = Some(QUERY_LIKE.to_string());
    }
    // 状态字段设置单选框
    if column_name.ends_with("status") {
        column.html_type = Some(HTML_RADIO.to_string());
    }
    // 类型&性别字段设置下拉框
    else if column_name.ends_with("type") || column_name.ends_with("sex") {
        column.html_type = Some(HTML_SELECT.to_string());
    }
    // 图片字段设置图片上传控件
    else if column_name.ends_with("image") {
        column.html_type = Some(HTML_IMAGE_UPLOAD.to_string());
    }
    // 文件字段设置文件上传控件
    else if column_name.ends_with("file") {
        column.html_type = Some(HTML_FILE_UPLOAD.to_string());
    }
    // 内容字段设置富文本控件
    else if column_name.ends_with("content") {
        column.html_type = Some(HTML_EDITOR.to_string());
    }
}

/**
 * 获取字段长度
 *
 * @param column_type 列类型
 * @return 截取后的列类型
 */
pub fn get_column_length(column_type: &str) -> usize {
    let start = column_type.find("(").unwrap_or(0);
    if start > 0 {
        let end = column_type.find(")").unwrap_or(0);
        if end > start {
            column_type[start + 1..end].parse::<usize>().unwrap_or(0)
        } else { 0 }
    }
    else { 0 }
}

/**
 * 获取数据库类型字段
 *
 * @param column_type 列类型
 * @return 截取后的列类型
 */
pub fn get_db_type(column_type: &str) -> &str
{
    let index = column_type.find("(").unwrap_or(0);
    if index > 0 {
        &column_type[..index]
    } else {
        column_type
    }
}

/**
 * 获取业务名
 *
 * @param tableName 表名
 * @return 业务名
 */
pub fn get_business_name(table_name: Option<String>) -> Option<String> {
    if let Some(table_name) = table_name {
        if let Some(index) = table_name.find("_") {
            Some(table_name[index + 1..table_name.len()].to_string())
        } else { None }
    } else { None }
}

/**
 * 关键字替换
 *
 * @param text 需要被替换的名字
 * @return 替换后的名字
 */
pub fn replace_text(text: Option<String>) -> Option<String> {
    if let Some(text) = text {
        let re = Regex::new(r"表|若依").unwrap();
        Some(re.replace(&text, "").to_string())
    } else { None }
}