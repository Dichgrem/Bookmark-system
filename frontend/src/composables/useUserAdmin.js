import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessage, ElMessageBox } from "element-plus";
import request from "../utils/request.js";

export function useUserAdmin() {
  const { t } = useI18n();

  const showAddUser = ref(false);
  const showChangePwd = ref(false);
  const newUserForm = ref({ username: "", password: "" });
  const changePwdForm = ref({ oldPassword: "", newPassword: "" });
  const userList = ref([]);

  const loadUsers = async () => {
    try {
      const res = await request.get("/user/list");
      if (res.data.code === 200) userList.value = res.data.data;
    } catch {
      ElMessage.error(t("bookmark.importNetworkError"));
    }
  };

  const handleAddUser = async () => {
    if (!newUserForm.value.username || !newUserForm.value.password) {
      ElMessage.warning(t("login.usernameRequired"));
      return;
    }
    try {
      const res = await request.post("/user/create", newUserForm.value);
      if (res.data.code === 200) {
        ElMessage.success("用户创建成功");
        newUserForm.value = { username: "", password: "" };
        loadUsers();
      } else ElMessage.error(res.data.msg || "创建失败");
    } catch {
      ElMessage.error("创建失败");
    }
  };

  const handleDeleteUser = async (userId, username) => {
    try {
      await ElMessageBox.confirm(
        `确定要删除用户「${username}」吗？`,
        "删除用户",
        {
          confirmButtonText: "删除",
          cancelButtonText: "取消",
          type: "warning",
        },
      );
      const res = await request.post("/user/delete", { id: userId });
      if (res.data.code === 200) {
        ElMessage.success("用户已删除");
        loadUsers();
      } else ElMessage.error(res.data.msg || "删除失败");
    } catch {
      /* cancelled or error */
    }
  };

  const handleChangePwd = async () => {
    if (!changePwdForm.value.oldPassword || !changePwdForm.value.newPassword) {
      ElMessage.warning(t("login.usernameRequired"));
      return;
    }
    try {
      const res = await request.post("/user/changePassword", {
        oldPassword: changePwdForm.value.oldPassword,
        newPassword: changePwdForm.value.newPassword,
      });
      if (res.data.code === 200) {
        ElMessage.success("密码修改成功");
        showChangePwd.value = false;
        changePwdForm.value = { oldPassword: "", newPassword: "" };
      } else ElMessage.error(res.data.msg || "修改失败");
    } catch {
      ElMessage.error("修改失败");
    }
  };

  return {
    showAddUser,
    showChangePwd,
    newUserForm,
    changePwdForm,
    userList,
    loadUsers,
    handleAddUser,
    handleDeleteUser,
    handleChangePwd,
  };
}
