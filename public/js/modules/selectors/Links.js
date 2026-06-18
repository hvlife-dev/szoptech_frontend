import { SelectorEventManager } from "../DefaultSelectorEventManager.js";

const CLICK = "click";
const TYPE = "links";

export class Links extends SelectorEventManager {
  static instance = null;

  constructor() {
    if (Links.instance) return Links.instance;

    super(TYPE);
    Links.instance = this;

    super.addAllEvents(CLICK);
    super.addSiteEvents(CLICK);
  }
}
