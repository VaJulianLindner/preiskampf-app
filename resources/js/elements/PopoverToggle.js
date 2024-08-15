import { HtmxEventListener } from "../lib/HtmxEventListener";

class PopoverToggle extends HtmxEventListener {
    constructor(el, options) {
        super(el, options);

        this.list = document.querySelector(`[xui-toggle-target='${this.el.getAttribute("xui-toggle-btn")}']`);

        this.on("click", (e) => {
            e.stopPropagation();
            const isActive = this.list.classList.contains("opacity-100");
            if (isActive) {
                this.hide(this.list);
            } else {
                this.show(this.list);
            }
        });
    }

    hide(list) {
        list.classList.add("opacity-0", "scale-95", "-z-50");
        list.classList.remove("opacity-100", "scale-100", "z-10");
    }

    show(list) {
        list.classList.add("opacity-100", "scale-100", "z-10");
        list.classList.remove("opacity-0", "scale-95", "-z-50");
    }
}

PopoverToggle.select = (content) => {
    return content.querySelectorAll(`[xui-toggle-btn]`);
};

PopoverToggle.initialize = (content) => {
    const candidates = PopoverToggle.select(content);
    candidates.forEach(el => new PopoverToggle(el));
}

export { PopoverToggle };