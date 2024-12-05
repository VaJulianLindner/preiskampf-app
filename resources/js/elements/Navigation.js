import { HtmxEventListener } from "../lib/HtmxEventListener";
import { isEventDisabled } from "../HttpHeaderEnrichedUi";

const NAVIGATION_TOGGLE_ATTRIBUTE = "xui-navigation-toggle";
const NAVIGATION_TOGGLE_ICON = "x-navigation-icon";
const NAVIGATION_CONTENT = "x-navigation-content";
const NAVIGATION_LIST = "x-navigation-list";
const HEADER_MAIN_ATTRIBUTE = "x-header-main";
const MAIN_HEADER_ACTIVE_CLASSES = ["bg-zinc-900"];
const MAIN_HEADER_PASSIVE_CLASSES = ["backdrop-blur", "bg-zinc-900/[var(--bg-opacity-dark)]"];
const NAVIGATION_ACTIVE_CLASSES = ["font-bold", "text-white"];
const NAVIGATION_PASSIVE_CLASS = "text-zinc-400";
const NAVIGATION_ACTIVE_CLASSES_MOBILE = ["text-white"];
const NAVIGATION_PASSIVE_CLASS_MOBILE = "text-zinc-400";

class Navigation extends HtmxEventListener {
    constructor(el, options) {
        super(el, options);

        // TODO actually use the href attribute on navigation/links and make these event-listeners obsolete
        this.on("click", this.onClick.bind(this));
        this.on("auxclick", this.onClick.bind(this));
    }

    onClick(e) {
        e.preventDefault();
        e.stopPropagation();

        const isClick = e.type === "click";
        const isAuxClick = e.type === "auxclick";
        const openInNewTab = isAuxClick || (isClick && e?.ctrlKey);

        if (isAuxClick && (String(e.which) !== "2")) {
            return;
        }

        const url = this.el.getAttribute("hx-get");
        const isDisabled = this.el.getAttribute("xui-hx-disabled") === "1";
        const isBlocked = isEventDisabled("get", url, null, e.target);

        if (isDisabled || isBlocked) {
            return;
        }

        const detailViewSelector = this.el.getAttribute("xui-detail-view");
        if (detailViewSelector) {
            const img = document.querySelector(detailViewSelector);
            if (img) {
                img.style.viewTransitionName = "thumbnail-detail-transition";
            }
        }

        const resetScroll = this.el.hasAttribute("xui-is-nav") || this.el.hasAttribute("xui-is-nav-mobile");
        
        Router.push(url, {
            withTransition: this.el.getAttribute("hx-swap")?.indexOf("transition:true") > -1,
            openInNewTab: openInNewTab,
            isEnteringDetailView: detailViewSelector?.length > 0,
        }, false, resetScroll);
    }
}

Navigation.toggle = (forcedCurrentState) => {
    const navigationToggle = document.querySelector(`[${NAVIGATION_TOGGLE_ATTRIBUTE}]`);

    if (!navigationToggle) {
        console.warn("Navigation::toggle is disabled, no el found for", `[${NAVIGATION_TOGGLE_ATTRIBUTE}]`);
        return;
    }

    const navigationContent = document.querySelector(`[${NAVIGATION_CONTENT}]`);
    const navigationList = document.querySelector(`[${NAVIGATION_LIST}]`);
    const mainHeader = document.querySelector(`[${HEADER_MAIN_ATTRIBUTE}]`);
    const currentState = navigationToggle.getAttribute(NAVIGATION_TOGGLE_ATTRIBUTE);

    if (forcedCurrentState && (currentState !== forcedCurrentState)) {
        return;
    }

    let state = forcedCurrentState || currentState;

    // TODO, fix the transition
    if (navigationContent) {
        // navigationContent.remove();
    }

    switch (state) {
        case "open":
            document.querySelectorAll(`[${NAVIGATION_TOGGLE_ICON}='${state}']`).forEach(el => el.style.display = "none");

            state = "closed";
            document.querySelectorAll(`[${NAVIGATION_TOGGLE_ICON}='${state}']`).forEach(el => el.style.display = "initial");
            navigationToggle.setAttribute(NAVIGATION_TOGGLE_ATTRIBUTE, state);
            mainHeader.classList.add(...MAIN_HEADER_PASSIVE_CLASSES);
            mainHeader.classList.remove(...MAIN_HEADER_ACTIVE_CLASSES);
            if (navigationContent) {
                navigationContent.style.opacity = "0";
            }
            if (navigationList) {
                navigationList.style.transform = "translateX(-100%)";
            }
            break;
        case "closed":
            document.querySelectorAll(`[${NAVIGATION_TOGGLE_ICON}='${state}']`).forEach(el => el.style.display = "none");

            state = "open";
            document.querySelectorAll(`[${NAVIGATION_TOGGLE_ICON}='${state}']`).forEach(el => el.style.display = "initial");
            navigationToggle.setAttribute(NAVIGATION_TOGGLE_ATTRIBUTE, state);
            mainHeader.classList.add(...MAIN_HEADER_ACTIVE_CLASSES);
            mainHeader.classList.remove(...MAIN_HEADER_PASSIVE_CLASSES);
            if (navigationContent) {
                navigationContent.style.opacity = "1";
            }
            if (navigationList) {
                navigationList.style.transform = "translateX(0px)";
            }
            break;
        default:
            break;
    }
};

Navigation.markActive = (content, urlPath) => {
    const navigationLinks = content.querySelectorAll("[xui-el='Navigation'][xui-is-nav]");
    navigationLinks.forEach(el => {
        const href = el.getAttribute("hx-get");
        if (href.startsWith(urlPath)) {
            el.classList.add(...NAVIGATION_ACTIVE_CLASSES);
            el.classList.remove(NAVIGATION_PASSIVE_CLASS);
        } else {
            el.classList.remove(...NAVIGATION_ACTIVE_CLASSES);
            el.classList.add(NAVIGATION_PASSIVE_CLASS);
        }
    });
    
    const navigationLinksMobile = content.querySelectorAll("[xui-el='Navigation'][xui-is-nav-mobile]");
    navigationLinksMobile.forEach(el => {
        const href = el.getAttribute("hx-get");
        if (href.startsWith(urlPath)) {
            el.classList.add(...NAVIGATION_ACTIVE_CLASSES_MOBILE);
            el.classList.remove(NAVIGATION_PASSIVE_CLASS_MOBILE);
            el.querySelector("[xui-nav-bg-mark]").classList.add("bg-emerald-400");
        } else {
            el.classList.remove(...NAVIGATION_ACTIVE_CLASSES_MOBILE);
            el.classList.add(NAVIGATION_PASSIVE_CLASS_MOBILE);
            el.querySelector(".bg-emerald-400")?.classList.remove("bg-emerald-400");
        }
    });
};

export { Navigation, NAVIGATION_CONTENT };