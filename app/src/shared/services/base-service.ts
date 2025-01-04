import { BaseRepository } from "../repositories/base-repository";

export class BaseService<T extends BaseRepository> {
  protected readonly _repository: T;

  constructor(repository: T) {
    this._repository = repository;
  }
}
