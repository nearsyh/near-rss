function initializePwa() {
    window.addEventListener('beforeinstallprompt', (event) => {
        console.log('👍', 'beforeinstallprompt', event);
        // Stash the event so it can be triggered later.
        window.deferredPrompt = event;
    });

    window.addEventListener('appinstalled', (event) => {
        console.log('👍', 'appinstalled', event);
        // Clear the deferredPrompt so it can be garbage collected
        window.deferredPrompt = null;
    });
}
