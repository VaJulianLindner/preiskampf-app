import { HtmxEventListener } from "../lib/HtmxEventListener";

/**
 * @typedef {Object} HereSuggestion
 * @property {string} id - Here-API internal id of that POI
 * @property {string} title - printed address in iso format
 * @property {object} position - geo position of the address with longitude and latitude
 * @property {float} position.lat
 * @property {float} position.lng
 */

class GeocodingInput extends HtmxEventListener {
    constructor(el, options) {
        super(el, options);

        if (!window.HERE_CLIENT_API_KEY) {
            return;
        }

        this.debounceInMs = options?.debounceInMs || 400;
        this.debounceHandle = null;

        this.hasSuggestions = false;

        this.liFragment = document.getElementById("input_address_suggestion_li");
        this.lngInput = this.el.nextElementSibling;
        this.latInput = this.lngInput.nextElementSibling;
        this.listEl = this.latInput.nextElementSibling;

        this.on("input", this.onChange.bind(this));
        this.on("focus", this.onFocus.bind(this));
    }

    onChange(e) {
        if (this.debounceHandle) {
            clearTimeout(this.debounceHandle);
        }
        this.debounceHandle = setTimeout(this.onChangeDebounced.bind(this), this.debounceInMs);
    }

    onFocus() {
        if (!this.hasSuggestions) {
            return;
        }

        this.showSuggestions();
    }

    /**
     * set the current address and geo location for the geocoding suggestion
     * @constructor
     * @param {HereSuggestion} suggestion - The suggestion
     */
    onClickSuggestion(suggestion) {
        const { title, position } = suggestion;
        this.el.value = title;
        this.lngInput.value = position.lng;
        this.latInput.value = position.lat;

        this.hideSuggestions();
        this.hasSuggestions = false;
        this.listEl.innerHTML = "";
    }

    async onChangeDebounced(isInitialLoad) {
        this.debounceHandle = null;

        /** @type {HereSuggestion[]}} */
        const suggestions = await this.fetchSuggestions();
        this.listEl.innerHTML = "";

        if (!suggestions?.length) {
            this.hideSuggestions();
            this.hasSuggestions = false;
            return;
        }

        suggestions.forEach((suggestion) => {
            const clone = this.liFragment.content.cloneNode(true);
            clone.querySelector("[xui-li-text]").innerHTML = suggestion.title;
            if (this.isActiveSuggestion(suggestion)) {
                this.setCloneActive(clone);
            }
            clone.firstElementChild.addEventListener("click", this.onClickSuggestion.bind(this, suggestion));
            this.listEl.appendChild(clone);
        });
        this.hasSuggestions = true;
        this.showSuggestions();
    }

    async fetchSuggestions() {
        if (!this.el.value) {
            return [];
        }

        const url = new URL("https://autosuggest.search.hereapi.com/v1/autosuggest");
        url.searchParams.set("apikey", window.HERE_CLIENT_API_KEY);
        url.searchParams.set("at", "51.1657,10.4515");
        url.searchParams.set("q", this.el.value);
        const response = await fetch(url.href);
        const json = await response.json();
        return json?.items || [];
    }

    /**
     * @constructor
     * @param {HereSuggestion} suggestion - The suggestion
     */
    isActiveSuggestion(suggestion) {
        return ((suggestion.position.lng == this.lngInput.value) && (suggestion.position.lat == this.latInput.value));
    }

    /**
     * @constructor
     * @param {HTMLElement} clone
     */
    setCloneActive(clone) {
        clone.querySelector("[xui-li-checkmark]").classList.remove("hidden");
        const cl = clone.querySelector("[xui-block-toggle]").classList;
        cl.add("text-emerald-400");
        cl.remove("text-zinc-400", "hover:text-white");
    }

    hideSuggestions() {
        this.listEl.classList.add("opacity-0", "scale-95", "-z-50");
        this.listEl.classList.remove("opacity-100", "scale-100", "z-10");
    }

    showSuggestions() {
        this.listEl.classList.add("opacity-100", "scale-100", "z-10");
        this.listEl.classList.remove("opacity-0", "scale-95", "-z-50");
    }
};

export { GeocodingInput };