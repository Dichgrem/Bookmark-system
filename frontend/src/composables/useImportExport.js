import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessage } from "element-plus";
import request from "../utils/request.js";

export function useImportExport(loadData) {
  const { t } = useI18n();

  const fileInput = ref(null);
  const isImportingAnim = ref(false);
  const isExportingAnim = ref(false);

  const exportBookmarks = () => {
    isExportingAnim.value = true;
    setTimeout(async () => {
      try {
        const res = await request.get("/bookmark/export", {
          responseType: "blob",
        });
        const blob = res.data;
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = "bookmarks.html";
        a.click();
        URL.revokeObjectURL(url);
      } catch {
        ElMessage.error(t("bookmark.exportFailed"));
      }
      isExportingAnim.value = false;
    }, 1500);
  };

  const triggerImport = () => {
    if (fileInput.value) {
      fileInput.value.accept = ".html";
      fileInput.value.value = "";
      fileInput.value.click();
    }
  };

  const handleFileUpload = async (event) => {
    const file = event.target.files[0];
    if (!file) return;
    const formData = new FormData();
    formData.append("file", file);
    isImportingAnim.value = true;
    try {
      const res = await request.post("/bookmark/import", formData, {
        timeout: 60000,
      });
      setTimeout(() => {
        isImportingAnim.value = false;
        if (res.data && res.data.code === 200) {
          ElMessage.success(
            t("bookmark.importSuccess", { count: res.data.data }),
          );
          loadData();
        } else ElMessage.error(res.data.msg || t("bookmark.importFailed"));
        if (fileInput.value) fileInput.value.value = "";
      }, 1500);
    } catch {
      setTimeout(() => {
        isImportingAnim.value = false;
        ElMessage.error(t("bookmark.importNetworkError"));
        if (fileInput.value) fileInput.value.value = "";
      }, 1500);
    }
  };

  return {
    fileInput,
    isImportingAnim,
    isExportingAnim,
    exportBookmarks,
    triggerImport,
    handleFileUpload,
  };
}
