document.addEventListener('DOMContentLoaded', () => {
    document.body.addEventListener('htmx:beforeSwap', (evt) => {
        if ([400, 401, 403, 404, 500].includes(evt.detail.xhr.status)) {
            evt.detail.shouldSwap = true;
            evt.detail.isError = false;
        }
    });

    class Bucket {
        constructor() {
            this._notify = new Notyf({
                dismissible: true,
                duration: 5000,
            });
        }

        notify(message) {
            this._notify.success(message);
        }
    }

    window.bckt = new Bucket();
});
