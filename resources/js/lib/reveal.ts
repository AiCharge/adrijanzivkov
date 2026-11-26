import type { Directive } from 'vue';

let observer: IntersectionObserver | null = null;

function getObserver(): IntersectionObserver {
    if (!observer) {
        observer = new IntersectionObserver(
            (entries) => {
                for (const entry of entries) {
                    if (entry.isIntersecting) {
                        const element = entry.target as HTMLElement;

                        const onTransitionEnd = (event: TransitionEvent) => {
                            if (event.target === element && event.propertyName === 'transform') {
                                element.removeEventListener('transitionend', onTransitionEnd);
                                cleanUp(element);
                            }
                        };

                        element.addEventListener('transitionend', onTransitionEnd);
                        element.classList.add('is-revealed');
                        observer?.unobserve(element);
                    }
                }
            },
            { rootMargin: '0px 0px -8% 0px', threshold: 0.08 },
        );
    }

    return observer;
}

/**
 * Hands the element back to its own transitions (hover effects) once it has appeared.
 */
function cleanUp(element: HTMLElement): void {
    element.classList.remove('reveal', 'is-revealed');
    element.style.removeProperty('--reveal-delay');
}

/**
 * Fades and lifts an element into view the first time it scrolls into the viewport.
 * Pass a number to delay it in milliseconds, e.g. v-reveal="150" to stagger siblings.
 */
export const reveal: Directive<HTMLElement, number | undefined> = {
    getSSRProps() {
        return {};
    },
    mounted(element, binding) {
        if (typeof IntersectionObserver === 'undefined' || window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
            return;
        }

        element.classList.add('reveal');

        if (binding.value) {
            element.style.setProperty('--reveal-delay', `${binding.value}ms`);
        }

        getObserver().observe(element);
    },
    unmounted(element) {
        observer?.unobserve(element);
    },
};
