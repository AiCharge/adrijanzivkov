// The admin's rich-text editor (Tiptap), bundled by esbuild into public/js/editor.js
// (`pnpm run build:editor`). It's the only part of the site that needs a JavaScript library.
//
// Markup it expects (see templates/admin/components/ui.html, `rich_editor`):
//   <div data-rich-editor>
//     <div data-toolbar> <button data-command="bold">…</button> … </div>
//     <div data-editor-content></div>
//     <input type="hidden" name="description" value="<p>…</p>" data-editor-input>
//     <input type="file" accept="image/*" hidden data-editor-image>
//   </div>
// The hidden input holds the HTML, so the surrounding <form> sends it like any other field.

import { Editor } from '@tiptap/core';
import { Image } from '@tiptap/extension-image';
import { TextAlign } from '@tiptap/extension-text-align';
import StarterKit from '@tiptap/starter-kit';

const csrfToken = () => document.querySelector('meta[name="csrf-token"]')?.content ?? '';

function setUp(root) {
    const input = root.querySelector('[data-editor-input]');
    const imageInput = root.querySelector('[data-editor-image]');
    const buttons = [...root.querySelectorAll('[data-command]')];

    const editor = new Editor({
        element: root.querySelector('[data-editor-content]'),
        content: input.value,
        extensions: [
            StarterKit.configure({
                heading: { levels: [2, 3] },
                link: { openOnClick: false, autolink: true, defaultProtocol: 'https' },
                trailingNode: false,
            }),
            Image,
            TextAlign.configure({ types: ['heading', 'paragraph'] }),
        ],
        editorProps: {
            attributes: {
                class: 'admin-rich-content min-h-64 px-3 py-2.5 text-sm outline-none',
                ...(root.dataset.id ? { id: root.dataset.id } : {}),
            },
        },
        onUpdate: ({ editor }) => {
            input.value = editor.isEmpty ? '' : editor.getHTML();
        },
        // Re-checks which toolbar buttons are "on" whenever the selection or content changes.
        onTransaction: () => updateButtons(),
    });

    const commands = {
        bold: () => editor.chain().focus().toggleBold().run(),
        italic: () => editor.chain().focus().toggleItalic().run(),
        underline: () => editor.chain().focus().toggleUnderline().run(),
        strike: () => editor.chain().focus().toggleStrike().run(),
        link: () => setLink(editor),
        h2: () => editor.chain().focus().toggleHeading({ level: 2 }).run(),
        h3: () => editor.chain().focus().toggleHeading({ level: 3 }).run(),
        alignLeft: () => editor.chain().focus().setTextAlign('left').run(),
        alignCenter: () => editor.chain().focus().setTextAlign('center').run(),
        alignRight: () => editor.chain().focus().setTextAlign('right').run(),
        bulletList: () => editor.chain().focus().toggleBulletList().run(),
        orderedList: () => editor.chain().focus().toggleOrderedList().run(),
        blockquote: () => editor.chain().focus().toggleBlockquote().run(),
        image: () => imageInput.click(),
        undo: () => editor.chain().focus().undo().run(),
        redo: () => editor.chain().focus().redo().run(),
    };

    const isActive = {
        bold: () => editor.isActive('bold'),
        italic: () => editor.isActive('italic'),
        underline: () => editor.isActive('underline'),
        strike: () => editor.isActive('strike'),
        link: () => editor.isActive('link'),
        h2: () => editor.isActive('heading', { level: 2 }),
        h3: () => editor.isActive('heading', { level: 3 }),
        alignLeft: () => editor.isActive({ textAlign: 'left' }),
        alignCenter: () => editor.isActive({ textAlign: 'center' }),
        alignRight: () => editor.isActive({ textAlign: 'right' }),
        bulletList: () => editor.isActive('bulletList'),
        orderedList: () => editor.isActive('orderedList'),
        blockquote: () => editor.isActive('blockquote'),
    };

    function updateButtons() {
        for (const button of buttons) {
            const check = isActive[button.dataset.command];
            if (check) {
                button.setAttribute('aria-pressed', String(check()));
            }
        }
    }

    for (const button of buttons) {
        button.addEventListener('click', () => commands[button.dataset.command]?.());
    }

    imageInput.addEventListener('change', async () => {
        const file = imageInput.files[0];
        imageInput.value = '';
        if (!file) {
            return;
        }

        root.dataset.uploading = 'true';
        try {
            const url = await uploadImage(file);
            editor.chain().focus().setImage({ src: url, alt: '' }).run();
        } catch (error) {
            window.adminToast?.(error.message, 'error');
        } finally {
            delete root.dataset.uploading;
        }
    });

    updateButtons();
}

function setLink(editor) {
    const previous = editor.getAttributes('link').href;
    const url = window.prompt('Внесете URL на линкот', previous ?? 'https://');

    if (url === null) {
        return;
    }
    if (url.trim() === '') {
        editor.chain().focus().extendMarkRange('link').unsetLink().run();
        return;
    }
    editor.chain().focus().extendMarkRange('link').setLink({ href: url.trim() }).run();
}

async function uploadImage(file) {
    const body = new FormData();
    body.append('file', file);

    const response = await fetch('/admin/rich-content-attachments', {
        method: 'POST',
        body,
        credentials: 'same-origin',
        headers: { Accept: 'application/json', 'X-CSRF-Token': csrfToken() },
    });
    const payload = await response.json().catch(() => ({}));

    if (!response.ok) {
        throw new Error(payload.errors?.file?.[0] ?? 'Сликата не можеше да се прикачи.');
    }

    return payload.url;
}

document.querySelectorAll('[data-rich-editor]').forEach(setUp);
