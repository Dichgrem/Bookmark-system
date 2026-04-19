<template>
  <div class="app">
    <div class="nav">📚 书签管理系统</div>

    <div v-if="!isLogin" class="login-box">
      <h3>登录 / 注册</h3>
      <el-input
        v-model="loginForm.username"
        placeholder="用户名"
        class="mb10"
      />
      <el-input
        v-model="loginForm.password"
        placeholder="密码"
        type="password"
        class="mb10"
      />
      <div class="btn-row">
        <el-button type="primary" @click="login">登录</el-button>
        <el-button @click="register">注册</el-button>
      </div>
    </div>

    <div v-else class="main">
      <div class="cate-add">
        <el-input
          v-model="cateName"
          placeholder="分类名称"
          style="width: 240px"
        />
        <el-button type="primary" @click="addCate">+ 添加分类</el-button>
      </div>

      <div class="cate-list">
        <div v-for="cate in cateList" :key="cate.id" class="cate-card">
          <div class="cate-header">
            <h4>{{ cate.name }}</h4>
            <div>
              <el-button size="small" @click="openEditCate(cate)"
                >编辑</el-button
              >
              <el-button size="small" type="danger" @click="delCate(cate.id)"
                >删除</el-button
              >
            </div>
          </div>

          <div class="book-add">
            <el-input
              v-model="bookTitle"
              placeholder="书签名"
              style="width: 140px"
            />
            <el-input
              v-model="bookUrl"
              placeholder="网址"
              style="width: 240px"
            />
            <el-button type="success" size="small" @click="addBook(cate.id)"
              >添加</el-button
            >
          </div>

          <div class="book-list">
            <div
              v-for="book in bookList.filter((b) => b.categoryId === cate.id)"
              :key="book.id"
              class="book-item"
            >
              <a :href="book.url" target="_blank">{{ book.title }}</a>
              <div>
                <el-button size="mini" @click="openEditBook(book)"
                  >编辑</el-button
                >
                <el-button size="mini" type="danger" @click="delBook(book.id)"
                  >删除</el-button
                >
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <el-dialog v-model="showEditCate" title="编辑分类">
      <el-input v-model="editCate.name" />
      <template #footer>
        <el-button @click="showEditCate = false">取消</el-button>
        <el-button type="primary" @click="saveEditCate">保存</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="showEditBook" title="编辑书签">
      <el-input v-model="editBook.title" class="mb10" />
      <el-input v-model="editBook.url" />
      <template #footer>
        <el-button @click="showEditBook = false">取消</el-button>
        <el-button type="primary" @click="saveEditBook">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref } from "vue";
import request from "./utils/request.js";
import { ElMessage } from "element-plus";

const isLogin = ref(false);
const loginForm = ref({ username: "", password: "" });
const cateList = ref([]);
const bookList = ref([]);
const cateName = ref("");
const bookTitle = ref("");
const bookUrl = ref("");

const showEditCate = ref(false);
const editCate = ref({});
const showEditBook = ref(false);
const editBook = ref({});

const login = async () => {
  const res = await request.post("/user/login", loginForm.value);
  if (res.data.code === 200) {
    isLogin.value = true;
    ElMessage.success("登录成功");
    loadData();
  } else {
    ElMessage.error("账号密码错误");
  }
};

const register = async () => {
  await request.post("/user/register", loginForm.value);
  ElMessage.success("注册成功");
};

const loadData = async () => {
  const c = await request.get("/category/list?userId=1");
  const b = await request.get("/bookmark/list?userId=1");
  cateList.value = c.data.data;
  bookList.value = b.data.data;
};

const addCate = async () => {
  await request.post("/category/add", { name: cateName.value, userId: 1 });
  cateName.value = "";
  loadData();
};

const delCate = async (id) => {
  await request.post("/category/delete", { id });
  ElMessage.success("分类已删除");
  loadData();
};

const openEditCate = (cate) => {
  editCate.value = { ...cate };
  showEditCate.value = true;
};

const saveEditCate = async () => {
  await request.post("/category/add", editCate.value);
  showEditCate.value = false;
  loadData();
};

const addBook = async (cid) => {
  await request.post("/bookmark/add", {
    title: bookTitle.value,
    url: bookUrl.value,
    categoryId: cid,
    userId: 1,
  });
  bookTitle.value = "";
  bookUrl.value = "";
  loadData();
};

const delBook = async (id) => {
  await request.post("/bookmark/delete", { id });
  ElMessage.success("书签已删除");
  loadData();
};

const openEditBook = (book) => {
  editBook.value = { ...book };
  showEditBook.value = true;
};

const saveEditBook = async () => {
  await request.post("/bookmark/add", editBook.value);
  showEditBook.value = false;
  loadData();
};
</script>

<style scoped>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
.app {
  background: #f5f7fa;
  min-height: 100vh;
}
.nav {
  background: #409eff;
  color: white;
  padding: 14px 30px;
  font-size: 18px;
}
.login-box {
  background: white;
  width: 400px;
  margin: 80px auto;
  padding: 30px;
  border-radius: 12px;
  box-shadow: 0 2px 12px #00000010;
}
.mb10 {
  margin-bottom: 10px;
}
.btn-row {
  display: flex;
  gap: 10px;
  justify-content: center;
  margin-top: 10px;
}
.main {
  max-width: 1000px;
  margin: 0 auto;
  padding: 20px;
}
.cate-add {
  background: white;
  padding: 16px 20px;
  border-radius: 8px;
  margin-bottom: 20px;
  display: flex;
  gap: 10px;
  align-items: center;
}
.cate-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.cate-card {
  background: white;
  padding: 20px;
  border-radius: 12px;
  box-shadow: 0 2px 8px #0000000c;
}
.cate-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}
.book-add {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
}
.book-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.book-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background: #f9f9f9;
  border-radius: 6px;
}
a {
  text-decoration: none;
  color: #409eff;
}
</style>
