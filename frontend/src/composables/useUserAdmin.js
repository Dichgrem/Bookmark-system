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
      ElMessage.error(t("bookmark.loadFailed"));
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
        ElMessage.success(t("userAdmin.addSuccess"));
        newUserForm.value = { username: "", password: "" };
        loadUsers();
      } else ElMessage.error(res.data.msg || t("userAdmin.addFailed"));
    } catch {
      ElMessage.error(t("userAdmin.addFailed"));
    }
  };

  const handleDeleteUser = async (userId, username) => {
    try {
      await ElMessageBox.confirm(
        t("userAdmin.deleteConfirm", { username }),
        t("userAdmin.deleteTitle"),
        {
          confirmButtonText: t("userAdmin.delete"),
          cancelButtonText: t("bookmark.cancel"),
          type: "warning",
        },
      );
      const res = await request.post("/user/delete", { id: userId });
      if (res.data.code === 200) {
        ElMessage.success(t("userAdmin.deleteSuccess"));
        loadUsers();
      } else ElMessage.error(res.data.msg || t("userAdmin.deleteFailed"));
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
        ElMessage.success(t("userAdmin.changePwdSuccess"));
        showChangePwd.value = false;
        changePwdForm.value = { oldPassword: "", newPassword: "" };
      } else ElMessage.error(res.data.msg || t("userAdmin.changePwdFailed"));
    } catch {
      ElMessage.error(t("userAdmin.changePwdFailed"));
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
