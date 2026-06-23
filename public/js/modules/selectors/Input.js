import { SelectorHelper } from "../../helpers/SelectorHelper.js";
import { SelectorEventManager } from "../DefaultSelectorEventManager.js";

const TYPE = "input";
const EVENTS = {
  INPUT: "input",
  BLUR: "blur",
  FOCUS: "focus",
  CLICK: "click",
  MOUSEENTER: "mouseenter",
};

export class Input extends SelectorEventManager {
  static instance = null;
  timer = null;

  constructor() {
    if (Input.instance) return Input.instance;

    super(TYPE);
    Input.instance = this;

    this.defaultEventCallback = this.defaultEventCallback.bind(this);

    super.addSiteEvents(EVENTS.INPUT);
    super.addAllEvents(EVENTS.INPUT);
  }

  // override member
  defaultEventCallback(item, querySelector = null, type = null) {
    clearTimeout(this.timer);
    this.timer = setTimeout(() => {
      super.defaultEventCallback(item, querySelector, type);
    }, 1000);
  }
}
