import { ErrorManager } from "../helpers/ErrorManager.js";

export class NoFileError extends Error {
  constructor(err) {
    super();
    ErrorManager.invoke(err, super.message);
  }
}
