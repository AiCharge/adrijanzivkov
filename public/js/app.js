// The only JavaScript the public site needs, replacing the Vue `v-reveal` directive and the
// navbar's menu state. Everything works without it: it only adds animation and the mobile menu.

// Scroll reveal: elements marked `data-reveal` (optionally `data-reveal="150"` for a 150 ms delay)
// fade and lift into view the first time they scroll onto the screen. app.css hides them only
// while the `js` class is on <html>, which base.html adds before the page paints.
function setUpReveal() {
    const elements = document.querySelectorAll('[data-reveal]');

    if (!('IntersectionObserver' in window) || window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
        elements.forEach(finishReveal);
        return;
    }

    const observer = new IntersectionObserver(
        (entries) => {
            for (const entry of entries) {
                if (!entry.isIntersecting) {
                    continue;
                }

                const element = entry.target;
                element.addEventListener('transitionend', function onEnd(event) {
                    if (event.target === element && event.propertyName === 'transform') {
                        element.removeEventListener('transitionend', onEnd);
                        finishReveal(element);
                    }
                });
                element.classList.add('is-revealed');
                observer.unobserve(element);
            }
        },
        { rootMargin: '0px 0px -8% 0px', threshold: 0.08 },
    );

    for (const element of elements) {
        const delay = Number(element.dataset.reveal);
        if (delay > 0) {
            element.style.setProperty('--reveal-delay', `${delay}ms`);
        }
        observer.observe(element);
    }
}

// Hands the element back to its own transitions (such as card hover effects) once it has appeared.
function finishReveal(element) {
    element.removeAttribute('data-reveal');
    element.classList.remove('is-revealed');
    element.style.removeProperty('--reveal-delay');
}

// Mobile menu: the hamburger button opens and closes the menu panel in the same navbar.
function setUpMenus() {
    for (const root of document.querySelectorAll('[data-menu-root]')) {
        const button = root.querySelector('[data-menu-toggle]');
        const menu = root.querySelector('[data-menu]');

        button.addEventListener('click', () => {
            const isOpen = menu.hidden;
            menu.hidden = !isOpen;
            button.setAttribute('aria-expanded', String(isOpen));
            button.setAttribute('aria-label', isOpen ? 'Затвори мени' : 'Отвори мени');
            root.querySelector('[data-menu-icon="open"]').classList.toggle('hidden', isOpen);
            root.querySelector('[data-menu-icon="close"]').classList.toggle('hidden', !isOpen);
        });
    }
}

// Forms marked `data-submitting`: once sent, the button is disabled and shows its spinner, so a
// double click can't send the form twice.
function setUpSubmitting() {
    for (const form of document.querySelectorAll('form[data-submitting]')) {
        form.addEventListener('submit', () => {
            const button = form.querySelector('button[type="submit"]');
            button.disabled = true;
            button.querySelector('[data-spinner]')?.classList.remove('hidden');
            const label = button.querySelector('[data-label]');
            if (label) {
                label.textContent = label.dataset.label;
            }
        });
    }
}

setUpReveal();
setUpMenus();
setUpSubmitting();
