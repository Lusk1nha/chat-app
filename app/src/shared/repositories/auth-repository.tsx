import api from "@/lib/api";

import {
  SignupResponse,
  LoginResponse,
  LogoutResponse,
  RefreshTokenResponse,
  ValidateSession
} from "../models/auth-model";
import { BaseRepository } from "./base-repository";

export class AuthRepository extends BaseRepository {
  constructor() {
    super();

    this.signup = this.signup.bind(this);
    this.login = this.login.bind(this);
    this.logout = this.logout.bind(this);
    this.refreshToken = this.refreshToken.bind(this);
    this.validateSession = this.validateSession.bind(this);
    this.forgotPassword = this.forgotPassword.bind(this);
  }

  async signup(
    email: string,
    password: string,
    confirmPassword: string
  ): Promise<SignupResponse> {
    const endpoint = `${this.baseUrl}/auth/signup`;

    try {
      const response = await api.post<SignupResponse>(endpoint, {
        email,
        password,
        confirmPassword
      });

      return response.data;
    } catch (error) {
      throw this._createErrorMessage(
        error,
        "An error occurred while trying to sign up"
      );
    }
  }

  async login(email: string, password: string): Promise<LoginResponse> {
    const endpoint = `${this.baseUrl}/auth/login`;

    try {
      const request = await api.post<LoginResponse>(endpoint, {
        email,
        password
      });

      return request.data;
    } catch (error) {
      throw this._createErrorMessage(
        error,
        "An error occurred while trying to log in"
      );
    }
  }

  async logout(): Promise<LogoutResponse> {
    const endpoint = `${this.baseUrl}/auth/logout`;

    try {
      const response = await api.post<LogoutResponse>(endpoint);

      return response.data;
    } catch (error) {
      throw this._createErrorMessage(
        error,
        "An error occurred while trying to log out"
      );
    }
  }

  async refreshToken(): Promise<RefreshTokenResponse> {
    const endpoint = `${this.baseUrl}/auth/refresh`;
    try {
      const response = await api.post<RefreshTokenResponse>(endpoint);
      return response.data;
    } catch (error) {
      throw this._createErrorMessage(
        error,
        "An error occurred while refreshing the token"
      );
    }
  }

  async validateSession(): Promise<ValidateSession> {
    const endpoint = `${this.baseUrl}/auth/validate-session`;

    try {
      const response = await api.post<ValidateSession>(endpoint);
      return response.data;
    } catch (error) {
      throw this._createErrorMessage(
        error,
        "An error occurred while validating the session"
      );
    }
  }

  async forgotPassword(email: string) {
    const endpoint = `${this.baseUrl}/auth/forgot-password`;

    try {
      await api.post(endpoint, { email });
    } catch (error) {
      throw this._createErrorMessage(
        error,
        "An error occurred while trying to reset the password"
      );
    }
  }
}
