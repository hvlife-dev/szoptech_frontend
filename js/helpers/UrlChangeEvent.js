export class UrlChangeEvent {
  static instance = null;
  events = {};

  constructor() {
    if (UrlChangeEvent.instance) {
      return UrlChangeEvent.instance;
    }
    UrlChangeEvent.instance = this;

    const pushState = window.history.pushState;
    const replaceState = window.history.replaceState;

    window.history.pushState = (...args) => {
      const result = pushState.apply(window.history, args);

      this.dispatch();
      return result;
    };

    window.history.replaceState = (...args) => {
      const result = replaceState.apply(window.history, args);

      this.dispatch();
      return result;
    };

    window.addEventListener("popstate", () => this.dispatch());
  }

  dispatch() {
    window.dispatchEvent(
      new CustomEvent("urlChange", {
        detail: { path: window.location.pathname },
      }),
    );
  }

  pushEvent(site, callback) {
    this.events[site] ||= [];
    this.events[site].push(callback);
  }
}
