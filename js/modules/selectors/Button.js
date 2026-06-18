import { SelectorEventManager } from "../DefaultSelectorEventManager";

const TYPE = {
  CLICK: "click",
};

const BUTTON = "button";

export class Button extends SelectorEventManager {
  static instance = null;

  constructor() {
    if (Button.instance) {
      return Button.instance;
    }

    super(BUTTON);

    Button.instance = this;

    super.addAllEvents(TYPE.CLICK);
    super.addSiteEvents(TYPE.CLICK);
  }
}
