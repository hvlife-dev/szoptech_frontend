import { SelectorEventManager } from "../DefaultSelectorEventManager.js";

const CHANGE = "change";
const TYPE = "select";

export class Select extends SelectorEventManager {
  static instance = null;

  constructor() {
    if (Select.instance) return Select.instance;
    super(TYPE);

    super.addAllEvents(CHANGE);
    super.addSiteEvents(CHANGE);
  }
}
