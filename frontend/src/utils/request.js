import axios from "axios";
import { LS_TOKEN, LS_USER } from "./constants.js";

const service = axios.create({
  baseURL: import.meta.env.DEV ? "http://localhost:8989" : "",
  timeout: 5000,
});

service.interceptors.request.use((config) => {
  const token = localStorage.getItem(LS_TOKEN);
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

service.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response && error.response.status === 401) {
      localStorage.removeItem(LS_TOKEN);
      localStorage.removeItem(LS_USER);
      if (window.location.pathname !== "/login") {
        window.location.href = "/login";
      }
    }
    return Promise.reject(error);
  },
);

export default service;
