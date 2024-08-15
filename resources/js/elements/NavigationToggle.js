import { HtmxEventListener } from "../lib/HtmxEventListener";
import { Navigation } from "./Navigation";
import { transition } from "../lib/ViewTransition";

class NavigationToggle extends HtmxEventListener {
    constructor(el, options) {
        super(el, options);

        this.on("click", this.toggle);
    }

    toggle() {
        Navigation.toggle();
    }
}

export { NavigationToggle };