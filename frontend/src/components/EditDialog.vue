<template>
  <el-dialog
    v-model="show"
    :title="
      book && book.id ? $t('bookmark.editBookmark') : $t('bookmark.addBookmark')
    "
    width="480px"
    @close="$emit('close')"
  >
    <div class="dialog-form">
      <label class="dialog-label">{{ $t("bookmark.title") }}</label>
      <input
        v-model="form.title"
        class="modern-input full-width"
        :placeholder="$t('bookmark.title')"
      />
      <label class="dialog-label">{{ $t("bookmark.url") }}</label>
      <input
        v-model="form.url"
        class="modern-input full-width"
        placeholder="https://..."
      />
      <label class="dialog-label">{{ $t("bookmark.category") }}</label>
      <select v-model="form.categoryId" class="modern-input full-width">
        <option :value="null">{{ $t("bookmark.uncategorized") }}</option>
        <option v-for="c in categories" :key="c.id" :value="c.id">
          {{ c.name }}
        </option>
      </select>
      <label class="dialog-label">{{ $t("bookmark.icon") }}</label>
      <input
        v-model="form.icon"
        class="modern-input full-width"
        :placeholder="$t('bookmark.iconUrl')"
      />
    </div>
    <template #footer>
      <button class="outline-btn" @click="show = false">
        {{ $t("bookmark.cancel") }}
      </button>
      <button class="primary-btn" @click="$emit('save', form)">
        {{ $t("bookmark.save") }}
      </button>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, watch } from "vue";

const props = defineProps({
  visible: Boolean,
  book: Object,
  categories: Array,
});
const emit = defineEmits(["update:visible", "save", "close"]);

const show = ref(false);
const form = ref({});

watch(
  () => props.visible,
  (v) => {
    show.value = v;
    if (v && props.book) {
      form.value = { ...props.book };
    }
  },
  { immediate: true },
);
watch(show, (v) => emit("update:visible", v));
</script>

<style scoped>
.dialog-form {
  display: flex;
  flex-direction: column;
  gap: 14px;
}
.dialog-label {
  font-size: 13px;
  color: #888;
  font-weight: 500;
  margin-bottom: -10px;
}
.modern-input {
  padding: 10px 14px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  font-size: 14px;
  outline: none;
  transition: border-color 0.2s;
  background: white;
  color: #333;
}
.modern-input:focus {
  border-color: #dda142;
  box-shadow: 0 0 0 3px rgba(221, 161, 66, 0.1);
}
.full-width {
  width: 100%;
  box-sizing: border-box;
}
.outline-btn {
  background-color: transparent;
  border: 1px solid #333;
  color: #333;
  padding: 10px 20px;
  border-radius: 30px;
  cursor: pointer;
  font-size: 13px;
  transition: all 0.2s;
}
.outline-btn:hover {
  background-color: #f5f5f5;
}
.primary-btn {
  background-color: #333333;
  color: #ffffff;
  border: none;
  padding: 10px 20px;
  border-radius: 30px;
  cursor: pointer;
  font-weight: 500;
  font-size: 13px;
  transition: all 0.2s;
}
.primary-btn:hover {
  background-color: #111111;
  transform: translateY(-1px);
}
</style>
