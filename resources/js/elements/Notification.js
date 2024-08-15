import { HtmxEventListener } from "../lib/HtmxEventListener";
// import { transition } from "../lib/ViewTransition";

class Notification extends HtmxEventListener {
    constructor(el, options) {
        super(el, options);

        this.show();
    }

    show() {
        if (!this.el?.innerHTML?.trim().length) {
            return;
        }

        const timeoutHandle = setTimeout(() => this.hide(), 3000);
        this.el.querySelector("[xui-close-button]").onclick = () => {
            clearTimeout(timeoutHandle);
            this.hide();
        };

        this.el.classList.add("translate-y-0", "opacity-100", "sm:translate-x-0");
        this.el.classList.remove("-translate-y-12", "opacity-0", "sm:translate-y-0", "sm:translate-x-2");
    }

    hide() {
        this.el.classList.add("opacity-0");
        this.el.classList.remove("opacity-100");
        this.destroy?.();
    }

}

export { Notification };