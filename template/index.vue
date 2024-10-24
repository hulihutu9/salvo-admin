<template>
  <div class="app-container">
    <el-form :model="queryParams" ref="queryRef" :inline="true" v-show="showSearch" label-width="68px">
{%- for column in columns -%}
{%- if column.isQuery -%}
{%- set dictType = column.dictType -%}
{%- set AttrName= column.javaField | capitalize -%}
{%- set commentArray = column.columnComment | split(pat="/") -%}
{%- set comment = commentArray | first -%}
{%- if column.htmlType == "input" %}
      <el-form-item label="{{comment}}" prop="{{column.javaField}}">
        <el-input
          v-model="queryParams.{{column.javaField}}"
          placeholder="请输入{{comment}}"
          clearable
          @keyup.enter="handleQuery"
        />
      </el-form-item>
{%- elif (column.htmlType == "select" or column.htmlType == "radio") and "" != dictType -%}
      <el-form-item label="{{comment}}" prop="{{column.javaField}}">
        <el-select v-model="queryParams.{{column.javaField}}" placeholder="请选择{{comment}}" clearable>
          <el-option
            v-for="dict in {{dictType}}"
            :key="dict.value"
            :label="dict.label"
            :value="dict.value"
          />
        </el-select>
      </el-form-item>
{%- elif (column.htmlType == "select" or column.htmlType == "radio") and dictType -%}
      <el-form-item label="{{comment}}" prop="{{column.javaField}}">
        <el-select v-model="queryParams.{{column.javaField}}" placeholder="请选择{{comment}}" clearable>
          <el-option label="请选择字典生成" value="" />
        </el-select>
      </el-form-item>
{%- elif column.htmlType == "datetime" and column.queryType != "BETWEEN" -%}
      <el-form-item label="{{comment}}" prop="{{column.javaField}}">
        <el-date-picker clearable
          v-model="queryParams.{{column.javaField}}"
          type="date"
          value-format="YYYY-MM-DD"
          placeholder="请选择{{comment}}">
        </el-date-picker>
      </el-form-item>
{%- elif column.htmlType == "datetime" and column.queryType == "BETWEEN" -%}
      <el-form-item label="{{comment}}" style="width: 308px">
        <el-date-picker
          v-model="daterange{{AttrName}}"
          value-format="YYYY-MM-DD"
          type="daterange"
          range-separator="-"
          start-placeholder="开始日期"
          end-placeholder="结束日期"
        ></el-date-picker>
      </el-form-item>
{%- endif -%}
{%- endif -%}
{%- endfor -%}
      <el-form-item>
        <el-button type="primary" icon="Search" @click="handleQuery">搜索</el-button>
        <el-button icon="Refresh" @click="resetQuery">重置</el-button>
      </el-form-item>
    </el-form>

    <el-row :gutter="10" class="mb8">
      <el-col :span="1.5">
        <el-button
          type="primary"
          plain
          icon="Plus"
          @click="handleAdd"
          v-hasPermi="['{{module_name}}:{{business_name}}:add']"
        >新增</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button
          type="success"
          plain
          icon="Edit"
          :disabled="single"
          @click="handleUpdate"
          v-hasPermi="['{{module_name}}:{{business_name}}:edit']"
        >修改</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button
          type="danger"
          plain
          icon="Delete"
          :disabled="multiple"
          @click="handleDelete"
          v-hasPermi="['{{module_name}}:{{business_name}}:remove']"
        >删除</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button
          type="warning"
          plain
          icon="Download"
          @click="handleExport"
          v-hasPermi="['{{module_name}}:{{business_name}}:export']"
        >导出</el-button>
      </el-col>
      <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>

    <el-table v-loading="loading" :data="{{business_name}}List" @selection-change="handleSelectionChange">
      <el-table-column type="selection" width="55" align="center" />
{%- for column in columns -%}
{%- set javaField = column.javaField -%}
{%- set commentArray = column.columnComment | split(pat="/") -%}
{%- set comment = commentArray | first -%}
{%- if column.isPk -%}
      <el-table-column label="{{comment}}" align="center" prop="{{javaField}}" />
{%- elif column.isList and column.htmlType == "datetime" -%}
      <el-table-column label="{{comment}}" align="center" prop="{{javaField}}" width="180">
        <template #default="scope">
          <span>{%- raw -%} {{ {%- endraw -%} parseTime(scope.row.{{javaField}}, '{y}-{m}-{d}') {%- raw -%} }} {%- endraw -%}</span>
        </template>
      </el-table-column>
{%- elif column.isList and column.htmlType == "imageUpload" -%}
      <el-table-column label="{{comment}}" align="center" prop="{{javaField}}" width="100">
        <template #default="scope">
          <image-preview :src="scope.row.{{javaField}}" :width="50" :height="50"/>
        </template>
      </el-table-column>
{%- elif column.isList and "" != column.dictType -%}
      <el-table-column label="{{comment}}" align="center" prop="{{javaField}}">
        <template #default="scope">
{%- if column.htmlType == "checkbox" -%}
          <dict-tag :options="{{column.dictType}}" :value="scope.row.{{javaField}} ? scope.row.{{javaField}}.split(',') : []"/>
{%- else -%}
          <dict-tag :options="{{column.dictType}}" :value="scope.row.{{javaField}}"/>
{%- endif -%}
        </template>
      </el-table-column>
{%- elif column.isList and "" != javaField -%}
      <el-table-column label="{{comment}}" align="center" prop="{{javaField}}" />
{%- endif -%}
{%- endfor -%}
      <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
        <template #default="scope">
          <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row)" v-hasPermi="['{{module_name}}:{{business_name}}:edit']">修改</el-button>
          <el-button link type="primary" icon="Delete" @click="handleDelete(scope.row)" v-hasPermi="['{{module_name}}:{{business_name}}:remove']">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
    
    <pagination
      v-show="total>0"
      :total="total"
      v-model:page="queryParams.pageNum"
      v-model:limit="queryParams.pageSize"
      @pagination="getList"
    />

    <!-- 添加或修改{{function_name}}对话框 -->
    <el-dialog :title="title" v-model="open" width="500px" append-to-body>
      <el-form ref="{{business_name}}Ref" :model="form" :rules="rules" label-width="80px">
{%- for column in columns -%}
{%- set field = column.javaField -%}
{%- if column.isInsert and not column.isPk -%}
{%- if column.usableColumn or not column.superColumn -%}
{%- set commentArray = column.columnComment | split(pat="/") -%}
{%- set comment = commentArray | first -%}
{%- set dictType = column.dictType -%}
{%- if column.htmlType == "input" -%}
        <el-form-item label="{{comment}}" prop="{{field}}">
          <el-input v-model="form.{{field}}" placeholder="请输入{{comment}}" />
        </el-form-item>
{%- elif column.htmlType == "imageUpload" -%}
        <el-form-item label="{{comment}}" prop="{{field}}">
          <image-upload v-model="form.{{field}}"/>
        </el-form-item>
{%- elif column.htmlType == "fileUpload" -%}
        <el-form-item label="{{comment}}" prop="{{field}}">
          <file-upload v-model="form.{{field}}"/>
        </el-form-item>
{%- elif column.htmlType == "editor" -%}
        <el-form-item label="{{comment}}">
          <editor v-model="form.{{field}}" :min-height="192"/>
        </el-form-item>
{%- elif column.htmlType == "select" and "" != dictType -%}
        <el-form-item label="{{comment}}" prop="{{field}}">
          <el-select v-model="form.{{field}}" placeholder="请选择{{comment}}">
            <el-option
              v-for="dict in {{dictType}}"
              :key="dict.value"
              :label="dict.label"
{%- if column.javaType == "i32" or column.javaType == "i64" -%}
              :value="parseInt(dict.value)"
{%- else -%}
              :value="dict.value"
{%- endif -%}
            ></el-option>
          </el-select>
        </el-form-item>
{%- elif column.htmlType == "select" and dictType -%}
        <el-form-item label="{{comment}}" prop="{{field}}">
          <el-select v-model="form.{{field}}" placeholder="请选择{{comment}}">
            <el-option label="请选择字典生成" value="" />
          </el-select>
        </el-form-item>
{%- elif column.htmlType == "checkbox" and "" != dictType -%}
        <el-form-item label="{{comment}}" prop="{{field}}">
          <el-checkbox-group v-model="form.{{field}}">
            <el-checkbox
              v-for="dict in {{dictType}}"
              :key="dict.value"
              :label="dict.value">
              {{dict.label}}
            </el-checkbox>
          </el-checkbox-group>
        </el-form-item>
{%- elif column.htmlType == "checkbox" and dictType -%}
        <el-form-item label="{{comment}}" prop="{{field}}">
          <el-checkbox-group v-model="form.{{field}}">
            <el-checkbox>请选择字典生成</el-checkbox>
          </el-checkbox-group>
        </el-form-item>
{%- elif column.htmlType == "radio" and "" != dictType -%}
        <el-form-item label="{{comment}}" prop="{{field}}">
          <el-radio-group v-model="form.{{field}}">
            <el-radio
              v-for="dict in {{dictType}}"
              :key="dict.value"
{%- if column.javaType == "i32" or column.javaType == "i64" -%}
              :label="parseInt(dict.value)"
{%- else -%}
              :label="dict.value"
{%- endif -%}
            >{{dict.label}}</el-radio>
          </el-radio-group>
        </el-form-item>
{%- elif column.htmlType == "radio" and dictType -%}
        <el-form-item label="{{comment}}" prop="{{field}}">
          <el-radio-group v-model="form.{{field}}">
            <el-radio label="1">请选择字典生成</el-radio>
          </el-radio-group>
        </el-form-item>
{%- elif column.htmlType == "datetime" -%}
        <el-form-item label="{{comment}}" prop="{{field}}">
          <el-date-picker clearable
            v-model="form.{{field}}"
            type="date"
            value-format="YYYY-MM-DD"
            placeholder="请选择{{comment}}">
          </el-date-picker>
        </el-form-item>
{%- elif column.htmlType == "textarea" -%}
        <el-form-item label="{{comment}}" prop="{{field}}">
          <el-input v-model="form.{{field}}" type="textarea" placeholder="请输入内容" />
        </el-form-item>
{%- endif -%}
{%- endif -%}
{%- endif -%}
{%- endfor -%}
{%- if table.sub -%}
        <el-divider content-position="center">{{subTable.functionName}}信息</el-divider>
        <el-row :gutter="10" class="mb8">
          <el-col :span="1.5">
            <el-button type="primary" icon="Plus" @click="handleAdd{{subClassName}}">添加</el-button>
          </el-col>
          <el-col :span="1.5">
            <el-button type="danger" icon="Delete" @click="handleDelete{{subClassName}}">删除</el-button>
          </el-col>
        </el-row>
        <el-table :data="{{subclassName}}List" :row-class-name="row{{subClassName}}Index" @selection-change="handle{{subClassName}}SelectionChange" ref="{{subclassName}}">
          <el-table-column type="selection" width="50" align="center" />
          <el-table-column label="序号" align="center" prop="index" width="50"/>
{%- for column in subTable.columns -%}
{%- set javaField = column.javaField -%}
{%- set commentArray = column.columnComment | split(pat="/") -%}
{%- set comment = commentArray | first -%}
{%- if column.isPk or javaField == subTableFkclassName -%}
{%- elif column.isList and column.htmlType == "input" -%}
          <el-table-column label="comment" prop="{{javaField}}" width="150">
            <template #default="scope">
              <el-input v-model="scope.row.javaField" placeholder="请输入comment" />
            </template>
          </el-table-column>
{%- elif column.isList and column.htmlType == "datetime" -%}
          <el-table-column label="comment" prop="{{javaField}}" width="240">
            <template #default="scope">
              <el-date-picker clearable
                v-model="scope.row.javaField"
                type="date"
                value-format="YYYY-MM-DD"
                placeholder="请选择comment">
              </el-date-picker>
            </template>
          </el-table-column>
{%- elif column.isList and (column.htmlType == "select" or column.htmlType == "radio") and "" != column.dictType -%}
          <el-table-column label="comment" prop="{{javaField}}" width="150">
            <template #default="scope">
              <el-select v-model="scope.row.javaField" placeholder="请选择comment">
                <el-option
                  v-for="dict in column.dictType"
                  :key="dict.value"
                  :label="dict.label"
                  :value="dict.value"
                ></el-option>
              </el-select>
            </template>
          </el-table-column>
{%- elif column.isList and (column.htmlType == "select" or column.htmlType == "radio") and "" == column.dictType -%}
          <el-table-column label="comment" prop="{{javaField}}" width="150">
            <template #default="scope">
              <el-select v-model="scope.row.javaField" placeholder="请选择comment">
                <el-option label="请选择字典生成" value="" />
              </el-select>
            </template>
          </el-table-column>
{%- endif -%}
{%- endfor -%}
        </el-table>
{%- endif -%}
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button type="primary" @click="submitForm">确 定</el-button>
          <el-button @click="cancel">取 消</el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup name="{{business_name}}">
import { list{{business_name}}, get{{business_name}}, del{{business_name}}, add{{business_name}}, update{{business_name}} } from "@/api/{{module_name}}/{{business_name}}";

const { proxy } = getCurrentInstance();
{%- if dicts != '' -%}
{%- set dictsNoSymbol = dicts | replace(from="'", to="") -%}
const { {{dictsNoSymbol}} } = proxy.useDict({{dicts}});
{%- endif -%}

const {{business_name}}List = ref([]);
{%- if table.sub -%}
const {{subclassName}}List = ref([]);
{%- endif -%}
const open = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref([]);
{%- if table.sub -%}
const checked{{subClassName}} = ref([]);
{%- endif -%}
const single = ref(true);
const multiple = ref(true);
const total = ref(0);
const title = ref("");
{%- for column in columns -%}
{%- if column.htmlType == "datetime" and column.queryType == "BETWEEN" -%}
{%- set AttrName= column.javaField | captilize -%}
const daterange{{AttrName}} = ref([]);
{%- endif -%}
{%- endfor -%}

const data = reactive({
  form: {},
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    {%- for column in columns -%}
{%- if column.isQuery -%}
    column.javaField: null{%- if loop.last -%},{%- endif -%}
{%- endif -%}
{%- endfor -%}
  },
  rules: {
{%- for column in columns -%}
{%- if column.isRequired -%}
{%- set commentArray = column.columnComment | split(pat="/") -%}
{%- set comment = commentArray | first -%}
    column.javaField: [
      { required: true, message: "comment不能为空", trigger: {%- if column.htmlType == "select" or column.htmlType == "radio" -%}"change"{%- else -%}"blur"{%- endif -%} }
    ]{%- if loop.last -%},{%- endif -%}
{%- endif -%}
{%- endfor -%}
  }
});

const { queryParams, form, rules } = toRefs(data);

/** 查询{{function_name}}列表 */
function getList() {
  loading.value = true;
{%- for column in columns -%}
{%- if column.htmlType == "datetime" and column.queryType == "BETWEEN" -%}
  queryParams.value.params = {};
#break
{%- endif -%}
{%- endfor -%}
{%- for column in columns -%}
{%- if column.htmlType == "datetime" and column.queryType == "BETWEEN" -%}
{%- set AttrName= column.javaField | captilize -%}
  if (null != daterange{{AttrName}} and '' != daterange{{AttrName}}) {
    queryParams.value.params["begin{{AttrName}}"] = daterange{{AttrName}}.value[0];
    queryParams.value.params["end{{AttrName}}"] = daterange{{AttrName}}.value[1];
  }
{%- endif -%}
{%- endfor -%}
  list{{business_name}}(queryParams.value).then(response => {
    {{business_name}}List.value = response.rows;
    total.value = response.total;
    loading.value = false;
  });
}

// 取消按钮
function cancel() {
  open.value = false;
  reset();
}

// 表单重置
function reset() {
  form.value = {
{%- for column in columns -%}
{%- if column.htmlType == "checkbox" -%}
    column.javaField: []{%- if loop.last -%},{%- endif -%}
{%- else -%}
    column.javaField: null{%- if loop.last -%},{%- endif -%}
{%- endif -%}
{%- endfor -%}
  };
{%- if table.sub -%}
  {{subclassName}}List.value = [];
{%- endif -%}
  proxy.resetForm("{{business_name}}Ref");
}

/** 搜索按钮操作 */
function handleQuery() {
  queryParams.value.pageNum = 1;
  getList();
}

/** 重置按钮操作 */
function resetQuery() {
{%- for column in columns -%}
{%- if column.htmlType == "datetime" and column.queryType == "BETWEEN" -%}
{%- set AttrName= column.javaField | captilize -%}
  daterange{{AttrName}}.value = [];
{%- endif -%}
{%- endfor -%}
  proxy.resetForm("queryRef");
  handleQuery();
}

// 多选框选中数据
function handleSelectionChange(selection) {
  ids.value = selection.map(item => item.{{pk_column.javaField}});
  single.value = selection.length != 1;
  multiple.value = !selection.length;
}

/** 新增按钮操作 */
function handleAdd() {
  reset();
  open.value = true;
  title.value = "添加{{function_name}}";
}

/** 修改按钮操作 */
function handleUpdate(row) {
  reset();
  const _{{pk_column.javaField}} = row.{{pk_column.javaField}} || ids.value
  get{{business_name}}(_{{pk_column.javaField}}).then(response => {
    form.value = response.data;
{%- for column in columns -%}
{%- if column.htmlType == "checkbox" -%}
    form.value.column.javaField = form.value.{{column.javaField}}.split(",");
{%- endif -%}
{%- endfor -%}
{%- if table.sub -%}
    {{subclassName}}List.value = response.data.{{subclassName}}List;
{%- endif -%}
    open.value = true;
    title.value = "修改{{function_name}}";
  });
}

/** 提交按钮 */
function submitForm() {
  proxy.#[[$]]#refs["{{business_name}}Ref"].validate(valid => {
    if (valid) {
{%- for column in columns -%}
{%- if column.htmlType == "checkbox" -%}
      form.value.column.javaField = form.value.{{column.javaField}}.join(",");
{%- endif -%}
{%- endfor -%}
{%- if table.sub -%}
      form.value.{{subclassName}}List = {{subclassName}}List.value;
{%- endif -%}
      if (form.value.{{pk_column.javaField}} != null) {
        update{{business_name}}(form.value).then(response => {
          proxy.#[[$modal]]#.msgSuccess("修改成功");
          open.value = false;
          getList();
        });
      } else {
        add{{business_name}}(form.value).then(response => {
          proxy.#[[$modal]]#.msgSuccess("新增成功");
          open.value = false;
          getList();
        });
      }
    }
  });
}

/** 删除按钮操作 */
function handleDelete(row) {
  const _{{pk_column.javaField}}s = row.{{pk_column.javaField}} || ids.value;
  proxy.#[[$modal]]#.confirm('是否确认删除{{function_name}}编号为"' + _{{pk_column.javaField}}s + '"的数据项？').then(function() {
    return del{{business_name}}(_{{pk_column.javaField}}s);
  }).then(() => {
    getList();
    proxy.#[[$modal]]#.msgSuccess("删除成功");
  }).catch(() => {});
}

{%- if table.sub -%}
/** {{subTable.functionName}}序号 */
function row{{subClassName}}Index({ row, rowIndex }) {
  row.index = rowIndex + 1;
}

/** {{subTable.functionName}}添加按钮操作 */
function handleAdd{{subClassName}}() {
  let obj = {};
{%- for column in subTable.columns -%}
{%- if column.isPk or column.javaField == subTableFkclassName -%}
{%- elif column.isList and "" != javaField -%}
  obj.column.javaField = "";
{%- endif -%}
{%- endfor -%}
  {{subclassName}}List.value.push(obj);
}

/** {{subTable.functionName}}删除按钮操作 */
function handleDelete{{subClassName}}() {
  if (checked{{subClassName}}.value.length == 0) {
    proxy.#[[$modal]]#.msgError("请先选择要删除的{{subTable.functionName}}数据");
  } else {
    const {{subclassName}}s = {{subclassName}}List.value;
    const checked{{subClassName}}s = checked{{subClassName}}.value;
    {{subclassName}}List.value = {{subclassName}}s.filter(function(item) {
      return checked{{subClassName}}s.indexOf(item.index) == -1
    });
  }
}

/** 复选框选中数据 */
function handle{{subClassName}}SelectionChange(selection) {
  checked{{subClassName}}.value = selection.map(item => item.index)
}

{%- endif -%}
/** 导出按钮操作 */
function handleExport() {
  proxy.download('{{module_name}}/{{business_name}}/export', {
    ...queryParams.value
  }, `{{business_name}}_#[[${new Date().getTime()}]]#.xlsx`)
}

getList();
</script>
