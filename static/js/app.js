// Global variables
var splide;

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
    init_on_page("experience", allowSidewaysScroll, document.getElementById("scrollable"), splide)
}

// Splide.JS code
function init_splide() {
    // Init splide
    splide = new Splide(".splide", {
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
function init_on_page(page_name, func, ...args) {
    // Check if current page is specified page
    var path = window.location.pathname;
    var page = path.split("/").pop();

    if (page !== page_name) return;

    // Call function
    func(...args);
}

// Allow sideways scrolling for Splide.JS
function allowSidewaysScroll(element, splide) {
    let debounceDelay = 500;
    let lastScrollTime = Date.now();

    //Add horizontal scrolling support to splide sliders
    element.addEventListener('wheel', function (event) {

        if (event.ctrlKey) {
            return; // Exit the function if Ctrl key is held
        }

        let deltaX = event.deltaX;
        let deltaY = event.deltaY;

        //Windows shift-scroll
        if (event.shiftKey && Math.abs(deltaX) === 0) {
            event.preventDefault();

            if (deltaY > 0) {
                splide.go('+1');

            } else if (deltaY < 0) {
                splide.go('-1');
            }
        }

        //Mac sideways touch scroll
        if (!event.shiftKey && Math.abs(deltaX) !== 0) {
            if (Math.abs(event.deltaX) < Math.abs(event.deltaY)) {
                // Scrolling more vertically than horizontally. Let it be!
                return;
            }

            const scrollLeftMax = element.scrollWidth - element.offsetWidth

            if (
                element.scrollLeft + event.deltaX < 0 ||
                element.scrollLeft + event.deltaX > scrollLeftMax
            ) {
                //Stop backwards nagivation on sideways scroll on element
                event.preventDefault();
            }

            let currentTime = Date.now();
            let timeElapsed = currentTime - lastScrollTime;

            //Allow no delay on first scroll of slider per second
            if (timeElapsed > 1000) {
                if (deltaX > 0) {
                    splide.go('+1');
                } else if (deltaX < 0) {
                    splide.go('-1');
                }

                lastScrollTime = currentTime;
            } else {
                //Debounce scrolling to avoid scrolling to the end of the slider on first scroll
                if (timeElapsed >= debounceDelay) {

                    if (deltaX > 3) {
                        splide.go('+1');
                    } else if (deltaX < -3) {
                        splide.go('-1');
                    }

                    lastScrollTime = currentTime;
                }
            }
        }
    }, false);
}
