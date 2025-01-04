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

export interface ValidateSession {
  valid: boolean;
  message: string;
}
