import { AuthRepository } from "../repositories/auth-repository";
import { AuthService } from "../services/auth-service";

const authRepository = new AuthRepository();
const authService = new AuthService(authRepository);

export default authService;
