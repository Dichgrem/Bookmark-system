<template>
  <span class="dialog-footer-inline">
    <button class="outline-btn small-btn" @click="$emit('cancel')">
      {{ $t("bookmark.cancel") }}
    </button>
    <button class="primary-btn small-btn" @click="$emit('save')">
      {{ $t("bookmark.save") }}
    </button>
  </span>
</template>
