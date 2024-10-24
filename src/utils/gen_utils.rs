use std::io::{Seek, Write};
use std::collections::{BTreeMap, HashSet};
use tera::{Tera, Context};
use crate::utils::common;
use regex::Regex;
use zip::write::FileOptions;
use zip::ZipWriter;
use crate::entity::gen_table_entity::{GenTableEntity, GenTableColumnEntity};
use crate::GLOBAL_DB;
use crate::model::gen_table_model::GenTableColumnList;
use crate::mapper::gen_table_mapper;

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

pub fn init_column_field(column: &mut GenTableColumnEntity, table: &GenTableEntity) {
    let column_type = column.column_type.clone().unwrap_or("".to_string());
    let column_name_lowercase = column.column_name.clone().unwrap_or("".to_string())
        .to_lowercase();
    let data_type = get_db_type(column_type.as_str());
    let column_name = column_name_lowercase.as_str();
    column.table_id = table.table_id.map(|v| v.to_string());
    column.create_by = table.create_by.clone();
    column.java_field = Some(common::to_camel_case(column_name));
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
        else if data_type == "bigint" {
            column.java_type = Some(TYPE_LONG.to_string());
        }
        // 长整形
        else
        {
            column.java_type = Some(TYPE_INTEGER.to_string());
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

// get template
pub fn get_template_list() -> Vec<String> {
    let path = "";

    vec!["README.md", "entity.rs", "model.rs", "router.rs", "controller.rs",
         "service.rs", "mapper.rs", "xml.html", "index.vue"]
        .iter().map(|s| path.to_owned() + s).collect::<Vec<String>>()
}

pub async fn init_context(id: String) -> Context {
    let tables = gen_table_mapper::get_gen_table_by_id(
        &mut GLOBAL_DB.clone(),id.clone()).await.unwrap();
    let table = tables.get(0).unwrap();

    // get sub table info
    let sub_table_name = table.sub_table_name.clone();
    let sub_tables: Vec<GenTableEntity> = match sub_table_name.clone() {
        Some(table_name) => gen_table_mapper::get_gen_table_by_name(
            &mut GLOBAL_DB.clone(),table_name).await.unwrap(),
        None => vec![],
    };
    let sub_table = sub_tables.get(0).cloned();
    let sub_class_name = match sub_table {
        Some(ref table) => table.class_name.clone().unwrap_or("".to_string()),
        None => "".to_string(),
    };
    let having_sub_table = match sub_table {
        Some(_) => true,
        None => false,
    };

    // get primary key column info
    let mut pk_column: Option<&GenTableColumnList> = None;
    let columns = gen_table_mapper::get_gen_table_column_by_id(
        &mut GLOBAL_DB.clone(),id).await.unwrap();
    for column in columns.iter() {
        if column.is_pk == Some("1".to_string()) {
            pk_column = Some(column);
            break;
        }
    }

    // get sub_table columns
    let sub_columns_list = match sub_table_name.clone() {
        Some(table_name) => gen_table_mapper::get_db_table_columns_by_name(
            &mut GLOBAL_DB.clone(), table_name.clone()).await.unwrap(),
        None => vec![],
    };
    let sub_columns: Vec<GenTableColumnList> =
        sub_columns_list.iter().map(|c| (*c).clone().into()).collect();

    let mut context = Context::new();
    // e.g.: sys_post
    context.insert("table_name".to_string(), &table.table_name.clone().unwrap());
    // e.g.: SysPost
    context.insert("class_name".to_string(), &table.class_name.clone().unwrap());
    // e.g.: system
    context.insert("module_name".to_string(), &table.module_name.clone().unwrap());
    // e.g.: post
    context.insert("business_name".to_string(), &table.business_name.clone().unwrap());
    // e.g.: 岗位信息
    context.insert("function_name".to_string(), &table.function_name.clone().unwrap());
    // e.g.: columns.column_id,
    context.insert("columns".to_string(), &columns);
    // if having sub_table,
    context.insert("having_sub_table".to_string(), &having_sub_table);
    // e.g.: subTable,
    context.insert("subTable".to_string(), &sub_table);
    // e.g.: subclassName,
    context.insert("subclassName".to_string(), &sub_class_name.clone());
    // e.g.: pk_column,
    context.insert("pk_column".to_string(), &pk_column);
    // table and sub_table dict
    let dicts = get_dicts(columns, sub_columns).await;
    context.insert("dicts".to_string(), &dicts);

    context
}

// render template list
pub fn render_template(
    context: Context, list: Vec<String>
) -> BTreeMap<String, String> {
    let tera = match Tera::new("template/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };

    let mut res = BTreeMap::new();
    for file_name in list.iter() {
        let output = tera.render(file_name, &context).unwrap();
        res.insert(file_name.to_string(), output);
    }
    res
}


/// 在内存中生成zip文件
/// 一般在做web开发时，都喜欢在内存中动态生成报表多，然后直接打包成一个
pub fn compress<T>(zip: &mut ZipWriter<T>, file_name: &str, b: &[u8]) -> zip::result::ZipResult<()>
where T: Write + Seek
{
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Bzip2) //直接用了bzip2压缩方式，其它参看枚举
        .unix_permissions(0o755); //unix系统权限
    zip.start_file(file_name, options)?;
    zip.write_all(b)?;
    Ok(())
}

pub fn generate_zip_file(render_files: BTreeMap<String, String>) {
    let buf=vec![];
    // A Cursor wraps an in-memory buffer and provides it with a Seek implementation.
    let writer = std::io::Cursor::new(buf);
    let mut zip = ZipWriter::new(writer);

    for (file_name, file) in render_files.iter() {
        compress(&mut zip, file_name, file.as_bytes()).unwrap();
    }
    //到这一步就转成生成的字节了
    let writer= zip.finish().unwrap();
}

pub fn is_not_empty_column(field: Option<String>) -> bool {
    match field {
        Some(field) => {
            if field.is_empty() {
                false
            } else {
                true
            }
        }
        None => false
    }
}

pub fn is_super_column(java_field: Option<String>) -> bool {
    match java_field {
        Some(field) => {
            ["createBy", "createTime", "updateBy", "updateTime", "remark",  // BaseEntity
                "parentName", "parentId", "orderNum", "ancestors"  // TreeEntity
            ].contains(&field.as_str())
        }
        None => false
    }
}

/**
 * 添加字典列表
 *
 * @param dicts 字典列表
 * @param columns 列集合
 */
pub fn add_dicts(dicts: &mut HashSet<String> , columns: Vec<GenTableColumnList>) {
    for column in columns {
        if !is_super_column(column.java_field) &&
            is_not_empty_column(column.dict_type.clone()) &&
            [HTML_SELECT, HTML_RADIO, HTML_CHECKBOX].contains(&column.html_type.unwrap().as_str()) {
            dicts.insert("'".to_string() + &column.dict_type.unwrap() + "'");
        }
    }
}

/**
 * 根据列类型获取字典组
 *
 * @param genTable 业务表对象
 * @return 返回字典组
 */
pub async fn get_dicts(
    columns: Vec<GenTableColumnList>, sub_columns: Vec<GenTableColumnList>
) -> String {
    let mut dicts: HashSet<String> = HashSet::new();
    add_dicts(&mut dicts, columns);
    add_dicts(&mut dicts, sub_columns);

    let dict_str: Vec<String> = dicts.iter().map(|s| s.to_string()).collect();
    dict_str.join(", ")
}


