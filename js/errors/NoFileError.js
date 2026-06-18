import { ErrorManager } from "../helpers/ErrorManager";

export class NoFileError extends Error {
  constructor(err) {
    super();
    ErrorManager.invoke(err, super.message);
  }
}
