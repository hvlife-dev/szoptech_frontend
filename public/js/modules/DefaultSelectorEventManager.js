import { Config } from "../config/config.js";
import { SelectorHelper } from "../helpers/SelectorHelper.js";
import { Sender } from "../helpers/Sender.js";
import { SiteChangeObserver } from "../helpers/SiteChangeObserver.js";

export class SelectorEventManager {
  all = null;
  site = null;
  allsite = null;
  assignedElements = new Set();
  ifOnce = false;

  constructor(type) {
    this.all = Config.all?.[type];
    this.allsite = Config?.allsite?.[type];
    this.site = new Map();

    Config.paths.forEach((path) => {
      const value = Config.getPathValues(path)?.[type];

      if (value) this.site.set(path, value);
    });
  }

  defaultEventCallback(item, querySelector = null, type = null) {
    const senderValues = SelectorHelper.senderValues(item, querySelector, type);
    Sender.sendEvent(senderValues);
  }

  #addListeners(values, type, callback) {
    values?.forEach((item) => {
      const documentSelector = document.querySelectorAll(
        SelectorHelper.selectors(item.selectors),
      );

      const added = new WeakSet();

      documentSelector.forEach((element) => {
        if (added.has(element)) return;

        element.addEventListener(
          type,
          (e) => {
            if (!e.currentTarget) callback(item, e.target, type);
            else callback(item, e.currentTarget, type);
          },
          {
            once: item.once,
          },
        );

        added.add(element);
      });
    });
  }

  addSiteEvents(type, callback = this.defaultEventCallback) {
    if (this.site) {
      this.site.forEach((values, key) => {
        if (window.location.pathname === key) {
          this.#addListeners(values, type, callback);
        }

        new SiteChangeObserver(() => {
          if (window.location.pathname === key) {
            this.#addListeners(values, type, callback);
          }
        });
      });
    }
    if (!this.allsite) return;

    this.#addListeners(this.allsite, type, callback);

    new SiteChangeObserver(() => {
      this.#addListeners(this.allsite, type, callback);
    });
  }

  addAllEvents(type, callback = this.defaultEventCallback) {
    if (!this.all) return;

    this.#addListeners(this.all, type, callback);
  }
}
