self.addEventListener("install", event => {
    console.log("Service worker installing...");
});

self.addEventListener("activate", event => {
    console.log("Service worker activating...");
});

self.addEventListener("fetch", event => {
    event.respondWith((async () => {
        return await fetch(event.request);
      })());
});