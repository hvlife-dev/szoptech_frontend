import { Config } from "../config/config.js";
import { ErrorManager } from "../helpers/ErrorManager.js";
import { Sender } from "../helpers/Sender.js";
import { Button } from "./selectors/Button.js";
import { Focus } from "./selectors/Focus.js";
import { Input } from "./selectors/Input.js";
import { Links } from "./selectors/Links.js";
import { Navbar } from "./selectors/Navbar.js";
import { Scroll } from "./selectors/Scroll.js";
import { Select } from "./selectors/Select.js";

export const loadingCallback = () => {
  const time = performance.now();

  Sender.sendEvent([
    "Loading",
    window.location.pathname,
    `${time.toFixed()} ms`,
  ]);
};

export const onLoadCallback = async () => {
  await new Config();

  const ModuleRegistry = {
    navbar: () => new Navbar(),
    scroll: () => new Scroll(),
    links: () => new Links(),
    focus: () => new Focus(),
    select: () => new Select(),
    button: () => new Button(),
    input: () => new Input(),
  };

  const modulesToInit = new Set();

  Object.keys(Config.all || {}).forEach((key) => {
    if (ModuleRegistry[key]) {
      modulesToInit.add(key);
    }
  });

  Object.keys(Config.allsite || {}).forEach((key) => {
    if (ModuleRegistry[key]) {
      if (ModuleRegistry[key]) {
        modulesToInit.add(key);
      }
    }
  });

  Object.values(Config.sites || {}).forEach((siteConfig) => {
    Object.keys(siteConfig || {}).forEach((key) => {
      if (ModuleRegistry[key]) {
        modulesToInit.add(key);
      }
    });
  });

  modulesToInit.forEach((key) => {
    const factory = ModuleRegistry[key];
    if (typeof factory === "function") factory();
  });
};

export const errorCallback = (err) => {
  ErrorManager.invoke(err);
  console.log(err);
};

let oldScrollHeight = 0;

export const resizeCallback = (e) => {
  if (Scroll.instance) new Scroll();

  const scrollHeight = e.currentTarget.document.documentElement.scrollHeight;

  setTimeout(() => {
    if (oldScrollHeight !== scrollHeight) {
      Scroll.instance.handeleWhenResize(e.currentTarget);
    }
  }, 1000);
};
