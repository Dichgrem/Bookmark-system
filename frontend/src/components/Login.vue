<template>
  <div class="login-view-container" :class="{ dark: isDark }">
    <!-- 背景元素 -->
    <div class="bg-container">
      <!-- 左上角浅灰色圆形 -->
      <div class="top-left-circle"></div>

      <!-- 底部山脉与吃豆人 -->
      <svg
        class="bottom-mountains"
        viewBox="0 0 1440 900"
        preserveAspectRatio="xMidYMax slice"
      >
        <g id="scrolling-mountains" ref="mountainsRef">
          <!-- 错落有致的山脉（左低右高） -->
          <path
            class="mountain"
            fill="#333333"
            stroke="#333333"
            stroke-width="1.5"
            d="M 0,900 L 0,650 Q 350,450 780,780 Q 1150,350 1440,650 L 1440,900 Z"
          />
          <!-- 左侧的补充山脉，用于无缝循环 -->
          <path
            class="mountain"
            fill="#333333"
            stroke="#333333"
            stroke-width="1.5"
            transform="translate(-1440, 0)"
            d="M 0,900 L 0,650 Q 350,450 780,780 Q 1150,350 1440,650 L 1440,900 Z"
          />
        </g>

        <!-- 黄色吃豆人，站在右侧山脉边缘，嘴巴朝左 -->
        <g
          id="pacman"
          class="pacman-group"
          ref="pacmanRef"
          transform="translate(1200, 485)"
        >
          <path
            class="pacman-mouth"
            fill="#DDA142"
            d="M 0,0 L -65,-37.5 A 75,75 0 1,1 -65,37.5 Z"
          />
          <circle class="pacman-eye" cx="-15" cy="-35" r="9" fill="#000" />
        </g>
      </svg>
    </div>

    <!-- 登录卡片容器 -->
    <div class="login-wrapper">
      <main class="login-card">
        <h2>{{ $t("login.title") }}</h2>
        <form action="#" method="POST" @submit.prevent="handleLogin">
          <div class="input-group">
            <input
              type="text"
              v-model="form.username"
              :placeholder="$t('login.username')"
              required
              autocomplete="username"
              :disabled="loading"
            />
          </div>
          <div class="input-group">
            <input
              type="password"
              v-model="form.password"
              :placeholder="$t('login.password')"
              required
              autocomplete="current-password"
              :disabled="loading"
              @keyup.enter="handleLogin"
            />
          </div>
          <div class="button-container">
            <button
              type="submit"
              class="login-btn primary-btn"
              :disabled="loading"
            >
              {{ loading ? $t("login.loggingIn") : $t("login.login") }}
            </button>
          </div>
        </form>
      </main>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessage } from "element-plus";
import { LS_DARK } from "../utils/constants.js";

const { t } = useI18n();
defineProps({ loading: Boolean });
const emit = defineEmits(["login"]);

const isDark = localStorage.getItem(LS_DARK) === "true";
const form = ref({ username: "", password: "" });

const handleLogin = () => {
  if (!form.value.username || !form.value.password) {
    ElMessage.warning(t("login.usernameRequired"));
    return;
  }
  emit("login", form.value);
};

const mountainsRef = ref(null);
const pacmanRef = ref(null);
let animationFrameId = null;

// --- Animation Logic ---
onMounted(() => {
  let offset = 0;
  const speed = 1.5;
  const pacmanX = 1200;

  const originalRadius = 75;
  const scale = 0.8;
  const radius = originalRadius * scale;

  let currentAngle = null;

  function getMountainY(x) {
    if (x < 780) {
      const t = (-700 + Math.sqrt(490000 + 320 * x)) / 160;
      return (
        Math.pow(1 - t, 2) * 650 + 2 * (1 - t) * t * 450 + Math.pow(t, 2) * 780
      );
    } else {
      const t = (740 - Math.sqrt(547600 + 320 * (780 - x))) / 160;
      return (
        Math.pow(1 - t, 2) * 780 + 2 * (1 - t) * t * 350 + Math.pow(t, 2) * 650
      );
    }
  }

  function getPhysicalCenterY(cx) {
    let minYc = Infinity;
    for (let dx = -radius; dx <= radius; dx += 2) {
      let sampleX = cx + dx;
      if (sampleX < 0) {
        sampleX = 1440 - (-sampleX % 1440);
      }
      sampleX = sampleX % 1440;

      let gy = getMountainY(sampleX);
      let requiredYc = gy - Math.sqrt(radius * radius - dx * dx);
      if (requiredYc < minYc) {
        minYc = requiredYc;
      }
    }
    return minYc;
  }

  function animate() {
    offset += speed;
    if (offset >= 1440) offset -= 1440;

    if (mountainsRef.value) {
      mountainsRef.value.setAttribute("transform", `translate(${offset}, 0)`);
    }

    let localX = pacmanX - offset;
    if (localX < 0) localX += 1440;

    const y = getPhysicalCenterY(localX);
    const yNext = getPhysicalCenterY(localX + 2);
    const yPrev = getPhysicalCenterY(localX - 2);
    const slope = (yNext - yPrev) / 4;

    const targetAngle = (Math.atan(slope) * 180) / Math.PI;

    if (currentAngle === null) {
      currentAngle = targetAngle;
    } else {
      currentAngle += (targetAngle - currentAngle) * 0.08;
    }

    if (pacmanRef.value) {
      pacmanRef.value.setAttribute(
        "transform",
        `translate(${pacmanX}, ${y}) scale(${scale}) rotate(${currentAngle})`,
      );
    }

    animationFrameId = requestAnimationFrame(animate);
  }

  animationFrameId = requestAnimationFrame(animate);
});

onUnmounted(() => {
  if (animationFrameId) cancelAnimationFrame(animationFrameId);
});
</script>

<style scoped>
.login-view-container {
  font-family:
    "Inter",
    -apple-system,
    BlinkMacSystemFont,
    "Segoe UI",
    Roboto,
    sans-serif;
  margin: 0;
  padding: 0;
  background-color: #f5f5f5;
  overflow: hidden;
  height: 100vh;
  width: 100vw;
  position: fixed;
  top: 0;
  left: 0;
  z-index: 1000;
  transition: background-color 0.3s;
}

.login-view-container.dark {
  background-color: #1e1e1e;
}

/* ---------------- 背景元素 ---------------- */
.bg-container {
  position: absolute;
  width: 100%;
  height: 100%;
  top: 0;
  left: 0;
  z-index: 1001;
  pointer-events: none;
}

.top-left-circle {
  position: absolute;
  top: -150px;
  left: -150px;
  width: 600px;
  height: 600px;
  background-color: #dddddd;
  border-radius: 50%;
  z-index: 1001;
  transition: background-color 0.3s;
}
.dark .top-left-circle {
  background-color: #2d2d2d;
}

.bottom-mountains {
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 1002;
}

/* ---------------- 登录卡片 ---------------- */
.login-wrapper {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  width: 100%;
  position: relative;
  z-index: 1003;
}

.login-card {
  background: rgba(255, 255, 255, 0.98);
  width: 360px;
  padding: 40px 36px;
  border-radius: 12px;
  box-shadow:
    0 4px 24px rgba(0, 0, 0, 0.06),
    0 1px 4px rgba(0, 0, 0, 0.04);
  border: 1px solid rgba(0, 0, 0, 0.06);
  transform: translateY(-30%);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  animation: cardEntrance 0.5s cubic-bezier(0.16, 1, 0.3, 1) forwards;
  transition:
    background-color 0.3s,
    border-color 0.3s;
}
.dark .login-card {
  background: rgba(45, 45, 45, 0.98);
  border-color: rgba(255, 255, 255, 0.08);
  box-shadow:
    0 4px 24px rgba(0, 0, 0, 0.3),
    0 1px 4px rgba(0, 0, 0, 0.2);
}

@keyframes cardEntrance {
  from {
    opacity: 0;
    transform: translateY(-15%);
  }
  to {
    opacity: 1;
    transform: translateY(-30%);
  }
}

.login-card h2 {
  text-align: center;
  margin: 0 0 32px;
  font-size: 22px;
  font-weight: 500;
  color: #222;
  letter-spacing: 1px;
  transition: color 0.3s;
}
.dark .login-card h2 {
  color: #ddd;
}

/* ---------------- 输入框 ---------------- */
.input-group {
  margin-bottom: 16px;
}

.input-group input {
  width: 100%;
  padding: 12px 18px;
  background-color: #f0f0f0;
  border: 1px solid transparent;
  border-radius: 10px;
  font-size: 14px;
  color: #333;
  outline: none;
  transition: all 0.2s;
  box-sizing: border-box;
}
.dark .input-group input {
  background-color: #363636;
  color: #ddd;
  border-color: #555;
}
.dark .input-group input:focus {
  background-color: #3a3a3a;
  border-color: #dda142;
  box-shadow: 0 0 0 3px rgba(221, 161, 66, 0.1);
}

.input-group input::placeholder {
  color: #aaa;
  font-weight: 400;
}
.dark .input-group input::placeholder {
  color: #888;
}

.input-group input:focus {
  background-color: #fff;
  border: 1px solid #dda142;
  box-shadow: 0 0 0 3px rgba(221, 161, 66, 0.08);
}

/* ---------------- 按钮 ---------------- */
.button-container {
  display: flex;
  justify-content: space-between;
  gap: 10px;
  margin-top: 28px;
}

.login-btn {
  flex: 1;
  padding: 11px 0;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  letter-spacing: 1px;
  transition: all 0.2s;
  border: none;
  box-sizing: border-box;
}

.primary-btn {
  background-color: #333;
  color: #fff;
}
.primary-btn:hover {
  background-color: #111;
  transform: translateY(-1px);
}
.primary-btn:active {
  transform: translateY(0);
}
.dark .primary-btn {
  background-color: #dda142;
  color: #1e1e1e;
}
.dark .primary-btn:hover {
  background-color: #e8b44f;
}

.outline-btn {
  background-color: transparent;
  color: #333;
  border: 1.5px solid #333;
}
.outline-btn:hover {
  background-color: #f0f0f0;
  transform: translateY(-1px);
}
.outline-btn:active {
  transform: translateY(0);
}
.dark .outline-btn {
  color: #ccc;
  border-color: #666;
}
.dark .outline-btn:hover {
  background-color: #363636;
  color: #ddd;
}

/* ---------------- 动画效果 ---------------- */
.pacman-mouth {
  animation: chomp 0.8s linear infinite;
  transform-origin: 0 0;
}

@keyframes chomp {
  0%,
  100% {
    d: path("M 0,0 L -65,-37.5 A 75,75 0 1,1 -65,37.5 Z");
  }
  50% {
    d: path("M 0,0 L -75,0 A 75,75 0 1,1 -75,0 Z");
  }
}
</style>
