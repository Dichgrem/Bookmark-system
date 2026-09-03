<template>
  <aside class="sidebar" :class="{ collapsed, dark: darkMode }">
    <div class="sidebar-header">
      <div class="sidebar-actions-row">
        <el-dropdown
          v-show="!collapsed"
          trigger="click"
          @command="$emit('command', $event)"
        >
          <button class="add-cate-btn" :title="$t('sidebar.dataManage')">
            <svg viewBox="0 0 24 24" class="btn-icon">
              <circle cx="12" cy="12" r="3"></circle>
              <path
                d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"
              ></path>
            </svg>
          </button>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="import-html">
                <svg
                  class="svg-icon"
                  viewBox="0 0 24 24"
                  style="margin-right: 5px; width: 14px; height: 14px"
                >
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                  <polyline points="7 10 12 15 17 10"></polyline>
                  <line x1="12" y1="15" x2="12" y2="3"></line>
                </svg>
                {{ $t("sidebar.importHTML") }}
              </el-dropdown-item>
              <el-dropdown-item command="export-html">
                <svg
                  class="svg-icon"
                  viewBox="0 0 24 24"
                  style="margin-right: 5px; width: 14px; height: 14px"
                >
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                  <polyline points="17 8 12 3 7 8"></polyline>
                  <line x1="12" y1="3" x2="12" y2="15"></line>
                </svg>
                {{ $t("sidebar.exportHTML") }}
              </el-dropdown-item>
              <el-dropdown-item divided command="fetch-icons">
                {{ $t("sidebar.fetchIcons") }}
              </el-dropdown-item>
              <el-dropdown-item command="check-links">
                {{ $t("checkLinks.checkBtn") }}
              </el-dropdown-item>
              <el-dropdown-item divided command="lang-zh"
                >中文</el-dropdown-item
              >
              <el-dropdown-item command="lang-en">English</el-dropdown-item>
              <el-dropdown-item divided command="change-password">{{
                $t("sidebar.changePassword")
              }}</el-dropdown-item>
              <el-dropdown-item v-if="isAdmin" command="add-user">{{
                $t("sidebar.addUser")
              }}</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
        <button
          v-show="!collapsed"
          class="add-cate-btn"
          @click="$emit('logout')"
          :title="$t('sidebar.logout')"
        >
          <svg viewBox="0 0 24 24" class="btn-icon">
            <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
            <polyline points="16 17 21 12 16 7"></polyline>
            <line x1="21" y1="12" x2="9" y2="12"></line>
          </svg>
        </button>
        <button
          v-show="!collapsed"
          class="add-cate-btn dark-toggle"
          @click="$emit('toggle-dark')"
          :title="darkMode ? 'Light' : 'Dark'"
        >
          <svg viewBox="0 0 24 24" class="dark-icon">
            <g v-if="darkMode">
              <path
                d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"
                fill="currentColor"
              />
            </g>
            <g v-else>
              <circle
                cx="12"
                cy="12"
                r="5"
                stroke="currentColor"
                stroke-width="2"
                fill="none"
              />
              <circle cx="12" cy="12" r="1.5" fill="currentColor" />
              <line
                x1="12"
                y1="1"
                x2="12"
                y2="3"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
              />
              <line
                x1="12"
                y1="21"
                x2="12"
                y2="23"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
              />
              <line
                x1="4.22"
                y1="4.22"
                x2="5.64"
                y2="5.64"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
              />
              <line
                x1="18.36"
                y1="18.36"
                x2="19.78"
                y2="19.78"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
              />
              <line
                x1="1"
                y1="12"
                x2="3"
                y2="12"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
              />
              <line
                x1="21"
                y1="12"
                x2="23"
                y2="12"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
              />
              <line
                x1="4.22"
                y1="19.78"
                x2="5.64"
                y2="18.36"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
              />
              <line
                x1="18.36"
                y1="5.64"
                x2="19.78"
                y2="4.22"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
              />
            </g>
          </svg>
        </button>
        <el-dropdown
          v-show="!collapsed"
          trigger="click"
          @command="$emit('add-menu', $event)"
        >
          <button class="add-cate-btn" :title="$t('sidebar.add')">+</button>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="bookmark">{{
                $t("sidebar.addBookmark")
              }}</el-dropdown-item>
              <el-dropdown-item command="category">{{
                $t("sidebar.addFolder")
              }}</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
      <button
        class="add-cate-btn collapse-btn"
        @click="$emit('update:collapsed', !collapsed)"
        :title="collapsed ? $t('sidebar.expand') : $t('sidebar.collapse')"
      >
        <svg
          class="collapse-arrow"
          :class="{ rotated: !collapsed }"
          viewBox="0 0 24 24"
        >
          <polyline
            points="8 18 15 12 8 6"
            fill="none"
            stroke="currentColor"
            stroke-width="3"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </button>
    </div>

    <div v-show="!collapsed" class="cate-tree-container">
      <el-tree
        :data="tree"
        node-key="id"
        draggable
        :allow-drop="() => true"
        :expand-on-click-node="false"
        :default-expanded-keys="expandedKeys"
        class="custom-tree"
        @node-drag-start="(n, e) => $emit('drag-start', n.data.id, e)"
        @node-drag-end="(n, d, t, e) => $emit('drag-end', n, d, t, e)"
        @node-drop="(n, d, t, e) => $emit('drop', n, d, t, e)"
      >
        <template #default="{ node, data }">
          <div
            class="cate-item"
            :class="{
              active: activeId === data.id,
              'drag-over': dragHoverId === data.id,
            }"
            @dragover.prevent="$emit('drag-over', data.id)"
            @dragleave="$emit('drag-leave')"
            @drop="onDrop($event, data.id)"
            @click.stop="onNodeClick(data)"
          >
            <div class="cate-content-wrapper">
              <span
                class="cate-name-text"
                :class="{
                  'hide-text': dragHoverId === data.id || eatingId === data.id,
                }"
              >
                <svg class="svg-icon menu-icon" viewBox="0 0 24 24">
                  <path
                    d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
                  ></path>
                </svg>
                {{ data.name }}
              </span>
              <div
                class="pacman-container"
                :class="{
                  'show-pacman':
                    dragHoverId === data.id || eatingId === data.id,
                  eating: eatingId === data.id,
                }"
              >
                <svg
                  class="pixel-pacman"
                  viewBox="0 0 13 13"
                  shape-rendering="crispEdges"
                >
                  <rect x="4" y="0" width="5" height="1" fill="#DDA142" />
                  <rect x="2" y="1" width="9" height="1" fill="#DDA142" />
                  <rect x="1" y="2" width="11" height="1" fill="#DDA142" />
                  <rect x="0" y="3" width="13" height="1" fill="#DDA142" />
                  <rect x="0" y="9" width="13" height="1" fill="#DDA142" />
                  <rect x="1" y="10" width="11" height="1" fill="#DDA142" />
                  <rect x="2" y="11" width="9" height="1" fill="#DDA142" />
                  <rect x="4" y="12" width="5" height="1" fill="#DDA142" />
                  <rect x="0" y="4" width="11" height="1" fill="#DDA142" />
                  <rect
                    x="11"
                    y="4"
                    width="2"
                    height="1"
                    class="pac-jaw-top"
                    fill="#DDA142"
                  />
                  <rect x="0" y="5" width="8" height="1" fill="#DDA142" />
                  <rect
                    x="8"
                    y="5"
                    width="5"
                    height="1"
                    class="pac-jaw-top"
                    fill="#DDA142"
                  />
                  <rect x="0" y="6" width="6" height="1" fill="#DDA142" />
                  <rect
                    x="6"
                    y="6"
                    width="7"
                    height="1"
                    class="pac-jaw-mid"
                    fill="#DDA142"
                  />
                  <rect x="0" y="7" width="8" height="1" fill="#DDA142" />
                  <rect
                    x="8"
                    y="7"
                    width="5"
                    height="1"
                    class="pac-jaw-bottom"
                    fill="#DDA142"
                  />
                  <rect x="0" y="8" width="11" height="1" fill="#DDA142" />
                  <rect
                    x="11"
                    y="8"
                    width="2"
                    height="1"
                    class="pac-jaw-bottom"
                    fill="#DDA142"
                  />
                  <rect x="7" y="2" width="2" height="2" fill="#000" />
                </svg>
              </div>
            </div>
            <div class="cate-actions">
              <span
                class="action-icon"
                @click.stop="$emit('add-sub', data.id)"
                :title="$t('sidebar.addSubCategory')"
              >
                <svg class="svg-icon" viewBox="0 0 24 24">
                  <line x1="12" y1="5" x2="12" y2="19"></line>
                  <line x1="5" y1="12" x2="19" y2="12"></line>
                </svg>
              </span>
              <span
                class="action-icon"
                @click.stop="$emit('edit-cate', data)"
                :title="$t('sidebar.editCategory')"
              >
                <svg class="svg-icon" viewBox="0 0 24 24">
                  <path d="M12 20h9"></path>
                  <path
                    d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"
                  ></path>
                </svg>
              </span>
            </div>
          </div>
        </template>
      </el-tree>
    </div>
  </aside>
</template>

<script setup>
import { ref, onMounted, watch } from "vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const STORAGE_KEY = "bookmark-expanded-keys";

const expandedKeys = ref([]);
let initialized = false;

onMounted(() => {
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved) {
      expandedKeys.value = JSON.parse(saved);
      initialized = true;
    }
  } catch {}
});

const props = defineProps({
  collapsed: Boolean,
  tree: Array,
  activeId: [Number, null],
  dragHoverId: [Number, null],
  eatingId: [Number, null],
  darkMode: Boolean,
  isAdmin: Boolean,
});

watch(
  () => props.tree,
  (tree) => {
    if (!initialized && tree && tree.length > 0) {
      expandedKeys.value = tree.map((n) => n.id);
      initialized = true;
      localStorage.setItem(STORAGE_KEY, JSON.stringify(expandedKeys.value));
    }
  },
  { immediate: true },
);

const emit = defineEmits([
  "update:collapsed",
  "select",
  "command",
  "logout",
  "add-menu",
  "add-sub",
  "edit-cate",
  "drag-start",
  "drag-end",
  "drop",
  "drag-over",
  "drag-leave",
  "drop-bookmark",
  "toggle-dark",
]);

const onNodeClick = (data) => emit("select", data.id);
const onDrop = (event, cateId) => {
  emit("drop-bookmark", event, cateId);
};
</script>

<style scoped>
.sidebar {
  width: 220px;
  background-color: white;
  border-right: 1px solid rgba(0, 0, 0, 0.06);
  display: flex;
  flex-direction: column;
  z-index: 5;
  transition: width 0.2s;
}
.sidebar.collapsed {
  width: 52px;
}
.sidebar.collapsed .sidebar-header {
  padding: 12px 4px;
}
.sidebar-header {
  padding: 10px 10px;
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 6px;
}
.sidebar-actions-row {
  display: flex;
  gap: 6px;
  align-items: center;
}
.add-cate-btn {
  width: 30px;
  height: 30px;
  border-radius: 50%;
  border: none;
  background: white;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  font-size: 16px;
  color: #aaa;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  transition:
    box-shadow 0.2s,
    color 0.2s;
}
.add-cate-btn:hover {
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.15);
  color: #dda142;
}
.add-cate-btn :deep(.el-icon) {
  font-size: 16px;
}
.btn-icon {
  width: 14px;
  height: 14px;
  stroke: currentColor;
  stroke-width: 2.3;
  fill: none;
  stroke-linecap: round;
  stroke-linejoin: round;
}
.dark-icon {
  width: 14px;
  height: 14px;
}
.collapse-btn {
  font-size: 18px;
  font-weight: 700;
}
.collapse-arrow {
  width: 15px;
  height: 15px;
  transition: transform 0.2s;
}
.collapse-arrow.rotated {
  transform: rotate(180deg);
}
.cate-tree-container {
  flex: 1;
  overflow-y: auto;
  padding: 10px;
}
:deep(.custom-tree) {
  background: transparent;
}
:deep(.el-tree-node__content) {
  padding: 0 !important;
  height: auto !important;
}
:deep(.el-tree-node__content:hover) {
  background-color: transparent !important;
}
:deep(.el-tree-node__expand-icon) {
  color: #aaaaaa;
  font-size: 16px;
  padding: 6px;
}
:deep(.el-tree-node__expand-icon.is-leaf) {
  color: transparent;
}
.cate-item {
  flex: 1;
  height: 48px;
  padding: 0 15px 0 5px;
  border-radius: 8px;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: all 0.2s ease;
  border: 1px solid transparent;
}
.cate-item:hover {
  background-color: rgba(0, 0, 0, 0.04);
}
.cate-item.active {
  background-color: rgba(221, 161, 66, 0.08);
  border-color: rgba(221, 161, 66, 0.2);
}
.cate-item.drag-over {
  background-color: rgba(221, 161, 66, 0.15);
  border-color: #dda142;
}
.cate-content-wrapper {
  flex: 1;
  display: flex;
  align-items: center;
  position: relative;
  overflow: hidden;
  height: 100%;
}
.cate-name-text {
  flex: 1;
  display: flex;
  align-items: center;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  transform-origin: left center;
}
.cate-name-text.hide-text {
  opacity: 0;
  transform: scale(0.5) translateX(-20px);
}
.pacman-container {
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%) translateX(-20px) scale(0.5);
  opacity: 0;
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  pointer-events: none;
}
.pacman-container.show-pacman {
  opacity: 1;
  transform: translateY(-50%) translateX(0) scale(1.3);
}
.pacman-container.eating {
  opacity: 1;
  animation: pounce 0.6s ease-in-out forwards;
}
.pixel-pacman {
  width: 18px;
  height: 18px;
  display: block;
}
.pac-jaw-top,
.pac-jaw-bottom,
.pac-jaw-mid {
  opacity: 0;
}
.eating .pixel-pacman .pac-jaw-top,
.eating .pixel-pacman .pac-jaw-bottom {
  animation: chomp-jaw 0.15s infinite alternate;
}
.eating .pixel-pacman .pac-jaw-mid {
  animation: chomp-mid 0.15s infinite alternate;
}
@keyframes chomp-jaw {
  0%,
  30% {
    opacity: 0;
  }
  70%,
  100% {
    opacity: 1;
  }
}
@keyframes chomp-mid {
  0%,
  40% {
    opacity: 0;
  }
  60%,
  100% {
    opacity: 1;
  }
}
@keyframes pounce {
  0% {
    transform: translateY(-50%) translateX(0) scale(1.3);
  }
  30% {
    transform: translateY(-50%) translateX(10px) scale(1.6);
  }
  100% {
    transform: translateY(-50%) translateX(-40px) scale(0);
    opacity: 0;
  }
}
.menu-icon {
  width: 16px;
  height: 16px;
  margin-right: 8px;
}
.cate-actions {
  display: none;
  gap: 4px;
  align-items: center;
  flex-shrink: 0;
}
.cate-item:hover .cate-actions {
  display: flex;
}
.action-icon {
  cursor: pointer;
  color: #ccc;
  transition: color 0.2s;
  display: flex;
  align-items: center;
}
.action-icon:hover {
  color: #dda142;
}
.action-icon .svg-icon {
  width: 14px;
  height: 14px;
}
.svg-icon {
  fill: none;
  stroke: currentColor;
  stroke-width: 2;
  stroke-linecap: round;
  stroke-linejoin: round;
}

.sidebar.dark {
  background-color: #252525;
  border-right-color: #333;
}
.sidebar.dark .add-cate-btn {
  background: #363636;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  color: #999;
}
.sidebar.dark .add-cate-btn:hover {
  color: #dda142;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.4);
}
.sidebar.dark .cate-item:hover {
  background-color: rgba(255, 255, 255, 0.06);
}
.sidebar.dark .cate-item.active {
  background-color: rgba(221, 161, 66, 0.12);
  border-color: rgba(221, 161, 66, 0.25);
}
.sidebar.dark .cate-name-text {
  color: #ccc;
}
.sidebar.dark :deep(.el-tree-node__expand-icon) {
  color: #777;
}
</style>
