<template>
  <el-dialog
    v-model="visible"
    :title="$t('checkLinks.title')"
    width="680px"
    :close-on-click-modal="false"
    @close="$emit('close')"
  >
    <div v-if="!started" class="check-start">
      <p>
        将对所有书签链接逐一发送 HTTP HEAD
        请求检测存活状态。检测过程中不会修改任何数据。24h 内重复检测将使用缓存。
      </p>
      <el-button type="primary" @click="startCheck">
        {{ $t("checkLinks.checkBtn") }}
      </el-button>
    </div>

    <div v-else-if="checking" class="check-progress">
      <el-progress :percentage="pct" :stroke-width="12" />
      <p class="progress-text">
        {{
          $t("checkLinks.checkingProgress", {
            checked: completed,
            total: total,
          })
        }}
      </p>
    </div>

    <div v-else-if="resultsLoaded" class="check-results">
      <div v-if="!report.hasIssues" class="no-issues">
        <p>{{ $t("checkLinks.noDeadLinks") }}</p>
      </div>

      <div class="summary-bar" v-if="report.hasIssues">
        <span class="summary-count ok">{{ report.okCount }} OK</span>
        <span v-if="report.pageDeadCount" class="summary-count page_dead"
          >{{ report.pageDeadCount }} {{ $t("checkLinks.totalPageDead") }}</span
        >
        <span v-if="report.siteDeadCount" class="summary-count site_dead"
          >{{ report.siteDeadCount }} {{ $t("checkLinks.totalSiteDead") }}</span
        >
        <span v-if="report.suspectCount" class="summary-count suspect"
          >{{ report.suspectCount }} {{ $t("checkLinks.totalSuspect") }}</span
        >
      </div>

      <div
        v-for="group in report.groups"
        :key="group.key"
        class="result-group"
        v-show="group.items.length && group.key !== 'ok'"
      >
        <h4 :class="['group-header', group.key]">
          {{ group.label }} ({{ group.items.length }})
        </h4>

        <div>
          <div
            v-for="r in group.items"
            :key="r.id"
            class="result-row"
            @click="$emit('edit', r.id)"
          >
            <span class="col-cache">
              <span :class="{ 'cache-on': r.cached }">{{
                $t("checkLinks.cached")
              }}</span>
            </span>
            <span class="col-title" :title="r.title">{{ r.title }}</span>
            <a
              class="col-url"
              :href="r.url"
              target="_blank"
              rel="noopener"
              :title="r.url"
              @click.stop
              >{{ urlHost(r.url) }}</a
            >
            <span class="col-status" :class="effectiveLevel(r)">{{
              statusText(r)
            }}</span>
            <button
              v-if="effectiveLevel(r) !== 'ok'"
              class="col-mark"
              :title="$t('checkLinks.markOk')"
              @click.stop="markOk(r.id)"
            >
              ✓
            </button>
            <button
              class="col-edit"
              :title="$t('bookmark.edit')"
              @click.stop="$emit('edit', r.id)"
            >
              <svg viewBox="0 0 24 24" width="13" height="13">
                <path
                  d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"
                />
                <path
                  d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"
                />
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>
  </el-dialog>
</template>

<script setup>
import { ref, computed } from "vue";
import { useI18n } from "vue-i18n";
import request from "../utils/request.js";

const { t } = useI18n();

const visible = defineModel("visible", { type: Boolean, default: false });
const emit = defineEmits(["edit", "close"]);

const started = ref(false);
const checking = ref(false);
const resultsLoaded = ref(false);
const total = ref(0);
const completed = ref(0);
const results = ref([]);
const whitelistedIds = ref(new Set());

const pct = computed(() =>
  total.value ? Math.round((completed.value / total.value) * 100) : 0,
);

const effectiveLevel = (r) => (whitelistedIds.value.has(r.id) ? "ok" : r.level);

const report = computed(() => {
  const wl = whitelistedIds.value;
  const ok = [];
  const page_dead = [];
  const site_dead = [];
  const suspect = [];

  for (const r of results.value) {
    const level = wl.has(r.id) ? "ok" : r.level;
    if (level === "ok") ok.push(r);
    else if (level === "page_dead") page_dead.push(r);
    else if (level === "site_dead") site_dead.push(r);
    else suspect.push(r);
  }

  return {
    hasIssues:
      page_dead.length > 0 || site_dead.length > 0 || suspect.length > 0,
    okCount: ok.length,
    pageDeadCount: page_dead.length,
    siteDeadCount: site_dead.length,
    suspectCount: suspect.length,
    groups: [
      { key: "ok", label: t("checkLinks.totalOk"), items: ok },
      {
        key: "page_dead",
        label: t("checkLinks.totalPageDead"),
        items: page_dead,
      },
      {
        key: "site_dead",
        label: t("checkLinks.totalSiteDead"),
        items: site_dead,
      },
      { key: "suspect", label: t("checkLinks.totalSuspect"), items: suspect },
    ],
  };
});

const markOk = (id) => {
  const next = new Set(whitelistedIds.value);
  next.add(id);
  whitelistedIds.value = next;
};

const urlHost = (url) => {
  try {
    const u = new URL(url);
    return u.hostname;
  } catch {
    return url;
  }
};

const statusText = (r) => {
  if (r.httpCode) return `HTTP ${r.httpCode}`;
  if (r.error) {
    const s = r.error.toLowerCase();
    if (s.includes("timeout")) return "timeout";
    if (s.includes("dns")) return "DNS error";
    if (s.includes("connect") || s.includes("refused")) return "conn refused";
    return s.length > 20 ? s.slice(0, 20) + "…" : s;
  }
  return "—";
};

let pollTimer = null;

const startCheck = async () => {
  started.value = true;
  checking.value = true;
  try {
    const res = await request.post("/bookmark/checkLinks");
    if (res.data.data === "cached") {
      const sr = await request.get("/bookmark/checkLinks/status");
      if (sr.data.code === 200 && sr.data.data) {
        results.value = sr.data.data.results || [];
        checking.value = false;
        resultsLoaded.value = true;
      }
    } else {
      pollStatus();
    }
  } catch {
    checking.value = false;
    started.value = false;
  }
};

const pollStatus = async () => {
  try {
    const res = await request.get("/bookmark/checkLinks/status");
    if (res.data.code === 200) {
      const d = res.data.data;
      total.value = d.total;
      completed.value = d.completed;
      if (d.finished) {
        results.value = d.results || [];
        checking.value = false;
        resultsLoaded.value = true;
        return;
      }
    }
  } catch {
    // retry on next poll
  }
  pollTimer = setTimeout(pollStatus, 1000);
};
</script>

<style scoped>
.check-start {
  text-align: center;
  padding: 20px 0;
}
.check-start p {
  margin-bottom: 16px;
  color: var(--el-text-color-secondary);
}

.check-progress {
  text-align: center;
  padding: 30px 0;
}
.progress-text {
  margin-top: 12px;
  color: var(--el-text-color-regular);
}

.no-issues {
  text-align: center;
  padding: 20px;
  color: var(--el-color-success);
}

.summary-bar {
  display: flex;
  gap: 12px;
  margin-bottom: 12px;
  padding: 6px 10px;
  background: var(--el-fill-color-light);
  border-radius: 4px;
  font-size: 12px;
}
.summary-count {
  font-weight: 600;
}
.summary-count.ok {
  color: var(--el-color-success);
}
.summary-count.page_dead {
  color: var(--el-color-warning-dark-2);
}
.summary-count.site_dead {
  color: var(--el-color-danger);
}
.summary-count.suspect {
  color: var(--el-color-info);
}

.result-group {
  margin-bottom: 6px;
}
.group-header {
  margin: 0 0 3px;
  padding: 3px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 600;
}
.group-header.ok {
  background: var(--el-color-success-light-9);
  color: var(--el-color-success);
}
.group-header.page_dead {
  background: var(--el-color-warning-light-9);
  color: var(--el-color-warning-dark-2);
}
.group-header.site_dead {
  background: var(--el-color-danger-light-9);
  color: var(--el-color-danger);
}
.group-header.suspect {
  background: var(--el-color-info-light-9);
  color: var(--el-color-info);
}

.result-row {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 3px 8px;
  font-size: 12px;
  line-height: 1.7;
  cursor: pointer;
  border-bottom: 1px solid var(--el-border-color-extra-light);
}
.result-row:hover {
  background: var(--el-fill-color-light);
}
.result-row:last-child {
  border-bottom: none;
}

.col-cache {
  width: 40px;
  flex-shrink: 0;
  font-size: 10px;
  text-align: left;
}
.col-cache span {
  display: inline-block;
  padding: 0 3px;
  border-radius: 2px;
  background: var(--el-color-info-light-9);
  color: var(--el-color-info);
  visibility: hidden;
}
.col-cache span.cache-on {
  visibility: visible;
}

.col-title {
  width: 170px;
  flex-shrink: 0;
  font-weight: 600;
  color: var(--el-text-color-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.col-url {
  min-width: 0;
  flex: 1 1 auto;
  color: var(--el-text-color-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  text-decoration: none;
}
.col-url:hover {
  text-decoration: underline;
  color: var(--el-color-primary);
}

.col-status {
  width: 90px;
  flex-shrink: 0;
  text-align: right;
  font-weight: 600;
  white-space: nowrap;
}

.col-status.page_dead {
  color: var(--el-color-warning-dark-2);
}
.col-status.site_dead {
  color: var(--el-color-danger);
}
.col-status.suspect {
  color: var(--el-color-info);
}

.col-mark {
  width: 22px;
  height: 22px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 3px;
  background: transparent;
  color: var(--el-color-success);
  cursor: pointer;
  font-size: 11px;
  font-weight: 600;
  opacity: 0;
  transition: opacity 0.15s;
}
.result-row:hover .col-mark {
  opacity: 1;
}
.col-mark:hover {
  background: var(--el-color-success-light-9);
}

.col-edit {
  width: 24px;
  height: 24px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 3px;
  background: transparent;
  color: var(--el-text-color-placeholder);
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.15s;
}
.result-row:hover .col-edit {
  opacity: 1;
}
.col-edit:hover {
  background: var(--el-fill-color);
  color: var(--el-color-primary);
}
</style>

<style>
html.dark .check-start p {
  color: #aaa;
}
html.dark .result-row:hover {
  background: rgba(255, 255, 255, 0.04);
}
html.dark .col-title {
  color: #ddd;
}
html.dark .col-url {
  color: #999;
}
html.dark .result-item,
html.dark .result-row {
  border-color: rgba(255, 255, 255, 0.06);
}
html.dark .summary-bar {
  background: rgba(255, 255, 255, 0.05);
}
html.dark .progress-text {
  color: #aaa;
}
html.dark .col-cache span {
  background: rgba(100, 140, 220, 0.15);
}
</style>
