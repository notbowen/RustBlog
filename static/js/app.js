const swup = new Swup();
swup.on("contentReplaced", () => {
    hljs.highlightAll();
})