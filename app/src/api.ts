import axios from "axios";
import authService from "./shared/factories/auth-factory";
import { Path } from "./shared/enums/Path.enum";

const api = axios.create({
  baseURL: process.env.NEXT_BASE_URL_API,
  withCredentials: true
});

// Intercepta respostas para lidar com 401 Unauthorized
api.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config;

    if (
      error.response?.status === 401 &&
      originalRequest.url.includes(Path.AuthGroup)
    ) {
      return Promise.reject(new Error(error.message));
    }

    if (error.response?.status === 401 && !originalRequest._retry) {
      originalRequest._retry = true; // Evita loops infinitos
      const { accessToken } = await authService.refreshToken();

      if (accessToken) {
        // Atualiza o cabeçalho com o novo access_token
        originalRequest.headers["Authorization"] = `Bearer ${accessToken}`;
        localStorage.setItem("accessToken", accessToken);
        return api(originalRequest); // Reenvia a solicitação original
      }
    }

    return Promise.reject(new Error(error.message));
  }
);

export default api;
