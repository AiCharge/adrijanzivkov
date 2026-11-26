// The admin's small behaviours, in plain JavaScript (no build step). Each part looks for `data-*`
// attributes in the server-rendered HTML; the pages still work, more plainly, without it.

const root = document.documentElement;

// --- Theme: light, dark or the system's (the head's inline script applies it before the first paint).
function applyTheme(choice) {
    const dark = choice === 'dark' || (choice === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);
    root.classList.toggle('dark', dark);
}

document.querySelectorAll('[data-theme-choice]').forEach((button) => {
    button.addEventListener('click', () => {
        const choice = button.dataset.themeChoice;
        localStorage.setItem('admin-theme', choice);
        applyTheme(choice);
        button.closest('details')?.removeAttribute('open');
    });
});

// --- Sidebar: folds to icons on desktop (remembered), opens as a drawer on phones.
document.querySelectorAll('[data-sidebar-toggle]').forEach((button) => {
    button.addEventListener('click', () => {
        if (window.matchMedia('(min-width: 768px)').matches) {
            const collapsed = root.toggleAttribute('data-sidebar-collapsed');
            localStorage.setItem('admin-sidebar', collapsed ? 'collapsed' : 'open');
        } else {
            root.toggleAttribute('data-sidebar-open');
        }
    });
});

document.querySelectorAll('[data-sidebar-close]').forEach((element) => {
    element.addEventListener('click', () => root.removeAttribute('data-sidebar-open'));
});

// --- Menus are <details> elements: clicking elsewhere, or pressing Escape, closes any open one.
document.addEventListener('click', (event) => {
    document.querySelectorAll('details[data-menu][open]').forEach((menu) => {
        if (!menu.contains(event.target)) {
            menu.removeAttribute('open');
        }
    });
});

document.addEventListener('keydown', (event) => {
    if (event.key === 'Escape') {
        document.querySelectorAll('details[data-menu][open]').forEach((menu) => menu.removeAttribute('open'));
        root.removeAttribute('data-sidebar-open');
    }
});

// --- Confirmation dialogs: a button with data-dialog-open="id" opens <dialog id="id">.
document.querySelectorAll('[data-dialog-open]').forEach((button) => {
    button.addEventListener('click', (event) => {
        event.preventDefault();
        event.stopPropagation();
        document.getElementById(button.dataset.dialogOpen)?.showModal();
    });
});

document.querySelectorAll('[data-dialog-close]').forEach((button) => {
    button.addEventListener('click', () => button.closest('dialog')?.close());
});

// --- Table rows that link somewhere: the whole row is clickable, not just its title.
document.querySelectorAll('tr[data-href]').forEach((row) => {
    row.addEventListener('click', (event) => {
        if (!event.target.closest('a, button, form, dialog')) {
            window.location.href = row.dataset.href;
        }
    });
});

// --- Search boxes send their form once typing pauses, keeping the other filters.
document.querySelectorAll('input[data-auto-submit]').forEach((input) => {
    let timer;
    input.addEventListener('input', () => {
        clearTimeout(timer);
        timer = setTimeout(() => input.form.requestSubmit(), 350);
    });
});

// --- Image pickers: show the chosen file before it's uploaded, and let it be cleared again.
document.querySelectorAll('[data-image-input]').forEach((picker) => {
    const input = picker.querySelector('input[type="file"]');
    const preview = picker.querySelector('[data-image-preview]');
    const placeholder = picker.querySelector('[data-image-placeholder]');
    const fileName = picker.querySelector('[data-image-name]');
    const clear = picker.querySelector('[data-image-clear]');
    const original = preview.getAttribute('src');

    picker.querySelector('[data-image-choose]').addEventListener('click', () => input.click());

    input.addEventListener('change', () => {
        const file = input.files[0];
        if (!file) {
            return;
        }
        preview.src = URL.createObjectURL(file);
        preview.hidden = false;
        placeholder.hidden = true;
        fileName.textContent = file.name;
        fileName.hidden = false;
        clear.hidden = false;
    });

    clear.addEventListener('click', () => {
        input.value = '';
        fileName.hidden = true;
        clear.hidden = true;
        if (original) {
            preview.src = original;
        } else {
            preview.hidden = true;
            placeholder.hidden = false;
        }
    });
});

// --- Forms marked data-submitting: disable the button once sent, so a double click sends once.
document.querySelectorAll('form[data-submitting]').forEach((form) => {
    form.addEventListener('submit', () => {
        form.querySelectorAll('button[type="submit"]').forEach((button) => (button.disabled = true));
    });
});

// --- Toasts: the flash message from the server, and messages from scripts (window.adminToast).
const toastArea = document.querySelector('[data-toasts]');

window.adminToast = (message, kind = 'success') => {
    const template = document.getElementById(kind === 'error' ? 'toast-error' : 'toast-success');
    const toast = template.content.firstElementChild.cloneNode(true);
    toast.querySelector('[data-toast-message]').textContent = message;
    toastArea.append(toast);
    setTimeout(() => toast.remove(), 4500);
};

document.querySelectorAll('[data-flash]').forEach((element) => {
    window.adminToast(element.dataset.flash);
    element.remove();
});
