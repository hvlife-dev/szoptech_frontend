import { Config } from "../config/config.js";

export class SiteChangeObserver {
  observer = null;

  constructor(callback) {
    const main = document.querySelector(Config.main);

    if (!main) return;

    this.observer = new MutationObserver(callback);

    this.observer.observe(main, {
      childList: true,
      subtree: true,
    });
  }
}
