import { AuthContext } from "@/contexts/auth-context";
import authService from "@/shared/factories/auth-factory";
import {
  LoginParams,
  SignupParams
} from "@/shared/repositories/auth-repository";
import { useContext } from "react";

export function useAuth() {
  const { accessToken, setAccessToken } = useContext(AuthContext);

  const signup = async (data: SignupParams): Promise<void> => {
    const { email, password, confirmPassword } = data;
    const response = await authService.signup({
      email,
      password,
      confirmPassword
    });

    setAccessToken(response.accessToken);
  };

  const login = async (data: LoginParams): Promise<void> => {
    const { email, password } = data;
    const response = await authService.login({ email, password });

    setAccessToken(response.accessToken);
  };

  const logout = async (): Promise<void> => {
    await authService.logout();

    setAccessToken(null);
  };

  const refreshToken = async (): Promise<void> => {
    const response = await authService.refreshToken();
    setAccessToken(response.accessToken);
  };

  return {
    accessToken,
    signup,
    login,
    logout,
    refreshToken
  };
}
