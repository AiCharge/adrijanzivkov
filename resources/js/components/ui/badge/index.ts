import type { VariantProps } from 'class-variance-authority';
import { cva } from 'class-variance-authority';

export { default as Badge } from './Badge.vue';

export const badgeVariants = cva(
    'inline-flex w-fit shrink-0 items-center justify-center gap-1 overflow-hidden rounded-md border px-2 py-0.5 text-xs font-medium whitespace-nowrap transition-[color,box-shadow] focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 [&>svg]:pointer-events-none [&>svg]:size-3',
    {
        variants: {
            variant: {
                default:
                    'border-transparent bg-primary text-primary-foreground [a&]:hover:bg-primary/90',
                secondary:
                    'border-transparent bg-secondary text-secondary-foreground [a&]:hover:bg-secondary/90',
                destructive:
                    'border-transparent bg-destructive text-white focus-visible:ring-destructive/20 dark:bg-destructive/60 dark:focus-visible:ring-destructive/40 [a&]:hover:bg-destructive/90',
                outline:
                    'text-foreground [a&]:hover:bg-accent [a&]:hover:text-accent-foreground',
                // The colours of App\Enums\Color: a badge's colour by what it means, sent by the server.
                success:
                    'border-emerald-500/30 bg-emerald-500/10 text-emerald-700 dark:text-emerald-400',
                warning:
                    'border-amber-500/30 bg-amber-500/10 text-amber-700 dark:text-amber-400',
                danger: 'border-red-500/30 bg-red-500/10 text-red-700 dark:text-red-400',
                info: 'border-sky-500/30 bg-sky-500/10 text-sky-700 dark:text-sky-400',
                neutral: 'border-border bg-muted text-muted-foreground',
                in: 'border-teal-500/30 bg-teal-500/10 text-teal-700 dark:text-teal-400',
                out: 'border-violet-500/30 bg-violet-500/10 text-violet-700 dark:text-violet-400',
            },
        },
        defaultVariants: {
            variant: 'default',
        },
    },
);
export type BadgeVariants = VariantProps<typeof badgeVariants>;
