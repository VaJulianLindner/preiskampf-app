import { transition } from "./ViewTransition";
import { Navigation, NAVIGATION_CONTENT } from "../elements/Navigation";
// TODO pre-loading, offline-mode..
// TODO from History API to Navigation API?!

/**
 * @typedef {Object} XuiRoutingEventProperties
 * @property {string} swap - define how the content will be swapped
 * @property {string} target - define where the content will be swapped into
 */

/**
 * @typedef {Object} HistoryState
 * @property {string} fromUrl - the current url, from which the new history stack entry got pushed
 * @property {string} toUrl - the url, that was pushed on to the history stack
 * @property {boolean} shouldTransition - whether to use the view-transition api or not
 * @property {[string, string, XuiRoutingEventProperties]} args - copy of the arguments, that are applied to the htmx.ajax call
 */

/**
 * @typedef {Object} RouterRoutingOptions
 * @property {boolean} [opts.openInNewTab] - defaults to false
 * @property {boolean} [opts.reload] - instead of using the client-side router, escape and reset window.location, defaults to false
 * @property {string} [opts.method=GET] - defaults to GET
 * @property {string} [opts.target=#swap-content] - defaults to #swap-content
 * @property {string} [opts.swap=innerHTML] - defaults to innerHTML
 * @property {boolean} [opts.withTransition=false] - defaults to false
 * @property {boolean} [opts.isEnteringDetailView=false] - defaults to false
 */

class Router {
    constructor() {
        window.addEventListener("popstate", this.on.bind(this));
        window.navigation?.addEventListener("navigate", this.onNavigate.bind(this));

        // window.DEBUG_CACHE = {};
    }

    on(event) {
        /** @type {HistoryState} */
        const historyState = event.state || {};
        const clientScroll = { scrollX: window.scrollX, scrollY: window.scrollY };

        this.execute(historyState, false, true, false, clientScroll);
    }

    /**
     * push an entry into the client-side router
     * @param {string} url
     * @param {RouterRoutingOptions} opts
     * @param {boolean} [replace=false]
     * @param {boolean} [resetScroll=false]
     */
    push(url, opts = {}, replace = false, resetScroll = false) {
        if (opts.openInNewTab) {
            window.open(url, "_blank");
            return;
        }

        if (opts.reload) {
            window.location.href = url;
            return;
        }

        const args = [opts.method || "GET", url, {
            ...opts,
            target: opts.target || "#swap-content",
            swap: (opts.swap || `innerHTML`).replace("transition:true", ""),
        }];

        const shouldTransition = opts.withTransition || (opts.swap?.indexOf("transition:true") > -1);
        /** @type {HistoryState} */
        const historyState = { 
            fromUrl: window.location.pathname,
            toUrl: url,
            shouldTransition,
            args,
        };
        const clientScroll = { scrollX: window.scrollX, scrollY: window.scrollY };

        // should have a possible scrollTarget
        console.warn("TODO implement scrollTarget option for RouterRoutingOptions for e.g. navigating during pagination actions (scroll to list top etc)")
        if (opts.isEnteringDetailView) {
            resetScroll = true;
        }

        this.execute(historyState, replace, false, opts.isEnteringDetailView, clientScroll, resetScroll);
    }

    replace(url, opts) {
        return this.push(url, {
            ...(opts || {}),
        }, true);
    }

    pop(opts) {
        if (opts?.withTransition) {
            transition(() => {
                window.history.back();
            });
        } else {
            window.history.back();
        }
    }

    onNavigate(e) {
        console.debug("onNavigate", e, "currentHistoryIndex", e.currentTarget.currentEntry.index, "destinationHistoryIndex", e.destination.index);
    }

    /**
     * set the current address and geo location for the geocoding suggestion
     * @param {HistoryState} historyState
     * @param {boolean} replace
     * @param {boolean} isHistoryEvent
     * @param {object} clientScroll
     * @param {number} clientScroll.scrollX
     * @param {number} clientScroll.scrollY
     * @param {boolean} resetScroll
     */
    execute(historyState, replace, isHistoryEvent, isEnteringDetailView, clientScroll, resetScroll) {
        if (!historyState.toUrl) {
            return window.location.reload();
        }

        async function update() {
            const headerEntries = Object.entries(historyState.args[2]?.headers || {});
            const isBoosted = headerEntries.find(entry => entry?.[0] === "Hx-Boosted");
            if (isBoosted && isHistoryEvent) {
                console.warn("navigated back to a boosted action, rewrite it to an ajax action..", historyState.args);
                historyState.args[2].headers = Object.fromEntries(headerEntries.filter(entry => entry?.[0] !== "Hx-Boosted"))
                historyState.args[2].swap = "innerHTML";
                historyState.args[2].target = "#swap-content";
            }

            // if (isHistoryEvent && window.DEBUG_CACHE[historyState.toUrl]) {
            //     const cachedVal = window.DEBUG_CACHE[historyState.toUrl];
            //     document.querySelector(historyState.args[2].target)[historyState.args[2].swap] = cachedVal;
            // } else {
            await window.htmx.ajax.apply(this, historyState.args);
            // }

            if (isHistoryEvent) {
                this.restoreClientScroll(historyState.clientScroll);
                this.restoreFormState(historyState.toUrl);
                return;
            }

            // window.DEBUG_CACHE[historyState.toUrl] = document.querySelector(historyState.args[2].target)[historyState.args[2].swap];

            if (!replace) {
                this.saveClientScroll(clientScroll);
            }
            if (resetScroll) {
                window.scrollTo(0, 0);
            }

            window.history[replace ? "replaceState" : "pushState"](historyState, null, historyState.toUrl);
        }

        this.updateNavigationState(historyState.toUrl);
        if (historyState.shouldTransition) {
            transition(update.bind(this), this.getTransitionCallbacksForUrl(isEnteringDetailView));
        } else {
            update.apply(this);
        }
    }

    restoreClientScroll(clientScroll) {
        console.warn("restoreClientScroll is currently disabled");
        // if (!clientScroll) {
        //     return;
        // }
        // window.scrollTo({
        //     behavior: "smooth",
        //     top: clientScroll.scrollY || 0,
        //     left: clientScroll.scrollX || 0,
        // });
    }

    saveClientScroll(clientScroll) {
        if (clientScroll && window.history.state) {
            // before navigating away, add the current scroll position to the current history stack entry
            window.history.replaceState({
                ...window.history.state,
                clientScroll,
            }, "", window.history.state.toUrl);
        }
    }

    updateNavigationState(urlPath) {
        let url = urlPath || window.location.pathname;
        if (url.indexOf("?") !== -1) {
            url = url.substring(0, url.indexOf("?"));
        }
        Navigation.markActive(document.body, url);
        Navigation.toggle("open");
    }

    restoreFormState(toUrl) {
        const url = new URL(window.location.origin + toUrl);
        url.searchParams.forEach((val, key) => {
            const formEl = document.querySelector(`[name='${key}']`);
            if (formEl) {
                formEl.value = decodeURIComponent(val);
            }
        });
    }

    getTransitionCallbacksForUrl(isEnteringDetailView) {       
        const navigationContent = document.querySelector(`[${NAVIGATION_CONTENT}]`);
        const callbacks = isEnteringDetailView ? {
            init: () => {
                document.body.classList.add("xui-entering-detail");
                navigationContent?.classList.add("hidden");
            },
            updateCallbackDone: () => {
                document.body.classList.remove("xui-entering-detail");
                navigationContent?.classList.remove("hidden");
            },
        } : {
            init: () => {
                navigationContent?.classList.add("hidden");
            },
            updateCallbackDone: () => {
                navigationContent?.classList.remove("hidden");
            }
        };
        return callbacks;
    }
};

export { Router };