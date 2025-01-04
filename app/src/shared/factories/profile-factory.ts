import { ProfileRepository } from "../repositories/profile-repository";
import { ProfileService } from "../services/profile-service";

const profileRepository = new ProfileRepository();
const profileService = new ProfileService(profileRepository);

export default profileService;
