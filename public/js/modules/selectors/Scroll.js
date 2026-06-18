import { Config } from "../../config/config.js";
import { Sender } from "../../helpers/Sender.js";

const SCROLL_DEPTH = "Scroll Depth";

export class Scroll {
  static instance = null;
  thresholds = {};
  sender = new Sender();
  constructor(target = window) {
    if (Scroll.instance) return Scroll.instance;
    Scroll.instance = this;

    this.#setThresholds();

    if (!this.thresholds) {
      return;
    }
    this.handleScroll = this.handleScroll.bind(this);
    target.addEventListener("scroll", this.handleScroll);
  }

  #setThresholds() {
    const forAll = Config.all?.scroll || [];
    const sites = Config.sites || {};

    const wynik = {};

    Object.keys(sites).forEach((path) => {
      const siteScroll = sites[path]?.scroll || [];

      const unikalneProgi = [...new Set([...forAll, ...siteScroll])];

      wynik[path] = {};
      unikalneProgi.forEach((item) => {
        wynik[path][item] = false;
      });
    });

    this.thresholds = wynik;
  }

  #setCurrentThresholdValue(pathname = window.location.pathname, key, value) {
    if (this.thresholds[pathname]) {
      this.thresholds[pathname][key] = value;
    }
  }

  handleScroll() {
    const scrollHeight = document.documentElement.scrollHeight;
    const clientHeight = document.documentElement.clientHeight;
    const totalScrollable = scrollHeight - clientHeight;
    if (totalScrollable <= 0) return;

    const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
    const scrolledPercent = Math.round((scrollTop / totalScrollable) * 100);

    const pathname = window.location.pathname;
    const threshold = this.thresholds[pathname];

    if (!threshold) return;

    Object.keys(threshold).forEach((key) => {
      const targetPercent = parseInt(key);
      const isValueSended = this.thresholds[pathname][key];

      if (isValueSended) return;

      if (scrolledPercent >= targetPercent) {
        this.#setCurrentThresholdValue(pathname, key, true);

        Sender.sendEvent([
          SCROLL_DEPTH,
          window.location.pathname,
          `${targetPercent}%`,
        ]);
      }
    });
  }

  handeleWhenResize(target) {
    target.addEventListener("scroll", this.handleScroll);
  }
}
