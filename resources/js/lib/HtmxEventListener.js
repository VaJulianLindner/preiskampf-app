class HtmxEventListener {
    /**
     * @constructor
     * @param {HTMLElement} el - html-element, that initialized this js-element
     */
    constructor(el, options) {
        this.el = el;
        this.abortControllers = [];

        // this.on("htmx:afterSettle", (e) => {
        //     console.log(`class "${this.constructor.name}" received event "htmx:afterSettle"`);
        //     const {srcElement, target} = e;
        //     if (this.el.isSameNode(srcElement)) {
        //         console.log(`class "${this.constructor.name}" is srcElement`);
        //     }
        //     if (this.el.isSameNode(target)) {
        //         console.log(`class "${this.constructor.name}" is target`);
        //     }
        // });
    }

    on(eventName, cb) {
        const controller = new AbortController();
        if (eventName.startsWith("htmx")) {
            document.body.addEventListener(eventName, cb, { signal: controller.signal });
        } else {
            this.el.addEventListener(eventName, cb, { signal: controller.signal });
        }
        this.abortControllers.push(controller);
    }

    destroy() {
        this.abortControllers.forEach(signal => signal.abort());
        this.el?.remove?.();
        delete this.abortControllers;
        delete this.el;
    }
}

export { HtmxEventListener };