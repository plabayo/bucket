function init() {
    document.body.addEventListener('htmx:beforeSwap', (evt) => {
        if ([400, 404].includes(evt.detail.xhr.status)) {
            evt.detail.shouldSwap = true;
            evt.detail.isError = false;
        }
    });
}

document.addEventListener("DOMContentLoaded", init);

export { };
