import htmx from "htmx.org";
import { onEvent as HttpHeaderEnrichedUiOnEvent } from "./HttpHeaderEnrichedUi";
import { Router } from "./lib/Router";
import { Notification } from "./elements/Notification";
import { PopoverToggle } from "./elements/PopoverToggle";
import { EmojiPicker } from "./elements/EmojiPicker";
import { Navigation } from "./elements/Navigation";
import { NavigationToggle } from "./elements/NavigationToggle";
import { AjaxFormInput } from "./elements/AjaxFormInput";
import { GeocodingInput } from "./elements/GeocodingInput";

const Elements = {
    Notification,
    PopoverToggle,
    EmojiPicker,
    Navigation,
    NavigationToggle,
    AjaxFormInput,
    GeocodingInput,
};

htmx.defineExtension("http-header-enriched-ui", { onEvent: HttpHeaderEnrichedUiOnEvent });

window.htmx = htmx;
window.Router = new Router();
window.HERE_CLIENT_API_KEY = "OLsRSTH_Diw-raJw5TB5UgfiCHhbCbXlEdOp2Z0IH8U";

if (window.location.search.includes("htmx-debug")) {
    htmx.logAll();
}

const WHITELIST_STATUSCODES_FOR_RENDER = [401, 404, 422];

document.onreadystatechange = function (e, state) {
    if (document.readyState !== "complete") {
        return;
    }

    document.body.addEventListener("htmx:beforeSwap", function (e) {
        if (WHITELIST_STATUSCODES_FOR_RENDER.includes(e.detail.xhr.status)) {
            // allow 401/422 responses to swap as we are using this as a signal that
            // a form was submitted with bad data and want to rerender with the errors
            e.detail.shouldSwap = true;
            e.detail.isError = false;

            // replace document with 404 status page instead of handling 404 of single elements/navigations etc
            if (e.detail.xhr.status === 404) {
                e.detail.target = document.body;
            }
        }
    });
}

htmx.onLoad(content => {
    content.addEventListener("mousedown", hideActiveDialogs);

    const elements = content.getAttribute("xui-el") ? [content] : content.querySelectorAll("[xui-el]");
    elements.forEach(el => {
        try {
            new Elements[el.getAttribute("xui-el")](el);
        } catch (e) {
            console.warn("constructor for", el.getAttribute("xui-el"), "errored:", e.toString());
        }
    });
});

function hideActiveDialogs(e) {
    if (e.target.hasAttribute("hx-get") || e.target.hasAttribute("hx-post") || e.target.hasAttribute("hx-delete") || e.target.hasAttribute("href")) {
        return;
    }

    const elementName = e.target.getAttribute("xui-toggle-btn") || e.target.parentNode?.getAttribute("xui-toggle-btn");
    const isBlockedToggle = e.target.hasAttribute("xui-block-toggle") || e.target.parentNode?.hasAttribute("xui-block-toggle");
    const isToggleButton = !!elementName;

    const activeDialogs = document.querySelectorAll(`.opacity-100[xui-toggle-target]`);
    activeDialogs.forEach(el => {
        if (isBlockedToggle) {
            return;
        } else if (isToggleButton && (el.getAttribute("xui-toggle-target") === elementName)) {
            return;
        }
        el.classList.add("opacity-0", "scale-95", "-z-50");
        el.classList.remove("opacity-100", "scale-100", "z-10");
    });
}