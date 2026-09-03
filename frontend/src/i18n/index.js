import { createI18n } from "vue-i18n";
import zhCN from "./zh-CN.js";
import enUS from "./en-US.js";
import { LS_LOCALE } from "../utils/constants.js";

const saved = localStorage.getItem(LS_LOCALE);
const locale = saved || "zh-CN";

const i18n = createI18n({
  legacy: false,
  locale,
  fallbackLocale: "zh-CN",
  messages: {
    "zh-CN": zhCN,
    "en-US": enUS,
  },
});

export default i18n;
