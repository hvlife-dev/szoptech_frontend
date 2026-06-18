export class SelectorHelper {
  static selectors = (selectors) => {
    return Array.isArray(selectors || "") ? selectors.join(" ") : selectors;
  };

  static senderValues = (item, documentSelector = null, type = null) => {
    console.log(documentSelector);

    const pathname = window.location.pathname;

    const setCategory = () => {
      return !item?.category
        ? type
          ? type.toUpperCase()
          : "Undefined category"
        : item.category;
    };

    const setValue = () => {
      const value = item?.value || "";

      let result = value;
      if (!value || value === "") return value;

      if (Array.isArray(value)) {
        const selector = value.join("");

        if (selector[0] === "[") {
          result = documentSelector.getAttribute(
            selector.substring(1, selector.length - 1),
          );
        } else
          result = documentSelector.querySelector(value.join("")).innerText;
      }
      result =
        result.length > 30
          ? `${result.substring(0, 27).trimRight()}...`
          : result;

      return result;
    };

    return [pathname, setCategory(), setValue()];
  };
}
