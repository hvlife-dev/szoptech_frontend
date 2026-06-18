import { NoFileError } from "../errors/NoFileError.js";
import { ServerError } from "../errors/ServerError.js";

const TRACKEVENT = "trackEvent";

export class Sender2 {
  url = "";
  static instance = null;

  constructor() {
    if (Sender.instance) {
      return Sender.instance;
    }

    this.url = "https://analytics.oskif.eu";
    this.#checkConnection(this.url);

    if (!window._paq) throw new NoFileError();

    this.instance = this;
  }

  async #checkConnection() {
    try {
      const response = await fetch(this.url);

      if (response.status >= 500) {
        throw new ServerError(`Server error: ${response.status}`);
      }

      return;
    } catch (err) {
      return err;
    }
  }

  sendEvent(args) {
    try {
      window._paq.push(TRACKEVENT, args);
    } catch (err) {
      return err;
    }
  }
}

export class Sender {
  static sendEvent(args) {
    console.log(TRACKEVENT, args);
  }
}
