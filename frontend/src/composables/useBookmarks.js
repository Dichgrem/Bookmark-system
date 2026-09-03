import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessage } from "element-plus";
import request from "../utils/request.js";

export function useBookmarks() {
  const { t } = useI18n();

  const bookList = ref([]);
  const cateList = ref([]);

  const showEditBook = ref(false);
  const editBook = ref({});

  const showEditCate = ref(false);
  const editCate = ref({});

  const showAddSubCate = ref(false);
  const addSubParentId = ref(null);
  const newSubCateName = ref("");

  const loadData = async () => {
    try {
      const [c, b] = await Promise.all([
        request.get("/category/list"),
        request.get("/bookmark/list"),
      ]);
      cateList.value = c.data.data || [];
      bookList.value = b.data.data || [];
    } catch {
      ElMessage.error(t("bookmark.loadFailed"));
    }
  };

  const openEditBook = (book) => {
    editBook.value = { ...book };
    showEditBook.value = true;
  };

  const saveEditBook = async (form) => {
    try {
      await request.post("/bookmark/add", { ...form });
      showEditBook.value = false;
      loadData();
    } catch {
      ElMessage.error(t("bookmark.saveFailed"));
    }
  };

  const addCate = (parentId) => {
    editCate.value = { parentId, name: "", sortOrder: 0 };
    showEditCate.value = true;
  };

  const openEditCate = (data) => {
    editCate.value = { ...data };
    showEditCate.value = true;
  };

  const saveEditCate = async () => {
    try {
      await request.post("/category/add", editCate.value);
      showEditCate.value = false;
      loadData();
    } catch {
      ElMessage.error(t("bookmark.saveFailed"));
    }
  };

  const openAddSubCate = (parentId) => {
    addSubParentId.value = parentId;
    newSubCateName.value = "";
    showAddSubCate.value = true;
  };

  const saveAddSubCate = async () => {
    if (!newSubCateName.value.trim()) {
      ElMessage.warning(t("category.nameRequired"));
      return;
    }
    try {
      await request.post("/category/add", {
        name: newSubCateName.value,
        parentId: addSubParentId.value,
        sortOrder: 0,
      });
      showAddSubCate.value = false;
      loadData();
    } catch {
      ElMessage.error(t("bookmark.saveFailed"));
    }
  };

  const fetchIcons = async () => {
    try {
      const res = await request.post("/bookmark/fetchIcons");
      if (res.data.code === 200) {
        ElMessage.success(
          t("bookmark.fetchIconSuccess", { count: res.data.data }),
        );
        loadData();
      } else {
        ElMessage.error(res.data.msg || t("bookmark.fetchIconFailed"));
      }
    } catch {
      ElMessage.error(t("bookmark.fetchIconFailed"));
    }
  };

  const handleAddMenu = (type) => {
    if (type === "bookmark") {
      editBook.value = {};
      showEditBook.value = true;
    } else {
      addCate(null);
    }
  };

  return {
    bookList,
    cateList,
    loadData,
    showEditBook,
    editBook,
    openEditBook,
    saveEditBook,
    showEditCate,
    editCate,
    openEditCate,
    saveEditCate,
    addCate,
    showAddSubCate,
    addSubParentId,
    newSubCateName,
    openAddSubCate,
    saveAddSubCate,
    fetchIcons,
    handleAddMenu,
  };
}
