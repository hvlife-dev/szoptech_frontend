import { SelectorEventManager } from "../DefaultSelectorEventManager";

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

  constructor() {
    if (Input.instance) return Input.instance;

    super(TYPE);
    Input.instance = this;

    let timer;
    let oldValue = "";

    const changeCallback = (e) => {
      clearTimeout(timer);

      timer = setTimeout(() => {
        oldValue = e.target.value;
        console.log(oldValue);
      }, 1000);
    };

    const blurCallback = (e) => {
      clearTimeout(timer);
      const value = e.target.value;
      if (value !== oldValue) console.log(value);
    };

    super.addSiteEvents(EVENTS.INPUT, changeCallback);
    super.addSiteEvents(EVENTS.BLUR, blurCallback);
    super.addSiteEvents(EVENTS.FOCUS, (e) => {
      oldValue = e.target.value;
    });

    super.addAllEvents(EVENTS.INPUT, changeCallback);
    super.addAllEvents(EVENTS.BLUR, blurCallback);
    super.addAllEvents(EVENTS.FOCUS, (e) => {
      oldValue = e.target.value;
    });
  }
}
