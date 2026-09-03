<template>
  <Login :loading="submitting" @login="handleLogin" />
</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { ElMessage } from "element-plus";
import Login from "../components/Login.vue";
import request from "../utils/request.js";
import { LS_TOKEN, LS_USER } from "../utils/constants.js";

const { t } = useI18n();
const router = useRouter();
const submitting = ref(false);

const handleLogin = async (payload) => {
  submitting.value = true;
  try {
    const res = await request.post("/user/login", payload);
    if (res.data.code === 200) {
      localStorage.setItem(LS_TOKEN, res.data.data.token);
      localStorage.setItem(
        LS_USER,
        JSON.stringify({
          id: res.data.data.id,
          username: res.data.data.username,
          role: res.data.data.role,
        }),
      );
      ElMessage.success(t("login.loginSuccess"));
      router.push({ name: "Home" });
    } else {
      ElMessage.error(res.data.msg || t("login.loginFailed"));
    }
  } catch {
    ElMessage.error(t("login.loginFailed"));
  } finally {
    submitting.value = false;
  }
};
</script>
