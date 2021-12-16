function registerServiceWorker() {
    if ('serviceWorker' in navigator) {
        navigator.serviceWorker
            .register('service-worker.js', {
                "scope": "/"
            })
            .then(_ => {
                console.log("Service worker registered")
            })
            .catch(error => {
                console.log("Fail to register service worker", error);
            });
    }
}

window.onload = async function() {
    showLoadingView();
    initializeControl();
    registerServiceWorker();
    initializePwa();
    if (state.login) {
        await loadItems();
    }
    refreshView();
}