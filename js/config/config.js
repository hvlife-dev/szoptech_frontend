export class Config {
  static instance = null;
  static all = null;
  static allsite = null;
  static sites = null;
  static paths = null;
  static main = null;

  constructor(path = "/config.json") {
    if (Config.instance) return Promise.resolve(Config.instance);
    Config.instance = this;

    return (async () => {
      try {
        const response = await fetch(path);

        if (!response.ok) throw new Error(`Cannot find file: ${path}`);

        const config = await response.json();

        Config.all = config?.all || null;
        Config.allsite = config?.sites["*"] || null;
        Config.paths = Object?.keys(config.sites) || null;

        Config.sites = Object.fromEntries(
          Object.entries(config.sites).filter(([key]) => key !== "*"),
        );

        Config.main = config?.all?.main || null;

        return this;
      } catch (err) {
        console.error("Config error:", err);
        Config.instance = null;
        throw err;
      }
    })();
  }

  static getPathValues(path) {
    if (!this.instance) return;
    const result = this.sites?.[path] || null;
    return result;
  }
}
