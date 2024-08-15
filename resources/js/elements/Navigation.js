import { HtmxEventListener } from "../lib/HtmxEventListener";

const NAVIGATION_TOGGLE_ATTRIBUTE = "xui-navigation-toggle";
const NAVIGATION_TOGGLE_ICON = "x-navigation-icon";
const NAVIGATION_CONTENT = "x-navigation-content";
const NAVIGATION_LIST = "x-navigation-list";
const HEADER_MAIN_ATTRIBUTE = "x-header-main";
const MAIN_HEADER_ACTIVE_CLASSES = ["bg-zinc-900"];
const MAIN_HEADER_PASSIVE_CLASSES = ["backdrop-blur", "bg-zinc-900/[var(--bg-opacity-dark)]"];
const NAVIGATION_ACTIVE_CLASSES = ["font-bold", "text-white"];
const NAVIGATION_PASSIVE_CLASS = "text-zinc-400";

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

        if (isDisabled) {
            return;
        }

        const isEnteringDetailView = this.el.getAttribute("xui-detail-view") == "1";
        if (isEnteringDetailView) {
            const img = this.el.querySelector("img");
            if (img) {
                img.style.viewTransitionName = "thumbnail-detail-transition";
            }
        }

        window.router.push(url, {
            withTransition: this.el.getAttribute("hx-swap")?.indexOf("transition:true") > -1,
            openInNewTab: openInNewTab,
            isEnteringDetailView: isEnteringDetailView,
        }, false, this.el.hasAttribute("xui-is-nav"));
    }
}

Navigation.toggle = (forcedCurrentState) => {
    const navigationToggle = document.querySelector(`[${NAVIGATION_TOGGLE_ATTRIBUTE}]`);
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
            document.querySelector(`[${NAVIGATION_TOGGLE_ICON}='${state}']`).style.display = "none";

            state = "closed";
            document.querySelector(`[${NAVIGATION_TOGGLE_ICON}='${state}']`).style.display = "initial";
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
            document.querySelector(`[${NAVIGATION_TOGGLE_ICON}='${state}']`).style.display = "none";

            state = "open";
            document.querySelector(`[${NAVIGATION_TOGGLE_ICON}='${state}']`).style.display = "initial";
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
};

export { Navigation, NAVIGATION_CONTENT };