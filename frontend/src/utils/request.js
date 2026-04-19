import axios from 'axios'

const service = axios.create({
  baseURL: 'http://localhost:8989', // 你的后端地址
  timeout: 5000
})

export default service