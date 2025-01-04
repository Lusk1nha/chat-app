import { AxiosError } from "axios";

export class BaseRepository {
  protected readonly baseUrl: string = process.env.NEXT_BASE_URL_API as string;

  constructor() {
    if (!this.baseUrl) {
      throw new Error("NEXT_BASE_URL_API is not defined");
    }

    this._createErrorMessage = this._createErrorMessage.bind(this);
  }

  protected _createErrorMessage(error: unknown, defaultMessage: string): Error {
    if (error instanceof AxiosError) {
      return new Error(error.response?.data.message);
    }

    if (error instanceof Error) {
      return new Error(error.message);
    }

    return new Error(defaultMessage);
  }
}
