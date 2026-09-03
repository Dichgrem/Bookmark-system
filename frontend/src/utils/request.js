import axios from "axios";
import { LS_TOKEN } from "./constants.js";

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

export default service;
