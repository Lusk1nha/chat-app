import {
  AuthRepository,
  LoginParams,
  LoginResponse,
  LogoutResponse,
  RefreshTokenResponse,
  SignupParams,
  SignupResponse
} from "../repositories/auth-repository";

export class AuthService {
  private readonly _repository: AuthRepository;

  constructor(repository: AuthRepository) {
    this._repository = repository;
  }

  async signup(data: SignupParams): Promise<SignupResponse> {
    const { email, password, confirmPassword } = data;

    const response = await this._repository.signup(
      email,
      password,
      confirmPassword
    );
    localStorage.setItem("accessToken", response.accessToken);

    return response;
  }

  async login(data: LoginParams): Promise<LoginResponse> {
    const { email, password } = data;

    const response = await this._repository.login(email, password);
    localStorage.setItem("accessToken", response.accessToken);

    return response;
  }

  async logout(): Promise<LogoutResponse> {
    const response = await this._repository.logout();
    localStorage.removeItem("accessToken");

    return response;
  }

  async refreshToken(): Promise<RefreshTokenResponse> {
    const response = await this._repository.refreshToken();
    localStorage.setItem("accessToken", response.accessToken);

    return response;
  }

  async validateSession(): Promise<boolean> {
    const response = await this._repository.validateSession();
    return response.valid;
  }

  async forgotPassword(email: string): Promise<void> {
    await this._repository.forgotPassword(email);
  }
}
