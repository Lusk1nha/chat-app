import api from "@/api";

export interface SignupParams {
  email: string;
  password: string;
  confirmPassword: string;
}

export interface SignupResponse {
  accessToken: string;
  message: string;
}

export interface LoginParams {
  email: string;
  password: string;
}

export interface LoginResponse {
  accessToken: string;
  message: string;
}

export interface LogoutResponse {
  message: string;
}

export interface RefreshTokenResponse {
  accessToken: string;
  message: string;
}

export class AuthRepository {
  private readonly baseUrl = "http://localhost:8000/api";

  async signup(
    email: string,
    password: string,
    confirmPassword: string
  ): Promise<SignupResponse> {
    const endpoint = `${this.baseUrl}/auth/signup`;

    const request = await fetch(endpoint, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({ email, password, confirmPassword })
    });

    const response = await request.json();

    return response as SignupResponse;
  }

  async login(email: string, password: string): Promise<LoginResponse> {
    const endpoint = `${this.baseUrl}/auth/login`;

    const request = await api.post<LoginResponse>(endpoint, {
      email,
      password
    });

    return request.data;
  }

  async logout(): Promise<LogoutResponse> {
    const endpoint = `${this.baseUrl}/auth/logout`;

    const request = await fetch(endpoint, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      credentials: "include"
    });

    const response = await request.json();

    return response as LogoutResponse;
  }

  async refreshToken() {
    const endpoint = `${this.baseUrl}/auth/refresh`;

    const request = await fetch(endpoint, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      credentials: "include"
    });

    const response = await request.json();

    return response as RefreshTokenResponse;
  }

  async verifyToken() {}

  async forgotPassword(email: string) {}
}
