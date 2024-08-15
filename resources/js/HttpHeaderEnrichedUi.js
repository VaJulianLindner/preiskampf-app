const crlfRegex = /(\r)/g;

function parseCustomHeaders(xhr) {
    if (!xhr) {
        return {};
    }

    return xhr.getAllResponseHeaders()
        .replace(crlfRegex, "")
        .split("\n")
        .filter(Boolean)
        .reduce((acc, curVal) => {
            if (curVal.startsWith("xui")) {
                const headerDelimiterPos = curVal.indexOf(":");
                acc[curVal.substring(0, headerDelimiterPos)] = curVal.substring(headerDelimiterPos + 1).trim();
            }
            return acc;
        }, {});
}

function handleClientAction(selector, method, args) {
    try {
        const el = document.querySelector(selector);
        if (method === "addClass") {
            el.classList.add(...args);
        } else if (method === "removeClass") {
            el.classList.remove(...args);
        } else {
            // fuq, some funcs need the el or sth bound as "this".. but sometimes document etc 
            // => might try with this first and then with el in catch: IllegalInvocationError
            el?.[method]?.apply(el, args);
        }
    } catch (e) {
        console.log(`handleClientAction: error while trying to invoke '${method}' on '${selector}': "${e.toString()}" "${e.code}"`);
    }
}

function onEvent(name, e) {
    if (name === "htmx:configRequest") {
        const verb = e.detail.verb;
        const isNavigationElement = e.detail.elt.getAttribute("xui-el") === "Navigation";
        //strip the query params off
        const path = new URL(window.location.origin + e.detail.path.replace(window.location.origin, "")).pathname;

        if (verb === "get" && isNavigationElement) {    
            console.debug(`'${verb}': '${path}' got prevented because is regarded as routing action handled by lib/router`);
            e.preventDefault();
        }

        const isDisabled = e.detail.elt.getAttribute("xui-hx-disabled") === "1";
        if (isDisabled) {
            console.warn(`'${verb}': '${path}' got prevented!`);
            e.preventDefault();
        }

        const propagationStop = e.detail.triggeringEvent?.target?.getAttribute?.("xui-stop-propagation");
        if (propagationStop === "1") {
            console.warn(`'${verb}': '${path}' got prevented because there is a blocking element!`);
            e.preventDefault();
        } else if (propagationStop === verb) {
            console.warn(`'${verb}': '${path}' got prevented because there is a blocking element for verb '${verb}'!`);
            e.preventDefault();
        } else if (propagationStop === path) {
            console.warn(`'${verb}': '${path}' got prevented because there is a blocking element for path '${path}'!`);
            e.preventDefault();
        }
    } else if (name === "htmx:beforeSwap") {
        const isRedirect = String(e.detail.xhr.status).startsWith("3");
        const toUrl = e.detail.xhr.getResponseHeader("xui-redirect");

        if (!isRedirect) {
            return;
        }

        if (!toUrl) {
            const location = e.detail.xhr.getResponseHeader("location");
            if (!location) {
                return;
            }
            return window.router.push(location, {reload: true});
        }

        // e.detail.shouldSwap = false;
        window.router.push(toUrl, {
            target: e.detail.xhr.getResponseHeader("xui-target"),
            swap: e.detail.xhr.getResponseHeader("xui-swap"),
            withTransition: true,
        });
    } else if (name === "htmx:afterSettle") {
        const customHeader = e.detail.xhr.getResponseHeader("xui-client-action");
        const actions = customHeader?.split(",");
        if (!actions?.length) {
            return;
        }

        actions.forEach((action) => {
            const parts = action.trim().split(":");
            const selector = parts.shift();
            const method = parts.shift();
            handleClientAction(selector, method, parts);
        });
    } else if (name === "xui:clientAction") {
        const clientActions = e.detail?.value || [];
        clientActions.forEach(action => {
            const { selector, method, args } = action;
            handleClientAction(selector, method, args);
        });
    }
}

export { onEvent };