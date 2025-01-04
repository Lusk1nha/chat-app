import { Profile } from "../models/profile-model";
import { ProfileRepository } from "../repositories/profile-repository";
import { BaseService } from "./base-service";

export class ProfileService extends BaseService<ProfileRepository> {
  constructor(repository: ProfileRepository) {
    super(repository);
    this.getCurrentProfile = this.getCurrentProfile.bind(this);
  }

  async getCurrentProfile(): Promise<Profile> {
    return this._repository.getCurrentProfile();
  }

  async getProfileById(id: string): Promise<Profile> {
    return this._repository.getProfileById(id);
  }

  async getProfileByUserId(id: string): Promise<Profile> {
    return this._repository.getProfileByUserId(id);
  }

  async createProfile(profile: Profile): Promise<Profile> {
    return this._repository.createProfile(profile);
  }

  async updateProfile(profile: Profile): Promise<Profile> {
    return this._repository.updateProfile(profile);
  }

  async deleteProfile(): Promise<void> {
    return this._repository.deleteProfile();
  }
}
