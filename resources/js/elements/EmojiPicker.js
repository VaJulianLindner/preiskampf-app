import { HtmxEventListener } from "../lib/HtmxEventListener";

class EmojiPicker extends HtmxEventListener {
    constructor(el, options) {
        super(el, options);

        this.emojiFor = this.el.getAttribute("xui-emoji-for");
        this.emoji = this.el.getAttribute("xui-picked-emoji");
        this.displayElement = document.querySelector(`[xui-emoji-value='${this.emojiFor}']`);
        this.inputElement = document.querySelector(`input[name='${this.emojiFor}']`);
        this.list = document.querySelector(`[xui-toggle-target="emoji-picker-${this.emojiFor}"]`);

        this.on("click", (e) => {
            this.inputElement.value = this.emoji;
            this.displayElement.className = "flex items-center text-left text-xl";
            this.displayElement.innerHTML = this.el.innerHTML;
            // this.list.classList.add("opacity-0", "scale-95", "-z-50");
            // this.list.classList.remove("opacity-100", "scale-100", "z-10");
        });
    }
}

export { EmojiPicker };