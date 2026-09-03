<template>
  <div class="search-bar">
    <input
      :value="modelValue"
      @input="$emit('update:modelValue', $event.target.value)"
      :placeholder="$t('bookmark.search')"
      class="search-input"
    />
    <svg class="search-icon" viewBox="0 0 24 24" width="18" height="18">
      <circle
        cx="10"
        cy="10"
        r="7"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
      ></circle>
      <line
        x1="15"
        y1="15"
        x2="21"
        y2="21"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
      ></line>
    </svg>
  </div>
</template>

<script setup>
defineProps({ modelValue: String });
defineEmits(["update:modelValue"]);
</script>

<style scoped>
.search-bar {
  display: flex;
  align-items: center;
  justify-content: center;
}
.search-icon {
  color: #bbb;
  margin-left: -32px;
  flex-shrink: 0;
  pointer-events: none;
}
.search-input {
  max-width: 520px;
  width: 100%;
  padding: 10px 42px 10px 20px;
  border: 1px solid #e0e0e0;
  border-radius: 24px;
  background: white;
  font-size: 14px;
  color: #333;
  outline: none;
  transition:
    border-color 0.2s,
    box-shadow 0.2s;
}
.search-input::placeholder {
  color: #bbb;
}
.search-input:focus {
  border-color: #dda142;
  box-shadow: 0 0 0 3px rgba(221, 161, 66, 0.1);
}
</style>
