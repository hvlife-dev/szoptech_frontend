import { SelectorEventManager } from "../DefaultSelectorEventManager";

const TYPE = "focus";
const MOUSE_ENTER = "mouseenter";

export class Focus extends SelectorEventManager {
  static instance = null;

  constructor() {
    if (Focus.instance) return Focus.instance;

    super(TYPE);
    Focus.instance = this;

    super.addAllEvents(MOUSE_ENTER);
    super.addSiteEvents(MOUSE_ENTER);
  }
}
