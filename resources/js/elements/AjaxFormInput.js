import { HtmxEventListener } from "../lib/HtmxEventListener";

const CHANGE_EVENT_ON_TAGS = ["radio", "checkbox"];

class AjaxFormInput extends HtmxEventListener {
    constructor(el, options) {
        super(el, options);

        this.debounceInMs = options?.debounceInMs || 400;

        const eventName = CHANGE_EVENT_ON_TAGS.includes(this.el.type) ? "change" : "input";
        this.on(eventName, this.onChange.bind(this));

        this.debounceHandle = null;
    }

    onChange(e) {
        if (this.debounceHandle) {
            clearTimeout(this.debounceHandle);
        }
        this.debounceHandle = setTimeout(this.onChangeDebounced.bind(this), this.debounceInMs);
    }

    onChangeDebounced() {
        this.debounceHandle = null;

        const method = String(this.el.form.getAttribute("hx-method") || "get").toUpperCase();
        if (method !== "GET") {
            this.el.form.requestSubmit();
            return;
        }

        const isValid = this.el.form.reportValidity();
        if (!isValid) {
            return;
        }

        let url = this.el.form.getAttribute("hx-get");
        const formData = new FormData(this.el.form);
        const entries = formData.entries();
        for (const [key, val] of entries) {
            if (url.indexOf("?") === -1) {
                url += `?${key}=${val}`;
            } else {
                url += `&${key}=${val}`;
            }
        }

        const swap = this.el.form.getAttribute("hx-swap");
        const target = this.el.form.getAttribute("hx-target");
        const pushUrl = this.el.form.getAttribute("hx-push-url");
        const headers = this.el.form.getAttribute("hx-headers");
        const routerOpts = {
            method: method,
            swap: swap,
            target: target,
        };

        if (headers) {
            try {
                routerOpts.headers = JSON.parse(headers);
            } catch (e) {/*ignore*/}
        }

        if (pushUrl) {
            window.router.push(url, routerOpts);
        } else {
            window.router.replace(url, routerOpts);
        }
    }
};

export { AjaxFormInput };