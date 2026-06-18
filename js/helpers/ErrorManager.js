import { NoFileError } from "../errors/NoFileError";

export class ErrorManager {
  static invoke(err) {
    if (typeof err === NoFileError)
      window._paq.push([
        "trackEvent",
        `Error: ${err.path}`,
        err.type,
        err.message,
      ]);
    return err;
  }
}
