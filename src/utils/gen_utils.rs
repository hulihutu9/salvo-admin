use crate::model::gen_table_model::{GenTableColumnList, GenTableList};
use crate::utils::common;
/**
 * 代码生成通用常量
 *
 * @author ruoyi
 */

/** 单表（增删改查） */
pub const TPL_CRUD: &str = "crud";

/** 树表（增删改查） */
pub const TPL_TREE: &str = "tree";

/** 主子表（增删改查） */
pub const TPL_SUB: &str = "sub";

/** 树编码字段 */
pub const TREE_CODE: &str = "treeCode";

/** 树父编码字段 */
pub const TREE_PARENT_CODE: &str = "treeParentCode";

/** 树名称字段 */
pub const TREE_NAME: &str = "treeName";

/** 上级菜单ID字段 */
pub const PARENT_MENU_ID: &str = "parentMenuId";

/** 上级菜单名称字段 */
pub const PARENT_MENU_NAME: &str = "parentMenuName";

/** 数据库字符串类型 */
pub const COLUMNTYPE_STR: [&str;4] = [ "char", "varchar", "nvarchar", "varchar2" ];

/** 数据库文本类型 */
pub const COLUMNTYPE_TEXT: [&str;4] = [ "tinytext", "text", "mediumtext", "longtext" ];

/** 数据库时间类型 */
pub const COLUMNTYPE_TIME: [&str;4] = [ "datetime", "time", "date", "timestamp" ];

/** 数据库数字类型 */
pub const COLUMNTYPE_NUMBER: [&str;11] = [ "tinyint", "smallint", "mediumint", "int", "number",
    "integer", "bit", "bigint", "float", "double", "decimal" ];

/** 页面不需要编辑字段 */
pub const COLUMNNAME_NOT_EDIT: [&str;4] = [ "id", "create_by", "create_time", "del_flag" ];

/** 页面不需要显示的列表字段 */
pub const COLUMNNAME_NOT_LIST: [&str;6] = [ "id", "create_by", "create_time", "del_flag", "update_by",
"update_time" ];

/** 页面不需要查询字段 */
pub const COLUMNNAME_NOT_QUERY: [&str;7] = [ "id", "create_by", "create_time", "del_flag", "update_by",
"update_time", "remark" ];

/** Entity基类字段 */
pub const BASE_ENTITY: [&str;5] = [ "createBy", "createTime", "updateBy", "updateTime", "remark" ];

/** Tree基类字段 */
pub const TREE_ENTITY: [&str;5] = [ "parentName", "parentId", "orderNum", "ancestors", "children" ];

/** 文本框 */
pub const HTML_INPUT: &str = "input";

/** 文本域 */
pub const HTML_TEXTAREA: &str = "textarea";

/** 下拉框 */
pub const HTML_SELECT: &str = "select";

/** 单选框 */
pub const HTML_RADIO: &str = "radio";

/** 复选框 */
pub const HTML_CHECKBOX: &str = "checkbox";

/** 日期控件 */
pub const HTML_DATETIME: &str = "datetime";

/** 图片上传控件 */
pub const HTML_IMAGE_UPLOAD: &str = "imageUpload";

/** 文件上传控件 */
pub const HTML_FILE_UPLOAD: &str = "fileUpload";

/** 富文本控件 */
pub const HTML_EDITOR: &str = "editor";

/** 字符串类型 */
pub const TYPE_STRING: &str = "String";

/** 整型 */
pub const TYPE_INTEGER: &str = "Integer";

/** 长整型 */
pub const TYPE_LONG: &str = "Long";

/** 浮点型 */
pub const TYPE_DOUBLE: &str = "Double";

/** 高精度计算类型 */
pub const TYPE_BIGDECIMAL: &str = "BigDecimal";

/** 时间类型 */
pub const TYPE_DATE: &str = "Date";

/** 模糊查询 */
pub const QUERY_LIKE: &str = "LIKE";

/** 相等查询 */
pub const QUERY_EQ: &str = "EQ";

/** 需要 */
pub const REQUIRE: &str = "1";

pub fn init_column_field(mut column: GenTableColumnList, table: GenTableList) {
    let data_type = get_db_type(column.column_type.unwrap().as_str());
    let column_name = column.column_name.unwrap().as_str();
    column.table_id = table.table_id.map(|v| v.to_string());
    column.create_by = table.create_by.clone();
    column.java_field = Some(common::to_pascal_case(column_name));
    // 设置java字段名
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
    } else if COLUMNTYPE_TIME.contains(&data_type) {
        column.java_type = Some(TYPE_DATE.to_string());
        column.html_type = Some(HTML_DATETIME.to_string());
    } else if COLUMNTYPE_NUMBER.contains(&data_type) {
        column.html_type= Some(HTML_INPUT.to_string());

        // 如果是浮点型 统一用BigDecimal
        String[] str = StringUtils.split(StringUtils.substringBetween(column.getColumnType(), "(", ")"), ",");
        if (str != null && str.length == 2 && Integer.parseInt(str[1]) > 0)
        {
            column.setJavaType(TYPE_BIGDECIMAL);
        }
        // 如果是整形
        else if (str != null && str.length == 1 && Integer.parseInt(str[0]) <= 10)
        {
            column.setJavaType(TYPE_INTEGER);
        }
        // 长整形
        else
        {
            column.setJavaType(TYPE_LONG);
        }
    }

    // 插入字段（默认所有字段都需要插入）
    column.is_insert = Some(REQUIRE.to_string());

    // 编辑字段
    if (!arraysContains(COLUMNNAME_NOT_EDIT, columnName) && !column.isPk())
    {
        column.setIsEdit(REQUIRE);
    }
    // 列表字段
    if (!arraysContains(COLUMNNAME_NOT_LIST, columnName) && !column.isPk())
    {
        column.setIsList(REQUIRE);
    }
    // 查询字段
    if (!arraysContains(COLUMNNAME_NOT_QUERY, columnName) && !column.isPk())
    {
        column.setIsQuery(REQUIRE);
    }

    // 查询字段类型
    if (StringUtils.endsWithIgnoreCase(columnName, "name"))
    {
        column.setQueryType(QUERY_LIKE);
    }
    // 状态字段设置单选框
    if (StringUtils.endsWithIgnoreCase(columnName, "status"))
    {
        column.setHtmlType(HTML_RADIO);
    }
    // 类型&性别字段设置下拉框
    else if (StringUtils.endsWithIgnoreCase(columnName, "type")
        || StringUtils.endsWithIgnoreCase(columnName, "sex"))
    {
        column.setHtmlType(HTML_SELECT);
    }
    // 图片字段设置图片上传控件
    else if (StringUtils.endsWithIgnoreCase(columnName, "image"))
    {
        column.setHtmlType(HTML_IMAGE_UPLOAD);
    }
    // 文件字段设置文件上传控件
    else if (StringUtils.endsWithIgnoreCase(columnName, "file"))
    {
        column.setHtmlType(HTML_FILE_UPLOAD);
    }
    // 内容字段设置富文本控件
    else if (StringUtils.endsWithIgnoreCase(columnName, "content"))
    {
        column.setHtmlType(HTML_EDITOR);
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
            end - start
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
    return if index > 0 {
        &column_type[..index]
    } else {
        column_type
    }
}