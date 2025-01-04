import api from "@/lib/api";
import { Profile, ProfileCreate, ProfileUpdate } from "../models/profile-model";
import { BaseRepository } from "./base-repository";

export class ProfileRepository extends BaseRepository {
  constructor() {
    super();

    this.getCurrentProfile = this.getCurrentProfile.bind(this);
    this.getProfileById = this.getProfileById.bind(this);
    this.getProfileByUserId = this.getProfileByUserId.bind(this);
    this.createProfile = this.createProfile.bind(this);
    this.updateProfile = this.updateProfile.bind(this);
    this.deleteProfile = this.deleteProfile.bind(this);
  }

  async getCurrentProfile(): Promise<Profile> {
    const endpoint = `${this.baseUrl}/profile/currentUser`;

    try {
      const response = await api.get<Profile>(endpoint);
      return response.data;
    } catch (error) {
      throw this._createErrorMessage(
        error,
        "An error occurred while trying to get the current profile"
      );
    }
  }

  async getProfileById(id: string): Promise<Profile> {
    const endpoint = `${this.baseUrl}/profile/${id}`;

    try {
      const response = await api.get<Profile>(endpoint);
      return response.data;
    } catch (error) {
      throw this._createErrorMessage(
        error,
        "An error occurred while trying to get the profile"
      );
    }
  }

  async getProfileByUserId(id: string): Promise<Profile> {
    const endpoint = `${this.baseUrl}/profile/user/${id}`;

    try {
      const response = await api.get<Profile>(endpoint);
      return response.data;
    } catch (error) {
      throw this._createErrorMessage(
        error,
        "An error occurred while trying to get the profile"
      );
    }
  }

  async createProfile(profile: ProfileCreate): Promise<Profile> {
    const endpoint = `${this.baseUrl}/profile`;

    try {
      const response = await api.post<Profile>(endpoint, profile);
      return response.data;
    } catch (error) {
      throw this._createErrorMessage(
        error,
        "An error occurred while trying to create the profile"
      );
    }
  }

  async updateProfile(profile: ProfileUpdate): Promise<Profile> {
    const endpoint = `${this.baseUrl}/profile`;

    try {
      const response = await api.patch<Profile>(endpoint, profile);
      return response.data;
    } catch (error) {
      throw this._createErrorMessage(
        error,
        "An error occurred while trying to update the profile"
      );
    }
  }

  async deleteProfile(): Promise<void> {
    const endpoint = `${this.baseUrl}/profile`;

    try {
      await api.delete(endpoint);
    } catch (error) {
      throw this._createErrorMessage(
        error,
        "An error occurred while trying to delete the profile"
      );
    }
  }
}
