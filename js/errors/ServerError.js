import { ErrorManager } from "../helpers/ErrorManager";

export class ServerError extends Error {
  constructor(err) {
    super();
    ErrorManager.invoke(err, super.message);
  }
}