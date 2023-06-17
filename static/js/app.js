// Swup.JS code
const options = {
    animationSelector: '[class*="transition-fade"]',
    animateHistoryBrowsing: true,
    plugins: [
        new SwupScrollPlugin({
            animateScroll: true,
        }),
    ],
};

const swup = new Swup(options);

let scrollValues = {};

swup.on("clickLink", () => {
    scrollValues[window.location.href] = window.scrollY;
});

swup.on("popState", () => {
    setTimeout(function () {
        window.scrollTo(0, scrollValues[window.location.href]);
    }, 100);
});

swup.on("contentReplaced", () => {
    on_page_load();
});

document.addEventListener("DOMContentLoaded", () => {
    on_page_load();
})

function on_page_load() {
    hljs.highlightAll();
    init_on_page("experience", init_splide);
    init_on_page("experience", load_cards);
}

// Splide.JS code
function init_splide() {
    // Init splide
    var splide = new Splide(".splide", {
        type: "loop",
        perPage: 2,
        perMove: 1,
        gap: 20,
        focus: 0,
        breakpoints: {
            480: {
                perPage: 1,
            },
        },
    });

    splide.mount();
}

// Helper function to call function if on page
function init_on_page(page_name, func) {
    // Check if current page is specified page
    var path = window.location.pathname;
    var page = path.split("/").pop();

    if (page !== page_name) return;

    // Call function
    func();
}
