const options = {
    animationSelector: '[class*="transition-fade"]',
    animateHistoryBrowsing: true,
    plugins: [
        new SwupScrollPlugin({
            animateScroll: true
        })
    ]
};

const swup = new Swup(options);

let scrollValues = {};

swup.on('clickLink', () => {
    scrollValues[window.location.href] = window.scrollY;
});

swup.on('popState', () => {
    setTimeout(function() {
        window.scrollTo(0, scrollValues[window.location.href]);
    }, 100);
});

swup.on("contentReplaced", () => {
    hljs.highlightAll();
    load_cards();
})