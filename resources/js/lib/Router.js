import { transition } from "./ViewTransition";
import { Navigation, NAVIGATION_CONTENT } from "../elements/Navigation";
// TODO pre-loading, offline-mode..
// TODO form History API to Navigation API?!

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

class Router {
    constructor() {
        window.addEventListener("popstate", this.on.bind(this));
        window.navigation?.addEventListener("navigate", this.onNavigate.bind(this));
    }

    on(event) {
        /** @type {HistoryState} */
        const historyState = event.state || {};
        const clientScroll = { scrollX: window.scrollX, scrollY: window.scrollY };

        this.execute(historyState, false, true, false, clientScroll);
    }

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

        const shouldTransition = opts.withTransition || (opts.swap.indexOf("transition:true") > -1);
        /** @type {HistoryState} */
        const historyState = { 
            fromUrl: window.location.pathname,
            toUrl: url,
            shouldTransition,
            args,
        };
        const clientScroll = { scrollX: window.scrollX, scrollY: window.scrollY };

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
     * @constructor
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

            await window.htmx.ajax.apply(this, historyState.args);

            if (isHistoryEvent) {   
                window.scrollTo({
                    behavior: "smooth",
                    top: historyState.clientScroll?.scrollY || 0,
                    left: historyState.clientScroll?.scrollX || 0,
                });
                this.restoreFormState(historyState.toUrl);
                return;
            }

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
                formEl.value = val;
            }
        });
    }

    getTransitionCallbacksForUrl(isEnteringDetailView) {       
        const navigationContent = document.querySelector(`[${NAVIGATION_CONTENT}]`);
        const callbacks = isEnteringDetailView ? {
            init: () => {
                document.body.classList.add("xui-entering-detail");
                navigationContent.classList.add("hidden");
            },
            updateCallbackDone: () => {
                document.body.classList.remove("xui-entering-detail");
                navigationContent.classList.remove("hidden");
            },
        } : {
            init: () => {
                navigationContent.classList.add("hidden");
            },
            updateCallbackDone: () => {
                navigationContent.classList.remove("hidden");
            }
        };
        return callbacks;
    }
};

export { Router };