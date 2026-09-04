import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessage } from "element-plus";
import request from "../utils/request.js";

export function useChangePassword() {
  const { t } = useI18n();

  const showChangePwd = ref(false);
  const changePwdForm = ref({ oldPassword: "", newPassword: "" });

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
    showChangePwd,
    changePwdForm,
    handleChangePwd,
  };
}
