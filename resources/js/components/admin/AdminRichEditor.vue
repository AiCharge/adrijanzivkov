<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { store as storeAttachment } from '@/actions/App/Http/Controllers/Admin/RichContentAttachmentController';
import { cn } from '@/lib/utils';
import {
    AlignCenter,
    AlignLeft,
    AlignRight,
    Bold,
    Heading2,
    Heading3,
    ImagePlus,
    Italic,
    Link2,
    List,
    ListOrdered,
    Loader2,
    Quote,
    Redo2,
    Strikethrough,
    Underline,
    Undo2,
} from '@lucide/vue';
import { Image } from '@tiptap/extension-image';
import { TextAlign } from '@tiptap/extension-text-align';
import StarterKit from '@tiptap/starter-kit';
import { EditorContent, useEditor } from '@tiptap/vue-3';
import type { Component } from 'vue';
import { onBeforeUnmount, ref, watch } from 'vue';
import { toast } from 'vue-sonner';

const props = defineProps<{
    id?: string;
    invalid?: boolean;
}>();

const model = defineModel<string | null>({ required: true });

const imageInput = ref<HTMLInputElement | null>(null);
const isUploading = ref(false);

const editor = useEditor({
    content: model.value ?? '',
    extensions: [
        StarterKit.configure({
            heading: { levels: [2, 3] },
            trailingNode: false,
            link: { openOnClick: false, autolink: true, defaultProtocol: 'https' },
        }),
        Image,
        TextAlign.configure({ types: ['heading', 'paragraph'] }),
    ],
    editorProps: {
        attributes: {
            id: props.id ?? '',
            class: 'admin-rich-content min-h-64 px-3 py-2.5 text-sm outline-none',
        },
    },
    onUpdate: ({ editor }) => {
        model.value = editor.isEmpty ? '' : editor.getHTML();
    },
});

watch(model, (value) => {
    if (!editor.value) {
        return;
    }

    const html = editor.value.isEmpty ? '' : editor.value.getHTML();

    if ((value ?? '') !== html) {
        editor.value.commands.setContent(value ?? '', { emitUpdate: false });
    }
});

onBeforeUnmount(() => editor.value?.destroy());

function setLink(): void {
    if (!editor.value) {
        return;
    }

    const previousUrl = editor.value.getAttributes('link').href as string | undefined;
    const url = window.prompt('Внесете URL на линкот', previousUrl ?? 'https://');

    if (url === null) {
        return;
    }

    if (url.trim() === '') {
        editor.value.chain().focus().extendMarkRange('link').unsetLink().run();

        return;
    }

    editor.value.chain().focus().extendMarkRange('link').setLink({ href: url.trim() }).run();
}

function xsrfToken(): string {
    const cookie = document.cookie.split('; ').find((row) => row.startsWith('XSRF-TOKEN='));

    return cookie ? decodeURIComponent(cookie.split('=')[1]) : '';
}

async function uploadImage(event: Event): Promise<void> {
    const target = event.target as HTMLInputElement;
    const file = target.files?.[0];
    target.value = '';

    if (!file || !editor.value) {
        return;
    }

    const body = new FormData();
    body.append('file', file);

    isUploading.value = true;

    try {
        const response = await fetch(storeAttachment.url(), {
            method: 'POST',
            body,
            credentials: 'same-origin',
            headers: { Accept: 'application/json', 'X-XSRF-TOKEN': xsrfToken() },
        });
        const payload = await response.json();

        if (!response.ok) {
            toast.error(payload.errors?.file?.[0] ?? 'Сликата не можеше да се прикачи.');

            return;
        }

        editor.value.chain().focus().setImage({ src: payload.url, alt: '' }).run();
    } catch {
        toast.error('Сликата не можеше да се прикачи.');
    } finally {
        isUploading.value = false;
    }
}

interface ToolbarButton {
    icon: Component;
    title: string;
    isActive?: () => boolean;
    isDisabled?: () => boolean;
    run: () => void;
}

const toolbarGroups: ToolbarButton[][] = [
    [
        {
            icon: Bold,
            title: 'Задебелено',
            isActive: () => !!editor.value?.isActive('bold'),
            run: () => editor.value?.chain().focus().toggleBold().run(),
        },
        {
            icon: Italic,
            title: 'Закосено',
            isActive: () => !!editor.value?.isActive('italic'),
            run: () => editor.value?.chain().focus().toggleItalic().run(),
        },
        {
            icon: Underline,
            title: 'Подвлечено',
            isActive: () => !!editor.value?.isActive('underline'),
            run: () => editor.value?.chain().focus().toggleUnderline().run(),
        },
        {
            icon: Strikethrough,
            title: 'Прецртано',
            isActive: () => !!editor.value?.isActive('strike'),
            run: () => editor.value?.chain().focus().toggleStrike().run(),
        },
        { icon: Link2, title: 'Линк', isActive: () => !!editor.value?.isActive('link'), run: setLink },
    ],
    [
        {
            icon: Heading2,
            title: 'Наслов 2',
            isActive: () => !!editor.value?.isActive('heading', { level: 2 }),
            run: () => editor.value?.chain().focus().toggleHeading({ level: 2 }).run(),
        },
        {
            icon: Heading3,
            title: 'Наслов 3',
            isActive: () => !!editor.value?.isActive('heading', { level: 3 }),
            run: () => editor.value?.chain().focus().toggleHeading({ level: 3 }).run(),
        },
        {
            icon: AlignLeft,
            title: 'Порамни лево',
            isActive: () => !!editor.value?.isActive({ textAlign: 'left' }),
            run: () => editor.value?.chain().focus().setTextAlign('left').run(),
        },
        {
            icon: AlignCenter,
            title: 'Центрирај',
            isActive: () => !!editor.value?.isActive({ textAlign: 'center' }),
            run: () => editor.value?.chain().focus().setTextAlign('center').run(),
        },
        {
            icon: AlignRight,
            title: 'Порамни десно',
            isActive: () => !!editor.value?.isActive({ textAlign: 'right' }),
            run: () => editor.value?.chain().focus().setTextAlign('right').run(),
        },
    ],
    [
        {
            icon: List,
            title: 'Листа',
            isActive: () => !!editor.value?.isActive('bulletList'),
            run: () => editor.value?.chain().focus().toggleBulletList().run(),
        },
        {
            icon: ListOrdered,
            title: 'Нумерирана листа',
            isActive: () => !!editor.value?.isActive('orderedList'),
            run: () => editor.value?.chain().focus().toggleOrderedList().run(),
        },
        {
            icon: Quote,
            title: 'Цитат',
            isActive: () => !!editor.value?.isActive('blockquote'),
            run: () => editor.value?.chain().focus().toggleBlockquote().run(),
        },
        { icon: ImagePlus, title: 'Прикачи слика', isDisabled: () => isUploading.value, run: () => imageInput.value?.click() },
    ],
    [
        { icon: Undo2, title: 'Врати', run: () => editor.value?.chain().focus().undo().run() },
        { icon: Redo2, title: 'Повтори', run: () => editor.value?.chain().focus().redo().run() },
    ],
];
</script>

<template>
    <div
        :class="
            cn(
                'border-input focus-within:border-ring focus-within:ring-ring/50 dark:bg-input/30 overflow-hidden rounded-md border bg-transparent shadow-xs transition-[color,box-shadow] focus-within:ring-3',
                invalid && 'border-destructive ring-destructive/20 dark:ring-destructive/40',
            )
        "
    >
        <div class="bg-muted/50 flex flex-wrap items-center gap-0.5 border-b px-1.5 py-1">
            <template v-for="(group, groupIndex) in toolbarGroups" :key="groupIndex">
                <span v-if="groupIndex > 0" class="bg-border mx-1 h-5 w-px" aria-hidden="true" />
                <Button
                    v-for="button in group"
                    :key="button.title"
                    type="button"
                    variant="ghost"
                    size="icon-sm"
                    :title="button.title"
                    :aria-label="button.title"
                    :aria-pressed="button.isActive ? button.isActive() : undefined"
                    :disabled="!editor || button.isDisabled?.()"
                    :class="cn('text-muted-foreground', button.isActive?.() && 'bg-accent text-primary')"
                    @click="button.run"
                >
                    <Loader2 v-if="button.isDisabled?.()" class="animate-spin" />
                    <component :is="button.icon" v-else />
                </Button>
            </template>
        </div>
        <EditorContent :editor="editor" />
        <input ref="imageInput" type="file" accept="image/*" class="sr-only" tabindex="-1" @change="uploadImage" />
    </div>
</template>

<style scoped>
:deep(.admin-rich-content p) {
    margin-bottom: 0.75rem;
    line-height: 1.7;
}

:deep(.admin-rich-content h2) {
    margin-top: 1.25rem;
    margin-bottom: 0.75rem;
    font-size: 1.375rem;
    font-weight: 700;
    color: var(--foreground);
}

:deep(.admin-rich-content h3) {
    margin-top: 1rem;
    margin-bottom: 0.5rem;
    font-size: 1.125rem;
    font-weight: 700;
    color: var(--foreground);
}

:deep(.admin-rich-content ul),
:deep(.admin-rich-content ol) {
    margin-bottom: 0.75rem;
    padding-left: 1.5rem;
}

:deep(.admin-rich-content ul) {
    list-style: disc;
}

:deep(.admin-rich-content ol) {
    list-style: decimal;
}

:deep(.admin-rich-content blockquote) {
    margin-bottom: 0.75rem;
    border-left: 3px solid var(--border);
    padding-left: 1rem;
    color: var(--muted-foreground);
    font-style: italic;
}

:deep(.admin-rich-content a) {
    color: var(--primary);
    text-decoration: underline;
}

:deep(.admin-rich-content img) {
    margin: 1rem 0;
    max-height: 24rem;
    border-radius: 0.5rem;
}

:deep(.admin-rich-content img.ProseMirror-selectednode) {
    outline: 3px solid color-mix(in oklch, var(--ring) 50%, transparent);
}
</style>
