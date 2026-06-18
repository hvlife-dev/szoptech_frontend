import { Sender } from "../../helpers/Sender.js";
import { Config } from "../../config/config.js";

const NAVIGATION = "Navigation";

export class Navbar {
  static instance = null;
  
  constructor() {
    if (Navbar.instance) return Navbar.instance;

    const selector = Config.all?.navbar || [];

    if (!selector) return;

    Navbar.instance = this;

    const selectorList = document.querySelectorAll(selector.join(" "));

    selectorList.forEach((item) => {
      item.addEventListener("click", () => {
        const selectorPathname = new URL(item.href).pathname;
        if (selectorPathname === window.location.pathname) return;

        Sender.sendEvent([
          NAVIGATION,
          `from: ${window.location.pathname}`,
          `to: ${selectorPathname}`,
        ]);
      });
    });
  }
}
