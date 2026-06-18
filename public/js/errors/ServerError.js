import { ErrorManager } from "../helpers/ErrorManager.js";

export class ServerError extends Error {
  constructor(err) {
    super();
    ErrorManager.invoke(err, super.message);
  }
}